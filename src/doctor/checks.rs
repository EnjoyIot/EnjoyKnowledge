//! doctor diagnostic checks for the knowledge base.
use crate::knowledge::KnowledgeSource;

/// Severity of a check finding.
#[derive(Debug, Clone)]
pub enum Severity {
    /// Hard failure — blocks correctness.
    Error,
    /// Soft warning — quality issue but not broken.
    Warning,
}

/// A single check result.
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub file: String,
    pub issue: String,
    pub severity: Severity,
}

/// Run all health checks against the knowledge base.
pub fn run_all(source: &dyn KnowledgeSource) -> Vec<CheckResult> {
    let mut results = Vec::new();

    results.extend(check_frontmatter(source));
    results.extend(check_budget(source));
    results.extend(check_missing_description(source));
    results.extend(check_agents_md(source));
    results.extend(check_pending_archive(source));

    results
}

/// Check 1: Every .md file has valid frontmatter.
fn check_frontmatter(source: &dyn KnowledgeSource) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let paths = source.all_entry_paths();
    for rel in &paths {
        match source.read_file(rel) {
            Ok(content) => {
                if crate::format::frontmatter::parse_frontmatter(&content).is_none() {
                    results.push(CheckResult {
                        file: rel.clone(),
                        issue: "missing or invalid frontmatter".to_string(),
                        severity: Severity::Error,
                    });
                }
            }
            Err(_) => {
                results.push(CheckResult {
                    file: rel.clone(),
                    issue: "unreadable file".to_string(),
                    severity: Severity::Error,
                });
            }
        }
    }
    results
}

/// Check 2: File budget — warn if a file has ≥20 ## entries.
fn check_budget(source: &dyn KnowledgeSource) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let paths = source.all_entry_paths();
    for rel in &paths {
        if let Ok(content) = source.read_file(rel) {
            let count = content.lines().filter(|l| l.starts_with("## ")).count();
            if count >= 20 {
                results.push(CheckResult {
                    file: rel.clone(),
                    issue: format!("over 20 limit ({count} entries)"),
                    severity: Severity::Warning,
                });
            }
        }
    }
    results
}

/// Check 3: Missing or empty description in frontmatter.
fn check_missing_description(source: &dyn KnowledgeSource) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let paths = source.all_entry_paths();
    for rel in &paths {
        if let Ok(content) = source.read_file(rel) {
            if let Some(fm) = crate::format::frontmatter::parse_frontmatter(&content) {
                if fm.description.is_none() || fm.description.as_deref() == Some("") {
                    results.push(CheckResult {
                        file: rel.clone(),
                        issue: "missing description".to_string(),
                        severity: Severity::Warning,
                    });
                }
            }
        }
    }
    results
}

/// Check 4: AGENTS.md includes the enjoyknowledge block.
fn check_agents_md(source: &dyn KnowledgeSource) -> Vec<CheckResult> {
    match source.read_agents_md() {
        None => {
            return vec![CheckResult {
                file: "AGENTS.md".to_string(),
                issue: "AGENTS.md not found — run `enjoyknowledge init` to create".to_string(),
                severity: Severity::Warning,
            }];
        }
        Some(content) => {
            if !content.contains("enjoyknowledge") {
                return vec![CheckResult {
                    file: "AGENTS.md".to_string(),
                    issue: "AGENTS.md missing enjoyknowledge section — run `enjoyknowledge init` to regenerate".to_string(),
                    severity: Severity::Warning,
                }];
            }
        }
    }
    Vec::new()
}

/// Check 5: Completed tasks in knowledge-tasks/ that are pending archive.
fn check_pending_archive(source: &dyn KnowledgeSource) -> Vec<CheckResult> {
    let mut results = Vec::new();

    for (rel, content) in source.list_knowledge_tasks() {
        if content.contains("status: completed") && !rel.contains("archive") {
            results.push(CheckResult {
                file: rel,
                issue: "pending archive (completed task not in archive)".to_string(),
                severity: Severity::Warning,
            });
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::FilesystemSource;

    fn make_source(project_root: &std::path::Path) -> FilesystemSource {
        let ek = project_root.join(".enjoyknowledge");
        std::fs::create_dir_all(&ek).unwrap();
        FilesystemSource::new(ek, project_root)
    }

    // ── check_agents_md ──

    #[test]
    fn agents_md_absent() {
        let dir = tempfile::TempDir::new().unwrap();
        let source = make_source(dir.path());
        let results = check_agents_md(&source);
        assert_eq!(results.len(), 1);
        assert!(results[0].issue.contains("not found"));
    }

    #[test]
    fn agents_md_missing_enjoyknowledge_section() {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::write(dir.path().join("AGENTS.md"), "# Project\n\njust random content\n").unwrap();
        let source = make_source(dir.path());
        let results = check_agents_md(&source);
        assert_eq!(results.len(), 1);
        assert!(results[0].issue.contains("missing enjoyknowledge"));
    }

    #[test]
    fn agents_md_has_enjoyknowledge_section() {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::write(dir.path().join("AGENTS.md"), "# Project\n\nenjoyknowledge setup here\n")
            .unwrap();
        let source = make_source(dir.path());
        let results = check_agents_md(&source);
        assert!(results.is_empty());
    }

    // ── check_pending_archive ──

    #[test]
    fn pending_archive_completed_task() {
        let dir = tempfile::TempDir::new().unwrap();
        let task_dir = dir.path().join("knowledge-tasks").join("test-req");
        std::fs::create_dir_all(&task_dir).unwrap();
        std::fs::write(task_dir.join("notes.md"), "status: completed\n\n## Done\nall good.\n")
            .unwrap();
        let source = make_source(dir.path());
        let results = check_pending_archive(&source);
        assert_eq!(results.len(), 1);
        assert!(results[0].issue.contains("pending archive"));
        assert!(results[0].file.contains("test-req"));
    }

    #[test]
    fn pending_archive_not_completed() {
        let dir = tempfile::TempDir::new().unwrap();
        let task_dir = dir.path().join("knowledge-tasks").join("test-req");
        std::fs::create_dir_all(&task_dir).unwrap();
        std::fs::write(task_dir.join("notes.md"), "status: in-progress\n\n## Doing\nworking.\n")
            .unwrap();
        let source = make_source(dir.path());
        let results = check_pending_archive(&source);
        assert!(results.is_empty());
    }

    #[test]
    fn pending_archive_skips_archived() {
        let dir = tempfile::TempDir::new().unwrap();
        let task_dir = dir.path().join("knowledge-tasks").join(".archive").join("old-req");
        std::fs::create_dir_all(&task_dir).unwrap();
        std::fs::write(task_dir.join("notes.md"), "status: completed\n\n## Archived entry\n")
            .unwrap();
        let source = make_source(dir.path());
        let results = check_pending_archive(&source);
        assert!(results.is_empty());
    }

    #[test]
    fn pending_archive_no_knowledge_tasks_dir() {
        let dir = tempfile::TempDir::new().unwrap();
        let source = make_source(dir.path());
        let results = check_pending_archive(&source);
        assert!(results.is_empty());
    }
}
