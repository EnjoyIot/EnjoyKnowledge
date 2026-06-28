//! `enjoyknowledge onboard` — establish a project mental model in ~30s.
use std::path::{Path, PathBuf};

use crate::format::frontmatter::parse_frontmatter;
use crate::kinds;
use crate::EK_DIR;

/// Structured output from the onboard workflow.
#[derive(Debug)]
pub struct OnboardOutput {
    pub agent_md_path: Option<PathBuf>,
    pub positioning_path: Option<PathBuf>,
    pub design_philosophy_path: Option<PathBuf>,
    pub design_path: Option<PathBuf>,
    pub critical_gotchas: Vec<GotchaEntry>,
    pub active_decisions: Vec<DecisionEntry>,
    pub total_knowledge_files: usize,
}

/// A single critical gotcha (severity 4–5).
#[derive(Debug, Clone)]
pub struct GotchaEntry {
    pub file: String,
    pub section: String,
    pub severity: u8,
    pub snippet: String,
}

/// A single active decision entry.
#[derive(Debug, Clone)]
pub struct DecisionEntry {
    pub file: String,
    pub section: String,
    pub snippet: String,
}

/// Execute the onboard workflow: establish a project mental model in ~30s.
///
/// Steps:
/// 1. Verify `.enjoyknowledge/` exists
/// 2. Read `AGENTS.md` (routing table)
/// 3. Locate POSITIONING / DESIGN-PHILOSOPHY / DESIGN files
/// 4. Collect critical gotchas (severity 4–5)
/// 5. Collect active decisions
/// 6. Count total knowledge files
pub fn run(project_root: &Path) -> anyhow::Result<()> {
    let output = run_onboard(project_root)?;
    print_onboard_output(&output);
    Ok(())
}

fn run_onboard(project_root: &Path) -> anyhow::Result<OnboardOutput> {
    let ek_dir = project_root.join(EK_DIR);

    if !ek_dir.exists() {
        anyhow::bail!(
            ".enjoyknowledge/ not found in {} — run 'enjoyknowledge init' first",
            project_root.display()
        );
    }

    // 1. AGENTS.md (routing table)
    let agent_md_path = project_root.join("AGENTS.md");
    let agent_md_path = agent_md_path.exists().then_some(agent_md_path);

    // 2. POSITIONING / DESIGN-PHILOSOPHY / DESIGN
    let positioning_path = find_file(&ek_dir, &["POSITIONING.md"]);
    let design_philosophy_path =
        find_file(&ek_dir, &["DESIGN-PHILOSOPHY.md", "architecture/design-philosophy.md"]);
    let design_path = find_file(&ek_dir, &["DESIGN.md", "architecture/design.md"]);

    // 3. Critical gotchas (severity 4–5)
    let critical_gotchas = collect_critical_gotchas(&ek_dir);

    // 4. Active decisions
    let active_decisions = collect_active_decisions(&ek_dir);

    // 5. Total knowledge files
    let total_knowledge_files = count_md_files(&ek_dir);

    Ok(OnboardOutput {
        agent_md_path,
        positioning_path,
        design_philosophy_path,
        design_path,
        critical_gotchas,
        active_decisions,
        total_knowledge_files,
    })
}

// ── helpers ───────────────────────────────────────────────────────────────

/// Find the first existing file from a list of relative paths under `ek_dir`.
fn find_file(ek_dir: &Path, candidates: &[&str]) -> Option<PathBuf> {
    candidates.iter().map(|c| ek_dir.join(c)).find(|p| p.exists())
}

/// Count all `.md` files under `.enjoyknowledge/` (max depth 3).
fn count_md_files(ek_dir: &Path) -> usize {
    walkdir::WalkDir::new(ek_dir)
        .max_depth(3)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| {
            e.path().extension().is_some_and(|ext| ext == "md")
                && !e.file_name().to_string_lossy().starts_with('.')
        })
        .count()
}

/// Collect gotchas with severity 4–5 from `.enjoyknowledge/gotcha/`.
fn collect_critical_gotchas(ek_dir: &Path) -> Vec<GotchaEntry> {
    let gotcha_dir = ek_dir.join(kinds::dir_for("gotcha"));
    if !gotcha_dir.exists() {
        return Vec::new();
    }

    let mut entries = Vec::new();
    for entry in walkdir::WalkDir::new(&gotcha_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let Ok(content) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        let fm = parse_frontmatter(&content);
        let severity = fm.as_ref().and_then(|f| f.severity).unwrap_or(0);
        if !(4..=5).contains(&severity) {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(ek_dir)
            .unwrap_or_else(|_| entry.path())
            .to_string_lossy()
            .replace('\\', "/");

        // Extract each ## section whose body mentions the severity context
        let body_start = crate::format::document::find_body_start(&content);
        let body = &content[body_start..];
        let sections = extract_sections(body);
        if sections.is_empty() {
            entries.push(GotchaEntry {
                file: rel,
                section: String::new(),
                severity,
                snippet: body.lines().take(3).collect::<Vec<_>>().join("\n"),
            });
        } else {
            for (title, body_text) in &sections {
                entries.push(GotchaEntry {
                    file: rel.clone(),
                    section: title.clone(),
                    severity,
                    snippet: body_text.lines().take(3).collect::<Vec<_>>().join("\n"),
                });
            }
        }
    }
    entries
}

/// Collect active decisions from `.enjoyknowledge/decision/`.
fn collect_active_decisions(ek_dir: &Path) -> Vec<DecisionEntry> {
    let decision_dir = ek_dir.join(kinds::dir_for("decision"));
    if !decision_dir.exists() {
        return Vec::new();
    }

    let mut entries = Vec::new();
    for entry in walkdir::WalkDir::new(&decision_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let Ok(content) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        let fm = parse_frontmatter(&content);
        let is_active = fm
            .as_ref()
            .and_then(|f| f.status.as_deref())
            .is_none_or(|s| s.eq_ignore_ascii_case("active"));
        if !is_active {
            continue;
        }
        let rel = entry
            .path()
            .strip_prefix(ek_dir)
            .unwrap_or_else(|_| entry.path())
            .to_string_lossy()
            .replace('\\', "/");

        let body_start = crate::format::document::find_body_start(&content);
        let body = &content[body_start..];
        let sections = extract_sections(body);
        if sections.is_empty() {
            entries.push(DecisionEntry {
                file: rel,
                section: String::new(),
                snippet: body.lines().take(3).collect::<Vec<_>>().join("\n"),
            });
        } else {
            for (title, body_text) in &sections {
                entries.push(DecisionEntry {
                    file: rel.clone(),
                    section: title.clone(),
                    snippet: body_text.lines().take(3).collect::<Vec<_>>().join("\n"),
                });
            }
        }
    }
    entries
}

/// Extract `##` section (title, body text) pairs from Markdown body.
fn extract_sections(body: &str) -> Vec<(String, String)> {
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut current_title = String::new();
    let mut current_body = String::new();

    for line in body.lines() {
        if line.starts_with("## ") && !line.starts_with("### ") {
            if !current_title.is_empty() {
                sections
                    .push((std::mem::take(&mut current_title), std::mem::take(&mut current_body)));
            }
            current_title = line[3..].trim().to_string();
        } else if !current_title.is_empty() {
            if !current_body.is_empty() {
                current_body.push('\n');
            }
            current_body.push_str(line);
        }
    }
    if !current_title.is_empty() {
        sections.push((current_title, current_body));
    }
    sections
}

// ── output ─────────────────────────────────────────────────────────────────

fn print_onboard_output(output: &OnboardOutput) {
    println!("== onboard: project mental model ==");
    println!();

    match &output.agent_md_path {
        Some(p) => println!("AGENTS.md         {}", p.display()),
        None => println!("AGENTS.md         (not found)"),
    }
    match &output.positioning_path {
        Some(p) => println!("POSITIONING       {}", p.display()),
        None => println!("POSITIONING       (not found)"),
    }
    match &output.design_philosophy_path {
        Some(p) => println!("DESIGN-PHILOSOPHY {}", p.display()),
        None => println!("DESIGN-PHILOSOPHY (not found)"),
    }
    match &output.design_path {
        Some(p) => println!("DESIGN            {}", p.display()),
        None => println!("DESIGN            (not found)"),
    }

    println!();
    println!("critical gotchas (severity 4–5): {}", output.critical_gotchas.len());
    for g in &output.critical_gotchas {
        if g.section.is_empty() {
            println!("  {}  severity={}", g.file, g.severity);
        } else {
            println!("  {}##{}  severity={}", g.file, g.section, g.severity);
        }
        for line in g.snippet.lines() {
            println!("    {line}");
        }
    }

    println!();
    println!("active decisions: {}", output.active_decisions.len());
    for d in &output.active_decisions {
        if d.section.is_empty() {
            println!("  {}", d.file);
        } else {
            println!("  {}##{}", d.file, d.section);
        }
        for line in d.snippet.lines() {
            println!("    {line}");
        }
    }

    println!();
    println!("total knowledge files: {}", output.total_knowledge_files);
}

// ── tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_ek_dir(tmp: &Path) -> std::path::PathBuf {
        let ek = tmp.join(".enjoyknowledge");
        fs::create_dir_all(ek.join("gotcha")).unwrap();
        fs::create_dir_all(ek.join("decision")).unwrap();
        fs::create_dir_all(ek.join("architecture")).unwrap();
        ek
    }

    // ── OnboardOutput struct ───────────────────────────────────────────

    #[test]
    fn onboard_output_struct_constructs() {
        let output = OnboardOutput {
            agent_md_path: Some(PathBuf::from("AGENTS.md")),
            positioning_path: None,
            design_philosophy_path: None,
            design_path: None,
            critical_gotchas: vec![],
            active_decisions: vec![],
            total_knowledge_files: 0,
        };
        assert!(output.agent_md_path.is_some());
        assert!(output.positioning_path.is_none());
    }

    // ── .enjoyknowledge/ missing ───────────────────────────────────────

    #[test]
    fn onboard_missing_ek_dir_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let err = run_onboard(tmp.path()).unwrap_err();
        assert!(err.to_string().contains(".enjoyknowledge/ not found"));
    }

    // ── Normal onboard run ─────────────────────────────────────────────

    #[test]
    fn onboard_with_files_succeeds() {
        let tmp = tempfile::tempdir().unwrap();
        let ek = setup_ek_dir(tmp.path());

        // Create AGENTS.md
        fs::write(
            tmp.path().join("AGENTS.md"),
            "# AGENTS.md\n\n<!-- enjoyknowledge_LS_START -->\ngotcha/\n<!-- enjoyknowledge_LS_END -->\n",
        )
        .unwrap();

        // Create a severity-5 gotcha
        fs::write(
            ek.join("gotcha").join("critical.md"),
            "---\nseverity: 5\ndescription: critical bug\n---\n\n## Encoding crash\n- **Impact**: data loss\n",
        )
        .unwrap();

        // Create an active decision
        fs::write(
            ek.join("decision").join("001-use-sqlite.md"),
            "---\nstatus: active\ndescription: pick db\n---\n\n## 001 — Use SQLite\n- **Decision**: use SQLite\n",
        )
        .unwrap();

        let output = run_onboard(tmp.path()).unwrap();

        assert!(output.agent_md_path.is_some());
        assert_eq!(output.critical_gotchas.len(), 1);
        assert_eq!(output.critical_gotchas[0].severity, 5);
        assert_eq!(output.critical_gotchas[0].section, "Encoding crash");
        assert_eq!(output.active_decisions.len(), 1);
        assert_eq!(output.active_decisions[0].section, "001 — Use SQLite");
        assert!(output.total_knowledge_files >= 2);
    }

    // ── Severity filtering ─────────────────────────────────────────────

    #[test]
    fn gotcha_severity_3_is_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        let ek = setup_ek_dir(tmp.path());

        fs::write(
            ek.join("gotcha").join("low.md"),
            "---\nseverity: 3\ndescription: minor thing\n---\n\n## Minor\nnot critical\n",
        )
        .unwrap();

        let output = run_onboard(tmp.path()).unwrap();
        assert!(output.critical_gotchas.is_empty());
    }

    // ── Inactive decision skipped ──────────────────────────────────────

    #[test]
    fn inactive_decision_is_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        let ek = setup_ek_dir(tmp.path());

        fs::write(
            ek.join("decision").join("002-deprecated.md"),
            "---\nstatus: deprecated\ndescription: old choice\n---\n\n## 002 — Old\nno longer active\n",
        )
        .unwrap();

        let output = run_onboard(tmp.path()).unwrap();
        assert!(output.active_decisions.is_empty());
    }

    // ── Decision with no status defaults to active ─────────────────────

    #[test]
    fn decision_no_status_defaults_active() {
        let tmp = tempfile::tempdir().unwrap();
        let ek = setup_ek_dir(tmp.path());

        fs::write(
            ek.join("decision").join("003-implicit.md"),
            "---\ndescription: implicit active\n---\n\n## 003 — Implicit\nno explicit status\n",
        )
        .unwrap();

        let output = run_onboard(tmp.path()).unwrap();
        assert_eq!(output.active_decisions.len(), 1);
    }
}
