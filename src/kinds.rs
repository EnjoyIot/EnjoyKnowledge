//! Kind registry — single source of truth for knowledge kinds.
//!
//! Parsed at compile-time from `.enjoyknowledge/_meta/kinds.md` (Markdown table).
//! All `dir_for()` lookups return `kind` directly — no "s" plural derivation.

use std::path::Path;
use std::sync::LazyLock;

/// A knowledge kind parsed from `kinds.md`.
#[derive(Debug, Clone)]
pub struct Kind {
    pub name: String,
    #[allow(dead_code)]
    pub required: Vec<String>,
    #[allow(dead_code)]
    pub summary: String,
}

/// Default `kinds.md` content — embedded at compile-time.
const KINDS_MD_DEFAULT: &str = include_str!("fixtures/kinds-default.md");

/// Lazily-parsed kind registry.
static KINDS: LazyLock<Vec<Kind>> = LazyLock::new(|| parse_kinds_md(KINDS_MD_DEFAULT));

/// Return all registered knowledge kinds.
pub fn all() -> &'static [Kind] {
    &KINDS
}

/// Read and parse user-editable `kinds.md` from the filesystem at runtime.
pub fn all_from_file(path: &Path) -> anyhow::Result<Vec<Kind>> {
    let content = std::fs::read_to_string(path)?;
    Ok(parse_kinds_md(&content))
}

/// Return the canonical directory name for a kind (= kind, no derivation).
pub const fn dir_for(name: &str) -> &str {
    name
}

/// Return required frontmatter fields for a given kind.
#[allow(dead_code)]
pub fn required_fields(name: &str) -> &'static [&'static str] {
    match name {
        "gotcha" => &["trigger"],
        "decision" => &["reversible", "decided_at"],
        "rule" | "contract" | "convention" | "template" | "command" => &["applies_to"],
        _ => &[],
    }
}

/// Return whether `name` is a registered knowledge kind.
#[allow(dead_code)]
pub fn is_valid_kind(name: &str) -> bool {
    KINDS.iter().any(|k| k.name == name)
}

/// Return the default `kinds.md` content (for seeding new projects).
pub const fn init_default_kinds() -> &'static str {
    KINDS_MD_DEFAULT
}

/// Parse `kinds.md` Markdown table into kind names only (for doctor checks).
pub fn parse_kinds_md_for_doctor(md: &str) -> Vec<String> {
    parse_kinds_md(md).into_iter().map(|k| k.name).collect()
}

/// Parse `kinds.md` Markdown table into a `Vec<Kind>`.
/// Panics on malformed input (fail-fast at startup).
pub fn parse_kinds_md(md: &str) -> Vec<Kind> {
    let mut kinds = Vec::new();
    let mut in_table = false;

    for line in md.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("| kind ") && trimmed.contains("| required ") {
            in_table = true;
            continue;
        }
        if !in_table {
            continue;
        }
        // Stop at end of table (first non-table line after header)
        if !trimmed.starts_with('|') {
            break;
        }
        // Skip separator line
        if trimmed.contains("---") {
            continue;
        }
        // Parse data row: | kind | required | summary |
        let cells: Vec<&str> = trimmed.split('|').map(str::trim).collect();
        // cells[0] is empty (before first |), cells[1]=kind, cells[2]=required, cells[3]=summary
        if cells.len() < 3 {
            continue;
        }
        let name = cells[1].to_string();
        let required: Vec<String> = if cells.len() > 2 && !cells[2].is_empty() {
            cells[2].split(',').map(|f| f.trim().to_string()).collect()
        } else {
            Vec::new()
        };
        let summary = cells.get(3).map_or_else(String::new, |s| s.trim().to_string());

        kinds.push(Kind { name, required, summary });
    }

    assert!(
        !kinds.is_empty(),
        "kinds.md: no kind rows found — check format (| kind | required | summary |)"
    );
    kinds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_11_kinds() {
        let all = all();
        assert_eq!(all.len(), 11, "expected 11 kinds, got {}", all.len());
    }

    #[test]
    fn dir_for_is_identity() {
        assert_eq!(dir_for("gotcha"), "gotcha");
        assert_eq!(dir_for("decision"), "decision");
        assert_eq!(dir_for("pattern"), "pattern");
        assert_eq!(dir_for("rule"), "rule");
        assert_eq!(dir_for("command"), "command");
        assert_eq!(dir_for("architecture"), "architecture");
    }

    #[test]
    fn is_valid_kind_checks_registry() {
        assert!(is_valid_kind("gotcha"));
        assert!(is_valid_kind("decision"));
        assert!(is_valid_kind("command"));
        assert!(!is_valid_kind("nonexistent"));
        assert!(!is_valid_kind("gotchas")); // plural = invalid
    }

    #[test]
    fn required_fields_per_kind() {
        assert_eq!(required_fields("gotcha"), &["trigger"]);
        assert_eq!(required_fields("decision"), &["reversible", "decided_at"]);
        assert_eq!(required_fields("rule"), &["applies_to"]);
        assert_eq!(required_fields("command"), &["applies_to"]);
        assert!(required_fields("architecture").is_empty());
    }

    #[test]
    fn init_default_kinds_is_valid_md() {
        let md = init_default_kinds();
        assert!(md.contains("| kind | required | summary |"));
        assert!(md.contains("| gotcha |"));
        assert!(md.contains("| command |"));
    }
}
