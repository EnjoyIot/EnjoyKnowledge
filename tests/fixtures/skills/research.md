---
name: enjoyknowledge-flow-research
description: 调研工作流 — AI 查现有知识（"查" / "找" / "看" / "调研" / "有什么"）
version: 1.0.0
---

# 调研工作流

## Purpose

AI 调研现有知识时，怎么用 enjoyknowledge 高效查找。

## When to use

- 用户说"查 X"
- 用户说"找 Y"
- 用户说"看看 Z"
- 用户说"我们之前讨论过 W 吗"

## Step-by-step

1. **读 `AGENTS.md`** = 了解 11 kind 分类
2. **用 `ek ls`** = 看 kind 目录列表
3. **用 `ek tree`** = 看目录树结构
4. **用 `ek cat <kind>`** = 看某个 kind 的所有文件
5. **用 `ek grep <pattern>`** = 全文搜索

## File Reading Order

1. `.enjoyknowledge/AGENTS.md`
2. `.enjoyknowledge/_meta/kinds.md`

## File Writing Order

（调研工作流只读不写）

## Common Patterns

- **精确查找** = `ek cat <kind> | grep "关键词"`
- **全局搜索** = `ek grep "关键词"`
- **看目录结构** = `ek tree`

## Pitfalls

- ❌ 不要直接 `cat kind/*`（**用 ek cat**）
- ❌ 不要忘了 `ek tree`（**看结构先看 tree**）
