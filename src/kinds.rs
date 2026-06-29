//! Kind registry — single source of truth for knowledge kinds.
//!
//! Defaults are embedded at compile-time from `src/fixtures/kinds-default.md`.
//! Call `init_kinds()` at startup to override with the user's
//! `.enjoyknowledge/_meta/kinds.md` when it exists.
//!
//! All `dir_for()` lookups return `kind` directly — no "s" plural derivation.

use std::path::Path;
use std::sync::RwLock;

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

/// Runtime kind registry, initialised from compile-time default.
/// Call `init_kinds()` to override from the user's filesystem copy.
static KINDS: RwLock<Vec<Kind>> = RwLock::new(Vec::new());

/// One-time initialisation: called at process start in `main()`.
/// Populates the registry from the user's kinds.md when present.
pub fn init_kinds(project_root: &Path) {
    let path = project_root.join(crate::config::KINDS_MD_REL);
    let kinds = if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|content| {
                let parsed = parse_kinds_md(&content);
                if parsed.is_empty() { None } else { Some(parsed) }
            })
            .unwrap_or_else(|| parse_kinds_md(KINDS_MD_DEFAULT))
    } else {
        parse_kinds_md(KINDS_MD_DEFAULT)
    };
    *KINDS.write().expect("kinds lock poisoned") = kinds;
}

/// Return all registered knowledge kinds (snapshot).
/// Lazily initialises from the compile-time default if `init_kinds()` was not called.
pub fn all() -> Vec<Kind> {
    ensure_initialized();
    KINDS.read().expect("kinds lock poisoned").clone()
}

/// Populate the registry from compile-time default when not yet initialised.
fn ensure_initialized() {
    if KINDS.read().expect("kinds lock poisoned").is_empty() {
        // read guard dropped before acquiring write
        let mut kinds = KINDS.write().expect("kinds lock poisoned");
        if kinds.is_empty() {
            *kinds = parse_kinds_md(KINDS_MD_DEFAULT);
        }
    }
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
/// Looks up the kind in the runtime registry and returns its required fields.
pub fn required_fields(name: &str) -> Vec<String> {
    ensure_initialized();
    let kinds = KINDS.read().expect("kinds lock poisoned");
    kinds
        .iter()
        .find(|k| k.name == name)
        .map(|k| k.required.clone())
        .unwrap_or_default()
}

/// Return whether `name` is a registered knowledge kind.
#[allow(dead_code)]
pub fn is_valid_kind(name: &str) -> bool {
    ensure_initialized();
    KINDS.read().expect("kinds lock poisoned").iter().any(|k| k.name == name)
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
    fn parse_all_kinds_non_empty() {
        let all = all();
        assert!(!all.is_empty(), "expected at least one kind, got 0");
    }

    #[test]
    fn dir_for_is_identity() {
        for k in all() {
            assert_eq!(dir_for(&k.name), k.name.as_str());
        }
    }

    #[test]
    fn is_valid_kind_checks_registry() {
        let all = all();
        assert!(!all.is_empty());
        // Every registered kind should be valid
        for k in all {
            assert!(is_valid_kind(&k.name), "expected {} to be valid", k.name);
        }
        assert!(!is_valid_kind("nonexistent"));
    }

    #[test]
    fn required_fields_per_kind() {
        // Verify required fields match the kinds-default.md table
        assert_eq!(required_fields("gotcha"), &["trigger"]);
        assert_eq!(required_fields("decision"), &["reversible", "decided_at"]);
        assert_eq!(required_fields("rule"), &["applies_to"]);
        assert_eq!(required_fields("command"), &["applies_to"]);
        assert_eq!(required_fields("architecture"), &["applies_to"]);
        assert_eq!(required_fields("pattern"), &["applies_to"]);
        // Unknown kind returns empty
        assert!(required_fields("nonexistent_kind_xyz").is_empty());
    }

    #[test]
    fn init_default_kinds_is_valid_md() {
        let md = init_default_kinds();
        assert!(md.contains("| kind | required | summary |"));
        assert!(md.contains("| gotcha |"));
        assert!(md.contains("| command |"));
    }
}
