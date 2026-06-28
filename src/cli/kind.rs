//! `enjoyknowledge kind` — manage knowledge kinds (add/rm/list).

use std::fs;
use std::path::Path;

use crate::kinds;
use crate::EK_DIR;

const KIND_META_DIR: &str = "_meta";
const KINDS_FILE: &str = "kinds.md";

/// `ek kind add <name> [--required <csv>] [--summary <text>] [--yes]`
pub fn run_add(
    project_root: &Path,
    name: &str,
    required: &[String],
    summary: &str,
    yes: bool,
) -> anyhow::Result<()> {
    // 1. Validate name
    if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        anyhow::bail!("kind name must be alphanumeric/underscore/dash, got '{name}'");
    }

    // 2. Check kinds.md exists
    let kinds_md_path = project_root.join(EK_DIR).join(KIND_META_DIR).join(KINDS_FILE);
    if !kinds_md_path.exists() {
        anyhow::bail!("{} not found — run `ek init` first", kinds_md_path.display());
    }

    // 3. Read user kinds.md
    let content = fs::read_to_string(&kinds_md_path)?;
    let user_kinds = kinds::parse_kinds_md(&content);
    if user_kinds.iter().any(|k| k.name == name) {
        anyhow::bail!("kind '{name}' already exists in kinds.md");
    }

    // 4. Confirm (unless --yes)
    if !yes {
        eprintln!("About to add kind '{name}' to {EK_DIR}/ (use --yes to skip confirmation)");
    }

    // 5. Add row to Markdown table
    let new_row = format!("| {name} | {} | {summary} |", required.join(", "));
    let updated = append_to_kind_table(&content, &new_row);
    fs::write(&kinds_md_path, updated)?;

    // 6. Create directory
    let dir = project_root.join(EK_DIR).join(name);
    fs::create_dir_all(&dir)?;

    // 7. Create seed file
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let seed = format!(
        "---\ndescription: {name} knowledge entries — <add description>\ntimestamp: {today}\n---\n\n# {name} Knowledge\n\n> Add your first {name}.md entry here.\n",
    );
    let seed_path = dir.join(format!("{name}.md"));
    if !seed_path.exists() {
        fs::write(&seed_path, seed)?;
    }

    println!("enjoyknowledge: added kind '{name}' → {EK_DIR}/");
    Ok(())
}

/// `ek kind rm <name> [--force] [--yes]`
pub fn run_rm(project_root: &Path, name: &str, force: bool, yes: bool) -> anyhow::Result<()> {
    let kinds_md_path = project_root.join(EK_DIR).join(KIND_META_DIR).join(KINDS_FILE);

    if !kinds_md_path.exists() {
        anyhow::bail!("{} not found — run `ek init` first", kinds_md_path.display());
    }

    let content = fs::read_to_string(&kinds_md_path)?;
    let user_kinds = kinds::parse_kinds_md(&content);

    // Validate kind exists
    if !user_kinds.iter().any(|k| k.name == name) {
        anyhow::bail!("kind '{name}' not found in kinds.md");
    }

    // Check directory content
    let dir = project_root.join(EK_DIR).join(name);
    if dir.exists() {
        let count = fs::read_dir(&dir)?.count();
        if count > 0 && !force {
            anyhow::bail!("kind '{name}' has {count} entries in {name}/ — use --force to delete");
        }
    }

    // Confirm
    if !yes {
        eprintln!("About to remove kind '{name}' from {EK_DIR}/ (use --yes to confirm)");
    }

    // Remove from kinds.md
    let updated = remove_from_kind_table(&content, name);
    fs::write(&kinds_md_path, updated)?;

    // Remove directory (only with --force)
    if force && dir.exists() {
        fs::remove_dir_all(&dir)?;
    }

    println!("enjoyknowledge: removed kind '{name}'");
    Ok(())
}

/// `ek kind list`
pub fn run_list(project_root: &Path) -> anyhow::Result<()> {
    let kinds_md_path = project_root.join(EK_DIR).join(KIND_META_DIR).join(KINDS_FILE);

    if !kinds_md_path.exists() {
        anyhow::bail!("{} not found — run `ek init` first", kinds_md_path.display());
    }

    let content = fs::read_to_string(&kinds_md_path)?;
    let user_kinds = kinds::parse_kinds_md(&content);

    if user_kinds.is_empty() {
        println!("(no kinds registered)");
        return Ok(());
    }

    println!("| kind | required | summary |");
    println!("|------|----------|---------|");
    for k in &user_kinds {
        println!("| {} | {} | {} |", k.name, k.required.join(", "), k.summary);
    }
    Ok(())
}

// ── helpers ─────────────────────────────────────────────────────────────────

/// Append a row to the kinds.md Markdown table.
/// Inserts before the first HTML comment (`<!--`) or at end of file.
fn append_to_kind_table(content: &str, new_row: &str) -> String {
    let comment_marker = "<!--";
    content.find(comment_marker).map_or_else(
        || format!("{}\n{new_row}", content.trim_end()),
        |pos| {
            let before = &content[..pos];
            let after = &content[pos..];
            let before = before.trim_end();
            format!("{before}\n{new_row}\n\n{after}")
        },
    )
}

/// Remove a row from the kinds.md Markdown table by kind name.
fn remove_from_kind_table(content: &str, name: &str) -> String {
    let mut result = String::new();
    for line in content.lines() {
        let trimmed = line.trim();
        // Match table row starting with | name |
        if trimmed.starts_with(&format!("| {name} ")) || trimmed.starts_with(&format!("| {name} |"))
        {
            continue; // skip this row
        }
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(line);
    }
    // Preserve trailing newline
    if content.ends_with('\n') {
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kinds_md() -> String {
        "---\ndescription: test\ntimestamp: 2026-06-28\n---\n\n# Knowledge Kinds\n\n".to_string()
    }

    fn sample_table() -> String {
        concat!(
            "| kind | required | summary |\n",
            "|------|----------|---------|\n",
            "| gotcha | trigger | Pitfall |\n",
            "| decision | reversible, decided_at | Decision |\n",
            "<!-- dir is derived from kind -->",
        )
        .to_string()
    }

    #[test]
    fn append_to_table_before_comment() {
        let content = sample_kinds_md() + &sample_table();
        let result = append_to_kind_table(&content, "| bug |  | Bug tracker |");
        assert!(result.contains("| bug |  | Bug tracker |"));
        // New row should appear before the comment
        let bug_pos = result.find("| bug |").unwrap();
        let comment_pos = result.find("<!--").unwrap();
        assert!(bug_pos < comment_pos);
    }

    #[test]
    fn append_to_table_no_comment() {
        let content = concat!(
            "| kind | required | summary |\n",
            "|------|----------|---------|\n",
            "| gotcha | trigger | Pitfall |",
        )
        .to_string();
        let result = append_to_kind_table(&content, "| bug |  | Bug |");
        assert!(result.contains("| bug |  | Bug |"));
    }

    #[test]
    fn remove_from_table() {
        let content = sample_kinds_md() + &sample_table();
        let result = remove_from_kind_table(&content, "gotcha");
        assert!(!result.contains("| gotcha |"));
        assert!(result.contains("| decision |"));
        assert!(result.contains("<!-- dir is derived from kind -->"));
    }

    #[test]
    fn remove_nonexistent_keeps_table() {
        let content = sample_kinds_md() + &sample_table();
        let result = remove_from_kind_table(&content, "nonexistent");
        assert_eq!(result, content);
    }
}
