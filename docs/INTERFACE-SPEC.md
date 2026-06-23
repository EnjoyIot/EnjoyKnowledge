# enjoyknowledge 接口规范

> OKF v0.1 兼容
>
> enjoyknowledge 兼容实现的最小接口合约。第三方适配器、工具生成器、AI 工具集成的唯一参考。
> 对齐 Google OKF (Open Knowledge Format) v0.1。

---

## 1. 兼容性级别

| 级别 | 要求 | 标识 |
|---|---|---|
| **L1 Core 完整兼容** | 实现本文档全部 Core 合约 | `enjoyknowledge-core` |
| **L2 格式兼容** | 读/写 OKF 兼容的 Markdown 文件，不实现 CLI | `enjoyknowledge-format` |
| **L3 应用兼容** | 基于 Core 定义某个 for X 应用的目录、入口和工作流 | `enjoyknowledge-app` |

---

## 2. 目录结构

### 2.1 for Coding 默认结构

```
<项目根目录>/
├── AGENTS.md               # 必需，AI 入口（内含知识摘要推送块）
├── .enjoyknowledge/
│   ├── architecture/       # 架构知识
│   ├── gotchas/            # 踩坑记录
│   ├── patterns/           # 最佳实践
│   ├── business/           # 业务规则
│   ├── decisions/          # 架构决策记录
│   ├── index.md            # OKF 保留，目录的目录
└── knowledge-tasks/        # 可选，任务过程材料；审核后再迁入 .enjoyknowledge/
    └── <REQ-ID>/
```

### 2.2 规则

| 规则 | 约束 |
|---|---|
| 深度 | `.enjoyknowledge/` 内不超过 2 层：`category/file.md` |
| 目录名即分类 | 文件所在目录名决定了它的概念类型，无需在 frontmatter 中重复声明 |
| 文件名自解释 | 文件名表达主题，无需在文件名中重复目录名 |
| 应用目录 | `architecture/`、`gotchas/`、`knowledge-tasks/` 等属于 for Coding，不属于 Core 强制目录 |
| 任务暂存区 | `knowledge-tasks/<REQ-ID>/` 与 `.enjoyknowledge/` 并列，不进入长期知识索引，审核后再迁入 |

---

## 3. 知识文档格式

### 3.1 Frontmatter

```yaml
---
title: 导出功能踩坑       # 推荐，人类可读标题
description: 超过10万行时接口超时，当前分批导出  # 强烈推荐，一行摘要
tags: [export, excel, performance]  # 推荐，跨分类过滤
timestamp: 2026-06-21     # 推荐，ISO 8601 日期
---
```

> 文件所在目录名即分类（如 `gotchas/` 下的文件属于 Gotcha 类）。不再需要 frontmatter 中的 `type` 字段。

### 3.2 字段约束

| 字段 | 类型 | 必需 | 约束 |
|---|---|---|---|
| `title` | string | 否 | 人类可读标题 |
| `description` | string | 强烈推荐 | ≦200 字符，一行摘要。出现在 `ls`、`tree`、AGENTS.md 推送块中 |
| `tags` | string[] | 否 | 纯小写字母+连字符，跨分类检索用 |
| `timestamp` | string | 否 | ISO 8601 `YYYY-MM-DD` |

### 3.3 正文

自由 Markdown。每个 `##` 二级标题为一个**条目**（entry），是知识的基本叙事单元：

- `grep` 输出定位到 `##` 段
- doctor 以 `##` 标题数统计条目量（超过 20 条建议拆分）
- `add` 追加内容推荐以 `##` 为开头
- Markdown 链接可用于表达概念间关系：`相关架构见 [导出模块](../architecture/overview.md)`

### 3.4 保留文件名

| 文件 | 位置 | 作用 |
|---|---|---|
| `index.md` | 任何目录下（可选） | 该目录的目录，列出所有概念文件及 description |

---

## 4. CLI 命令

### 4.1 命令总览

所有命令以 `enjoyknowledge` 为前缀。

| 命令 | 语义 | 输出 |
|---|---|---|
| `enjoyknowledge init [--ai <tool>] [--template <name|list>] [--link <path>]` | 初始化知识库 | 目录骨架 + AGENTS.md |
| `enjoyknowledge ls [path] [--bare]` | 列出目录/文件 | 文件列表，默认每文件附带 `description` |
| `enjoyknowledge tree [--bare]` | 递归目录树 | 目录树，默认每文件附带 `description` |
| `enjoyknowledge cat <path>` | 查看文件内容 | 文件全文（stdout） |
| `enjoyknowledge grep <pattern> [--type] [--tags] [--path]` | 结构化搜索 | `文件##段标题\n  上下文 snippet` |
| `enjoyknowledge add <path> <content>` | 新增/追加知识 | 确认消息（stderr） |
| `enjoyknowledge doctor [--ci]` | 合规检查 | 问题清单；`--ci` 模式 warning 也返回非零 |
| `enjoyknowledge fix` | 自动修复 | 修复结果 |

### 4.2 `ls` — 核心入口

```
$ enjoyknowledge ls
architecture/
  overview.md           — 项目整体架构、模块划分
  tech-stack.md         — 技术栈选型
gotchas/
  export.md             — 导出超时、OOM、status字段缺失（3条）
  auth.md               — Token刷新失效、权限缓存不一致（2条）
patterns/
  batch-processing.md   — 分批处理大数据集的通用模式
business/
  water-billing.md      — 水费计算规则、分段计价公式
```

指定路径（相对于 `.enjoyknowledge/`）：

```
$ enjoyknowledge ls gotchas/
  export.md   — 导出超时、OOM、status字段缺失（3条）
  auth.md     — Token刷新失效、权限缓存不一致（2条）
```

`--bare`：只列文件名，不含 description。

### 4.3 `tree` — 递归目录树

```
$ enjoyknowledge tree
.enjoyknowledge/
├── architecture/
│   ├── overview.md       — 项目整体架构
│   └── tech-stack.md     — 技术栈选型
├── gotchas/
│   ├── export.md         — 导出相关踩坑（3条）
│   └── auth.md           — 认证相关（2条）
├── patterns/
│   └── batch-processing.md — 分批处理模式
└── business/
    └── water-billing.md  — 水费计算规则
```

`--bare` 同 `ls`，去掉 description。

### 4.4 `grep` — 结构化搜索

**与系统 `grep` 的差异**：系统 grep 是行匹配器，`enjoyknowledge grep` 是知识检索器——输出定位到 `##` 段，附带上下文 snippet。

```
$ enjoyknowledge grep "导出超时"
gotchas/export.md##大数据量超时
  - 超过 10 万行时接口超时
  - 当前方案：分批导出，单次最多 10 万行
```

选项：

| 选项 | 说明 |
|---|---|
| `--type <type>` | 限定概念类型（即目录名，如 `gotchas`） |
| `--tags <tag>` | tag 过滤（AND 逻辑，可重复） |
| `--path <dir>` | 限定搜索目录 |
| `--archive` | 含已归档的任务材料 |

实现要求：

| 要求 | 说明 |
|---|---|
| 匹配范围 | 正文（`##` 段），不搜 frontmatter。frontmatter 已通过 `type` / `tags` 过滤 |
| 大小写 | 不区分 |
| 排序 | 按匹配段的知识密度（段越长越靠前） |
| 段界定位 | 每个匹配行定位到最近的 `##` 标题 |

### 4.5 `add` — 新增知识

```
enjoyknowledge add gotchas/export.md "## Excel内存溢出
- 现象: SXSSFWorkbook 未关闭导致 OOM
- 方案: try-with-resources 自动关闭"
```

行为：
- 文件存在 → 追加到末尾
- 如果 frontmatter 有 `timestamp`，更新为当前日期
- 文件不存在 → 创建（生成含 `description` 和 `timestamp` 的 frontmatter 模板），写入内容
- 目录不存在 → 自动创建
- 追加后自动更新 AGENTS.md 中的知识摘要块

### 4.6 `cat` — 查看文件

路径相对于 `.enjoyknowledge/`。补全前缀后输出文件全文。

### 4.7 `fix` — 自动修复

`enjoyknowledge fix` 自动修复可程序化处理的合规问题。

**可自动修复：**

| 问题 | 修复方式 |
|---|---|
| 缺 `description` | 从正文首段提取或填入模板占位 |
| AGENTS.md 过期 | 重新生成摘要块（同步 `ls` 输出） |
| 超出预算（>20 条目） | 将最早的 `##` 条目移到归档文件中 |
| 待归档任务 | 将 `knowledge-tasks/<REQ-ID>/` 下可复用条目提取到 `.enjoyknowledge/` |

**不可自动修复（需手动）：**

| 问题 | 原因 |
|---|---|
| 缺 frontmatter | 无法推断描述和元数据 |

**归档机制：**

归档针对已完成任务的 `knowledge-tasks/<REQ-ID>/` 目录：
- `doctor` 检测到已完成的任务目录 → 提示归档
- `fix` 将 `knowledge-tasks/<REQ-ID>/` 下的可复用条目提取到 `.enjoyknowledge/` 对应分类，原目录标记为已归档
- `grep --archive` 搜索范围包含已归档的任务材料

### 4.8 AGENTS.md 生成

`enjoyknowledge init` 生成 AGENTS.md，内嵌知识摘要块（`<!-- enjoyknowledge_LS_START -->` ... `<!-- enjoyknowledge_LS_END -->` 之间）。
`enjoyknowledge add` 时自动更新此摘要块，保持与 `ls` 输出一致。

---

## 5. 模板系统与 `init --template`

### 5.1 默认模板（for Coding）

`enjoyknowledge init` 默认使用 for Coding 模板，生成：
- `.enjoyknowledge/{architecture,gotchas,patterns,business,decisions}/`
- `knowledge-tasks/`
- `AGENTS.md`（含知识摘要推送块）

### 5.2 自定义模板

```bash
enjoyknowledge init --template legal
```

模板加载优先级（高到低）：
1. `.enjoyknowledge/templates/<name>/`（项目级）
2. `~/.enjoyknowledge/templates/<name>/`（用户级）

模板目录结构约定：
```
<template-dir>/
├── .enjoyknowledge/          # 目录骨架（直接复制）
│   ├── contracts/
│   └── regulations/
├── knowledge-tasks/          # 可选，任务暂存区骨架
└── AGENTS.md.template        # AGENTS.md 模板
```

引擎不写死目录名——目录名即分类，`contracts/` 下的文件属于 Contract 类，`regulations/` 下的文件属于 Regulation 类。

### 5.3 关键行为

- `--link <path>` 引用外部知识库时，不创建 `.enjoyknowledge/` 目录，只在 AGENTS.md 中指向外部路径
- `--template list` 列出所有可用模板（全局 + 项目级）并退出，不初始化
- `init --ai auto` 自动检测当前 AI 工具
- 无论指定哪个 AI 工具，AGENTS.md 始终生成（作为通用标准入口）

---

## 6. AI 工具文件生成

`enjoyknowledge init --ai <tool>` 生成：

| `--ai` | 生成的工具专有文件 | 格式 |
|---|---|---|
| 默认（不指定） | `AGENTS.md` | Markdown（含知识摘要推送块） |
| `cursor` | `.cursor/rules/enjoyknowledge.mdc` | YAML frontmatter + Markdown |
| `claude` | `.claude/skills/enjoyknowledge.md` | Markdown |
| `copilot` | `.github/copilot-instructions.md` | 追加 Markdown 块 |
| `windsurf` | `.windsurf/rules/enjoyknowledge.md` | Markdown |
| `cline` | `.clinerules/enjoyknowledge.md` | Markdown |
| `codex` | `.codex/prompts/enjoyknowledge.md` | Markdown |
| `trae` | `.trae/rules/enjoyknowledge.md` | Markdown |
| `gemini` | `GEMINI.md` | 追加 Markdown 块 |
| `generic` | `enjoyknowledge.md` (project root) | Markdown |
| `auto` | 自动检测当前环境中的 AI 工具 | — |

---

## 7. doctor 检查

`enjoyknowledge doctor` 是知识文件的 linter。

| 检查项 | 说明 |
|---|---|
| 缺 frontmatter | 每个 `.md` 有可解析 YAML frontmatter |
| 缺 description | 强烈推荐有 |
| 超出预算 | 单文件超过 20 条 `##` 条目时建议拆分 |
| AGENTS.md 过期 | `ls` 输出与 AGENTS.md 摘要不一致 |
| 待归档任务 | `knowledge-tasks/` 下已完成未归档的任务目录 |

### `doctor --ci`

`enjoyknowledge doctor --ci` 用于 CI 流水线：
- 任何 warning 也触发非零退出码（非 CI 模式下 warning 不阻塞）
- 退出码 3 表示格式问题（含 warning）

---

## 8. 错误码

| 码 | 含义 |
|---|---|
| 0 | 成功 |
| 1 | 输入参数错误 |
| 2 | 文件/路径不存在 |
| 3 | 格式校验失败（frontmatter 不可解析、description 缺失等 fix 无法自动修复的问题） |
| 4 | 文件不可读写 |
