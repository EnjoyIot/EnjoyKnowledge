---
name: enjoyknowledge-flow-review
description: 复盘工作流 — AI 完成任务后怎么沉淀（"复盘" / "总结" / "沉淀" / "完成任务"）
version: 1.0.0
---

# 复盘工作流

## Purpose

AI 完成任务后，怎么把经验沉淀到 enjoyknowledge 知识库。

## When to use

- 用户说"复盘"
- 用户说"总结一下"
- 用户说"沉淀这个"
- 任务完成时（**主动复盘**）

## Step-by-step

1. **跑 `ek onboard`** = 看活跃任务
2. **写 `drafts/{review}.md`** = 写复盘内容
3. **选 kind** = 决策 → decision / 坑 → gotcha / 模式 → pattern
4. **`ek promote drafts/{review}.md → kind/{kind}/`** = 提升到知识库
5. **跑 `ek doctor`** = 检查知识库健康

## File Reading Order

1. `.enjoyknowledge_stage/drafts/` 当前文件
2. `.enjoyknowledge/_meta/kinds.md`

## File Writing Order

1. `.enjoyknowledge_stage/drafts/{review}.md`（先写草稿）
2. `.enjoyknowledge/{kind}/{name}.md`（promote 后）

## Common Patterns

- **决策** = `decision/{name}.md`（**重要决策必写**）
- **坑** = `gotcha/{name}.md`（**踩过的坑必写**）
- **模式** = `pattern/{name}.md`（**通用模式必写**）

## Pitfalls

- ❌ 不要直接写 `kind/{kind}/`（**先 drafts/ 后 promote**）
- ❌ 不要忘了跑 `ek onboard`（**复盘必跑**）
