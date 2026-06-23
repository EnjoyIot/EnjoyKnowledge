//! doctor diagnostic checks for the knowledge base.
use crate::knowledge::source::KnowledgeSource;
use crate::knowledge::FilesystemSource;
use std::path::Path;

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
pub fn run_all(source: &FilesystemSource, _project_root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    results.extend(check_frontmatter(source));
    results.extend(check_budget(source));
    results.extend(check_missing_description(source));
    results.extend(check_agents_md(source));
    results.extend(check_pending_archive(source));

    results
}

/// Check 1: Every .md file has valid frontmatter.
fn check_frontmatter(source: &FilesystemSource) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let files = source.walk_md_files(None);
    for (_abs, rel) in &files {
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
fn check_budget(source: &FilesystemSource) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let files = source.walk_md_files(None);
    for (_abs, rel) in &files {
        if let Ok(content) = source.read_file(rel) {
            let count =
                content.lines().filter(|l| l.starts_with("## ") && !l.starts_with("### ")).count();
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
fn check_missing_description(source: &FilesystemSource) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let files = source.walk_md_files(None);
    for (_abs, rel) in &files {
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
fn check_agents_md(source: &FilesystemSource) -> Vec<CheckResult> {
    let agents_path = source.project_root.join("AGENTS.md");
    if !agents_path.exists() {
        return vec![CheckResult {
            file: "AGENTS.md".to_string(),
            issue: "AGENTS.md not found — run `enjoyknowledge init` to create".to_string(),
            severity: Severity::Warning,
        }];
    }
    match std::fs::read_to_string(&agents_path) {
        Ok(content) => {
            if !content.contains("enjoyknowledge") {
                return vec![CheckResult {
                    file: "AGENTS.md".to_string(),
                    issue: "AGENTS.md missing enjoyknowledge section — run `enjoyknowledge init` to regenerate".to_string(),
                    severity: Severity::Warning,
                }];
            }
        }
        Err(_) => {
            return vec![CheckResult {
                file: "AGENTS.md".to_string(),
                issue: "unreadable file".to_string(),
                severity: Severity::Error,
            }];
        }
    }
    Vec::new()
}

/// Check 5: Completed tasks in knowledge-tasks/ that are pending archive.
fn check_pending_archive(source: &FilesystemSource) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let tasks_dir = source.project_root.join("knowledge-tasks");
    if !tasks_dir.exists() {
        return results;
    }

    let walker = walkdir::WalkDir::new(&tasks_dir)
        .max_depth(3)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"));

    for entry in walker {
        let rel = entry
            .path()
            .strip_prefix(&source.project_root)
            .unwrap_or_else(|_| entry.path())
            .to_string_lossy()
            .replace('\\', "/");
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            if content.contains("status: completed") && !rel.contains("archive") {
                results.push(CheckResult {
                    file: rel,
                    issue: "pending archive (completed task not in archive)".to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }
    results
}
