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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "cursor" => Some(Self::Cursor),
            "claude" => Some(Self::Claude),
            "copilot" => Some(Self::Copilot),
            "windsurf" => Some(Self::Windsurf),
            "cline" => Some(Self::Cline),
            "codex" => Some(Self::Codex),
            "trae" => Some(Self::Trae),
            "gemini" => Some(Self::Gemini),
            "generic" => Some(Self::Generic),
            "auto" => Some(Self::Auto),
            _ => None,
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
            std::fs::write(dir.join("enjoyknowledge.md"), WINDSURF_RULES)?;
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
const GENERIC_TOOL_FILE: &str = r"# enjoyknowledge - Shared AI Context

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

const WINDSURF_RULES: &str = r"# enjoyknowledge

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

#[cfg(test)]
mod tests {
    use super::*;

    // ── Variant mapping: every tool name → correct variant ──

    #[test]
    fn from_str_cursor() {
        assert_eq!(AiTool::from_str("cursor"), Some(AiTool::Cursor));
    }

    #[test]
    fn from_str_claude() {
        assert_eq!(AiTool::from_str("claude"), Some(AiTool::Claude));
    }

    #[test]
    fn from_str_copilot() {
        assert_eq!(AiTool::from_str("copilot"), Some(AiTool::Copilot));
    }

    #[test]
    fn from_str_windsurf() {
        assert_eq!(AiTool::from_str("windsurf"), Some(AiTool::Windsurf));
    }

    #[test]
    fn from_str_cline() {
        assert_eq!(AiTool::from_str("cline"), Some(AiTool::Cline));
    }

    #[test]
    fn from_str_codex() {
        assert_eq!(AiTool::from_str("codex"), Some(AiTool::Codex));
    }

    #[test]
    fn from_str_trae() {
        assert_eq!(AiTool::from_str("trae"), Some(AiTool::Trae));
    }

    #[test]
    fn from_str_gemini() {
        assert_eq!(AiTool::from_str("gemini"), Some(AiTool::Gemini));
    }

    #[test]
    fn from_str_generic() {
        assert_eq!(AiTool::from_str("generic"), Some(AiTool::Generic));
    }

    #[test]
    fn from_str_auto() {
        assert_eq!(AiTool::from_str("auto"), Some(AiTool::Auto));
    }

    // ── Case insensitivity ──

    #[test]
    fn from_str_case_all_upper() {
        assert_eq!(AiTool::from_str("CURSOR"), Some(AiTool::Cursor));
    }

    #[test]
    fn from_str_case_mixed_pascal() {
        assert_eq!(AiTool::from_str("Cursor"), Some(AiTool::Cursor));
    }

    #[test]
    fn from_str_case_mixed_random() {
        assert_eq!(AiTool::from_str("cUrSoR"), Some(AiTool::Cursor));
    }

    // ── Unknown / invalid input → None ──

    #[test]
    fn from_str_unknown_tool() {
        assert_eq!(AiTool::from_str("notarealtool"), None);
    }

    #[test]
    fn from_str_empty_string() {
        assert_eq!(AiTool::from_str(""), None);
    }

    #[test]
    fn from_str_whitespace_not_valid() {
        // " auto" with leading space is not a valid tool name
        assert_eq!(AiTool::from_str(" auto"), None);
    }

    // ── Regression: C1 fix must not break windsurf lookup ──

    #[test]
    fn from_str_windsurf_still_works_after_c1_fix() {
        assert_eq!(AiTool::from_str("windsurf"), Some(AiTool::Windsurf));
    }

    // ── Auto is Auto, not a fallback to another tool ──

    #[test]
    fn from_str_auto_is_not_cursor() {
        assert_eq!(AiTool::from_str("auto"), Some(AiTool::Auto));
        assert_ne!(AiTool::from_str("auto"), Some(AiTool::Cursor));
    }
}
