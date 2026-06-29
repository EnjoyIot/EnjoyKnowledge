//! `enjoyknowledge init` — initialize the knowledge base.
use crate::core::template::TemplateProvider;
use crate::core::Profile;
use crate::init;
use crate::EK_DIR;
use std::path::Path;

pub fn run(
    project_path: Option<&str>,
    ai: Option<&str>,
    template: Option<&str>,
    link: Option<&str>,
    profile: &dyn Profile,
) -> anyhow::Result<()> {
    let project_root = match project_path {
        Some(p) => Path::new(p).to_path_buf(),
        None => std::env::current_dir()?,
    };

    // Handle --template list: display available templates and exit
    if let Some(tpl) = template {
        if tpl.eq_ignore_ascii_case("list") {
            let provider = init::default_template_provider();
            let names = provider.list_all();
            if names.is_empty() {
                eprintln!("enjoyknowledge: no templates found (check ~/.enjoyknowledge/templates/ and .enjoyknowledge/templates/)");
            } else {
                println!("Available templates:");
                for name in &names {
                    println!("  {name}");
                }
            }
            return Ok(());
        }
    }

    if let Some(link_path) = link {
        // Link mode: only generate AGENTS.md pointing to external knowledge base
        let linked = Path::new(link_path);
        if !linked.join(EK_DIR).exists() {
            eprintln!("enjoyknowledge: {link_path} does not contain a .enjoyknowledge/ directory");
            std::process::exit(1);
        }

        let agents_content = format!(
            r#"# AGENTS.md

This project uses [enjoyknowledge](https://enjoyknowledge.dev) for shared AI context.

**Knowledge base**: `{link_path}`

## Commands

Run enjoyknowledge commands from the knowledge base directory, or use `enjoyknowledge` with the path:
```bash
enjoyknowledge --root {link_path} ls
```

| Command | Why |
|---|---|
| `enjoyknowledge ls` | List files with descriptions |
| `enjoyknowledge grep <pattern>` | Search inside `##` sections |
| `enjoyknowledge cat <path>` | Read a knowledge file |
| `enjoyknowledge add <path> "<content>"` | Record a new entry |
| `enjoyknowledge doctor` | Health check |
| `enjoyknowledge fix` | Auto-fix common issues |
"#
        );
        std::fs::write(project_root.join("AGENTS.md"), agents_content)?;
        eprintln!("enjoyknowledge: AGENTS.md generated -> linked to {link_path}");
        return Ok(());
    }

    // Check for template
    if let Some(template_name) = template {
        let provider = init::default_template_provider();
        if !init::templates::apply_template(&project_root, template_name, &provider, profile)? {
            eprintln!(
                "enjoyknowledge: template '{template_name}' not found (check ~/.enjoyknowledge/templates/ or .enjoyknowledge/templates/)"
            );
            std::process::exit(1);
        }
        eprintln!("enjoyknowledge: applied template '{template_name}'");
    } else {
        // Default skeleton from profile
        init::skeleton::generate_skeleton(&project_root, profile)?;
    }

    // v0.4: Generate stage skeleton + both AGENTS.md + update .gitignore
    init::skeleton::generate_stage_skeleton(&project_root)?;
    init::skeleton::generate_stage_defaults_md(&project_root)?;
    init::skeleton::generate_ek_agents_md(&project_root)?;
    init::skeleton::generate_stage_agents_md(&project_root)?;
    init::skeleton::generate_skills_skeleton(&project_root)?;
    init::skeleton::update_gitignore(&project_root)?;

    // Generate AGENTS.md
    init::skeleton::generate_agents_md(&project_root, ai, profile)?;

    // Populate the knowledge summary block in AGENTS.md
    {
        let ek = project_root.join(EK_DIR);
        let source = crate::knowledge::filesystem::FilesystemSource::new(&ek, &project_root);
        init::skeleton::sync_agents_md_summary(&project_root, &source)?;
    }

    // Generate AI tool files
    if let Some(tool) = ai {
        let Some(ai_tool) = crate::init::ai_tools::AiTool::from_str(tool) else {
            eprintln!("enjoyknowledge: unknown AI tool '{tool}'");
            eprintln!("Available: auto, cursor, claude, copilot, windsurf, cline, codex, trae, gemini, generic");
            std::process::exit(1);
        };
        crate::init::ai_tools::generate_tool_files(&project_root, ai_tool)?;
    }

    eprintln!("enjoyknowledge: initialized at {}", project_root.display());

    Ok(())
}
