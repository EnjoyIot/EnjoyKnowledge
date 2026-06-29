---
name: enjoyknowledge-stage
description: enjoyknowledge 任务执行区 AGENTS.md（多 AI 工具通用）
version: 1.0.0
---

# enjoyknowledge 任务执行

## 能力

`.enjoyknowledge_stage/` 是**任务执行区**：
- `drafts/` 草稿（**写在这里** = 任务开始）
- `tasks/` 任务文件（**提升到这里** = 知识沉淀）
- `.archive/` 归档（**任务完成**）

## 任务执行流程

1. **用户给任务** → AI 读 `drafts/` 当前文件
2. **AI 写文件** → `drafts/{task-name}.md`
3. **任务完成** → `ek promote` 把 `drafts/{task-name}.md` 提升到对应 `kind/` 目录
4. **沉淀** → 知识在 `.enjoyknowledge/{kind}/` 长期保存

## 读写原则

- **AI 工具接到任务时**：先读本文件 + `.enjoyknowledge/AGENTS.md`
- **AI 工具写文件时**：先 `drafts/` 后 `tasks/`（promote 后）
- **AI 工具遇到流程问题**：参考 `.enjoyknowledge/skills/` 下的流程类文档（v0.4.8 候选）
- **用户改本文件**：重跑 `ek init` **不会覆盖**（v0.4.4 模式）

## ek 命令

| 任务 | ek 命令 |
|------|---------|
| 列出草稿 | `ek ls drafts` |
| 提升草稿 | `ek promote <draft>` |
| 归档任务 | `ek stage clean` |
