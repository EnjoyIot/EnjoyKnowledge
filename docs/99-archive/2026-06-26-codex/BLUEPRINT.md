<!-- OMC:START -->
<!-- OMC:VERSION:4.14.4 -->
<!-- OMC:END -->

# enjoyknowledge 蓝图

> 版本 1.1 | 2026-06-26
>
> 本文档描述 enjoyknowledge 的完整愿景与架构蓝图。
> 它回答「enjoyknowledge 最终长什么样」——不止今天实现了什么，更是成熟形态的全貌。
>
> **关联文档**:
> [DESIGN-PHILOSOPHY.md](./DESIGN-PHILOSOPHY.md) — 为什么这样设计 |
> [DESIGN-V3.md](./DESIGN-V3.md) — 当前系统架构 |
> [POSITIONING.md](./POSITIONING.md) — 生态定位 |
> [ROADMAP.md](./ROADMAP.md) — 分阶段路线图

---

## 1. 愿景

> **enjoyknowledge 是 AI 时代的工程知识资产管理层。它让文档资产拥有与代码资产同等的工程纪律：格式校验、过期检测、结构化检索、自动化推送。**

在 AI 加速编码 10 倍的时代，真正的瓶颈不再是「怎么写代码」，而是「知道该写什么」和「旧的决策还有效吗」。enjoyknowledge 解决的就是这个瓶颈——不是再做一个 AI 工具，而是给知识文件加上工程化治理。

三个关键词定义了 enjoyknowledge 的边界：

| 关键词 | 含义 | 排除了什么 |
|---|---|---|
| **知识资产** | 架构、决策、踩坑、业务规则——代码之外的结构化知识 | 不管理代码本身 |
| **工程化** | 格式校验、过期检测、结构搜索、自动化推送 | 不是手写 Markdown 的松散约定 |
| **管理层** | 在文件系统之上提供索引、检索、健康检查 | 不是取代 Git / 编辑器 / AI 工具 |

### 1.1 成熟形态

当 enjoyknowledge 成熟时，一个工程团队的知识工作流是这样的：

```
开发者在编码间隙随手记录 → add 自动补全 frontmatter → 知识进入索引
                                              ↓
CI 管线自动运行 doctor → 过期/缺失/冲突即时报警
                                              ↓
AI 工具启动时自动加载 AGENTS.md → 零成本感知知识库全貌
                                              ↓
需要深入时通过 ls/grep/cat 按需拉取 → token 精准不浪费
                                              ↓
任务结束后 knowledge-tasks/ 材料审核 → 可复用内容沉入 .enjoyknowledge/
```

### 1.2 战略优先级：纵向做深 for Coding

enjoyknowledge 的架构天然支持多领域应用（for Coding、for Support、for Research ……），但 **当前战略重心在 for Coding**。在编程场景下把产品做深、做透，比急于横向扩展到其他领域更重要。

原因：
- for Coding 是当前唯一有真实用户的场景，深度和体验质量直接决定采用率
- 编程场景的知识价值链最完整（捕获→检索→推送→维护），做深后的能力可复用到其他领域
- 横向扩展会分散研发资源，而这个阶段每个场景都做不深

### 1.3 核心洞察

代码有编译器、测试、依赖分析、过期检测。文档只有 `git`。

文档的错误——格式错误、描述缺失、与代码不一致、过期腐烂——发现不了，悄悄积累，直到 AI 读到这些错误信息，做出错误决策。

enjoyknowledge 把代码资产的工程纪律带到文档资产上：

| 代码资产有 | 知识文档有 |
|---|---|
| 编译器 = 格式校验 | `doctor` 检查 frontmatter 完整性 |
| Linter = 风格检查 | `doctor` 检查 description 缺失、条目超量 |
| 过期检测 = deprecation | `doctor` 检查 timestamp 过期、AGENTS.md 过期 |
| 代码审查 = PR review | 知识文件在 `.enjoyknowledge/` 里跟着代码走 git PR |
| 依赖分析 = dependency graph | 跨文件 tags + 目录索引 |

---

## 2. 三层架构

enjoyknowledge 的核心架构原则是**三层分离**。三层各自独立演进，换上层不伤下层：

```
┌─────────────────────────────────────────────┐
│  Layer 3: 领域应用 (Profiles)                │
│  for Coding (primary) / for Support / …     │
│  定义：目录结构、种子文件、工作流、AI 入口     │
├─────────────────────────────────────────────┤
│  Layer 2: 知识管理原语 (Core Engine)          │
│  add / doctor / grep / ls / cat / tree / fix  │
│  不写死目录名，不关心领域语义                  │
├─────────────────────────────────────────────┤
│  Layer 1: OKF 文件格式                        │
│  Markdown + YAML frontmatter                 │
│  通用格式，任何工具都能消费                    │
└─────────────────────────────────────────────┘
```

### 2.1 Layer 1 — OKF 文件格式

所有知识文件都是标准 Markdown + YAML frontmatter，兼容 Google OKF v0.1。不引入专有格式。任何编辑器、任何 AI 工具、任何文档生成器都能直接读写。

核心字段：
- `description` — 一行摘要。索引、搜索、推送的核心字段
- `tags` — 跨分类标签。分类由目录名承载，tags 用于跨目录关联
- `timestamp` — 最后更新时间。doctor 用此判断过期
- `title` — 人类可读标题

### 2.2 Layer 2 — 知识管理原语 (Core Engine)

引擎是一套 Unix 风格的动词命令。它只做四件事：

1. **遍历目录** — 递归读取 `.enjoyknowledge/`，构建内存索引
2. **解析 frontmatter** — 提取 description、tags、timestamp
3. **提供读写** — `ls` / `tree` / `cat` / `grep` / `add`
4. **健康检查** — `doctor` / `fix`

引擎的关键设计决策：**不写死分类名**。`gotchas/`、`architecture/`、`contracts/`、`regulations/` —— 引擎不关心这些名字意味着什么。它只看目录名，推导单数形式作为 type，然后索引。换领域应用不需要改引擎一行代码。

### 2.3 Layer 3 — 领域应用 (Profiles)

领域应用是一个**配置包**，包含：
- 目录结构定义（哪些子目录放在 `.enjoyknowledge/` 下）
- 种子文件模板（每个目录下的 `*.md` 骨架）
- AI 入口模板（AGENTS.md 或等效文件）
- 工作流约定（knowledge-tasks/ 结构、审核流程）

当前内置的领域应用是 **for Coding**，定义了 `architecture/`、`gotchas/`、`patterns/`、`business/`、`decisions/` 五个长期知识目录。这是当前战略重心——**在这五个目录下把产品体验做深，而非扩展更多目录。**

### 2.4 for Coding 的纵深蓝图

"做深 for Coding"的具体含义，不是加目录，而是在每个环节上提升知识的密度和可用性：

| 维度 | 当前（v0.1） | 做深后 |
|---|---|---|
| **种子文件** | 简单骨架 + 占位符 | 带填写指南、常见场景示例、反例警示、跨文件关联提示 |
| **捕获体验** | `add` 追加条目 | `add` 自动检测重复/相似条目，提示合并；自动建议 tags；从 git commit 提取可能的知识入口 |
| **质量保障** | `doctor` 5 项结构检查 | 增加内容质量检查：描述与内容一致性、跨文件引用有效性、知识新鲜度评分 |
| **搜索能力** | `grep` 结构感知搜索 | 关联推荐（`grep --related`）、语义搜索可选增强（`grep --semantic`） |
| **AI 集成深度** | AGENTS.md 推送摘要 | 按上下文智能选择推送范围（不是全量推送）；对不同 AI 工具的接入优化 |
| **团队工作流** | 文件在 `.enjoyknowledge/` 里跟 git | 知识 PR 审核流程、知识变更与代码变更的关联审查、上游知识库更新检测 |
| **代码编织** | — | `.enjoyknowledge/rules/` 规则管理（三层防护）：源规则带「适用范围」标注 → AGENTS.md `RULES` 块按语言/框架分区推送 → `doctor` 检查缺标注、源与推送块一致性；AI 自行判断规则适用性，不依赖机械路径分发；工具特有规则（Cursor `globs`）留原生文件；git hooks；PR 模板 |

### 2.5 远期：其他领域应用

架构支持以后扩展到其他领域，但这是远期方向，不是当前优先级：

```
for Coding (primary │ 当前战略重心)
       ↓ 未来可能扩展
for Support │ for Research │ for Sales │ for Legal │ for GameDev │ for DataEng
```

这些领域应用共享同一个 Core 引擎，只是换一套目录结构和种子文件。for Coding 做深后的核心能力（质量检查、关联推荐、团队工作流）自然复用到其他领域。

---

## 3. 知识生命周期

一条知识从产生到被 AI 消费，穿越五个阶段：

```
捕获 (Capture) → 索引 (Index) → 发现 (Discover) → 消费 (Consume) → 维护 (Maintain)
```

### 3.1 捕获

两阶段写入设计：

**碎片记录** — 编码间隙，一行命令追加：
```bash
enjoyknowledge add gotchas/export.md "## Excel 内存溢出
- 现象: SXSSFWorkbook 未关闭导致 OOM
- 方案: try-with-resources"
```

`add` 自动补全 frontmatter（description 从标题提取，timestamp 写当前时间），自动更新 AGENTS.md 推送块。

**格式化** — 任务边界或独立会话，人工补充因果链、标签、关联上下文。在编辑器中完成，不是 CLI 的事。

### 3.2 索引

启动时一次性加载所有 frontmatter 到内存。文本索引不是向量索引——在当前知识库体量下（几十到几百个文件），结构搜索比语义搜索更可靠、更可解释、zero token 成本。

目录名即分类。引擎自动推导单数形式（`gotchas/` → `Gotcha`），不需要配置文件声明映射。

### 3.3 发现

以 `grep` 为核心的**结构感知搜索**——不同于系统 grep，它能：
- 定位到 Markdown `##` 段结构
- 附带上下文 snippet（description + 匹配段落）
- 按 `--type`（目录名）和 `--tags` 过滤
- 区分长期知识（`.enjoyknowledge/`）和临时材料（`knowledge-tasks/`）

### 3.4 消费

**双通道设计**：

| 通道 | 机制 | 时机 | 成本 |
|---|---|---|---|
| **推送** | AGENTS.md 内嵌 `ls` 输出摘要块 | AI 工具启动时自动加载 | 零——AI 还没开口就知道知识库全貌 |
| **拉取** | `ls` / `tree` / `cat` / `grep` | AI 需要深入时按需探查 | 极低——只拉取相关文件 |

推送回答「有什么」，拉取回答「具体是什么」。

**做深方向**：推送不是永远全量推送。当知识库增大时，推送块应该根据上下文智能缩减（只推送与当前任务相关的知识），保持 token 成本低。

### 3.5 维护

`doctor` 是知识文件的 linter，执行 5 项检查：

| 检查项 | 检测内容 |
|---|---|
| 缺 frontmatter | 文件没有 YAML 头部 |
| 缺 description | frontmatter 中无 description 字段 |
| 超出预算 | 单文件条目数超过阈值 |
| AGENTS.md 过期 | 推送块与实际索引不一致 |
| 待归档任务 | knowledge-tasks/ 中有完成但未迁移的材料 |

`fix` 自动修复可处理项，`doctor --ci` 在 CI 管线中零人工介入。

**做深方向**：增加内容质量检查——不只检查结构是否完整，还检查内容是否有效（描述与正文一致、引用的其他文件是否存在、知识是否已过时）。

---

## 4. 多项目与团队架构

### 4.1 独立知识库模式

```
team-knowledge/          ← 独立 Git 仓库
  .enjoyknowledge/
    architecture/
    gotchas/
    patterns/
    …

project-a/               ← 项目仓库
  AGENTS.md              ← 引用 team-knowledge 的摘要

project-b/
  AGENTS.md
```

通过 `init --link` 引用上游知识库。引擎透明合并本项目和上游的索引——`ls` 同时列出两边的文件，`doctor` 标注知识归属。

### 4.2 权限与治理

权限控制归 Git。知识库的「谁可以改什么」就是 Git 仓库的 branch protection + PR review。enjoyknowledge 不重新发明权限系统。

```
Tech Lead → 审批知识库 PR
开发者   → fork + PR 贡献知识
CI       → doctor --ci 阻止不合规合并
```

### 4.3 多仓库联动（远期）

```
org-knowledge/           ← 组织级知识库
  .enjoyknowledge/
    architecture/        ← 全公司共享的架构决策
    business/            ← 全公司共享的业务规则

team-a-knowledge/        ← 团队级知识库
  .enjoyknowledge/
    gotchas/             ← A 团队特有的踩坑

project-a-1/             ← 项目仓库
  AGENTS.md              ← 合并 org + team 两级摘要
```

知识溯源链：每个文件通过 OKF 的 `resource` 字段标注来源仓库 URL。

---

## 5. OKF 生态角色

### 5.1 兼容策略

enjoyknowledge 的文件格式完全兼容 Google OKF v0.1。这意味着：
- enjoyknowledge 生成的文件可以被任何 OKF 兼容工具消费
- 第三方适配器、工具生成器、AI 工具集成都可以直接读取 `.enjoyknowledge/`
- 不绑定特定 AI 工具——只要工具能读 Markdown，就能消费这些知识

### 5.2 开放标准参与

enjoyknowledge 遵循 OKF 规范但不越界扩展。Core 的原语（`add`、`grep`、`doctor`）是 OKF 之上的增值层，不修改 OKF 格式本身。

如果 OKF 规范演进，enjoyknowledge 跟进；如果社区出现更好的格式标准，Layer 1 可以替换，Layer 2/3 不受影响——这正是三层分离的意义。

---

## 6. 集成架构

### 6.1 AI 工具集成

```
┌──────────────────────────────────┐
│  Cursor / Copilot / Claude Code  │  ← AI 工具
└──────────────┬───────────────────┘
               │ 启动时加载
               ▼
┌──────────────────────────────────┐
│  AGENTS.md (或等效入口文件)       │  ← 推送通道
│  ┌────────────────────────────┐  │
│  │ enjoyknowledge_LS_START    │  │
│  │ architecture/overview.md   │  │
│  │   → 项目模块划分与边界      │  │
│  │ gotchas/export.md          │  │
│  │   → 导出 OOM 与 status 丢失 │  │
│  │ enjoyknowledge_LS_END      │  │
│  └────────────────────────────┘  │
└──────────────────────────────────┘
               │ 需要深入时
               ▼
┌──────────────────────────────────┐
│  enjoyknowledge CLI              │  ← 拉取通道
│  ls / grep / cat / tree          │
└──────────────────────────────────┘
```

AI 工具的接入只需要做一件事：在启动时加载 `AGENTS.md`。enjoyknowledge 生成的 `AGENTS.md` 遵循通用 Markdown 格式，不要求工具做特殊处理。

### 6.2 CI/CD 集成

```bash
# CI 管线中的典型用法
enjoyknowledge doctor --ci          # 不通过则 pipeline 失败
enjoyknowledge fix --check          # 预览可自动修复的问题
```

### 6.3 MCP Server（远期）

当前不提供 MCP Server——文件系统驱动已经足够，MCP 会引入额外的进程管理和复杂度。当以下条件之一满足时，MCP Server 作为可选增强：

1. 用户反馈文件系统调用在特定 AI 工具中有性能或上下文问题
2. 需要跨网络访问远程知识库（非本地文件系统）
3. 社区贡献了高质量的 MCP 适配器且需求明确

MCP Server 是 Core 的一个可选前端，不影响 Core 的设计和功能。

---

## 7. 非目标

明确 enjoyknowledge **不做**的事，保持边界清晰：

| 非目标 | 原因 |
|---|---|
| **代码管理** | 代码属于 Git + 编译器 + 测试，enjoyknowledge 只管理知识文件 |
| **通用知识库** (Notion/Confluence) | 不是文档协作平台，是文件系统的工程化层 |
| **AI 编码工具** | 不做 agent、不做 spec 框架、不做 prompt 编排 |
| **项目管理** (Jira/Linear) | 知识库不是任务追踪系统 |
| **AI Memory 系统** (mem0/MemOS) | 不做向量存储和语义记忆，以文件为唯一真相源 |
| **专有格式** | 不发明新格式，全部采用标准 Markdown + YAML |
| **可视化界面** (初期) | CLI 优先；Web UI 仅作为远期可选项 |

---

## 8. 设计原则

1. **文件即真相源** — 没有数据库，`.enjoyknowledge/` 就是全部状态。git diff 就是变更记录。
2. **目录即分类** — 不额外声明类型映射，目录名承载语义，引擎只做推导。
3. **Unix 动词复用** — `ls`、`cat`、`grep`、`tree` 就是入口。AI 不需要学新 DSL。
4. **推送 + 拉取** — 推送降低感知成本，拉取提供按需深入。不堆上下文。
5. **纠错优于组织** — `doctor` 比完美的分类体系更重要。先知道错了，再慢慢整理。
6. **通用引擎 + 可换预设** — Core 不写死领域名，Profile 可插拔。
7. **OKF 兼容但不绑定** — 兼容开放标准，但 Layer 1 可替换。
8. **渐进复杂度** — 个人项目用 `init` 默认即可，团队按需扩展 `--template` 和 `--link`。

---

*文档版本: 1.1 | 最后更新: 2026-06-26*
