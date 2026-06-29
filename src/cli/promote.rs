//! `enjoyknowledge promote` — promote a draft from stage/drafts/ to the knowledge base.

use crate::config::{self, EK_DIR, STAGE_DIR, DRAFTS_DIR};
use crate::kinds;

use std::path::Path;

pub fn run(
    project_root: &Path,
    draft_file: &str,
    kind: &str,
    id: Option<&str>,
    author: Option<&str>,
) -> anyhow::Result<()> {
    let draft_path = project_root.join(STAGE_DIR).join(DRAFTS_DIR).join(draft_file);

    if !draft_path.exists() {
        anyhow::bail!("draft not found: {STAGE_DIR}/{DRAFTS_DIR}/{draft_file}");
    }

    // Validate kind
    let kind_dir = kinds::dir_for(kind);

    // Derive target filename
    let target_stem = id.unwrap_or_else(|| draft_file.strip_suffix(".md").unwrap_or(draft_file));
    let target_rel = format!("{kind_dir}/{target_stem}.md");

    let target_path = project_root.join(EK_DIR).join(&target_rel);
    if let Some(parent) = target_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if target_path.exists() {
        anyhow::bail!(
            "target already exists: .enjoyknowledge/{target_rel} (use a different --id or remove the existing file)"
        );
    }

    // Read draft content (skip any frontmatter — use body only)
    let draft_content = std::fs::read_to_string(&draft_path)?;
    let body = crate::format::document::find_body_start(&draft_content);
    let body_content = draft_content[body..].trim().to_string();

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let author_val = author.unwrap_or(config::DEFAULT_AUTHOR);
    let promote_id = target_stem;

    // Build KB file with 4-field frontmatter
    let kb_content = format!(
        "---\nid: {promote_id}\nkind: {kind}\ncreated: {today}\nauthor: {author_val}\n---\n\n{body_content}\n"
    );

    std::fs::write(&target_path, kb_content)?;

    // Mark draft as promoted
    let promoted_marker =
        format!("\n\n<!-- [PROMOTED] → .enjoyknowledge/{target_rel} on {today} -->\n");
    let mut draft_with_marker = draft_content;
    if !draft_with_marker.ends_with('\n') {
        draft_with_marker.push('\n');
    }
    draft_with_marker.push_str(&promoted_marker);
    std::fs::write(&draft_path, draft_with_marker)?;

    eprintln!("enjoyknowledge: promoted {promote_id} → .enjoyknowledge/{target_rel} [{kind}]");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn promote_creates_file_with_frontmatter() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();

        // Set up stage structure
        let drafts_dir = root.join(".enjoyknowledge_stage/drafts");
        std::fs::create_dir_all(&drafts_dir).unwrap();
        std::fs::write(
            drafts_dir.join("utf8-gotcha.md"),
            "---\ntitle: Draft\n---\n\n## Windows UTF-8\n- Gotcha about encoding\n",
        )
        .unwrap();

        // Set up KB structure
        std::fs::create_dir_all(root.join(".enjoyknowledge/gotcha")).unwrap();

        run(root, "utf8-gotcha.md", "gotcha", None, Some("enjoy")).unwrap();

        let target = root.join(".enjoyknowledge/gotcha/utf8-gotcha.md");
        assert!(target.exists());
        let content = std::fs::read_to_string(&target).unwrap();
        assert!(content.contains("id: utf8-gotcha"));
        assert!(content.contains("kind: gotcha"));
        assert!(content.contains("author: enjoy"));
        assert!(content.contains("## Windows UTF-8"));
        assert!(!content.contains("title: Draft")); // frontmatter stripped
    }

    #[test]
    fn promote_adds_promoted_marker_to_draft() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();

        let drafts_dir = root.join(".enjoyknowledge_stage/drafts");
        std::fs::create_dir_all(&drafts_dir).unwrap();
        std::fs::write(drafts_dir.join("my-draft.md"), "## My Draft\n- Some content\n").unwrap();

        std::fs::create_dir_all(root.join(".enjoyknowledge/gotcha")).unwrap();

        run(root, "my-draft.md", "gotcha", None, Some("enjoy")).unwrap();

        let draft = std::fs::read_to_string(drafts_dir.join("my-draft.md")).unwrap();
        assert!(draft.contains("[PROMOTED]"));
        assert!(draft.contains(".enjoyknowledge/gotcha/my-draft.md"));
    }

    #[test]
    fn promote_with_custom_id() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();

        let drafts_dir = root.join(".enjoyknowledge_stage/drafts");
        std::fs::create_dir_all(&drafts_dir).unwrap();
        std::fs::write(drafts_dir.join("draft.md"), "## Content\n- test\n").unwrap();

        std::fs::create_dir_all(root.join(".enjoyknowledge/gotcha")).unwrap();

        run(root, "draft.md", "gotcha", Some("custom-id"), Some("enjoy")).unwrap();

        assert!(root.join(".enjoyknowledge/gotcha/custom-id.md").exists());
        let content =
            std::fs::read_to_string(root.join(".enjoyknowledge/gotcha/custom-id.md")).unwrap();
        assert!(content.contains("id: custom-id"));
    }

    #[test]
    fn promote_missing_draft_errors() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();

        let result = run(root, "nonexistent.md", "gotcha", None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("draft not found"));
    }

    #[test]
    fn promote_existing_target_errors() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();

        let drafts_dir = root.join(".enjoyknowledge_stage/drafts");
        std::fs::create_dir_all(&drafts_dir).unwrap();
        std::fs::write(drafts_dir.join("dup.md"), "## duplicate\n- test\n").unwrap();

        let gd = root.join(".enjoyknowledge/gotcha");
        std::fs::create_dir_all(&gd).unwrap();
        std::fs::write(gd.join("dup.md"), "existing content").unwrap();

        let result = run(root, "dup.md", "gotcha", None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }
}
