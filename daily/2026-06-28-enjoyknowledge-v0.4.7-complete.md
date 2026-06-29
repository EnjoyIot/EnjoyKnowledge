# v0.4.7 完工复盘 — enjoyknowledge AGENTS.md 默认内容重写

**日期**：2026-06-28
**版本**：v0.4.7

## 做了什么

2 个 AGENTS.md 默认内容重写：静态目录说明 + briefly 提到流程（1-2 句）。

## 改动清单

| 文件 | 改动 |
|------|------|
| `src/init/skeleton.rs` | 重写 `ek_agents_md_content` + `STAGE_AGENTS_MD_CONTENT` + 更新 2 个单元测试断言 |
| `tests/integration.rs` | 更新 2 个现有集成测试断言 + 新增 3 个集成测试 |
| `tests/fixtures/minimal-project/.enjoyknowledge/AGENTS.md` | 新建 fixture（v0.4.7 默认内容） |
| `tests/fixtures/minimal-project/.enjoyknowledge_stage/AGENTS.md` | 新建 fixture（v0.4.7 默认内容） |
| `tests/fixtures/minimal-project/test-project/.enjoyknowledge/AGENTS.md` | 更新 fixture |
| `tests/fixtures/minimal-project/test-project/.enjoyknowledge_stage/AGENTS.md` | 更新 fixture |
| `docs/04-changelog/CHANGELOG.md` | 加 v0.4.7 段 |

## 新增测试

```rust
// 1. init 不覆盖用户改的 ek AGENTS.md（核心：USER-MARKER-12345 保留）
fn init_does_not_overwrite_user_ek_agents_md()

// 2. 默认模板包含静态目录说明 + briefly 提到流程
fn init_creates_default_ek_agents_md_with_static_dir_and_briefly_flow()

// 3. stage AGENTS.md 包含任务执行流程
fn init_creates_stage_agents_md_with_task_execution_workflow()
```

## AGENTS.md 默认内容变化

| 维度 | v0.4.6 | v0.4.7 |
|------|--------|--------|
| 语言 | 英文 | 中文 |
| 11 kind 介绍 | KB_INDEX 表格 | 目录树图 |
| metadata.hermes | ✅ | ❌（多 AI 工具通用） |
| 流程类 | 5 Phases x 3 Gates（详细） | briefly 1-2 句（skills/ 引用） |
| ek 命令 | 英文表 | 中文场景表 |
| 用户拥有标记 | ✅ | ✅ |

## 哲学一致性

- v0.4.6 实现 init 不覆盖 ± 此版本不改行为
- 只改默认内容，不改 init 逻辑
- 0 新格式，0 新文件类型
- 多 AI 工具通用（AGENTS.md 文件名 = 唯一通用入口）

## v0.4.8 候选

- `.enjoyknowledge/skills/{flow}.md` 4 个工作流（编码/调研/复盘/设计）
- v0.4.7 briefly 提到的流程类 = v0.4.8 落地
