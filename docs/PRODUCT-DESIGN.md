# enjoyknowledge 产品设计

> 版本 1.0 | 2026-06-22
>
> 回答用户怎么用 enjoyknowledge——Core / for Coding 分层、交付形态、入口、端到端流程、AI 工具集成、团队共享。
> 本文档不涉及内部实现（那是 [DESIGN-V3.md](./DESIGN-V3.md) 的职责）。设计决策的"为什么"见 [DESIGN-PHILOSOPHY.md](./DESIGN-PHILOSOPHY.md)。
>
> **关联文档**: [DESIGN-V3.md](./DESIGN-V3.md) — 系统架构与实现设计 | [INTERFACE-SPEC.md](./INTERFACE-SPEC.md) — CLI 行为合约 | [DESIGN-PHILOSOPHY.md](./DESIGN-PHILOSOPHY.md) — 设计哲学

---

## 1. 交付形态

**Rust CLI 单二进制，默认附带 for Coding 应用。**

```bash
# 安装
cargo install enjoyknowledge

# 或直接下载二进制
curl -fsSL https://enjoyknowledge.dev/install.sh | bash
```

| 选型 | 理由 |
|---|---|
| **Rust CLI** | 单二进制，不依赖 Node/npm/Python。安装快，启动快，跨平台 |
| **Core / 应用分层** | Core 提供通用知识管理原语；for Coding 提供 AI 编程默认目录、AGENTS.md 和任务暂存区 |
| **文件即知识** | `.enjoyknowledge/` 下是标准 Markdown + YAML frontmatter（OKF 兼容），任何编辑器可读写 |
| **不做 MCP**（MVP） | 文件驱动就能覆盖所有场景。MCP 作为后期可选增强 |

用户需要先知道一句话：**enjoyknowledge 不是只能做编程知识。当前默认应用是 for Coding，Core 后续可以承载其他 for X 应用。**

---

## 2. 用户入口

`enjoyknowledge init` 默认初始化 **for Coding**：创建 `.enjoyknowledge/` 长期知识目录、`knowledge-tasks/` 任务暂存区，以及项目根目录的 `AGENTS.md`。三种初始化方式：

```bash
# 模式 1：默认应用 for Coding（最简，5 个长期知识目录 + 任务暂存区）
enjoyknowledge init
# → 生成 .enjoyknowledge/{architecture,gotchas,patterns,business,decisions}/
# → 生成 knowledge-tasks/（短期任务工作区，审核后再迁入 .enjoyknowledge/）
# → 生成 AGENTS.md（项目根目录，含知识摘要推送块）

# 模式 2：指定领域应用 / 模板
enjoyknowledge init --template legal       # 法务场景
enjoyknowledge init --template gamedev     # 游戏开发
enjoyknowledge init --template data-eng    # 数据工程
# → 用指定模板的目录结构替换默认长期知识目录集合

# 模式 3：引用已有知识库
enjoyknowledge init --link ../team-knowledge
# → 只生成 AGENTS.md，指向外部 .enjoyknowledge/
# → 不创建任何 .enjoyknowledge/ 目录
```

### 2.1 知识库放哪里

四种场景，一个命令解决：

```bash
# 场景 1：默认，知识库在项目仓库里（个人/小团队）
enjoyknowledge init

# 场景 2：放到后端项目下（前后端分离但后端拥有知识）
enjoyknowledge init ./backend

# 场景 3：独立仓库，多个项目共用
enjoyknowledge init ../team-knowledge
# → 在 team-knowledge/ 下创建 .enjoyknowledge/
# → 在各项目里分别跑 enjoyknowledge init --link ../team-knowledge

# 场景 4：引用已有知识库，不创建文件
enjoyknowledge init --link ../team-knowledge
# → 只在当前项目写 AGENTS.md，指向 ../team-knowledge/.enjoyknowledge/
```

### 2.2 独立仓库模式详解

团队想把知识库作为独立资产（限制权限、独立版本）时：

```
workspace/
├── frontend/                    # 前端项目
│   ├── src/
│   ├── AGENTS.md                # "知识库在 ../team-knowledge"
│   └── package.json
│
├── backend/                     # 后端项目
│   ├── src/
│   ├── AGENTS.md                # "知识库在 ../team-knowledge"
│   └── pom.xml
│
└── team-knowledge/              # 独立仓库（只有 Tech Lead 能 merge）
    ├── .enjoyknowledge/
    │   ├── architecture/
    │   ├── gotchas/
    │   ├── patterns/
    │   ├── business/
    │   └── decisions/
    ├── knowledge-tasks/
    │   └── REQ-042/
    ├── .git/
    └── README.md
```

```bash
$ cd backend
$ enjoyknowledge init --link ../team-knowledge
✓ 检测到 ../team-knowledge/.enjoyknowledge/
✓ AGENTS.md 已更新 → 指向 ../team-knowledge/.enjoyknowledge/

$ cd ../frontend
$ enjoyknowledge init --link ../team-knowledge
✓ …同上
```

**关键行为**：
- `--link` 不创建 `.enjoyknowledge/`，不改动知识库内容
- 只生成 AGENTS.md，路径指向外部的知识库
- 多个项目可链接到同一个知识库——共享一份知识
- 知识库仓库可独立管理权限（限制谁能 approve PR）

---

## 3. AI 工具集成

### 3.1 推送 + 拉取双通道

enjoyknowledge 解决 AI 的两个核心问题：不知道知识库存在、不知道里面有什么。

**推送通道**——AGENTS.md。AI 启动时自动读到知识库全貌，一行 description 就能判断哪些文件相关。

**拉取通道**——CLI 命令。确定方向后，`grep`、`cat` 深入获取具体内容。

```
[AI 启动，读到 AGENTS.md 中的知识摘要]       ← 推送，零成本感知
    ↓
[判断：修导出 bug，gotchas/export.md 相关]    ← 基于 description 判断
    ↓
enjoyknowledge cat gotchas/export.md            ← 拉取，按需深入
    ↓
[编码...]
    ↓
enjoyknowledge add gotchas/export.md "## 新坑"  ← 回写，自动更新摘要
```

### 3.2 `init --ai` 自动集成

```bash
enjoyknowledge init --ai cursor       # Cursor
enjoyknowledge init --ai claude       # Claude Code
enjoyknowledge init --ai copilot      # GitHub Copilot
enjoyknowledge init --ai windsurf     # Windsurf
enjoyknowledge init --ai cline        # Cline
enjoyknowledge init --ai codex        # OpenAI Codex CLI
enjoyknowledge init --ai trae         # Trae
enjoyknowledge init --ai gemini       # Google Gemini CLI
enjoyknowledge init --ai generic      # 生成工具无关的 enjoyknowledge.md
enjoyknowledge init --ai auto         # 自动检测（默认）
```

**AGENTS.md 始终生成**——不管指定哪个 AI 工具。这是 Linux 基金会 AI Agent 基金会采纳的通用标准，已覆盖 20+ 工具。

工具专用文件只在需要工具特有功能时才生成：

| AI 工具 | 生成的文件 | 作用 |
|---|---|---|
| **所有工具** | `AGENTS.md` | 通用标准，所有工具原生读取，含知识摘要推送块 |
| **Cursor** | `.cursor/rules/enjoyknowledge.mdc` | `alwaysApply: true` |
| **Claude Code** | `.claude/skills/enjoyknowledge.md` | Skill 系统，可被 `/enjoyknowledge` 调用 |
| **Copilot** | `.github/copilot-instructions.md` | GitHub 原生格式 |
| **Windsurf** | `.windsurf/rules/enjoyknowledge.md` | Windsurf 规则目录 |
| **Cline** | `.clinerules/enjoyknowledge.md` | Cline 规则目录 |
| **Codex** | `.codex/prompts/enjoyknowledge.md` | Codex prompt 目录 |`r`n| **Trae** | `.trae/rules/enjoyknowledge.md` | Trae 规则目录 |
| **Gemini CLI** | `GEMINI.md`（追加块） | Gemini CLI 原生格式 |

---

## 4. 日常知识管理

enjoyknowledge 的命令复用 Unix 动词语义——AI 零学习成本。

### 4.1 浏览：`ls` / `tree`

```bash
$ enjoyknowledge ls
architecture/
  overview.md           — 项目整体架构、模块划分、部署拓扑
  tech-stack.md         — Rust + React + PostgreSQL 技术栈选型
gotchas/
  export.md             — 导出超时、OOM、status字段缺失（3条）
  auth.md               — Token刷新失效、权限缓存不一致（2条）
patterns/
  batch-processing.md   — 分批处理大数据集的通用模式
business/
  water-billing.md      — 水费计算规则、分段计价公式
```

每个文件一行 `description`，不用打开文件就知道内容。`ls --bare` 只列文件名。`tree` 递归显示子目录。

### 4.2 搜索：`grep`

```bash
$ enjoyknowledge grep "OOM" --type Gotcha --tags performance
gotchas/export.md##Excel内存溢出
  SXSSFWorkbook 未关闭导致 OOM。方案：try-with-resources
```

与系统 `grep` 的关键差异：定位到 Markdown `##` 段，不是乱序行号。支持 `--type` 和 `--tags` 过滤。

### 4.3 查看：`cat`

```bash
$ enjoyknowledge cat gotchas/export.md
```

路径相对于 `.enjoyknowledge/`，自动补全前缀。输出完整文件内容。

### 4.4 记录：`add`

```bash
$ enjoyknowledge add gotchas/export.md "## Excel内存溢出
- 现象: SXSSFWorkbook 未关闭导致 OOM
- 方案: try-with-resources 自动关闭"
```

行为：
- 文件存在 → 追加到末尾，更新 `timestamp`
- 文件不存在 → 创建（自动生成 frontmatter 模板），写入内容
- 目录不存在 → 自动创建中间目录
- 追加后自动更新 AGENTS.md 中的知识摘要

**分类自动推导**：`add` 从文件路径推导分类——`gotchas/export.md` → 属于 Gotcha 类。不需要手动填。

---

## 5. 维护与健康

### 5.1 `doctor` 检查

```bash
$ enjoyknowledge doctor
✓ architecture/overview.md — OK
✓ gotchas/export.md — OK
⚠ gotchas/auth.md — 缺 description
⚠ patterns/ — 目录为空
✗ .enjoyknowledge/gotchas/export/archive/ — 深度超标（3 层）

$ enjoyknowledge fix
✓ gotchas/auth.md — 已补充 description 模板
⚠ patterns/ — 需手动添加文件
```

`doctor` 是知识文件的 linter，5 项检查：缺 frontmatter、缺 description、超出预算（单文件 >20 条）、AGENTS.md 过期、待归档任务。`doctor --ci` 在 CI 管线中使用，不通过则失败。

### 5.2 知识库进 git

`.enjoyknowledge/` 整个目录进 git——Markdown 文件是团队共享的知识源。`index.md` 由系统生成但也可以 diff。

---

## 6. 目录结构

长期知识收进 `.enjoyknowledge/`，任务过程材料放在并列的 `knowledge-tasks/`：

```
项目根目录/
├── AGENTS.md                     # AI 入口，含知识摘要推送块
│
├── .enjoyknowledge/
│   ├── architecture/             # 项目结构、技术选型、模块边界
│   │   ├── overview.md
│   │   └── tech-stack.md
│   ├── gotchas/                  # 开发中踩过的坑
│   │   ├── export.md
│   │   └── auth.md
│   ├── patterns/                 # 验证过的模式和最佳实践
│   ├── business/                 # 业务规则、计算公式
│   ├── decisions/                # 架构决策记录 (ADR)
│   ├── index.md                  # OKF 保留，目录的目录
│
├── knowledge-tasks/               # 短期任务工作区，审核后再沉淀
│   └── REQ-042/
│       ├── notes.md
│       ├── gotchas.md
│       └── decisions.md
│
├── .cursor/rules/                # Cursor 专用（指定 --ai cursor 时）
│   └── enjoyknowledge.mdc
└── .claude/skills/               # Claude Code 专用（指定 --ai claude 时）
    └── enjoyknowledge.md
```

**结构约束**：

- `.enjoyknowledge/` 内深度不超过 2 层（`gotchas/export.md`）
- 目录名即分类：`gotchas/` 下的文件 type 自动为 `Gotcha`
- 文件名自解释：`gotchas/export.md`，不需要 `gotcha-export.md`
- 聚合而非原子：一个文件聚合一个主题的所有条目。超过 20 条时 doctor 建议拆分
- `knowledge-tasks/` 与 `.enjoyknowledge/` 并列，放短期任务过程材料；审核后再迁入长期知识目录

知识文件的格式为标准 Markdown + YAML frontmatter（OKF 兼容）：

```yaml
---
title: 导出功能踩坑
description: 导出超时、OOM、status字段缺失的记录
tags: [export, excel, performance]
timestamp: 2026-06-22
---
```

---

## 7. 团队共享

### 7.1 独立仓库模式

知识库作为独立 Git 仓库存在，多个项目通过 `--link` 引用。权限控制通过 Git——Tech Lead 审批知识库 PR，开发者 fork + PR。

### 7.2 跨项目知识源

```bash
enjoyknowledge init --link ../team-knowledge
```

引擎透明地合并本项目和上游知识源的索引——`ls` 同时列出两边文件，`doctor` 标注知识出处。

### 7.3 引擎不绑定 Git

只要文件在 `.enjoyknowledge/` 目录下，引擎不关心来源——本地路径、clone 下来的远程仓库、挂载的卷，格式一样就能工作。

---

## 8. 与其他工具的关系

### 8.1 分工不同层

| 框架 | 管什么 | enjoyknowledge 跟它的关系 |
|---|---|---|
| **Superpowers** | 执行纪律（TDD、code review） | enjoyknowledge 提供上下文，Superpowers 管怎么执行 |
| **oh-my-claudecode** | 代理编排（多 Agent 协作） | OMC 调度 Agent，Agent 从 enjoyknowledge 拉知识 |
| **spec-kit** | 规范流程（constitution → plan） | spec-kit 管 Spec 制品，enjoyknowledge 管共享记忆 |
| **enjoyknowledge** | **知识供给**（当前任务需要什么上下文） | 只管上下文，不管执行/编排/规范 |

### 8.2 文件层面隔离

```
├── .claude/          ← OMC 领地
├── .speckit/         ← spec-kit 领地
├── .superpowers/     ← Superpowers 领地
├── .enjoyknowledge/  ← enjoyknowledge 领地
├── AGENTS.md         ← enjoyknowledge 主文件（通用标准）
├── CLAUDE.md         ← OMC 领地
```

三条规则保证不冲突：
- **独立文件**——enjoyknowledge 生成自己的文件，不修改已有文件
- **独立目录**——所有文件在 `.enjoyknowledge/`
- **Skill 自描述**——生成的 Skill 文件写清楚"我只提供上下文，执行方式由你的其他 Skill 决定"

---

## 9. MVP 命令清单

```bash
enjoyknowledge init [path]                  # 初始化（5 个长期知识目录 + 任务暂存区）
enjoyknowledge init --template <name>       # 指定领域模板
enjoyknowledge init --link <path>           # 引用已有知识库
enjoyknowledge init --ai <tool>             # 指定 AI 工具

enjoyknowledge ls [path] [--bare]           # 浏览知识库（含 description）
enjoyknowledge tree [--bare]                # 递归浏览
enjoyknowledge grep <pattern> [--type] [--tags]  # 结构感知搜索
enjoyknowledge cat <path>                   # 查看文件内容
enjoyknowledge add <path> <content>         # 追加知识条目

enjoyknowledge doctor                      # 健康检查（7 项）（8 项）
enjoyknowledge doctor                      # 健康检查（5 项）
enjoyknowledge fix                          # 自动修复
```

---

## 10. MVP 不做的事

- MCP Server（文件驱动够用，MCP 后期可选）
- 可视化界面（CLI 优先）
- 语义检索 / 向量索引（`grep` + `ls` 够用，知识库体量下结构搜索更可靠）
- `config.yaml` 配置系统（目录名和 frontmatter 就是配置）
- 领域应用运行时（`knowledge-tasks/` 支持 for Coding 的需求组织，for X 应用机制作为扩展点保留）
- CI/CD 集成（`doctor --ci` 已支持 CI 管线调用）
- `--scan` / `--describe` 自动扫描（init 创建的骨架足够简单，不需要自动推理）

---

*文档版本: 1.0 | 最后更新: 2026-06-22*
