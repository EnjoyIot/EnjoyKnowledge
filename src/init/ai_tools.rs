/// AI 工具文件生成
use std::path::Path;

/// 支持的 AI 工具
#[derive(Debug, Clone, Copy)]
pub enum AiTool {
    Auto,
    Cursor,
    Claude,
    Copilot,
    Windsurf,
    Cline,
    Codex,
    Gemini,
}

impl AiTool {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "cursor" => Self::Cursor,
            "claude" => Self::Claude,
            "copilot" => Self::Copilot,
            "windsurf" => Self::Windsurf,
            "cline" => Self::Cline,
            "codex" => Self::Codex,
            "gemini" => Self::Gemini,
            _ => Self::Auto,
        }
    }
}

/// 生成 AGENTS.md（所有工具通用）
pub fn generate_agents_md(root: &Path) -> anyhow::Result<()> {
    let content = AGENTS_MD_TEMPLATE;
    std::fs::write(root.join("AGENTS.md"), content)?;
    Ok(())
}

/// 根据 --ai 选项生成工具专用文件
pub fn generate_tool_files(root: &Path, tool: AiTool) -> anyhow::Result<()> {
    match tool {
        AiTool::Cursor => {
            let dir = root.join(".cursor").join("rules");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyflow.mdc"), CURSOR_RULES)?;
        }
        AiTool::Claude => {
            let dir = root.join(".claude").join("skills");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyflow.md"), CLAUDE_SKILL)?;
        }
        AiTool::Copilot => {
            let dir = root.join(".github");
            std::fs::create_dir_all(&dir)?;
            let path = dir.join("copilot-instructions.md");
            let existing = std::fs::read_to_string(&path).unwrap_or_default();
            std::fs::write(&path, format!("{existing}\n\n{COPILOT_BLOCK}"))?;
        }
        AiTool::Windsurf => {
            let dir = root.join(".windsurf").join("rules");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyflow.md"), WINDSULF_RULES)?;
        }
        AiTool::Cline => {
            let dir = root.join(".clinerules");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyflow.md"), CLINE_RULES)?;
        }
        AiTool::Codex => {
            let dir = root.join(".codex").join("prompts");
            std::fs::create_dir_all(&dir)?;
            std::fs::write(dir.join("enjoyflow.md"), CODEX_PROMPT)?;
        }
        AiTool::Gemini => {
            let path = root.join("GEMINI.md");
            let existing = std::fs::read_to_string(&path).unwrap_or_default();
            std::fs::write(&path, format!("{existing}\n\n{GEMINI_BLOCK}"))?;
        }
        AiTool::Auto => {
            // auto 检测：默认只生成 AGENTS.md
        }
    }
    Ok(())
}

/// 追加 EnjoyFlow 条目到 .gitignore
pub fn update_gitignore(root: &Path) -> anyhow::Result<()> {
    let path = root.join(".gitignore");
    let existing = std::fs::read_to_string(&path).unwrap_or_default();
    let entries = "\n# EnjoyFlow\n.enjoyflow/.doctor-cache\n";
    if !existing.contains(".enjoyflow/.doctor-cache") {
        std::fs::write(&path, format!("{existing}{entries}"))?;
    }
    Ok(())
}

const AGENTS_MD_TEMPLATE: &str = r#"# AGENTS.md

本项目使用 [EnjoyFlow](https://enjoyflow.dev) 管理 AI 编程的上下文知识。

## 可用命令

- `enjoyflow search <query> [--class <class>] [--tag <tag>]` — 搜索知识库
- `enjoyflow record gotcha --tag <tag> --content "..."` — 记录踩坑
- `enjoyflow record pattern --tag <tag> --content "..."` — 记录最佳实践
- `enjoyflow record decision --task <REQ-ID> --content "..."` — 记录架构决策
- `enjoyflow show <REQ-ID>` — 查看任务状态
- `enjoyflow doctor` — 诊断知识库健康度

## 场景建议

| 场景 | 建议先查 |
|---|---|
| 修 Bug | `--class gotchas --class architecture --class known_issues` |
| 新功能 | `--class architecture --class code_standards --class patterns` |
| 重构 | `--class architecture --class gotchas --class decisions` |
| Hotfix | `--class architecture --class gotchas --class known_issues` |
| 架构决策 | `--class architecture --class decisions` |
| 发版部署 | `--class environments --class deployment` |
| 代码审查 | `--class code_standards --class review_checklist` |
| 监控响应 | `--class architecture --class known_issues --class failure_modes` |

这不是强制预设——AI 根据实际情况自己判断需要搜什么。
"#;

const CURSOR_RULES: &str = r#"---
alwaysApply: true
---
# EnjoyFlow — 上下文知识供给

本项目使用 EnjoyFlow 管理 AI 编程的上下文知识。

当用户提到任务时，运行对应搜索命令获取上下文。
"#;

const CLAUDE_SKILL: &str = r#"# EnjoyFlow Skill

本项目使用 EnjoyFlow 管理 AI 编程的上下文知识。

## 使用方式

当用户开始编码任务时，按以下规则判断：

1. 识别场景（new_feature / bug_fix / refactor / hotfix / architecture_decision）
2. 运行 `enjoyflow search <query>` 获取相关上下文
3. 任务结束后，运行 `enjoyflow record` 记录新发现的坑/模式
"#;

const COPILOT_BLOCK: &str = "\n## EnjoyFlow\n\n本项目使用 EnjoyFlow 管理 AI 编程上下文。运行 `enjoyflow search <query>` 获取相关知识。";

const WINDSULF_RULES: &str = "# EnjoyFlow\n本项目使用 EnjoyFlow 管理 AI 编程上下文。运行 `enjoyflow search <query>` 获取相关知识。";

const CLINE_RULES: &str = "# EnjoyFlow\n本项目使用 EnjoyFlow 管理 AI 编程上下文。运行 `enjoyflow search <query>` 获取相关知识。";

const CODEX_PROMPT: &str = "# EnjoyFlow\n本项目使用 EnjoyFlow 管理 AI 编程上下文。运行 `enjoyflow search <query>` 获取相关知识。";

const GEMINI_BLOCK: &str = "\n## EnjoyFlow\n\n本项目使用 EnjoyFlow 管理 AI 编程上下文。运行 `enjoyflow search <query>` 获取相关知识。";
