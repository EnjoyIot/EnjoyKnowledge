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

    /// Project-relative file path where this tool expects its config.
    pub const fn file_path(self) -> &'static str {
        match self {
            Self::Cursor => ".cursor/rules/enjoyknowledge.mdc",
            Self::Claude => ".claude/skills/enjoyknowledge.md",
            Self::Copilot => ".github/copilot-instructions.md",
            Self::Windsurf => ".windsurf/rules/enjoyknowledge.md",
            Self::Cline => ".clinerules/enjoyknowledge.md",
            Self::Codex => ".codex/prompts/enjoyknowledge.md",
            Self::Trae => ".trae/rules/enjoyknowledge.md",
            Self::Gemini => "GEMINI.md",
            Self::Generic => "enjoyknowledge.md",
            Self::Auto => "AGENTS.md",
        }
    }

    /// Whether this tool appends to an existing file (vs. creating a standalone file).
    pub const fn is_append_mode(self) -> bool {
        matches!(self, Self::Copilot | Self::Gemini)
    }

    /// The template content for this tool's config file.
    pub const fn content(self) -> &'static str {
        match self {
            Self::Cursor => CURSOR_RULES,
            Self::Claude => CLAUDE_SKILL,
            Self::Copilot => COPILOT_BLOCK,
            Self::Windsurf => WINDSURF_RULES,
            Self::Cline => CLINE_RULES,
            Self::Codex => CODEX_PROMPT,
            Self::Trae => TRAE_RULES,
            Self::Gemini => GEMINI_BLOCK,
            Self::Generic => GENERIC_TOOL_FILE,
            Self::Auto => "",
        }
    }
}

/// Generate the tool-specific rules/prompts file under `project_root`.
///
/// `Auto` does nothing (AGENTS.md alone is sufficient).
pub fn generate_tool_files(project_root: &Path, tool: AiTool) -> anyhow::Result<()> {
    if tool == AiTool::Auto {
        return Ok(());
    }

    let rel_path = tool.file_path();
    let full_path = project_root.join(rel_path);

    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = tool.content();

    if tool.is_append_mode() {
        let existing = std::fs::read_to_string(&full_path).unwrap_or_default();
        if !existing.contains("enjoyknowledge") {
            std::fs::write(&full_path, format!("{existing}\n{content}"))?;
        }
    } else {
        std::fs::write(&full_path, content)?;
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

const CURSOR_RULES: &str = include_str!("../fixtures/tools/cursor.md");

const CLAUDE_SKILL: &str = include_str!("../fixtures/tools/claude.md");

const COPILOT_BLOCK: &str = include_str!("../fixtures/tools/copilot.md");

/// Tool-agnostic enjoyknowledge.md for any AI coding tool that can read a
/// project-level Markdown file.
const GENERIC_TOOL_FILE: &str = include_str!("../fixtures/tools/generic.md");

const WINDSURF_RULES: &str = include_str!("../fixtures/tools/windsurf.md");

const CLINE_RULES: &str = include_str!("../fixtures/tools/cline.md");

const CODEX_PROMPT: &str = include_str!("../fixtures/tools/codex.md");

const TRAE_RULES: &str = include_str!("../fixtures/tools/trae.md");

const GEMINI_BLOCK: &str = include_str!("../fixtures/tools/gemini.md");

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
