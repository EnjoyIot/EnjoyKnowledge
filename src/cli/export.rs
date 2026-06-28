//! `enjoyknowledge export` — generate AI tool entry files from .enjoyknowledge/
//!
//! v0.2 状态：
//! - 真正支持：Cursor + Claude（v0.2 首发 2 工具）
//! - 其他 7 工具（Copilot / Windsurf / Cline / Codex / Trae / Gemini / Generic）：
//!   v0.2 报"暂未实现"（v0.3+ 渐进；init 一次性仍支持 9 工具）
//! - `auto` = 默认 Claude
//!
//! 行为：复用 [`init::ai_tools::generate_tool_files`] 的工具入口文件生成逻辑，
//! v0.2 export 不做"动态 `SoT` 内容注入"（v0.3+ 工作）；MVP 阶段 = 跨工具入口文件可重生成。

use std::path::Path;

use crate::init::ai_tools::{generate_tool_files, AiTool};

/// v0.2 export 真正支持的工具（首发 2 工具）。
const SUPPORTED_TOOLS: &[AiTool] = &[AiTool::Cursor, AiTool::Claude];

/// 解析 `--tool` 参数为 [`AiTool`]，处理 v0.2 工具矩阵。
///
/// - 真正支持：Cursor / Claude（v0.2 首发）
/// - Auto：默认 Claude
/// - 其他 7 工具：v0.2 报"暂未实现"错误
fn parse_tool(tool_arg: &str) -> anyhow::Result<AiTool> {
    let tool = AiTool::from_str(tool_arg).ok_or_else(|| {
        anyhow::anyhow!("unknown AI tool '{tool_arg}' (valid: cursor, claude, auto)")
    })?;

    if tool == AiTool::Auto {
        return Ok(AiTool::Claude); // v0.2 默认 Claude
    }

    if !SUPPORTED_TOOLS.contains(&tool) {
        anyhow::bail!(
            "v0.2 export --tool {}: 暂未实现（v0.2 首发 2 工具：cursor / claude；其他工具 v0.3+）",
            tool.label()
        );
    }

    Ok(tool)
}

/// 运行 export 命令。
///
/// # Arguments
/// - `project_root`: 项目根路径（通常是 `.`）
/// - `tool_arg`: `--tool` 参数（cursor / claude / auto / 其他 7 工具）
/// - `dry_run`: true = 只打印不写
pub fn run(project_root: &Path, tool_arg: &str, dry_run: bool) -> anyhow::Result<()> {
    let tool = parse_tool(tool_arg)?;

    if dry_run {
        println!(
            "[dry-run] would generate {} entry file at {}",
            tool.label(),
            project_root.display()
        );
        match tool {
            AiTool::Cursor => {
                println!("  → .cursor/rules/enjoyknowledge.mdc");
            }
            AiTool::Claude => {
                println!("  → .claude/skills/enjoyknowledge.md");
            }
            _ => unreachable!("parse_tool 已过滤"),
        }
        return Ok(());
    }

    generate_tool_files(project_root, tool)?;

    let target = match tool {
        AiTool::Cursor => ".cursor/rules/enjoyknowledge.mdc",
        AiTool::Claude => ".claude/skills/enjoyknowledge.md",
        _ => unreachable!("parse_tool 已过滤"),
    };

    println!("✓ Generated {} entry file: {}", tool.label(), target);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cursor_supported() {
        assert_eq!(parse_tool("cursor").unwrap(), AiTool::Cursor);
    }

    #[test]
    fn parse_claude_supported() {
        assert_eq!(parse_tool("claude").unwrap(), AiTool::Claude);
    }

    #[test]
    fn parse_auto_defaults_to_claude() {
        assert_eq!(parse_tool("auto").unwrap(), AiTool::Claude);
    }

    #[test]
    fn parse_codex_unsupported_v0_2() {
        let err = parse_tool("codex").unwrap_err();
        assert!(err.to_string().contains("v0.2"));
        assert!(err.to_string().contains("暂未实现"));
    }

    #[test]
    fn parse_unknown_error() {
        let err = parse_tool("notarealtool").unwrap_err();
        assert!(err.to_string().contains("unknown AI tool"));
    }

    #[test]
    fn supported_tools_only_cursor_and_claude() {
        assert_eq!(SUPPORTED_TOOLS.len(), 2);
        assert!(SUPPORTED_TOOLS.contains(&AiTool::Cursor));
        assert!(SUPPORTED_TOOLS.contains(&AiTool::Claude));
    }
}
