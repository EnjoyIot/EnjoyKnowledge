//! doctor diagnostic checks for the knowledge base.
//! v0.2: 4 checks per GLOSSARY L51 —
//!   missing frontmatter / missing required fields / `SoT` staleness / export consistency.
use crate::knowledge::KnowledgeSource;
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

/// Run all 4 v0.2 health checks against the knowledge base (GLOSSARY L51).
pub fn run_all(source: &dyn KnowledgeSource, project_root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    results.extend(check_frontmatter(source));
    results.extend(check_required_fields(source));
    results.extend(check_sot_staleness(source));
    results.extend(check_export_consistency(source, project_root));
    results
}

// ── Check 1: Every .md file has valid YAML frontmatter ──────────────────────

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

// ── Check 2: Required fields per knowledge kind ─────────────────────────────

fn check_required_fields(source: &dyn KnowledgeSource) -> Vec<CheckResult> {
    let mut results = Vec::new();
    for rel in &source.all_entry_paths() {
        let Some(kind) = kind_from_path(rel) else {
            continue;
        };
        let Ok(content) = source.read_file(rel) else {
            continue;
        };
        let Some(keys) = parse_raw_frontmatter_keys(&content) else {
            continue;
        };

        match kind {
            "gotcha" => {
                if !keys.iter().any(|k| k == "trigger") {
                    results.push(CheckResult {
                        file: rel.clone(),
                        issue: "gotcha missing required field 'trigger'".to_string(),
                        severity: Severity::Error,
                    });
                }
            }
            "rule" | "contract" | "convention" | "template" => {
                if !keys.iter().any(|k| k == "applies_to") {
                    results.push(CheckResult {
                        file: rel.clone(),
                        issue: format!("{kind} missing required field 'applies_to'"),
                        severity: Severity::Error,
                    });
                }
            }
            "decision" => {
                if !keys.iter().any(|k| k == "reversible") {
                    results.push(CheckResult {
                        file: rel.clone(),
                        issue: "decision missing required field 'reversible'".to_string(),
                        severity: Severity::Error,
                    });
                }
                if !keys.iter().any(|k| k == "decided_at") {
                    results.push(CheckResult {
                        file: rel.clone(),
                        issue: "decision missing required field 'decided_at'".to_string(),
                        severity: Severity::Error,
                    });
                }
            }
            _ => {}
        }
    }
    results
}

// ── Check 3: SoT staleness — warn if timestamp > 180 days ───────────────────

fn check_sot_staleness(source: &dyn KnowledgeSource) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let today = chrono::Local::now().date_naive();
    for rel in &source.all_entry_paths() {
        let Ok(content) = source.read_file(rel) else {
            continue;
        };
        let Some(fm) = crate::format::frontmatter::parse_frontmatter(&content) else {
            continue;
        };
        let Some(ref ts) = fm.timestamp else {
            continue;
        };
        let Ok(date) = chrono::NaiveDate::parse_from_str(ts, "%Y-%m-%d") else {
            continue;
        };
        let days = (today - date).num_days();
        if days > 180 {
            results.push(CheckResult {
                file: rel.clone(),
                issue: format!("timestamp is {days} days old — consider reviewing"),
                severity: Severity::Warning,
            });
        }
    }
    results
}

// ── Check 4: Multi-tool export consistency ──────────────────────────────────

fn check_export_consistency(
    _source: &dyn KnowledgeSource,
    project_root: &Path,
) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let claude = project_root.join(".claude/skills/enjoyknowledge.md");
    let cursor = project_root.join(".cursor/rules/enjoyknowledge.mdc");

    let has_claude = claude.exists();
    let has_cursor = cursor.exists();

    match (has_claude, has_cursor) {
        (true, true) | (false, false) => {} // both present or both absent = OK
        (true, false) => results.push(CheckResult {
            file: ".cursor/rules/enjoyknowledge.mdc".to_string(),
            issue: "Claude export present but Cursor export missing — run `enjoyknowledge export cursor`".to_string(),
            severity: Severity::Warning,
        }),
        (false, true) => results.push(CheckResult {
            file: ".claude/skills/enjoyknowledge.md".to_string(),
            issue: "Cursor export present but Claude export missing — run `enjoyknowledge export claude`".to_string(),
            severity: Severity::Warning,
        }),
    }
    results
}

// ── helpers ─────────────────────────────────────────────────────────────────

/// Determine knowledge kind from the file's top-level directory.
/// "gotchas/x.md" → "gotcha", "rules/x.md" → "rule", etc.
fn kind_from_path(path: &str) -> Option<&'static str> {
    let top_dir = path.split('/').next()?;
    match top_dir {
        "gotchas" => Some("gotcha"),
        "rules" => Some("rule"),
        "decisions" => Some("decision"),
        "contracts" => Some("contract"),
        "conventions" => Some("convention"),
        "templates" => Some("template"),
        _ => None,
    }
}

/// Parse raw YAML frontmatter and return the top-level key names.
/// Returns `None` if the frontmatter is missing or unparseable.
fn parse_raw_frontmatter_keys(content: &str) -> Option<Vec<String>> {
    let content = content.trim_start();
    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return None;
    }
    let after_first = &content[3..];
    let end = after_first.find("\n---").or_else(|| after_first.find("\r\n---"))?;
    let yaml_str = &after_first[..end];
    let value: serde_yaml::Value = serde_yaml::from_str(yaml_str).ok()?;
    let mapping = value.as_mapping()?;
    Some(mapping.keys().filter_map(|k| k.as_str().map(String::from)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::knowledge::FilesystemSource;

    fn make_source(project_root: &Path) -> FilesystemSource {
        let ek = project_root.join(".enjoyknowledge");
        std::fs::create_dir_all(&ek).unwrap();
        FilesystemSource::new(ek, project_root)
    }

    /// Write a file under `.enjoyknowledge/` in the given project root.
    fn write_ek(project_root: &Path, rel_path: &str, content: &str) {
        let full = project_root.join(".enjoyknowledge").join(rel_path);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(full, content).unwrap();
    }

    // ── check_frontmatter ───────────────────────────────────────────────

    #[test]
    fn frontmatter_valid() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(
            dir.path(),
            "ok.md",
            "---\ndescription: hi\ntags: [a]\ntimestamp: 2026-06-28\n---\n\n# Body\n",
        );
        let source = make_source(dir.path());
        let results = check_frontmatter(&source);
        assert!(results.is_empty(), "expected no errors, got {results:?}");
    }

    #[test]
    fn frontmatter_missing() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(dir.path(), "nofm.md", "# No frontmatter\n");
        let source = make_source(dir.path());
        let results = check_frontmatter(&source);
        assert_eq!(results.len(), 1);
        assert!(results[0].issue.contains("missing or invalid"));
    }

    #[test]
    fn frontmatter_invalid_yaml() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(dir.path(), "bad.md", "---\n[\n---\n\n# Body\n");
        let source = make_source(dir.path());
        let results = check_frontmatter(&source);
        assert_eq!(results.len(), 1);
        assert!(results[0].issue.contains("missing or invalid"));
    }

    // ── check_required_fields ───────────────────────────────────────────

    #[test]
    fn required_gotcha_with_trigger_ok() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(dir.path(), "gotchas/g.md", "---\ndescription: x\ntags: [a]\ntrigger: \"file save\"\ntimestamp: 2026-06-28\n---\n\n## Save bug\n");
        let source = make_source(dir.path());
        let results = check_required_fields(&source);
        assert!(results.is_empty(), "expected no errors, got {results:?}");
    }

    #[test]
    fn required_gotcha_missing_trigger_error() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(
            dir.path(),
            "gotchas/g.md",
            "---\ndescription: x\ntags: [a]\ntimestamp: 2026-06-28\n---\n\n## Bug\n",
        );
        let source = make_source(dir.path());
        let results = check_required_fields(&source);
        assert_eq!(results.len(), 1);
        assert!(results[0].issue.contains("trigger"));
        assert!(matches!(results[0].severity, Severity::Error));
    }

    #[test]
    fn required_rule_with_applies_to_ok() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(dir.path(), "rules/r.md", "---\ndescription: x\ntags: [a]\napplies_to: \"*.rs\"\ntimestamp: 2026-06-28\n---\n\n## Rule\n");
        let source = make_source(dir.path());
        let results = check_required_fields(&source);
        assert!(results.is_empty(), "expected no errors, got {results:?}");
    }

    #[test]
    fn required_rule_missing_applies_to_error() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(
            dir.path(),
            "rules/r.md",
            "---\ndescription: x\ntags: [a]\ntimestamp: 2026-06-28\n---\n\n## Rule\n",
        );
        let source = make_source(dir.path());
        let results = check_required_fields(&source);
        assert_eq!(results.len(), 1);
        assert!(results[0].issue.contains("applies_to"));
    }

    #[test]
    fn required_decision_both_fields_ok() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(dir.path(), "decisions/d.md", "---\ndescription: x\ntags: [a]\nreversible: true\ndecided_at: 2026-06-28\ntimestamp: 2026-06-28\n---\n\n## Decision\n");
        let source = make_source(dir.path());
        let results = check_required_fields(&source);
        assert!(results.is_empty(), "expected no errors, got {results:?}");
    }

    #[test]
    fn required_decision_missing_both() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(
            dir.path(),
            "decisions/d.md",
            "---\ndescription: x\ntags: [a]\ntimestamp: 2026-06-28\n---\n\n## Decision\n",
        );
        let source = make_source(dir.path());
        let results = check_required_fields(&source);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn required_unknown_kind_skipped() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(
            dir.path(),
            "patterns/p.md",
            "---\ndescription: x\ntags: [a]\ntimestamp: 2026-06-28\n---\n\n## Pattern\n",
        );
        let source = make_source(dir.path());
        let results = check_required_fields(&source);
        assert!(results.is_empty(), "unknown kind should be skipped, got {results:?}");
    }

    #[test]
    fn required_no_frontmatter_skipped() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(dir.path(), "gotchas/nofm.md", "# No frontmatter\n");
        let source = make_source(dir.path());
        let results = check_required_fields(&source);
        assert!(results.is_empty(), "no frontmatter should be skipped, got {results:?}");
    }

    // ── check_sot_staleness ─────────────────────────────────────────────

    #[test]
    fn sot_staleness_recent_ok() {
        let dir = tempfile::TempDir::new().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        write_ek(
            dir.path(),
            "recent.md",
            &format!("---\ndescription: x\ntags: [a]\ntimestamp: {today}\n---\n\n## Recent\n"),
        );
        let source = make_source(dir.path());
        let results = check_sot_staleness(&source);
        assert!(results.is_empty(), "recent timestamp should be OK, got {results:?}");
    }

    #[test]
    fn sot_staleness_old_warns() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(
            dir.path(),
            "old.md",
            "---\ndescription: x\ntags: [a]\ntimestamp: 2025-01-15\n---\n\n## Old entry\n",
        );
        let source = make_source(dir.path());
        let results = check_sot_staleness(&source);
        assert_eq!(results.len(), 1);
        assert!(results[0].issue.contains("days old"));
        assert!(matches!(results[0].severity, Severity::Warning));
    }

    #[test]
    fn sot_staleness_missing_timestamp_skipped() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(dir.path(), "nots.md", "---\ndescription: x\ntags: [a]\n---\n\n## No timestamp\n");
        let source = make_source(dir.path());
        let results = check_sot_staleness(&source);
        assert!(results.is_empty(), "missing timestamp should be skipped, got {results:?}");
    }

    #[test]
    fn sot_staleness_no_frontmatter_skipped() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(dir.path(), "nofm.md", "# No frontmatter\n");
        let source = make_source(dir.path());
        let results = check_sot_staleness(&source);
        assert!(results.is_empty());
    }

    // ── check_export_consistency ────────────────────────────────────────

    #[test]
    fn export_both_present_ok() {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude/skills")).unwrap();
        std::fs::create_dir_all(dir.path().join(".cursor/rules")).unwrap();
        std::fs::write(dir.path().join(".claude/skills/enjoyknowledge.md"), "content").unwrap();
        std::fs::write(dir.path().join(".cursor/rules/enjoyknowledge.mdc"), "content").unwrap();
        let source = make_source(dir.path());
        let results = check_export_consistency(&source, dir.path());
        assert!(results.is_empty(), "both present = OK, got {results:?}");
    }

    #[test]
    fn export_both_missing_ok() {
        let dir = tempfile::TempDir::new().unwrap();
        let source = make_source(dir.path());
        let results = check_export_consistency(&source, dir.path());
        assert!(
            results.is_empty(),
            "both missing = OK (user hasn't enabled multi-tool), got {results:?}"
        );
    }

    #[test]
    fn export_only_claude_warns_cursor_missing() {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude/skills")).unwrap();
        std::fs::write(dir.path().join(".claude/skills/enjoyknowledge.md"), "content").unwrap();
        let source = make_source(dir.path());
        let results = check_export_consistency(&source, dir.path());
        assert_eq!(results.len(), 1);
        assert!(results[0].file.contains("cursor"));
        assert!(results[0].issue.contains("Cursor"));
        assert!(matches!(results[0].severity, Severity::Warning));
    }

    #[test]
    fn export_only_cursor_warns_claude_missing() {
        let dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".cursor/rules")).unwrap();
        std::fs::write(dir.path().join(".cursor/rules/enjoyknowledge.mdc"), "content").unwrap();
        let source = make_source(dir.path());
        let results = check_export_consistency(&source, dir.path());
        assert_eq!(results.len(), 1);
        assert!(results[0].file.contains("claude"));
        assert!(results[0].issue.contains("Claude"));
        assert!(matches!(results[0].severity, Severity::Warning));
    }

    // ── run_all integration ─────────────────────────────────────────────

    #[test]
    fn run_all_empty_project_has_frontmatter_errors() {
        let dir = tempfile::TempDir::new().unwrap();
        write_ek(dir.path(), "bad.md", "# No frontmatter\n");
        let source = make_source(dir.path());
        let results = run_all(&source, dir.path());
        // Should have at least the frontmatter error
        assert!(results.iter().any(|r| r.issue.contains("frontmatter")));
    }

    #[test]
    fn run_all_clean_project_no_results() {
        let dir = tempfile::TempDir::new().unwrap();
        let source = make_source(dir.path());
        let results = run_all(&source, dir.path());
        assert!(results.is_empty(), "empty project should have no issues, got {results:?}");
    }
}
