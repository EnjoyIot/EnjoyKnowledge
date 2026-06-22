# enjoyknowledge 术语表

> 所有文档共享的统一术语定义。按字母序排列。

---

## 数字

### `.enjoyknowledge/`
enjoyknowledge 长期知识库的根目录。位于项目根目录下。包含类型子目录（architecture/、gotchas/、patterns/、business/、decisions/）和 OKF 保留文件（index.md、log.md）。只放已审核、可复用、值得推送给 AI 的知识；短期任务材料放在并列的 `knowledge-tasks/`。

### 2 层深度
`.enjoyknowledge/` 目录结构的深度约束。长期知识文件使用 `category/file.md`，不超过 2 层。不允许 `a/b/c.md`。`knowledge-tasks/<REQ-ID>/` 是任务暂存区，不属于长期知识索引。

### 三层分离
enjoyknowledge 的核心架构原则。Layer 1: OKF 文件格式（通用，不关心领域）。Layer 2: Core 知识管理原语（通用引擎，不写死目录名）。Layer 3: 领域应用（for Coding / for X）。换应用不伤引擎。

---

## A

### add
`enjoyknowledge add <path> <content>`。新增/追加知识的命令。路径决定分类（`gotchas/xxx.md` → Gotcha 类）。行为：文件存在则追加并更新 timestamp；不存在则创建（含 frontmatter 模板）。追加后自动更新 AGENTS.md 推送摘要。

### AGENTS.md
项目根目录下的 AI 入口文件。由 `enjoyknowledge init` 生成。AGENTS.md 内嵌知识摘要（`<!-- enjoyknowledge_LS_START -->` 块），AI 启动时自动加载，零成本感知知识库内容。

---

## B

### bare
`ls --bare` 和 `tree --bare` 的标志。去掉 description，只显示文件名和目录名。用于 AI 已知结构、只需确认文件的场景。

---

## C

### cat
`enjoyknowledge cat <path>`。查看文件内容。路径相对于 `.enjoyknowledge/`。薄封装——补全路径前缀后输出全文。

### Core
enjoyknowledge Core。通用知识资产引擎，负责 OKF 兼容格式、目录索引、frontmatter 解析、结构搜索、写入和健康检查。不绑定编程、客服、研究等具体领域。

---

## D

### description
OKF frontmatter 的核心字段。一行摘要（≤200 字符）。出现在三个地方：AGENTS.md 推送块、`ls` 输出、`tree` 输出。AI 根据 description 判断是否值得 `cat` 这个文件。没有 description = AI 大概率略过。

### doctor
`enjoyknowledge doctor`。健康检查命令。7 项检查：缺 frontmatter、缺 description、超出预算、疑似重复、深度超标、AGENTS.md 过期、待归档任务。

---

## F

### fix
`enjoyknowledge fix`。自动修复命令。可修复的项目：缺 description（填模板）、AGENTS.md 过期（重新生成摘要块）、超出预算（归档旧条目）、待归档任务（提取到 .enjoyknowledge/）。缺 frontmatter 等需手动处理。

### frontmatter
Markdown 文件开头的 YAML 元数据块，以 `---` 分隔。使用 OKF 兼容格式：`type`（必需）、`title`、`description`、`tags`、`timestamp`、`resource`。

### for Coding
enjoyknowledge 基于 Core 的第一个领域应用，面向 AI 编程场景。它定义默认目录（architecture、gotchas、patterns、business、decisions）、任务暂存区（knowledge-tasks）和 AI 入口（AGENTS.md）。这些属于领域应用，不属于 Core。

### for X
enjoyknowledge 的未来领域应用统称。含义是"同一个 Core，换一套领域目录、入口说明和工作流"，例如 for Support、for Research、for Sales、for Legal。

---

## G

### grep
`enjoyknowledge grep <pattern> [--type] [--tags] [--path]`。结构化搜索命令。与系统 `grep` 的本质差异：定位到 Markdown `##` 段结构，附带上下文 snippet，支持按 `--type`（目录名）和 `--tags` 过滤。

---

## I

### index.md
OKF 保留文件名。目录的目录（table of contents），列出本目录下所有概念文件及其 description。可选存在，

### knowledge-tasks/
项目根目录下的任务暂存区，与 `.enjoyknowledge/` 并列。用于保存短期任务材料、临时判断、任务内笔记和未审核发现。默认不进入 AGENTS.md 推送块，也不作为长期知识索引；任务结束或审查后，再把可复用内容迁入 `.enjoyknowledge/`。

---

## L

### log.md
OKF 保留文件名。变更历史。ISO 8601 日期作为 `##` 标题，记录增删改。可选存在。

### ls
`enjoyknowledge ls [path] [--bare]`。核心入口命令。列出目录/文件，默认每文件附带 description。与系统 `ls` 的关键差异：description 让 AI 不打开文件就知道内容。

---

## O

### OKF
Open Knowledge Format。Google 发布的开放规范（v0.1，2026-06），定义 AI 可消费的知识文件的 Markdown + YAML frontmatter 格式和目录结构约定。enjoyknowledge 的文件格式与 OKF 兼容。

---

## P

### 推送通道
AI 被动感知知识库的机制。AGENTS.md 内嵌 `ls` 输出（`enjoyknowledge_LS_START` / `enjoyknowledge_LS_END` 标记之间），AI 启动时自动加载。由 `init` 生成，`add` 自动更新。

### 拉取通道
AI 主动探查知识库的机制。`ls`、`tree`、`cat`、`grep` 四个命令。推送通道已告知"有什么"，拉取通道用于深入查看。

---

## T

### timestamp
OKF frontmatter 字段。最后更新时间，ISO 8601 `YYYY-MM-DD` 格式。`add` 时自动更新。

### tree
`enjoyknowledge tree [--bare]`。递归目录树命令，深度缩进，默认每文件附带 description。

### tags
OKF frontmatter 字段。跨分类标签列表。tags 承担跨分类关联——目录名已经是分类，tags 用于跨目录过滤。

### type
OKF frontmatter 的**必需**字段。概念类型，如 `Gotcha`、`Architecture`、`Pattern`、`BusinessRule`、`Decision`。值等于文件所在目录名单数大写形式。`add` 创建新文件时自动推导。

### 推送块
AGENTS.md 中 `<!-- enjoyknowledge_LS_START -->` ... `<!-- enjoyknowledge_LS_END -->` 之间的内容。包含当前知识库的 `ls` 输出。是 AI 感知知识库的推送通道载体。

---

## W

### 文件资产
与代码资产对应的概念。指工程团队产生的知识文件——架构文档、业务规则、踩坑记录、设计决策、代码规范等。代码有编译器、测试、依赖分析；文件资产只有 `git`。enjoyknowledge 把代码资产的工程纪律带到文件资产上。

---

## Y

### 引擎
enjoyknowledge 的 Layer 2——知识管理原语层。引擎不写死目录名，不关心领域语义。只做四件事：走目录、解析 frontmatter、提供读写、运行 doctor 检查。目录名叫 `gotchas/` 还是 `contracts/`，引擎不感兴趣。`type` 推导算法（目录名单数大写）适用任何英文目录名。

### 预设
见"领域应用"。预设 = 某个领域应用提供的默认目录和工作流。当前默认预设是 for Coding。Core 不变，预设可变。

---

## Z

### 知识管理原语
enjoyknowledge Layer 2 的 CLI 命令集：`add`（带 schema 校验的创建）、`doctor`（知识文件的结构检查）、`grep`（结构感知搜索）、`ls`/`cat`（带摘要的浏览）、`init`（初始化骨架）。所有原语不写死目录名，不关心领域语义。
