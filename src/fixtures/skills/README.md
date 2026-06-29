---
name: enjoyknowledge-skills-index
description: enjoyknowledge 流程类 skill 索引（4 个工作流）
version: 1.0.0
---

# enjoyknowledge 流程类 Skill 索引

## 4 个工作流

| 工作流 | 触发场景 | 文件 |
|--------|---------|------|
| **编码** | AI 接任务时 | `coding.md` |
| **调研** | AI 查现有知识 | `research.md` |
| **复盘** | AI 完成任务后 | `review.md` |
| **设计** | AI 设计新功能 | `design.md` |

## 怎么用

1. **AI 工具启动时** = 读 `.enjoyknowledge/AGENTS.md`（briefly 提到 skills/）
2. **AI 遇到流程问题** = 读对应的 `{flow}.md`
3. **用户改本文件** = 重跑 `ek init` **不会覆盖**（v0.4.8 模式）

## 自定义

- **加新工作流** = 创建 `{new-flow}.md`（跟 4 个模板同结构）
- **改工作流** = 直接编辑 `{flow}.md`（**用户拥有**）
