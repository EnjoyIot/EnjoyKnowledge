/// Template system for enjoyknowledge — apply pre-built knowledge skeletons.
///
/// Templates live in two places (resolved by `TemplateProvider`):
/// - Global: `~/.enjoyknowledge/templates/<name>/` (shared across projects)
/// - Local:  `.enjoyknowledge/templates/<name>/` (project-specific)
///
/// A template directory contains:
/// - `.enjoyknowledge/` — the knowledge base skeleton (directories + seed files)
/// - `knowledge-tasks/` — optional task directory
/// - `_AGENTS.md` — optional AGENTS.md template (overrides the default)
///
/// # Custom Template Management
///
/// Create a template by copying an existing `.enjoyknowledge/` directory
/// into the templates folder and adding a `_AGENTS.md` if desired:
///
/// ```bash
/// # Create a global template
/// mkdir -p ~/.enjoyknowledge/templates/my-template/.enjoyknowledge
/// cp -r .enjoyknowledge/* ~/.enjoyknowledge/templates/my-template/.enjoyknowledge/
/// cp AGENTS.md ~/.enjoyknowledge/templates/my-template/_AGENTS.md
/// ```
///
/// The default `for-coding` template ships with 5 knowledge categories:
/// `architecture`, `gotchas`, `patterns`, `business`, `decisions`.
use crate::core::{Profile, TemplateProvider};
use crate::EK_DIR;
use std::path::Path;

/// Apply a named template to `project_root`.
///
/// Returns `true` if the template was found and applied, `false` if
/// no template with that name exists in any search path.
pub fn apply_template(
    project_root: &Path,
    name: &str,
    provider: &dyn TemplateProvider,
    profile: &dyn Profile,
) -> anyhow::Result<bool> {
    let Some(template_dir) = provider.resolve(name) else { return Ok(false) };

    // 1. Copy .enjoyknowledge/ skeleton
    let ek_src = template_dir.join(EK_DIR);
    if ek_src.exists() {
        copy_dir_recursive(&ek_src, &project_root.join(EK_DIR))?;
    } else {
        // Fall back to profile-defined skeleton
        super::skeleton::generate_skeleton(project_root, profile)?;
    }

    // 2. Copy knowledge-tasks/ if present
    let tasks_src = template_dir.join("knowledge-tasks");
    if tasks_src.exists() {
        copy_dir_recursive(&tasks_src, &project_root.join("knowledge-tasks"))?;
    } else {
        std::fs::create_dir_all(project_root.join("knowledge-tasks"))?;
    }

    // 3. Generate AGENTS.md from template _AGENTS.md or default
    let agents_src = template_dir.join("_AGENTS.md");
    let agents_content = if agents_src.exists() {
        let mut content = std::fs::read_to_string(&agents_src)?;
        // Replace template name placeholder
        content = content.replace("{{TEMPLATE_NAME}}", name);
        content
    } else {
        super::skeleton::generate_agents_md_content(None, profile)
    };

    std::fs::write(project_root.join("AGENTS.md"), agents_content)?;

    Ok(true)
}

/// Recursively copy a directory tree.
fn copy_dir_recursive(src: &Path, dst: &Path) -> anyhow::Result<()> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
