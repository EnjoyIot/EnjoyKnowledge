---
name: enjoyknowledge-flow-coding
description: 编码工作流 — AI 接任务时怎么用 stage + ek（"接到任务" / "开始编码" / "实现功能" / "改 bug"）
version: 1.0.0
---

# 编码工作流

## Purpose

AI 接到编码任务时，怎么用 enjoyknowledge 完成任务。

## When to use

- 用户说"实现 X 功能"
- 用户说"修这个 bug"
- 用户说"开始任务"

## Step-by-step

1. **读 `stage/AGENTS.md`** = 了解任务执行流程
2. **读 `_meta/stage-defaults.md`** = 了解 stage 目录结构
3. **读 `_meta/kinds.md`** = 了解 11 kind 分类
4. **读 `drafts/` 当前文件** = 了解进行中的任务
5. **写 `drafts/{task-name}.md`** = 开始任务
6. **完成任务后** = `ek promote drafts/{task-name}.md → kind/{kind}/`

## File Reading Order

1. `.enjoyknowledge/AGENTS.md`
2. `.enjoyknowledge_stage/AGENTS.md`
3. `.enjoyknowledge/_meta/kinds.md`
4. `.enjoyknowledge/_meta/stage-defaults.md`
5. `.enjoyknowledge_stage/drafts/` 当前文件

## File Writing Order

1. `.enjoyknowledge_stage/drafts/{task-name}.md`（先写草稿）
2. `.enjoyknowledge/{kind}/{name}.md`（promote 后）

## Common Patterns

- **小任务** = 直接 `ek add {kind} "..."` 不用 stage
- **大任务** = `drafts/` → `ek promote` → 沉淀到 `kind/`
- **决策记录** = 写 `decision/{name}.md`（**重要决策必写**）

## Pitfalls

- ❌ 不要直接写 `kind/{kind}/`（**先 drafts/ 后 promote**）
- ❌ 不要忘了 `ek onboard`（**复盘必跑**）
- ❌ 不要在 `drafts/` 写完不 promote（**草稿不沉淀 = 知识丢失**）
