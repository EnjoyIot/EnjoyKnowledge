# v0.4.8 完工复盘 — enjoyknowledge skills/ 流程类 skill（4 个工作流）

**日期**：2026-06-28
**版本**：v0.4.8

## 做了什么

抽取 4 个工作流到 `.enjoyknowledge/skills/` 独立目录，AI 工具通过 AGENTS.md 引导读取。0 新格式（Hermes skill 格式），init 不覆盖用户修改。

## 改动清单

| 文件 | 改动 |
|------|------|
| `src/init/skeleton.rs` | 加 `SKILLS_DIR` 常量 + `SKILLS_*_MD_CONTENT` 5 个常量 + `generate_skills_skeleton()` 函数 |
| `src/cli/init.rs` | 加 1 行：`generate_skills_skeleton()` 调用 |
| `tests/integration.rs` | 加 6 个集成测试（4 flow frontmatter + 1 index + 1 不覆盖）|
| `tests/fixtures/minimal-project/.enjoyknowledge/skills/` | 新建 5 个 fixture 文件 |
| `tests/fixtures/minimal-project/test-project/.enjoyknowledge/skills/` | 新建 5 个 fixture 文件 |
| `tests/cmd/onboard-ok.stdout` | 更新快照（6→11 文件数）|
| `docs/04-changelog/CHANGELOG.md` | 加 v0.4.8 段 |

## 5 个 skills/ 默认文件

| 文件 | 内容 | 行数 |
|------|------|------|
| `coding.md` | 编码工作流（接任务 → drafts → promote）| ~55 |
| `research.md` | 调研工作流（ek ls/tree/cat/grep）| ~45 |
| `review.md` | 复盘工作流（onboard → drafts → promote → doctor）| ~50 |
| `design.md` | 设计工作流（ek kind add/rm/list）| ~50 |
| `README.md` | 4 个工作流索引 + 怎么用 + 自定义 | ~25 |

## 新增测试

```rust
// 1. 5 个文件创建测试
fn init_creates_skills_directory_with_4_flows_and_index()

// 2. 不覆盖测试（核心）
fn init_does_not_overwrite_user_skills_files()

// 3-6. 4 个 frontmatter 测试
fn init_skills_coding_md_has_correct_frontmatter()
fn init_skills_research_md_has_correct_frontmatter()
fn init_skills_review_md_has_correct_frontmatter()
fn init_skills_design_md_has_correct_frontmatter()
```

## 测试统计

- v0.4.7：106 单 + 25 trycmd + 28 集成 = 159
- v0.4.8：106 单 + 25 trycmd + 34 集成 = **165**

## 哲学一致性

| 维度 | 行为 |
|------|------|
| 新格式 | 0（Hermes skill 格式 = v0.4.6 已有）|
| init 覆盖 | 永不覆盖用户文件（v0.4.4/0.4.6/0.4.7 模式）|
| AGENTS.md | 不改（v0.4.7 已对）|
| stage AGENTS.md | 不改（v0.4.7 已对）|
| 多 AI 工具 | 项目内子目录 = AGENTS.md 引导 |

## v0.4.9 候选

- 无明确候选。v0.4.X 系列已完整（v0.4.3 ~ v0.4.8 = 6 个版本）。
- v0.5 方向：团队共享 / 多项目知识链接 / ek sync。
