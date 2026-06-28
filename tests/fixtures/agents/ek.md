---
name: enjoyknowledge-kb
description: enjoyknowledge 知识库根 AGENTS.md（多 AI 工具通用）
version: 1.0.0
---

# enjoyknowledge 知识库

## 能力

enjoyknowledge 是一个**项目级知识管理工具**：
- **.enjoyknowledge/** 知识库根（11 个 kind 目录 + _meta/）
- **.enjoyknowledge_stage/** 任务执行区（drafts/ + tasks/ + .archive/）
- **ek** CLI 命令（init / ls / tree / cat / grep / add / doctor / fix / export / onboard / promote / kind）

## 目录结构

```
.enjoyknowledge/
├── AGENTS.md            ← 本文件
├── _meta/               ← 元数据（kinds.md + stage-defaults.md）
├── architecture/        ← 架构决策
├── business/            ← 业务规则
├── context/             ← 上下文
├── contract/            ← 契约
├── convention/          ← 约定
├── decision/            ← 决策记录
├── gotcha/              ← 踩坑记录
├── pattern/             ← 模式
├── rule/                ← 规则
└── template/            ← 模板

.enjoyknowledge_stage/
├── AGENTS.md            ← 任务执行指令
├── drafts/              ← 草稿
├── tasks/               ← 任务
└── .archive/            ← 归档
```

## 读写原则

- **AI 工具启动时**：先读本文件 + `_meta/kinds.md` + 当前任务的 `_stage/AGENTS.md`
- **AI 工具遇到流程问题**：参考 `.enjoyknowledge/skills/` 下的流程类文档（v0.4.8 候选）
- **用户改本文件**：重跑 `ek init` **不会覆盖**（v0.4.6 模式）

## 常见操作

| 用户说 | ek 命令 |
|--------|---------|
| "列出知识库" | `ek ls` / `ek tree` |
| "看 kind X 的内容" | `ek cat <kind>` |
| "加一条决策" | `ek add decision "..."` |
| "加新 kind" | `ek kind add <name>` |
| "复盘" | `ek onboard` |
| "把草稿提升到知识库" | `ek promote <draft>` |
| "诊断" | `ek doctor` / `ek fix` |
