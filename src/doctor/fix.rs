//! Auto-fix logic: repair programmatically resolvable issues.
//!
//! Fixable issues:
//! - Missing descriptions (filled from first `##` heading)
//! - File budget exceeded (oldest entries moved to archive files)
//! - Completed tasks pending archive (extracted to `.enjoyknowledge/`)

use crate::doctor;
use crate::doctor::checks;
use crate::knowledge::{FilesystemSource, KnowledgeSource};
use std::path::Path;

/// Fill a missing `description` field in frontmatter while preserving all other fields.
/// Returns the new content, or the original if description already exists.
fn fill_missing_description(content: &str, desc: &str) -> String {
    let body_start = crate::format::document::find_body_start(content);
    let leading_ws_len = content.len() - content.trim_start().len();

    // No frontmatter at all → generate one
    if body_start == leading_ws_len {
        let fm = crate::format::frontmatter::generate_frontmatter(desc);
        return format!("{fm}{}", &content[leading_ws_len..]);
    }

    let fm_block = &content[..body_start];
    let body = &content[body_start..];

    // Already has description → unchanged
    if fm_block.lines().any(|l| l.trim_start().starts_with("description:")) {
        return content.to_string();
    }

    // Detect newline style from existing frontmatter
    let nl = if fm_block.contains("\r\n") { "\r\n" } else { "\n" };
    let leading_ws = &content[..leading_ws_len];

    // Locate opening "---" within fm_block and skip "---<nl>"
    let open_offset = fm_block.find("---").unwrap_or(0);
    let fm_inner = &fm_block[open_offset + 3 + nl.len()..];

    // Locate closing "<nl>---" within fm_inner
    let close_marker = format!("{nl}---");
    fm_inner.rfind(&close_marker).map_or_else(
        || content.to_string(),
        |close_start| {
            let inner = &fm_inner[..close_start];
            format!("{leading_ws}---{nl}{inner}{nl}description: {desc}{nl}---{nl}{body}")
        },
    )
}

/// v0.2.1 diagnostic: scan for files with valid frontmatter but missing description.
/// This runs independently of `checks::run_all` because v0.2 checks don't yet
/// produce "missing description" diagnostics (v0.3 refactor will unify this).
fn scan_missing_descriptions(source: &FilesystemSource) -> Vec<String> {
    let mut files = Vec::new();
    for rel in &source.all_entry_paths() {
        if let Ok(content) = source.read_file(rel) {
            if let Some(fm) = crate::format::frontmatter::parse_frontmatter(&content) {
                if fm.description.is_none() || fm.description.as_deref() == Some("") {
                    files.push(rel.clone());
                }
            }
        }
    }
    files
}

/// Run auto-fix for the knowledge base, optionally scoped to a specific task.
pub fn run_fix(
    source: &FilesystemSource,
    project_root: &Path,
    req: Option<&str>,
) -> anyhow::Result<()> {
    let violations = checks::run_all(source, project_root);

    // v0.2.1: also scan for missing descriptions independently (v0.2 checks don't
    // produce "missing description" diagnostics yet — v0.3 refactor will unify).
    let missing_desc_from_scan = scan_missing_descriptions(source);

    if violations.is_empty() && missing_desc_from_scan.is_empty() {
        eprintln!("enjoyknowledge: nothing to fix — all checks pass");
        return Ok(());
    }

    // Group violations by category
    let mut budget_files: Vec<String> = Vec::new();
    let mut missing_desc: Vec<String> = missing_desc_from_scan;
    let mut has_agents_issue = false;
    let mut has_pending_archive = false;

    for v in &violations {
        if v.issue.contains("over 20 limit") {
            budget_files.push(v.file.clone());
        } else if v.issue.contains("missing description") {
            missing_desc.push(v.file.clone());
        } else if v.file == "AGENTS.md" {
            has_agents_issue = true;
        } else if v.issue.contains("pending archive") {
            has_pending_archive = true;
        }
    }

    // Deduplicate missing_desc
    missing_desc.sort();
    missing_desc.dedup();

    // Fix 1: Fill missing descriptions (field-merge, preserves existing frontmatter)
    for rel in &missing_desc {
        if let Ok(content) = source.read_file(rel) {
            if let Some(fm) = crate::format::frontmatter::parse_frontmatter(&content) {
                if fm.description.is_none() || fm.description.as_deref() == Some("") {
                    let body_start = crate::format::document::find_body_start(&content);
                    let body = &content[body_start..];
                    let first_heading = body
                        .lines()
                        .find(|l| l.starts_with("## "))
                        .map(|l| l.trim_start_matches("## ").to_string());

                    let desc = first_heading.unwrap_or_else(|| "TODO: add description".to_string());

                    let new_content = fill_missing_description(&content, &desc);
                    if new_content != content {
                        std::fs::write(source.root.join(rel), new_content)?;
                        eprintln!("enjoyknowledge: filled description for {rel}");
                    }
                }
            }
        }
    }

    // Fix 2: Archive over-budget files
    for rel in &budget_files {
        doctor::budget::archive_old_entries(source, rel)?;
    }

    // Fix 3: Archive completed tasks
    if has_pending_archive {
        let results = doctor::archive::archive_completed_tasks(&source.root, project_root, req)?;
        for r in &results {
            eprintln!(
                "enjoyknowledge: archived {} ({} entries → {})",
                r.task_id,
                r.entries_archived,
                r.targets.join(", ")
            );
        }
        if results.is_empty() {
            eprintln!("enjoyknowledge: no completed tasks found for archiving");
        }
    }

    // Fix 4: Regenerate AGENTS.md summary
    if has_agents_issue {
        // Regenerate AGENTS.md
        let profile = crate::init::resolve_profile("for-coding")
            .unwrap_or_else(|| Box::new(crate::profile::coding::CodingProfile));
        crate::init::skeleton::generate_agents_md(project_root, None, profile.as_ref())?;
        eprintln!("enjoyknowledge: regenerated AGENTS.md");
    }

    // Always sync summary after fixes
    crate::init::skeleton::sync_agents_md_summary(project_root, source)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_missing_description_preserves_4_fields() {
        // v0.4.1 reproduce: promote writes 4 fields + no description
        let input = "---\nid: abc-123\nkind: gotcha\ncreated: 2026-06-20\nauthor: enjoy\n---\n\n## Export Bug\nSome body text\n";
        let result = fill_missing_description(input, "Export Bug");
        assert!(result.contains("id: abc-123"), "id field preserved");
        assert!(result.contains("kind: gotcha"), "kind field preserved");
        assert!(result.contains("created: 2026-06-20"), "created field preserved");
        assert!(result.contains("author: enjoy"), "author field preserved");
        assert!(result.contains("description: Export Bug"), "description added");
        assert!(result.contains("## Export Bug"), "body preserved");
    }

    #[test]
    fn test_fill_missing_description_already_present_skips() {
        let input =
            "---\ndescription: already here\ntimestamp: 2026-06-20\n---\n\n## Title\nbody\n";
        let result = fill_missing_description(input, "new desc");
        assert_eq!(result, input, "unchanged when description exists");
    }

    #[test]
    fn test_fill_missing_description_extra_fields_preserved() {
        let input = "---\nid: x-1\nkind: decision\ncreated: 2026-01-01\nauthor: enjoy\ntrigger: bug report\napplies_to: all platforms\n---\n\n## Decision Title\ncontent\n";
        let result = fill_missing_description(input, "Decision Title");
        assert!(result.contains("id: x-1"));
        assert!(result.contains("kind: decision"));
        assert!(result.contains("trigger: bug report"));
        assert!(result.contains("applies_to: all platforms"));
        assert!(result.contains("description: Decision Title"));
    }
}
