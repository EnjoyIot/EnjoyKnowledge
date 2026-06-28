# enjoyknowledge 术语表

> v0.4.2 | 2026-06-28
>
> 所有文档共享的统一术语。按类别排列。

---

## 核心概念

### SoT（Source of Truth）
唯一真值源 = `.enjoyknowledge/` 下的 markdown 文件。AI 工具入口文件是 export 出的副本，不是 SoT。

### 知识资产
工程团队产生的结构化知识文件——架构、决策、踩坑、规则。与代码资产对应，享受 `doctor` / `fix` / `grep` 等工程化治理。

### 知识管理原语
Core CLI 命令集：`init` / `ls` / `tree` / `cat` / `grep` / `add` / `doctor` / `fix` / `export` / `workflow` / `promote` / `stage clean`。所有原语不写死目录名、不关心领域语义。

### AGENTS.md
项目根目录的 AI 入口文件。内嵌 `enjoyknowledge ls` 摘要（`<!-- enjoyknowledge_LS_START -->` 块），AI 30 秒读完即知项目知识结构。`init` 生成，`add` 自动更新。**路由表模式**——不复制 SoT 内容，只放链接。

---

## 两层架构

### Core
通用知识资产引擎。提供 OKF 兼容格式、CLI 原语，不绑定具体领域。详见 [DESIGN.md](../02-design/DESIGN.md)。

### for Coding
基于 Core 的第一个领域应用，面向 AI 编程场景。内置 10 类知识资产 + onboard/capture 工作流 + 多工具 export。默认 profile：`--profile for-coding`。

### for X
未来领域应用统称（for Design / for Support / for Research 等）。同一 Core + 不同知识类型 + 不同工作流。

---

## 知识分类（10 类）

| kind | 目录 | 本质 | 必填 frontmatter 字段 |
|---|---|---|---|
| gotcha | gotchas/ | 踩坑记录 | `trigger` |
| decision | decisions/ | 架构决策 | `reversible` + `decided_at` |
| pattern | patterns/ | 最佳实践 | — |
| rule | rules/ | 强制规则 | `applies_to` |
| business | business/ | 业务规则 | — |
| architecture | architecture/ | 系统结构 | — |
| contract | contracts/ | 接口契约 | `applies_to` |
| convention | conventions/ | 命名/格式约定 | `applies_to` |
| context | context/ | 项目背景/运行时 | — |
| template | templates/ | 范式模板 | `applies_to` |

---

## 工作流

### onboard
AI 工具首次进入仓库时建立项目心智模型。触发：AI 工具启动 / `enjoyknowledge workflow onboard`。详见 [workflows.md](../02-design/architecture/workflows.md)。

### capture
把对话中发现的隐性知识沉淀到 SoT。触发：用户/AI 主动 / `enjoyknowledge workflow capture`。详见 [workflows.md](../02-design/architecture/workflows.md)。

### export
把 `.enjoyknowledge/` 内容导出到 AI 工具入口文件。v0.2 首发 Claude + Cursor。`--tool auto` 自动检测。详见 [rule-system.md](../02-design/architecture/rule-system.md)。

---

## v0.4 极简上下文层

### `.enjoyknowledge_stage/`
项目根下的物理暂存区，与 `.enjoyknowledge/` 并列。AI 在开发任务中**自动写入**过程材料。不进入长期知识索引，审核后 promote 迁入。

### promote
`enjoyknowledge promote <draft.md> --to <kind>`：把 stage drafts 落地到 KB。自动添加 4 字段 frontmatter（id/kind/created/author）。默认 author = `enjoy`。必须人类手动执行。

### stage clean
`enjoyknowledge stage clean [--dry-run] [--force] [--older-than <days>]`：清理 `.archive/` 过期文件。默认 TTL 180 天。

### 8 文件结构
`tasks/<task-id>/` 下 AI 维护的 8 个文件：requirements / design / plan / changes / tests / delivery / summary / review。3 硬门（P1 需求 / P2 设计 / P5 交付）需人类批准。

### doctor（健康检查）
4 项检查：frontmatter 有效 / 必填字段 / SoT 过期（> 180 天）/ export 一致性。`--ci` 模式 warning 也返回非零。

### fix（自动修复）
可自动修复：缺 description、AGENTS.md 过期、超出预算（> 20 条目）、待归档任务。不可自动修复：缺 frontmatter。`--req <REQ-ID>` 指定任务目录。

---

## 写作约定

### frontmatter
Markdown 文件开头的 YAML 元数据块。v0.4 极简：仅 id / kind / created / author 4 字段。`promote` 自动生成。

### kebab-case
文件名命名约定。✅ `rust-no-unwrap.md` · ❌ `Rust_NoUnwrap.md`。

### 2 层深度
`.enjoyknowledge/` 内深度 ≤ 2 层：`category/file.md`。`.enjoyknowledge_stage/` 不限制。

### 路由表模式
AGENTS.md 的核心内容——不放 SoT 正文，只放链接。≤ 50 行。`export` 自动生成。

---

## 版本约定

- **对外版本号**：v0.3 / v0.4 / v0.5 / v1.0 / v1.x+
- **Cargo.toml**：`version = "0.1.0"`（内部版本号走 git tag）
- **当前版本**：v0.4.2

---

*关联文档：[knowledge-types.md](../02-design/architecture/knowledge-types.md) · [workflows.md](../02-design/architecture/workflows.md) · [INTERFACE-SPEC.md](../02-design/INTERFACE-SPEC.md)*
