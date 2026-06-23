/// Generate AI-tool-specific configuration files so each tool can discover
/// and use the enjoyknowledge knowledge base.
///
/// Supported tools (9 total):
/// - Auto    — detect which tool is present at project root
/// - Cursor  — `.cursor/rules/enjoyknowledge.mdc`
/// - Claude  — `.claude/skills/enjoyknowledge.md`
/// - Copilot — `.github/copilot-instructions.md` (appended)
/// - Windsurf— `.windsurf/rules/enjoyknowledge.md`
/// - Cline   — `.clinerules/enjoyknowledge.md`
/// - Codex   — `.codex/prompts/enjoyknowledge.md`
/// - Trae    — `.trae/rules/enjoyknowledge.md`
/// - Gemini  — `GEMINI.md` (appended)
use std::path::Path;

/// Supported AI coding tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiTool {
    Auto,
    Cursor,
    Claude,
    Copilot,
    Windsurf,
    Cline,
    Codex,
    Trae,
    Gemini,
    Generic,
}

impl AiTool {
    /// Parse a tool name from the `--ai` flag value (case-insensitive).
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "cursor" => Self::Cursor,
            "claude" => Self::Claude,
            "copilot" => Self::Copilot,
            "windsurf" => Self::Windsurf,
            "cline" => Self::Cline,
            "codex" => Self::Codex,
            "trae" => Self::Trae,
            "gemini" => Self::Gemini,
            "generic" => Self::Generic,
            _ => Self::Auto,
        }
    }

    /// Human-readable label for diagnostics.
    #[allow(dead_code)]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Cursor => "Cursor",
            Self::Claude => "Claude",
            Self::Copilot => "Copilot",
            Self::Windsurf => "Windsurf",
            Self::Cline => "Cline",
            Self::Codex => "Codex",
            Self::Trae => "Trae",
            Self::Gemini => "Gemini",
            Self::Generic => "Generic",
        }
    }
}

/// Generate the tool-specific rules/prompts file under `project_root`.
///
/// `Auto` does nothing (AGENTS.md alone is sufficient).
pub fn generate_tool_files(project_root: &Path, tool: AiTool) -> anyhow::Result<()> {
    match tool {
        AiTool::Cursor => {
            let dir = project_root.join(".cursor").join("rules");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyknowledge.mdc"), CURSOR_RULES)?;
        }
        AiTool::Claude => {
            let dir = project_root.join(".claude").join("skills");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyknowledge.md"), CLAUDE_SKILL)?;
        }
        AiTool::Copilot => {
            let dir = project_root.join(".github");
            std::fs::create_dir_all(&dir)?;
            let path = dir.join("copilot-instructions.md");
            let existing = std::fs::read_to_string(&path).unwrap_or_default();
            if !existing.contains("enjoyknowledge") {
                std::fs::write(&path, format!("{existing}\n{COPILOT_BLOCK}"))?;
            }
        }
        AiTool::Windsurf => {
            let dir = project_root.join(".windsurf").join("rules");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyknowledge.md"), WINDSULF_RULES)?;
        }
        AiTool::Cline => {
            let dir = project_root.join(".clinerules");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyknowledge.md"), CLINE_RULES)?;
        }
        AiTool::Codex => {
            let dir = project_root.join(".codex").join("prompts");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyknowledge.md"), CODEX_PROMPT)?;
        }
        AiTool::Trae => {
            let dir = project_root.join(".trae").join("rules");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyknowledge.md"), TRAE_RULES)?;
        }
        AiTool::Gemini => {
            let path = project_root.join("GEMINI.md");
            let existing = std::fs::read_to_string(&path).unwrap_or_default();
            if !existing.contains("enjoyknowledge") {
                std::fs::write(&path, format!("{existing}\n{GEMINI_BLOCK}"))?;
            }
        }
        AiTool::Auto => {
            // auto-detect: AGENTS.md alone is sufficient
        }
        AiTool::Generic => {
            let path = project_root.join("enjoyknowledge.md");
            std::fs::write(&path, GENERIC_TOOL_FILE)?;
        }
    }
    Ok(())
}

/// Append an enjoyknowledge entry to `.gitignore`.
#[allow(dead_code)]
pub fn update_gitignore(root: &Path) -> anyhow::Result<()> {
    let path = root.join(".gitignore");
    let existing = std::fs::read_to_string(&path).unwrap_or_default();
    let entries = "\n# enjoyknowledge\n";
    if !existing.contains("# enjoyknowledge") {
        std::fs::write(&path, format!("{existing}{entries}"))?;
    }
    Ok(())
}

// ── Tool rule templates ────────────────────────────────────────────────

const CURSOR_RULES: &str = r"---
alwaysApply: true
---
# enjoyknowledge — Shared AI Context

This project uses enjoyknowledge to manage shared coding context.
When given a task, run the corresponding search command to load relevant knowledge:

```bash
enjoyknowledge ls                    # list available knowledge
enjoyknowledge grep <query>          # search inside ## sections  
enjoyknowledge cat <path>            # read a knowledge file
enjoyknowledge add <path> <content>  # record a new finding
```

Relevant directories: .enjoyknowledge/architecture/, gotchas/, patterns/, business/, decisions/
";

const CLAUDE_SKILL: &str = r"# enjoyknowledge Skill

This project uses enjoyknowledge to manage shared coding context.

## How to Use

When the user begins a coding task, follow these rules:
1. Identify the scenario (new feature / bug fix / refactor / hotfix / architecture decision)
2. Run `enjoyknowledge grep <query>` to get relevant context
3. After the task, run `enjoyknowledge add <path> <content>` to record new findings

Available commands:
- `enjoyknowledge ls` — list knowledge files with descriptions
- `enjoyknowledge grep <query>` — structure-aware search
- `enjoyknowledge cat <path>` — read a knowledge file
- `enjoyknowledge add <path> <content>` — record an entry
";

const COPILOT_BLOCK: &str = r"
## enjoyknowledge

This project uses enjoyknowledge to manage shared AI coding context.
Run `enjoyknowledge grep <query>` to search relevant knowledge before starting work.
";

/// Tool-agnostic enjoyknowledge.md for any AI coding tool that can read a
/// project-level Markdown file.
const GENERIC_TOOL_FILE: &str = r"# enjoyknowledge 鈥?Shared AI Context

This project uses [enjoyknowledge](https://enjoyknowledge.dev) to manage
shared context for AI coding tools.

## How to Use

Before starting any coding task, check the knowledge base for relevant context:

```bash
enjoyknowledge ls                    # list available knowledge
enjoyknowledge grep <query>          # search inside ## sections
enjoyknowledge cat <path>            # read a knowledge file
enjoyknowledge add <path> <content>  # record a new finding
enjoyknowledge doctor                # health check
```

Relevant directories under `.enjoyknowledge/`: architecture, gotchas, patterns,
business, decisions.

## When to Contribute

After a coding task, record new findings:
- Gotchas you encountered and how you resolved them
- Architecture decisions and their rationale
- Business rules or constraints that affected implementation
- Patterns or conventions the team follows

---
*Generated by enjoyknowledge init --ai generic*
";

const WINDSULF_RULES: &str = r"# enjoyknowledge

This project uses enjoyknowledge to manage shared AI coding context.
Before starting any coding task, run `enjoyknowledge grep <query>` to check for relevant knowledge.
";

const CLINE_RULES: &str = r"# enjoyknowledge

This project uses enjoyknowledge to manage shared AI coding context.
Before starting any coding task, run `enjoyknowledge grep <query>` to check for relevant knowledge.
";

const CODEX_PROMPT: &str = r"# enjoyknowledge

This project uses enjoyknowledge to manage shared AI coding context.
Before starting any coding task, run `enjoyknowledge grep <query>` to check for relevant knowledge.
";

const TRAE_RULES: &str = r"# enjoyknowledge

This project uses enjoyknowledge to manage shared AI coding context.
Before starting any coding task, run `enjoyknowledge grep <query>` to check for relevant knowledge.

Available commands:
- `enjoyknowledge ls` — list knowledge files
- `enjoyknowledge grep <query>` — search inside ## sections
- `enjoyknowledge cat <path>` — read a knowledge file
- `enjoyknowledge add <path> <content>` — record a new entry
";

const GEMINI_BLOCK: &str = r"
## enjoyknowledge

This project uses enjoyknowledge to manage shared AI coding context.
Run `enjoyknowledge grep <query>` to search relevant knowledge before starting work.
";
