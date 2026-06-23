//! Archive completed tasks from `knowledge-tasks/<REQ-ID>/` into `.enjoyknowledge/`.
//!
//! The archive process:
//! 1. Find completed task directories (containing `status: completed` marker)
//! 2. Extract `##` entries from task markdown files
//! 3. Map entries to appropriate `.enjoyknowledge/` category by filename heuristics
//! 4. Append entries to target knowledge files
//! 5. Move the task directory to `knowledge-tasks/.archive/<REQ-ID>/`

use crate::format;
use std::path::Path;

/// Result of archiving a single task directory.
pub struct ArchiveResult {
    /// Task directory name that was archived.
    pub task_id: String,
    /// Number of `##` entries extracted.
    pub entries_archived: usize,
    /// Files that received new entries (relative to `.enjoyknowledge/`).
    pub targets: Vec<String>,
}

/// Archive all completed tasks under `knowledge-tasks/`.
///
/// A task is considered "completed" if any `.md` file in its directory
/// contains the string `status: completed`.
pub fn archive_completed_tasks(
    knowledge_root: &Path,
    project_root: &Path,
    req: Option<&str>,
) -> anyhow::Result<Vec<ArchiveResult>> {
    let tasks_dir = project_root.join("knowledge-tasks");
    if !tasks_dir.exists() {
        return Ok(Vec::new());
    }

    let archive_dir = tasks_dir.join(".archive");
    let mut results = Vec::new();

    // If req is specified, only process that specific task
    if let Some(req_id) = req {
        let task_path = tasks_dir.join(req_id);
        if !task_path.is_dir() {
            eprintln!("enjoyknowledge: task directory '{req_id}' not found in knowledge-tasks/");
            return Ok(Vec::new());
        }
        if !is_task_completed(&task_path) {
            eprintln!("enjoyknowledge: task '{req_id}' is not marked as completed (needs 'status: completed' in a .md file)");
            return Ok(Vec::new());
        }
        if let Ok(result) = archive_single_task(knowledge_root, &task_path, &archive_dir) {
            results.push(result);
        }
        return Ok(results);
    }

    // Collect task directories (skip .archive and hidden dirs)
    for entry in std::fs::read_dir(&tasks_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }

        if is_task_completed(&path) {
            if let Ok(result) = archive_single_task(knowledge_root, &path, &archive_dir) {
                results.push(result);
            }
        }
    }

    Ok(results)
}

/// Check whether a task directory is completed by looking for `status: completed` marker.
fn is_task_completed(task_dir: &Path) -> bool {
    for entry in walkdir::WalkDir::new(task_dir)
        .max_depth(2)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            if content.contains("status: completed") {
                return true;
            }
        }
    }
    false
}

/// Archive a single task: extract entries and move directory.
fn archive_single_task(
    knowledge_root: &Path,
    task_dir: &Path,
    archive_root: &Path,
) -> anyhow::Result<ArchiveResult> {
    let task_id = task_dir.file_name().unwrap_or_default().to_string_lossy().to_string();
    let mut entries_archived = 0usize;
    let mut targets = Vec::new();

    // Process each .md file in the task directory
    for entry in walkdir::WalkDir::new(task_dir)
        .max_depth(2)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let content = std::fs::read_to_string(entry.path())?;

        // Map file to target category in .enjoyknowledge/
        let file_stem =
            entry.path().file_stem().unwrap_or_default().to_string_lossy().to_lowercase();
        let category = classify_file(&file_stem);

        // Extract `##` entries (skip frontmatter)
        let body_start = format::document::find_body_start(&content);
        let body = &content[body_start..];

        // Find all `##` sections and their content
        let sections = extract_entries(body);
        for section in &sections {
            if section.trim().is_empty() || section.trim().starts_with("---") {
                continue;
            }

            let target_path = format!("{category}/{file_stem}.md");
            let target_full = knowledge_root.join(&target_path);

            // Ensure parent directory exists
            if let Some(parent) = target_full.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Read existing, append, write back
            let mut existing = if target_full.exists() {
                std::fs::read_to_string(&target_full)?
            } else {
                let desc = section.lines().find(|l| l.starts_with("## ")).map_or_else(
                    || "Archived entry".to_string(),
                    |l| l.trim_start_matches("## ").to_string(),
                );
                format::frontmatter::generate_frontmatter(&desc)
            };

            if !existing.ends_with('\n') {
                existing.push('\n');
            }
            existing.push_str(section);
            if !section.ends_with('\n') {
                existing.push('\n');
            }

            std::fs::write(&target_full, &existing)?;
            entries_archived += 1;

            if !targets.contains(&target_path) {
                targets.push(target_path.clone());
            }
        }
    }

    // Move task directory to archive
    let dest = archive_root.join(&task_id);
    // Remove existing archive if any (e.g., re-archive)
    if dest.exists() {
        std::fs::remove_dir_all(&dest)?;
    }
    std::fs::create_dir_all(archive_root)?;
    std::fs::rename(task_dir, &dest)?;

    Ok(ArchiveResult { task_id, entries_archived, targets })
}

/// Heuristic: classify a markdown file into a knowledge category.
///
/// Looks for keywords in the filename to determine the target directory:
/// - gotcha → gotchas/
/// - pattern, convention → patterns/
/// - decision, adr → decisions/
/// - business, rule → business/
/// - default → architecture/
fn classify_file(file_stem: &str) -> &str {
    if file_stem.contains("gotcha") || file_stem.contains("bug") || file_stem.contains("fix") {
        "gotchas"
    } else if file_stem.contains("pattern")
        || file_stem.contains("convention")
        || file_stem.contains("style")
    {
        "patterns"
    } else if file_stem.contains("decision") || file_stem.contains("adr") {
        "decisions"
    } else if file_stem.contains("business")
        || file_stem.contains("rule")
        || file_stem.contains("domain")
    {
        "business"
    } else {
        "architecture"
    }
}

/// Extract `##`-headed sections from markdown body content.
///
/// Each section starts with a `## ` line and runs until the next `## ` line
/// or end of content. `###` sub-headings stay within their parent section.
fn extract_entries(body: &str) -> Vec<String> {
    let mut sections = Vec::new();
    let mut current = String::new();
    let mut in_section = false;

    for line in body.lines() {
        if line.starts_with("## ") && !line.starts_with("### ") {
            // Start new section
            if in_section {
                sections.push(std::mem::take(&mut current));
            }
            current.push_str(line);
            current.push('\n');
            in_section = true;
        } else if in_section {
            current.push_str(line);
            current.push('\n');
        }
    }

    // Don't forget the last section
    if in_section && !current.trim_start().starts_with("## ") {
        // Skip if it's just the heading line
    }
    if in_section {
        sections.push(current);
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_gotcha() {
        assert_eq!(classify_file("gotchas"), "gotchas");
        assert_eq!(classify_file("bug-fixes"), "gotchas");
    }

    #[test]
    fn test_classify_pattern() {
        assert_eq!(classify_file("code-convention"), "patterns");
        assert_eq!(classify_file("style-guide"), "patterns");
    }

    #[test]
    fn test_classify_decision() {
        assert_eq!(classify_file("adr-001"), "decisions");
        assert_eq!(classify_file("architecture-decision"), "decisions");
    }

    #[test]
    fn test_classify_default() {
        assert_eq!(classify_file("notes"), "architecture");
        assert_eq!(classify_file("random"), "architecture");
    }

    #[test]
    fn test_extract_entries() {
        let input = "## Section A\ncontent a\n## Section B\ncontent b\n";
        let sections = extract_entries(input);
        assert_eq!(sections.len(), 2);
        assert!(sections[0].contains("Section A"));
        assert!(sections[1].contains("Section B"));
    }
}
