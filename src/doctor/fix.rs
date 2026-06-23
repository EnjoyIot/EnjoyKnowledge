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

/// Run auto-fix for the knowledge base, optionally scoped to a specific task.
pub fn run_fix(
    source: &FilesystemSource,
    project_root: &Path,
    req: Option<&str>,
) -> anyhow::Result<()> {
    let violations = checks::run_all(source, project_root);

    if violations.is_empty() {
        eprintln!("enjoyknowledge: nothing to fix — all checks pass");
        return Ok(());
    }

    // Group violations by category
    let mut budget_files: Vec<String> = Vec::new();
    let mut missing_desc: Vec<String> = Vec::new();
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

    // Fix 1: Fill missing descriptions
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

                    let new_fm = crate::format::frontmatter::generate_frontmatter(&desc);
                    let new_content = format!("{new_fm}{body}");
                    std::fs::write(source.root.join(rel), new_content)?;
                    eprintln!("enjoyknowledge: filled description for {rel}");
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
        let profile = crate::profile::coding::CodingProfile;
        crate::init::skeleton::generate_agents_md(project_root, None, &profile)?;
        eprintln!("enjoyknowledge: regenerated AGENTS.md");
    }

    // Always sync summary after fixes
    crate::init::skeleton::sync_agents_md_summary(project_root, source)?;

    Ok(())
}
