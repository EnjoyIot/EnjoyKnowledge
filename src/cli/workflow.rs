//! `enjoyknowledge workflow` — run named workflows (v0.2: onboard, capture).
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::format::frontmatter::parse_frontmatter;
use crate::knowledge::filesystem::FilesystemSource;
use crate::knowledge::KnowledgeSource;
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

/// Structured input for the capture workflow.
#[derive(Debug)]
pub struct CaptureInput {
    /// Knowledge kind: gotcha / decision / pattern / rule / business / architecture /
    /// contract / convention / context / template
    pub kind: String,
    /// Required frontmatter fields indexed by kind (e.g. `trigger`, `applies_to`, `reversible`, `decided_at`).
    pub fields: HashMap<String, String>,
    /// Markdown body (## sections + text).
    pub body: String,
    /// Target file path under `.enjoyknowledge/`; auto-derived from kind when omitted.
    pub path: Option<String>,
}

/// Structured output from the capture workflow.
#[derive(Debug)]
pub struct CaptureOutput {
    /// Relative path under `.enjoyknowledge/` where the entry was written.
    pub written_path: String,
    /// Whether `.enjoyknowledge/index.md` was updated.
    pub index_updated: bool,
}

/// Run a named workflow.
pub fn run(
    workflow: &str,
    project_root: &Path,
    kind: Option<String>,
    fields: Vec<(String, String)>,
    body: Option<String>,
    path: Option<String>,
) -> anyhow::Result<()> {
    match workflow {
        "onboard" => {
            let output = run_onboard(project_root)?;
            print_onboard_output(&output);
            Ok(())
        }
        "capture" => {
            let input = CaptureInput {
                kind: kind.ok_or_else(|| anyhow::anyhow!("capture requires --kind"))?,
                fields: fields.into_iter().collect(),
                body: body.ok_or_else(|| anyhow::anyhow!("capture requires --body"))?,
                path,
            };
            let output = run_capture(project_root, &input)?;
            print_capture_output(&output);
            Ok(())
        }
        _ => {
            anyhow::bail!("unknown workflow '{workflow}' (v0.2: onboard, capture)")
        }
    }
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
pub fn run_onboard(project_root: &Path) -> anyhow::Result<OnboardOutput> {
    let ek_dir = project_root.join(".enjoyknowledge");

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

/// Collect gotchas with severity 4–5 from `.enjoyknowledge/gotchas/`.
fn collect_critical_gotchas(ek_dir: &Path) -> Vec<GotchaEntry> {
    let gotchas_dir = ek_dir.join("gotchas");
    if !gotchas_dir.exists() {
        return Vec::new();
    }

    let mut entries = Vec::new();
    for entry in walkdir::WalkDir::new(&gotchas_dir)
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

/// Collect active decisions from `.enjoyknowledge/decisions/`.
fn collect_active_decisions(ek_dir: &Path) -> Vec<DecisionEntry> {
    let decisions_dir = ek_dir.join("decisions");
    if !decisions_dir.exists() {
        return Vec::new();
    }

    let mut entries = Vec::new();
    for entry in walkdir::WalkDir::new(&decisions_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let Ok(content) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        let fm = parse_frontmatter(&content);
        // Active if status is explicitly "active" or if no status exists
        // (seeds default to active).
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

// ── capture ─────────────────────────────────────────────────────────────────

/// Valid knowledge kinds for capture.
const VALID_KINDS: &[&str] = &[
    "gotcha",
    "decision",
    "pattern",
    "rule",
    "business",
    "architecture",
    "contract",
    "convention",
    "context",
    "template",
];

/// Required frontmatter fields per kind.
fn required_fields(kind: &str) -> &[&str] {
    match kind {
        "gotcha" => &["trigger"],
        "decision" => &["reversible", "decided_at"],
        "rule" | "contract" | "convention" | "template" => &["applies_to"],
        _ => &[],
    }
}

/// Default file path under `.enjoyknowledge/` for a given kind.
fn default_path_for_kind(kind: &str) -> String {
    let dir = match kind {
        "gotcha" => "gotchas",
        "decision" => "decisions",
        "rule" => "rules",
        "pattern" => "patterns",
        _ => kind, // architecture, business, contract(s), convention(s), context, template(s)
    };
    let file = match kind {
        "gotcha" => "gotchas.md",
        "decision" => "decisions.md",
        "rule" => "rules.md",
        "pattern" => "patterns.md",
        "architecture" => "architecture.md",
        "business" => "business.md",
        "contract" => "contracts.md",
        "convention" => "conventions.md",
        "context" => "context.md",
        "template" => "templates.md",
        _ => "knowledge.md",
    };
    format!("{dir}/{file}")
}

/// Validate that `kind` is one of the 10 recognised knowledge types.
fn validate_kind(kind: &str) -> anyhow::Result<()> {
    if VALID_KINDS.contains(&kind) {
        Ok(())
    } else {
        anyhow::bail!("unknown kind '{kind}'. Valid kinds: {}", VALID_KINDS.join(", "))
    }
}

/// Validate that all required fields for the given kind are present.
fn validate_required_fields(kind: &str, fields: &HashMap<String, String>) -> anyhow::Result<()> {
    for req in required_fields(kind) {
        if !fields.contains_key(*req) {
            anyhow::bail!(
                "kind '{kind}' requires field '{req}' (missing). Use --field {req}=VALUE"
            );
        }
    }
    Ok(())
}

/// Build YAML frontmatter for a capture entry.
fn build_capture_frontmatter(kind: &str, fields: &HashMap<String, String>, body: &str) -> String {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let description = body.lines().find(|l| l.starts_with("## ")).map_or_else(
        || format!("{kind} entry"),
        |l| l.trim_start_matches("## ").trim().to_string(),
    );

    let mut fm = format!("---\ndescription: {description}\ntimestamp: {today}\n");

    // Write kind-specific fields first, then any extra fields
    for req in required_fields(kind) {
        if let Some(val) = fields.get(*req) {
            let _ = std::fmt::Write::write_fmt(&mut fm, format_args!("{req}: {val}\n"));
        }
    }

    // Write remaining non-required fields (e.g. severity for gotchas, status for decisions)
    let required: Vec<&str> = required_fields(kind).to_vec();
    for (k, v) in fields {
        if !required.contains(&k.as_str()) {
            let _ = std::fmt::Write::write_fmt(&mut fm, format_args!("{k}: {v}\n"));
        }
    }

    fm.push_str("---\n");
    fm
}

/// Write the capture entry to disk via `FilesystemSource`.
fn write_entry(
    source: &FilesystemSource,
    path: &str,
    kind: &str,
    fields: &HashMap<String, String>,
    body: &str,
) -> anyhow::Result<()> {
    let full = source.root.join(path);

    if full.exists() {
        // Append body (without frontmatter) and refresh timestamp.
        source.add_entry(path, body)?;
    } else {
        // New file: full frontmatter + body.
        let fm = build_capture_frontmatter(kind, fields, body);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&full, format!("{fm}\n{body}\n"))?;
    }

    Ok(())
}

/// Append a TOC entry to `.enjoyknowledge/index.md`.
fn update_index(ek_dir: &Path, written_path: &str, body: &str) -> anyhow::Result<bool> {
    let index_path = ek_dir.join("index.md");
    if !index_path.exists() {
        return Ok(false);
    }

    let title = body
        .lines()
        .find(|l| l.starts_with("## "))
        .map_or_else(|| "Untitled".to_string(), |l| l.trim_start_matches("## ").trim().to_string());

    let entry = format!("- [{title}]({written_path})");

    let mut content = std::fs::read_to_string(&index_path)?;
    // Only append if the path isn't already referenced
    if content.contains(&format!("]({written_path})")) {
        return Ok(false);
    }

    content.push('\n');
    content.push_str(&entry);
    content.push('\n');
    std::fs::write(&index_path, content)?;
    Ok(true)
}

/// Execute the capture workflow: classify → validate → write → index.
pub fn run_capture(project_root: &Path, input: &CaptureInput) -> anyhow::Result<CaptureOutput> {
    // 1. Validate kind
    validate_kind(&input.kind)?;

    // 2. Validate required fields
    validate_required_fields(&input.kind, &input.fields)?;

    // 3. Determine target path
    let target_path = input.path.clone().unwrap_or_else(|| default_path_for_kind(&input.kind));

    // 4. Write entry
    let ek_dir = project_root.join(EK_DIR);
    let source = FilesystemSource::new(&ek_dir, project_root);
    write_entry(&source, &target_path, &input.kind, &input.fields, &input.body)?;

    // 5. Update index.md
    let index_updated = update_index(&ek_dir, &target_path, &input.body)?;

    Ok(CaptureOutput { written_path: target_path, index_updated })
}

fn print_capture_output(output: &CaptureOutput) {
    println!("== capture: knowledge recorded ==");
    println!();
    println!("written:  .enjoyknowledge/{}", output.written_path);
    println!(
        "index:    {}",
        if output.index_updated { "updated" } else { "unchanged (already referenced)" }
    );
}

// ── tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_ek_dir(tmp: &Path) -> std::path::PathBuf {
        let ek = tmp.join(".enjoyknowledge");
        fs::create_dir_all(ek.join("gotchas")).unwrap();
        fs::create_dir_all(ek.join("decisions")).unwrap();
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
            "# AGENTS.md\n\n<!-- enjoyknowledge_LS_START -->\ngotchas/\n<!-- enjoyknowledge_LS_END -->\n",
        )
        .unwrap();

        // Create a severity-5 gotcha
        fs::write(
            ek.join("gotchas").join("critical.md"),
            "---\nseverity: 5\ndescription: critical bug\n---\n\n## Encoding crash\n- **Impact**: data loss\n",
        )
        .unwrap();

        // Create an active decision
        fs::write(
            ek.join("decisions").join("001-use-sqlite.md"),
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
            ek.join("gotchas").join("low.md"),
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
            ek.join("decisions").join("002-deprecated.md"),
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
            ek.join("decisions").join("003-implicit.md"),
            "---\ndescription: implicit active\n---\n\n## 003 — Implicit\nno explicit status\n",
        )
        .unwrap();

        let output = run_onboard(tmp.path()).unwrap();
        assert_eq!(output.active_decisions.len(), 1);
    }

    // ── capture: kind validation ─────────────────────────────────────

    #[test]
    fn capture_validates_10_kinds() {
        for kind in VALID_KINDS {
            assert!(validate_kind(kind).is_ok(), "kind '{kind}' should be valid");
        }
    }

    #[test]
    fn capture_rejects_invalid_kind() {
        let err = validate_kind("unknown_type").unwrap_err();
        assert!(err.to_string().contains("unknown kind"));
        assert!(err.to_string().contains("gotcha"));
    }

    // ── capture: required-field validation ─────────────────────────────

    #[test]
    fn capture_gotcha_requires_trigger() {
        let fields: HashMap<String, String> = HashMap::new();
        let err = validate_required_fields("gotcha", &fields).unwrap_err();
        assert!(err.to_string().contains("trigger"));
    }

    #[test]
    fn capture_rule_requires_applies_to() {
        let fields: HashMap<String, String> = HashMap::new();
        let err = validate_required_fields("rule", &fields).unwrap_err();
        assert!(err.to_string().contains("applies_to"));
    }

    #[test]
    fn capture_decision_requires_reversible_and_decided_at() {
        // Missing both
        let fields: HashMap<String, String> = HashMap::new();
        let err = validate_required_fields("decision", &fields).unwrap_err();
        assert!(err.to_string().contains("reversible"));

        // Missing decided_at only
        let mut fields = HashMap::new();
        fields.insert("reversible".to_string(), "true".to_string());
        let err = validate_required_fields("decision", &fields).unwrap_err();
        assert!(err.to_string().contains("decided_at"));
    }

    #[test]
    fn capture_architecture_no_required_fields() {
        let fields: HashMap<String, String> = HashMap::new();
        assert!(validate_required_fields("architecture", &fields).is_ok());
    }

    // ── capture: write new file ────────────────────────────────────────

    #[test]
    fn capture_creates_new_file_with_frontmatter() {
        let tmp = tempfile::tempdir().unwrap();
        let ek = setup_ek_dir(tmp.path());
        // Also create an index.md
        fs::write(
            ek.join("index.md"),
            "---\ndescription: index\ntimestamp: 2026-01-01\n---\n\n# Index\n",
        )
        .unwrap();

        let input = CaptureInput {
            kind: "gotcha".to_string(),
            fields: {
                let mut m = HashMap::new();
                m.insert("trigger".to_string(), "test trigger".to_string());
                m
            },
            body: "## Test Gotcha\nThis is a test gotcha entry.".to_string(),
            path: None,
        };

        let output = run_capture(tmp.path(), &input).unwrap();
        assert!(output.written_path.contains("gotchas"));
        // Confirm lowercase filename — prevents case-inconsistency regressions
        assert!(output.written_path.ends_with("gotchas.md"));
        assert!(output.index_updated);

        // Verify file was created
        let full = ek.join(&output.written_path);
        let content = fs::read_to_string(&full).unwrap();
        assert!(content.contains("description: Test Gotcha"));
        assert!(content.contains("trigger: test trigger"));
        assert!(content.contains("## Test Gotcha"));
    }

    /// Capture auto-routing produces lowercase paths for all 4 default-routing kinds.
    #[test]
    fn capture_auto_route_uses_lowercase_paths() {
        let tmp = tempfile::tempdir().unwrap();
        let ek = setup_ek_dir(tmp.path());
        fs::write(
            ek.join("index.md"),
            "---\ndescription: index\ntimestamp: 2026-01-01\n---\n\n# Index\n",
        )
        .unwrap();

        for (kind, expected_path) in [
            ("gotcha", "gotchas/gotchas.md"),
            ("decision", "decisions/decisions.md"),
            ("rule", "rules/rules.md"),
            ("pattern", "patterns/patterns.md"),
        ] {
            let input = CaptureInput {
                kind: kind.to_string(),
                fields: {
                    let mut m = HashMap::new();
                    match kind {
                        "gotcha" => {
                            m.insert("trigger".to_string(), "t".to_string());
                        }
                        "decision" => {
                            m.insert("reversible".to_string(), "yes".to_string());
                            m.insert("decided_at".to_string(), "2026-01-01".to_string());
                        }
                        "rule" => {
                            m.insert("applies_to".to_string(), "all".to_string());
                        }
                        _ => {}
                    }
                    m
                },
                body: format!("## {kind}\nTest body."),
                path: None,
            };
            let output = run_capture(tmp.path(), &input).unwrap();
            assert_eq!(
                output.written_path, expected_path,
                "kind '{kind}' should route to '{expected_path}', got '{}'",
                output.written_path
            );
        }
    }

    // ── capture: append to existing file ───────────────────────────────

    #[test]
    fn capture_appends_to_existing_file() {
        let tmp = tempfile::tempdir().unwrap();
        let ek = setup_ek_dir(tmp.path());
        // Create existing gotcha file
        let existing_path = "gotchas/gotchas.md";
        fs::create_dir_all(ek.join("gotchas")).unwrap();
        fs::write(
            ek.join(existing_path),
            "---\ndescription: existing\ntimestamp: 2026-01-01\n---\n\n## Existing Entry\nOld content.\n",
        )
        .unwrap();
        // Also need index.md
        fs::write(
            ek.join("index.md"),
            "---\ndescription: index\ntimestamp: 2026-01-01\n---\n\n# Index\n",
        )
        .unwrap();

        let input = CaptureInput {
            kind: "gotcha".to_string(),
            fields: {
                let mut m = HashMap::new();
                m.insert("trigger".to_string(), "another trigger".to_string());
                m
            },
            body: "## New Gotcha\nNew content here.".to_string(),
            path: None,
        };

        let output = run_capture(tmp.path(), &input).unwrap();
        assert_eq!(output.written_path, existing_path);

        let content = fs::read_to_string(ek.join(existing_path)).unwrap();
        // Original entry still present
        assert!(content.contains("## Existing Entry"));
        // New entry appended
        assert!(content.contains("## New Gotcha"));
        // Frontmatter should still have description from original
        assert!(content.contains("description: existing"));
    }

    // ── capture: index update ──────────────────────────────────────────

    #[test]
    fn capture_updates_index_md() {
        let tmp = tempfile::tempdir().unwrap();
        let ek = setup_ek_dir(tmp.path());
        fs::create_dir_all(ek.join("patterns")).unwrap();
        fs::write(
            ek.join("index.md"),
            "---\ndescription: index\ntimestamp: 2026-01-01\n---\n\n# Index\n",
        )
        .unwrap();

        let input = CaptureInput {
            kind: "pattern".to_string(),
            fields: HashMap::new(),
            body: "## Error Flow Pattern\nUse Result<T, AppError>.".to_string(),
            path: None,
        };

        let output = run_capture(tmp.path(), &input).unwrap();
        assert!(output.index_updated);

        let index_content = fs::read_to_string(ek.join("index.md")).unwrap();
        assert!(index_content.contains("[Error Flow Pattern]"));
        assert!(index_content.contains("patterns/patterns.md"));
    }

    // ── capture: custom path ───────────────────────────────────────────

    #[test]
    fn capture_uses_custom_path() {
        let tmp = tempfile::tempdir().unwrap();
        let ek = setup_ek_dir(tmp.path());
        fs::write(
            ek.join("index.md"),
            "---\ndescription: index\ntimestamp: 2026-01-01\n---\n\n# Index\n",
        )
        .unwrap();

        let input = CaptureInput {
            kind: "gotcha".to_string(),
            fields: {
                let mut m = HashMap::new();
                m.insert("trigger".to_string(), "custom trigger".to_string());
                m
            },
            body: "## Custom Path Entry\nCustom content.".to_string(),
            path: Some("gotchas/custom-file.md".to_string()),
        };

        let output = run_capture(tmp.path(), &input).unwrap();
        assert_eq!(output.written_path, "gotchas/custom-file.md");
        assert!(ek.join("gotchas/custom-file.md").exists());
    }
}
