# enjoyknowledge 术语表

> 所有文档共享的统一术语定义。按字母序排列。
>
> **v4.2 同步**：本表是 v3 → v4 → v4.2 后的统一版本。v3 概念（AGENTS.md 推送块/3 层分离/OKF 保留名）已下线；v4 概念（工作流/SoT/多工具路由）已加入；**v4.2 修订**（首发 2 工具 Claude + Cursor，命令 sync → export）。

---

## 数字

### 2 层深度
`.enjoyknowledge/` 目录深度约束：长期知识 ≤ 2 层（`category/file.md`）；任务暂存区不限制（`knowledge-tasks/<REQ-ID>/...`）。

### 2 个核心工作流
> enjoyknowledge v4.2 预置的工作流（YAML 定义，可被用户覆盖/扩展）：v0.2 收缩到 `onboard` / `capture` 2 个。详见 [architecture/workflows.md](../02-design/architecture/workflows.md)。

### 6 阶段设计流程
enjoyknowledge 文档的组织结构：`00-vision/`（定位+路线图）/ `01-philosophy/`（哲学+术语）/ `02-design/`（设计+架构）/ `03-discussion/`（过程讨论）/ `04-changelog/`（变更记录）/ `99-archive/`（归档）。

### v0.2 多工具入口
v4.2 决定：v0.2 首发 2 工具（**Claude + Cursor**）的入口文件。架构上保留 9 工具 adapter trait（Codex / Copilot / Windsurf / Cline / Trae / Gemini / Generic），v0.3+ 渐进。Claude 入口 `.claude/skills/*.md`（前文 `/remember` 追加到 `CLAUDE.md`）和 Cursor 入口 `.cursor/rules/*.mdc`（独立文件 + frontmatter globs/alwaysApply）是 v0.2 唯一支持的两个目标。

---

## A

### AGENTS.md
AI 入口文件（v4 定义）。位于项目根目录，**≤ 50 行的路由表**——不复制 SoT 内容，只放 6 阶段目录的链接 + 当前阶段标注 + 必读安全规则。AI 30 秒读完即知项目是什么 / 在哪 / 接下来做什么。`enjoyknowledge export` 自动生成（v0.2 重命名：sync → export）。

### apply
`filter: { applies_to: "..." }` 的语义：规则的"适用范围"（自然语言声明适用文件类型/语言/框架，如 `applies_to: ["*.rs", "src/cli/**"]`）。详见 workflows.md §7.3。

---

## C

### capture
2 个核心工作流之一（W2，v0.2 砍 5→2 工作流后保留）。把对话中发现的隐性知识沉淀到 SoT 的工作流。**CLI 接入**：`enjoyknowledge workflow capture`（v0.2.1 实现）。触发：`/remember "..."` 或 AI 建议。流程：分类（gotcha/decision/pattern/rule）→ 校验必填字段 → 写 SoT → 更新索引。详见 workflows.md §4.2。**v0.2.1.1 修复**（commit `7454797`）：默认路由路径全小写（`gotchas/gotchas.md` / `decisions/decisions.md` 等），与 `init` 命名一致。

### Core
enjoyknowledge Core。Layer 2：知识管理原语层（`init` / `ls` / `tree` / `grep` / `cat` / `add` / `doctor` / `fix` / `export` / `workflow`），不写死目录名、不关心领域语义。v0.2 共 10 个 CLI 命令（`workflow` 子命令下 `onboard` + `capture` 2 个工作流）。

---

## D

### decision
10 类知识资产之一。记录"为什么这样设计"的决策。**必填字段**：`reversible: bool` + `decided_at: YYYY-MM-DD`。分类：`kind: decision` 路径下。

### doctor
`enjoyknowledge doctor`。健康检查命令。v0.2 检查：缺 frontmatter / 缺必填字段（如 gotcha 的 `trigger`）/ SoT 过期 / 多工具 export 一致性。**4 项检查**（v0.2 砍 5→4：永久禁用 Rule-Code 一致性——扫描 API 格式需 NLP 级理解）。

---

## F

### filter
YAML 工作流步骤的过滤条件。5 种类型：`frontmatter_xxx`（按 frontmatter 字段）/ `trigger_file_match`（按触发文件 glob）/ `applies_to`（按规则适用范围 glob）/ `content_match`（按内容 substring/regex）/ `frontmatter_reversible`（按布尔字段）。**语法定义见 workflows.md §7**。

### for Coding
enjoyknowledge 基于 Core 的第一个领域应用，面向 AI 编程场景。定义默认知识结构（10 类资产）+ **v0.2 多工具入口（首发 Claude + Cursor）** + 2 个核心工作流（onboard + capture）。**v4 不再硬编码目录**（v3 写死 `architecture/` / `gotchas/` 等被废弃）——目录名是路由表的一个约定项，可由用户配置。

### for X
enjoyknowledge 的未来领域应用统称：`for Support` / `for Research` / `for Sales` / `for Legal` 等。同一 Core + 不同的知识类型 + 不同的入口 + 不同的工作流。

### frontmatter
Markdown 文件开头的 YAML 元数据块。v4 统一 schema：`id` / `kind` / `description` / `applies_to` / `trigger` / `reversible` / `decided_at` / `last_reviewed` / `tags` / `timestamp`（按 `kind` 选用子集，详见 knowledge-types.md §4）。

---

## G

### gotcha
10 类知识资产之一。记录"踩过的坑"。**必填字段**：`trigger`（触发条件字符串，缺它 = doctor 报错）。`trigger` 是灵魂字段——它决定 AI 在什么场景下应该想起来读这条 gotcha。

---

## I

### index.md
SoT 目录下的索引文件。`enjoyknowledge export` 自动维护（v0.2 重命名：sync → export）：列出本目录下所有知识资产的 `id` + `description`。`enjoyknowledge_LS_START` 块被 AGENTS.md 引用——v3 把这叫"推送块"，v4 废弃该术语，改用"路由表"。

### id
v4 知识资产的唯一标识符。**frontmatter 必填**。命名约定：`<kind>-<slug>`（如 `gotcha-utf8-encoding-broken`）。路径即 ID 的延伸（`gotchas/utf8-encoding-broken.md` ↔ `id: gotcha-utf8-encoding-broken`）。

---

## K

### knowledge-tasks/
项目根目录下的任务暂存区，与 `.enjoyknowledge/` 并列。用于保存短期任务材料、临时判断、任务内笔记。任务结束或审查后，把可复用内容迁入 `.enjoyknowledge/`。

---

## L

### last_reviewed
v4 frontmatter 字段。`YYYY-MM-DD` 格式，标记知识资产的最后人工审核时间。`doctor` 超过 90 天未审的会警告。

---

## M

### managed section
v0.2 首发 2 工具（Claude + Cursor）入口文件中"由 enjoyknowledge export 管理的部分"。通过 `<!-- ek:managed:start -->` / `<!-- ek:managed:end -->` 标记区分。export 时只重写 managed section，不破坏用户手写内容。

---

## O

### onboard
2 个核心工作流之一（W1，v0.2 砍 5→2 工作流后保留）。AI 工具首次进入仓库时建立项目心智模型。**CLI 接入**：`enjoyknowledge workflow onboard`（v0.2.1 实现）。触发：AI 工具启动 / `enjoyknowledge workflow onboard`。流程：读 AGENTS.md 路由表 → 读 POSITIONING → 读 DESIGN-PHILOSOPHY → 读 DESIGN → 读 gotchas（severity 4-5）→ 读 active decisions（status = active 或缺失）。详见 workflows.md §4.1。

---

## P

### preflight
~~5 个核心工作流之一（W3）~~ **v4.2 永久禁用**（v0.2 砍 5→2 工作流：只留 onboard + capture）——PR 提交前 / AI 大改前检查冲突。原触发：`on_pre_commit` / `on_pr_open` / `enjoyknowledge preflight`。原流程：找相关 gotcha（按 trigger_file_match）→ 找相关 decision（按 applies_to）→ 找相关 rule → 跑 rule_code_sync 检查。原详见 workflows.md §4.2（已删）。

### prd-preprocess
~~5 个核心工作流之一（W2）~~ **v4.2 永久禁用**（v0.2 砍 5→2 工作流）——把用户需求转化为结构化任务清单。原触发：用户输入需求。原流程：找相关 business / architecture / gotcha → 生成 task list 表格。原详见 workflows.md §4.5（已删）。

---

## R

### required
工作流步骤字段。`required: true`（默认）= 失败阻塞工作流；`required: false` = 失败仅 warning。详见 workflows.md §3.2 + §5。

### rule
10 类知识资产之一。`id` + `applies_to`（自然语言或 glob）。**v0.2 决策**：原 `doctor` 检查 Rule-Code 一致性（`rule_code_sync`）已永久禁用——扫描 API 格式需 NLP 级理解，不是 grep 能做的。R-Code 概念保留为未来研究方向。

### 路由表
AGENTS.md 的核心内容（v4 引入）。**不复制 SoT 内容，只放链接**：6 阶段目录的入口路径 + 当前阶段标注 + 必读安全规则。≤ 50 行。`enjoyknowledge export` 自动生成。

---

## S

### SoT
Source of Truth。`v0.4.2`（2026-06-08）后：3 表 SN 软连接 + `pipeline_node` 是 SoT 主体（litree 经验）。enjoyknowledge v4 把这概念借来——**唯一真值源 = `.enjoyknowledge/` 下的 markdown 文件**；v0.2 首发 2 工具（Claude + Cursor）的入口文件都是 export 出来的副本，不是 SoT。**"Rule-Code 同步检测"（致命反模式）**：SoT 缺失时不能静默降级，必须显式失败。

### sync
~~5 个核心工作流之一（W5）~~ **v0.2 重命名为 `export`**（1 工具时 sync 撒谎，export 诚实单向导出）。详见 [export](#export) + workflows.md §4.3。

### 版本节奏
**对外只承诺 5 个版本号**（v0.3 / v0.4 / v0.5 / v1.0 / v1.x+），不细分 v0.3.1 / v0.3.2。**v0.3 = 一站式收尾**（核心 2 大类：捕获体验 + 质量保障深化），预计 1-2 个迭代。**v0.4 合并发布**（原 v0.3 剩余 4 大类 + 原 v0.4 全部 5 大类 = 9 大类）。**理由**：版本号对外是噪音，不是价值；功能完整 + 端到端可用 > 细分版本号。详见 [ROADMAP.md](../00-vision/ROADMAP.md) §版本策略。

---

## T

### timestamp
frontmatter 字段。`YYYY-MM-DD` 格式。`enjoyknowledge add` 时自动更新。

### trigger
frontmatter 字段。**gotcha 必填**。值是触发条件字符串（自然语言或 glob），AI 读到对应场景时能想起这个 gotcha。**灵魂字段：缺它 = doctor 报错。**

### tools.yaml
`.enjoyknowledge/.config/tools.yaml`。声明启用哪些 AI 工具入口、每个工具用哪个模板。`export` 工作流读取（v0.2 重命名：sync → export）。

---

## W

### workflows/
`.enjoyknowledge/workflows/`。YAML 工作流定义目录。v0.2 每个工作流一个 YAML 文件（`onboard.yaml` / `capture.yaml`）。用户可加新文件 = 加新工作流（vibe-coding friendly）。

### workflow / 工作流
v4 定义：**stateless 的步骤序列**，由 YAML 定义，由 AI 工具在触发时执行。不需要 Rust 代码 / 不需要重新编译。状态全部 stateless，每次重读 SoT。

---

## Y

### YAML 元数据驱动
v4 工作流哲学。**工作流 = YAML 文件**（不是编译到命令的 Rust 代码）。**加新工作流 = 加 YAML 文件**（不需要懂 Rust）。对比 B 站 bili-fe-workflow（硬编码 3 个工作流）的最大差异。

---

## Z

### 知识管理原语
enjoyknowledge Core 的 CLI 命令集：`add`（带 schema 校验的创建）/ `ls`（带 description 的目录浏览）/ `cat`（读全文）/ `grep`（结构感知搜索）/ `doctor`（健康检查）/ `export`（多工具 export，v0.2 重命名自 sync）。所有原语不写死目录名，不关心领域语义。

### 知识资产
与代码资产对应的概念。指工程团队产生的知识文件——架构、决策、踩坑、业务规则、设计原则。代码有编译器 / 测试 / 依赖分析；文件资产只有 `git`。enjoyknowledge 把代码资产的工程纪律带到文件资产上。

---

## 变更记录

- **2026-06-27 v4.2 修订（part 2）**：v0.2 砍 5→2 工作流（onboard + capture，删 preflight/prd-preprocess）—— workflows.md §4.2 preflight 整段删 + §4.5 prd-preprocess 整段删，§4.3→§4.2 / §4.4→§4.3 章节号顺移，GLOSSARY.md 对应 3 处章节号引用同步改。preflight + prd-preprocess 词条改"v4.2 永久禁用"（保留历史描述）。workflows 词条 5 YAML → 2 YAML。
- **2026-06-27 v4 重写**：删 v3 残留（AGENTS.md 推送块 / 3 层分离 / `.enjoyknowledge/` 默认目录硬编码 / OKF 保留名 / 推送通道 vs 拉取通道二分）；加 v4 概念（路由表 / 5 工作流 / 9 工具 / SoT / frontmatter schema / `trigger` 灵魂字段 / 路由表 / managed section）。
