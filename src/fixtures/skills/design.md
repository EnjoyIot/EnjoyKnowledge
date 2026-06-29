---
name: enjoyknowledge-flow-design
description: 设计工作流 — AI 设计新功能时怎么用 ek kind（"加新分类" / "新 kind" / "设计"）
version: 1.0.0
---

# 设计工作流

## Purpose

AI 设计新功能时，怎么用 enjoyknowledge 的 kind 系统。

## When to use

- 用户说"加新分类 X"
- 用户说"我们需要一个新的 kind"
- 用户说"重新设计 kind"

## Step-by-step

1. **读 `AGENTS.md`** = 了解现有 11 kind
2. **跑 `ek kind list`** = 看所有 kind
3. **判断** = 新功能是不是已有 kind？要不要新 kind？
4. **`ek kind add <name>`** = 加新 kind（自动创建目录 + 更新 kinds.md）
5. **写 `kind/{new}/AGENTS.md`** = 教 AI 怎么用新 kind
6. **跑 `ek doctor`** = 验证

## File Reading Order

1. `.enjoyknowledge/AGENTS.md`
2. `.enjoyknowledge/_meta/kinds.md`

## File Writing Order

1. `.enjoyknowledge/_meta/kinds.md`（`ek kind add` 自动改）
2. `.enjoyknowledge/{kind}/AGENTS.md`（用户写）

## Common Patterns

- **复用已有 kind** = 不要加新 kind，先用 `decision` / `pattern` 等
- **加新 kind** = `ek kind add <name> --required "..." --summary "..."`
- **删 kind** = `ek kind rm <name> --force`（**慎用**）

## Pitfalls

- ❌ 不要轻易加新 kind（**先复用**）
- ❌ 加完 kind 要写 `AGENTS.md`（**AI 工具要知道怎么用**）
