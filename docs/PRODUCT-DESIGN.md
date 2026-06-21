# EnjoyFlow 产品设计

> 版本 1.0-draft | 2026-06-21
>
> 回答用户怎么用 EnjoyFlow——交付形态、入口、端到端流程、AI 工具集成、配置方式、团队共享。
> 本文档不涉及内部实现（那是 SYSTEM-DESIGN.md 的职责）。

---

## 1. 交付形态

**Rust CLI + Git 模板。**

```bash
# 安装
cargo install enjoyflow

# 或直接下载二进制
curl -fsSL https://enjoyflow.dev/install.sh | bash
```

| 选型 | 理由 |
|---|---|
| **Rust CLI** | 单二进制，不依赖 Node/npm/Python。安装快，启动快，跨平台 |
| **Git 模板** | `enjoyflow init` 从模板仓库拉取目录骨架，跟 `cargo init` 同理 |
| **不做 MCP**（MVP） | 文件驱动就能覆盖所有场景。MCP 作为后期可选增强 |

---

## 2. 用户入口

三种模式，操作同一个结果：

```bash
# 模式 1：默认模板（最简）
enjoyflow init
# → 生成 .enjoyflow/ 骨架 + 最小 3 类知识（A1 架构 + A2 规范 + C1 踩坑）

# 模式 2：扫描现有项目
enjoyflow init --scan
# → 自动识别技术栈、已有目录结构、package.json/pyproject.toml
# → 交互式补全：确认识别的模块、技术栈、部署方式

# 模式 3：自然语言描述
enjoyflow init --describe "一个 IoT 水务管理平台，前后端分离：Vue3 + Java Spring Boot"
# → 生成初始化 proposal 供用户确认
```

三种模式都是生成 `.enjoyflow/config.yaml` 和初始知识文件——后续手动改也完全一样。

### 2.1 知识库放哪里

四种场景，一个命令解决：

```bash
# 场景 1：默认，知识库在项目仓库里（个人/小团队）
enjoyflow init
# → 生成 .enjoyflow/ 在当前目录

# 场景 2：放到后端项目下（前后端分离但后端拥有知识）
enjoyflow init ./backend

# 场景 3：独立仓库，前后端项目共用（团队有自己的 docs 仓库）
enjoyflow init ../team-knowledge
# → 在 team-knowledge/ 下创建 .enjoyflow/
# → 在 frontend/ 和 backend/ 里分别跑 enjoyflow init --link ../team-knowledge

# 场景 4：引用已有知识库，不创建文件
enjoyflow init --link ../team-knowledge
# → 只在当前项目写 AGENTS.md，指向 ../team-knowledge/.enjoyflow/
# → 不创建任何 .enjoyflow/ 目录
```

### 2.2 独立仓库模式详解

团队想把知识库作为独立资产（限制权限、独立版本）时：

```
workspace/
├── frontend/                    # 前端项目（开发者有读写权限）
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
    ├── .enjoyflow/
    │   ├── knowledge-base/
    │   └── config.yaml
    ├── .git/
    └── README.md
```

```bash
$ cd backend
$ enjoyflow init --link ../team-knowledge
✓ 检测到 ../team-knowledge/.enjoyflow/
✓ AGENTS.md 已更新 → 指向 ../team-knowledge/.enjoyflow/
✓ .claude/skills/enjoyflow.md 已生成

$ cd ../frontend
$ enjoyflow init --link ../team-knowledge
✓ …同上
```

**关键行为**：
- `--link` 不创建 `.enjoyflow/`，不改动知识库内容
- 只生成 AI 工具文件（AGENTS.md 等），路径指向外部的知识库
- 多个项目可链接到同一个知识库——共享一份知识
- 知识库仓库可独立管理权限（限制谁能 approve PR）

---

## 3. AI 工具集成

### 3.1 核心策略：文件驱动（参考 Superpowers）

**不做 MCP，不做 RPC。** EnjoyFlow 生成 Skill 文件，AI 读到就知道怎么用。

### 3.2 `enjoyflow init` 自动集成

```bash
enjoyflow init --ai cursor       # Cursor
enjoyflow init --ai claude       # Claude Code
enjoyflow init --ai copilot      # GitHub Copilot
enjoyflow init --ai windsurf     # Windsurf
enjoyflow init --ai cline        # Cline
enjoyflow init --ai codex        # OpenAI Codex CLI
enjoyflow init --ai gemini       # Google Gemini CLI
enjoyflow init --ai auto         # 自动检测（默认）
```

**AGENTS.md 始终生成**——不管指定哪个 AI 工具。这是 Linux 基金会 AI Agent 基金会采纳的通用标准，已覆盖 20+ 工具（Cursor、Copilot、Codex、Gemini CLI、Aider、Windsurf、Zed、Cline……所有主流工具原生支持）。

工具专用文件只在需要工具特有功能时才生成：

| AI 工具 | 生成的文件 | 作用 |
|---|---|---|
| **所有工具** | `AGENTS.md` | 通用标准，所有工具原生读取 |
| **Cursor** | `.cursor/rules/enjoyflow.mdc` | `alwaysApply: true`，享受 Cursor 的 globs 激活机制 |
| **Claude Code** | `.claude/skills/enjoyflow.md` | Claude 的 Skill 系统，可被 `/enjoyflow` 调用 |
| **Copilot** | `.github/copilot-instructions.md` | GitHub 原生格式 |
| **Windsurf** | `.windsurf/rules/enjoyflow.md` | Windsurf 规则目录 |
| **Cline** | `.clinerules/enjoyflow.md` | Cline 规则目录 |
| **Codex** | `.codex/prompts/enjoyflow.md` | Codex prompt 目录 |
| **Gemini CLI** | `GEMINI.md`（追加块） | Gemini CLI 原生格式 |

### 3.3 两层文件策略

```
AGENTS.md                    ←  主文件，始终生成。所有工具的通用入口
    │
    └── 运行 enjoyflow search <REQ-ID> 获取任务上下文
    
.cursor/rules/enjoyflow.mdc   ←  辅助文件，只在指定 --ai cursor 时生成
.claude/skills/enjoyflow.md   ←  辅助文件，只在指定 --ai claude 时生成
...                           ←  每个工具一个独立文件，互不干扰
```

**AGENTS.md 是唯一真相源。** 工具专用文件只包含该工具特有的能力（如 Cursor 的 globs 作用域、Claude Code 的 Skill 调用），核心指令始终在 AGENTS.md 里——避免多个文件内容不同步。

```markdown
# AGENTS.md
# （EnjoyFlow 初始化为项目级——非全局）

## EnjoyFlow 上下文

本项目使用 [EnjoyFlow](https://enjoyflow.dev) 管理 AI 编程的上下文知识。

- 知识库: `.enjoyflow/knowledge-base/` + `.enjoyflow/knowledge-tasks/`

当用户提到 "修 bug"、"新功能"、"重构" 等任务时：
1. 运行 `enjoyflow search <REQ-ID>` 获取任务上下文
2. 上下文包含架构、契约、踩坑、进度——先读完再动手

可用命令：
- `enjoyflow search <query>` — 搜索知识库
- `enjoyflow record <type> --tag <tag>` — 记录新知识
- `enjoyflow show <REQ-ID>` — 查看任务状态
```

### 3.4 三层供给模型

| 层 | 触发方式 | 内容 | 覆盖 |
|---|---|---|---|
| **L0 常驻** | 工具配置文件自动注入 | A1 架构 + A2 规范 + C1 踩坑 | 所有会话 |
| **L1 任务** | Skill 教学 → AI 调 search 获取上下文 | ContextFlow 实时检索（任务级上下文） | 当前任务 |
| **L2 兜底** | 人手动 `enjoyflow search` 获取上下文 | 任意场景的完整上下文 | 任意 AI 工具 |

---

## 4. 端到端流程

### 4.1 一条命令启动任务

```
开发者: "帮我修一下导出 Excel 时内存溢出的 bug"

AI: [读到 AGENTS.md 中的 EnjoyFlow 指令]
    → 场景识别: bug_fix
    → 运行 enjoyflow search "REQ-042 导出 内存溢出" --class gotchas
    → 获取架构、业务规则、踩坑上下文
```

### 4.2 快照生成后

```
$ enjoyflow search "导出" --class gotchas --class architecture
✓ 架构上下文: backend/export 模块
✓ 业务规则: 单次最多 10 万行
✓ 已知踩坑: t_export_record 没有 status 字段
✓ 当前进度: IN_PROGRESS（来自 knowledge-tasks/REQ-042/progress.md）
```

AI 读快照，知道架构、知道坑、知道进度——开始修。

### 4.3 默认流程 ≠ 一刀切

框架默认 bug_fix 按标准流程走。如果 AI 检测到关键信号（涉及支付/数据完整性/多模块影响/自身置信度低），主动升级让人判断是否简化或走更重的流程。

```markdown
# .enjoyflow/skills/enjoyflow.md 中的指令（省略常驻层）

## 任务层

当用户开始编码任务时，按以下规则判断：

### 1. 识别场景
根据用户描述匹配场景：new_feature / bug_fix / refactor / hotfix / architecture_decision

### 2. 获取上下文
运行 `enjoyflow search <REQ-ID>` 获取任务相关上下文

### 3. 升级判断
遇到以下情况，主动向用户说明，不自行判断：
- 涉及支付、认证、数据完整性等关键模块
- 需要数据库迁移（DDL）
- 需要在多个可行方案中选择且缺少业务上下文
- 对当前方案的正确性置信度低于阈值

### 4. 任务结束后
提醒用户运行 `enjoyflow record` 记录新发现的坑/模式
```

---

## 5. 配置方式

三种入口，同一份数据：

```yaml
# .enjoyflow/config.yaml —— 三种入口操作的同一份文件
project:
  name: "IoT 水务平台"
  tech_stack: ["vue3", "java", "spring-boot"]

ai_tool: cursor           # auto / cursor / claude / copilot / codex

scenarios:
  active: ["new_feature", "bug_fix", "refactor", "hotfix",
           "architecture_decision", "release_deployment",
           "code_review", "monitoring_response"]
  custom: []

dimensions:
  custom: []              # 用户自定义维度（如 R1_rust_embedded）

paths:
  knowledge_base: ".enjoyflow/knowledge-base"
```

### 5.1 自然语言 → 配置

```bash
$ enjoyflow init --describe "IoT 水务平台，Vue3 + Spring Boot，Team 4人"
```

→ CLI 将自然语言转成 proposal → 用户确认 → 写入 config.yaml

### 5.2 CLI Wizard → 配置

```bash
$ enjoyflow wizard
? 项目名称: IoT 水务平台
? 技术栈: vue3, java, spring-boot
? AI 工具: cursor
? 启用场景: [默认 8 个]
? 知识库路径 (.enjoyflow/knowledge-base):
```

→ 交互式填写，直接写入 config.yaml

### 5.3 手动 → 配置

直接用编辑器修改 `.enjoyflow/config.yaml` 或 `enjoyflow edit config`

### 5.4 配置诊断

```bash
$ enjoyflow doctor
✓ 配置健康
⚠ knowledge-base/development/GOTCHAS.md 缺 class 字段
⚠ knowledge-base/project/ 下有 2 个无 tag 文件

$ enjoyflow fix
✓ 已自动补充 2 个文件的 class + tags
```

---

## 6. 快速添加知识

用户有想法就直接写，框架帮忙分类和加标签：

```bash
# 写内容
vim .enjoyflow/knowledge-base/development/my-notes.md

# 一键补标签
$ enjoyflow add-tag .enjoyflow/knowledge-base/development/my-notes.md

建议 class: C2_patterns
建议 tags: [api, security, rate-limiting]
? 接受？(y/n): y
✓ 已写入 frontmatter
```

不需要事先定义类目，不需要改配置。**先写内容，后补标签。** 下次 `enjoyflow snapshot` 自动索引。

---

## 7. 目录结构

全部收进 `.enjoyflow/`，零干扰用户项目：

```
项目根目录/
├── .enjoyflow/
│   ├── config.yaml              # 项目配置
│   │
│   ├── knowledge-base/          # ABCD 32 类知识
│   │   ├── project/             # A1 架构、A2 规范、A4 数据模型、A6 PRD
│   │   ├── contract/            # A3 API 契约、A5 接口规约
│   │   ├── business/            # B1 术语表、B2 业务规则
│   │   ├── development/         # C1 踩坑、C2 模式、C3 决策归档
│   │   ├── testing/             # C4 测试策略、C5 已知问题
│   │   ├── deployment/          # A11 环境、A12 发布、C8 部署清单
│   │   ├── shared/              # C6 团队约定、C7 审查清单
│   │   └── context/             # D2 会话、D3 决策、D7 失败模式
│   │
│   └── skills/
│       └── enjoyflow.md         # AI 工具 Skill 文件
│
├── AGENTS.md                     # 始终生成，所有工具的通用入口
├── .cursor/rules/enjoyflow.mdc   # Cursor（指定 --ai cursor 时）
├── .claude/skills/enjoyflow.md   # Claude Code（指定 --ai claude 时）
├── .github/copilot-instructions.md # Copilot（指定 --ai copilot 时）
└── .gitignore
```

### 7.1 知识库进 git

`.enjoyflow/knowledge-base/` 和 `.enjoyflow/knowledge-tasks/` 进 git——那是团队共享的知识源。`.index.json` 和 `.doctor-cache` 是本地产物，不进 git。

### 7.2 CLI 命令直达

用户不需要记路径：

```bash
enjoyflow edit architecture    # → vim .enjoyflow/knowledge-base/project/ARCHITECTURE.md
enjoyflow edit gotchas         # → vim .enjoyflow/knowledge-base/development/GOTCHAS.md
enjoyflow edit config          # → vim .enjoyflow/config.yaml
enjoyflow show REQ-042         # → 显示任务状态
```

---

## 8. 与其他工具的关系

### 8.1 分工不同层

| 框架 | 管什么 | EnjoyFlow 跟它的关系 |
|---|---|---|
| **Superpowers** | 执行纪律（TDD、code review） | EnjoyFlow 提供上下文，Superpowers 管怎么执行 |
| **oh-my-claudecode** | 代理编排（多 Agent 协作） | OMC 调度 Agent，Agent 从 EnjoyFlow 拉知识 |
| **spec-kit** | 规范流程（constitution → plan） | spec-kit 管 Spec 制品，EnjoyFlow 管共享记忆 |
| **EnjoyFlow** | **知识供给**（当前任务需要什么上下文） | 只管上下文，不管执行/编排/规范 |

### 8.2 文件层面隔离

```
├── .claude/          ← OMC 领地（EnjoyFlow 不碰）
├── .speckit/         ← spec-kit 领地
├── .superpowers/     ← Superpowers 领地
├── .enjoyflow/       ← EnjoyFlow 领地
├── AGENTS.md         ← EnjoyFlow 主文件（通用标准，不冲突）
├── CLAUDE.md         ← OMC 领地（EnjoyFlow 不碰）
├── .cursor/rules/    ← EnjoyFlow 只占用 enjoyflow.mdc 一个文件
└── .claude/skills/   ← EnjoyFlow 只占用 enjoyflow.md 一个文件
```

三条规则保证不冲突：
- **独立文件**——EnjoyFlow 生成自己的 Skill 文件（`.cursor/rules/enjoyflow.mdc`、`.claude/skills/enjoyflow.md`），不修改任何已有文件
- **独立目录**——EnjoyFlow 所有文件在 `.enjoyflow/`
- **Skill 自描述**——Skill 文件写清楚"我只负责提供上下文，执行方式由你的其他 Skill 决定"

---

## 9. MVP 命令清单

```bash
enjoyflow init [path]                  # 初始化（默认模板）
enjoyflow init --scan [path]           # 扫描现有项目初始化
enjoyflow init --describe "..." [path] # 自然语言初始化
enjoyflow init --link <path>           # 引用已有知识库
enjoyflow init --ai <tool>             # 指定 AI 工具

enjoyflow new <REQ-ID> ["描述"]        # 创建需求（交互式）
enjoyflow search <query> [--class] [--tag]  # 检索知识
enjoyflow show <REQ-ID>                # 查看任务状态

enjoyflow edit <target>                # 编辑指定文件（architecture/gotchas/config/…）
enjoyflow add-tag <file>               # AI 建议 tag + 确认

enjoyflow doctor                       # 配置诊断
enjoyflow fix                          # 自动修复

enjoyflow record gotcha --tag <tag>    # 记录踩坑
enjoyflow record pattern --tag <tag>   # 记录模式
```

---

## 10. MVP 不做的事

- MCP Server（文件驱动够用，MCP 后期可选）
- 可视化界面（CLI 优先）
- 语义检索（模板 + tag 够用）
- 全 32 类知识（3 类起步 → 11 项 P0 → 32 类全量）
- CI/CD 集成

---

*文档版本: 1.0-draft | 最后更新: 2026-06-21*
