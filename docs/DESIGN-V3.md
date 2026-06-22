# enjoyknowledge 设计 — V3

> 状态: 草案 | 2026-06-22
>
> 本文档描述 enjoyknowledge 的产品架构：知识的流向、各环节的设计选择、以及它们如何协作。
> 为什么这样设计，见 [DESIGN-PHILOSOPHY.md](./DESIGN-PHILOSOPHY.md)。

---

## 1. 整体架构

enjoyknowledge 做的事情可以概括为一条流：

```
记录 → 索引 → 发现 → 触达
```

一条知识从被写下到被 AI 消费，穿过四个环节。每个环节服务于两种功能——**索引**（快速定位"有没有"）和**叙事**（理解"为什么"）。这两种功能贯穿始终，不是分给不同模块。

整个系统分成 Core 和领域应用两层。Core 是通用知识资产引擎；for Coding 是当前默认领域应用，不是产品边界的全部。

| 层 | 职责 | 当前内容 |
|---|---|---|
| enjoyknowledge Core | 知识文件的格式、索引、检索、写入、健康检查 | OKF frontmatter、`ls`、`grep`、`cat`、`add`、`doctor` |
| enjoyknowledge for Coding | AI 编程场景的目录、任务暂存区、AI 入口 | `architecture/`、`gotchas/`、`patterns/`、`business/`、`decisions/`、`knowledge-tasks/`、AGENTS.md |
| 未来 for X | 其他领域应用 | support、research、sales、legal 等 |

在 for Coding 内部，系统由两部分协作，不是两个产品：

| 部分 | 职责 | 核心机制 |
|---|---|---|
| 知识引擎 | 知识的写、索引、检索、健康检查 | CLI 命令 (`add`, `ls`, `grep`, `cat`, `doctor`) |
| AI 适配层 | 让 AI 工具在正确时间拿到正确知识 | AGENTS.md 推送通道 + CLI 拉取通道 |

引擎不假设消费者是谁。AI 适配层用引擎的命令搭起来，不是独立的知识系统。

---

## 2. 知识载体：文件格式与目录

### 2.1 文件格式

每份知识文件是标准 Markdown + YAML frontmatter，兼容 [OKF](https://github.com/google/open-knowledge-format) v0.1。不引入专有格式。

```yaml
---
title: 导出功能踩坑
description: 导出超时、OOM、status 字段缺失的踩坑记录
tags: [export, excel, performance]
timestamp: 2026-06-21
---
```

字段约定：

| 字段 | 用途 | 必需 |
|---|---|---|
| `title` | 人类可读标题，缺省用文件名 | 否 |
| `description` | 一行摘要——索引功能的核心字段 | 强烈推荐 |
| `tags` | 跨分类过滤，正交于目录结构 | 否 |
| `timestamp` | ISO 8601 | 否 |
| `resource` | 知识来源 URI（OKF 标准，空=本项目原创） | 否 |

> 文件所在目录名即分类（如 `gotchas/` 下的文件属于 Gotcha 类）。不再需要 frontmatter 中的 `type` 字段。引擎不写死分类名——目录名叫什么，分类就是什么。

`description` 是系统中最重要的字段。所有索引路径——`ls`、`tree`、AGENTS.md 推送块——都用它来表达"这个文件讲了什么"，避免打开文件才知道不相关。一行足够长来传达信号，足够短来控制 token 成本。

正文自由 Markdown。推荐用 `##` 二级标题作为段界——`grep` 以此定位匹配内容所属的知识条目。每个 `##` 标题为一个**条目**（entry），是知识的基本叙事单元。doctor 以 `##` 数量统计条目数（超过 20 条建议拆分）。

### 2.2 目录结构

文件系统承担索引职能。目录名即分类，文件名即标题，路径即 ID。

```
.enjoyknowledge/
├── architecture/           # 项目结构、技术选型、模块边界
│   ├── overview.md
│   └── tech-stack.md
├── gotchas/                # 开发中踩过的坑
│   ├── export.md
│   └── auth.md
├── patterns/               # 验证过的模式和最佳实践
├── business/               # 业务规则、计算公式
├── decisions/              # 架构决策记录 (ADR)
├── index.md                # 目录的目录（OKF 保留文件）
└── log.md                  # 变更历史（OKF 保留文件）

knowledge-tasks/            # 项目根目录下，任务过程材料
└── REQ-042/
    ├── notes.md
    ├── gotchas.md
    └── decisions.md
```

`.enjoyknowledge/` 只放已审核、可复用、值得长期推送给 AI 的知识。`knowledge-tasks/` 与 `.enjoyknowledge/` 并列，放短期任务过程材料：调研笔记、临时判断、未确认踩坑、任务内决策草稿。任务材料默认不进入 AGENTS.md 推送块，也不作为长期知识索引；任务结束后，经人工或独立审查整理，再迁入 `.enjoyknowledge/` 的对应目录。

**结构约束**：

- 深度不超过 2 层。每多一层，定位成本翻倍——`ls gotchas/` 一次操作 vs `ls gotchas/export/` 两次操作
- 文件名自解释：在 `gotchas/` 下用 `export.md`，不需要 `gotcha-export.md`
- 聚合而非原子：一个文件聚合一个主题的所有条目。条目数（`##` 标题数）超过 20 时 doctor 建议拆分
- 任务过程材料不放入 `.enjoyknowledge/`。未审核内容先进入 `knowledge-tasks/<REQ-ID>/`，审核后再沉淀到长期目录

### 2.3 保留文件

`index.md` 和 `log.md` 是 OKF 保留文件，由 enjoyknowledge 生成和维护。不属于任何知识分类。

---

## 3. 写入：碎片到叙事

### 3.1 两阶段写入

索引和叙事对"能用的草稿"要求不同，写入拆成两个阶段：

**第一段——碎片记录**：编码间隙，一行描述 + 分类，零摩擦。用 `add` 追加到对应文件末尾：

```bash
enjoyknowledge add gotchas/export.md "## Excel内存溢出
- 现象: SXSSFWorkbook 未关闭导致 OOM
- 方案: try-with-resources"
```

`add` 自动补全 frontmatter（从文件名和标题提取 description、timestamp 写当前时间），自动更新 AGENTS.md 中的 `description` 摘要。

**第二段——格式化**：任务边界或独立会话中，补全标签、补充因果链条、关联上下文。不是 `add` 做的事——人在编辑器中完成。两个阶段对应两种心智，不是同一个人一次做完的事。

### 3.2 回写原则

追加，不修改已有条目。保证知识不丢失，冲突概率最低（不同人同时加不同主题的坑，编辑不同文件）。

---

## 4. 索引：定位而不阅读

索引功能回答"有没有相关知识，如果有，在哪里"。目标是让使用者在打开文件之前就能判断相关性。

### 4.1 `ls` / `tree`

```bash
enjoyknowledge ls
```

输出每个文件的 `description`，一行看全貌：

```
architecture/
  overview.md           — 项目整体架构、模块划分、部署拓扑
  tech-stack.md         — Rust + React + PostgreSQL 技术栈选型
gotchas/
  export.md             — 导出超时、OOM、status字段缺失（3条）
  auth.md               — Token刷新失效、权限缓存不一致（2条）
```

`--bare` 只输出文件名。`tree` 递归到所有子目录，带 description。

### 4.2 `grep`：结构感知搜索

```bash
enjoyknowledge grep "OOM" --type gotchas --tags performance
```

与系统 `grep` 的关键差异：

| | 系统 grep | enjoyknowledge grep |
|---|---|---|
| 匹配单元 | 文本行 | `##` 段 |
| 输出 | `文件:行号:内容` | `文件##段标题 + snippet` |
| 过滤 | 无 | `--type`、`--tags` |
| 结构 | 不理解 Markdown | 定位到所属 `##` 段 |

输出格式：

```
gotchas/export.md##Excel内存溢出
  SXSSFWorkbook 未关闭导致 OOM。方案：try-with-resources
```

`--archive` 标志可将搜索范围扩展到已归档的任务材料。

### 4.3 `index.md`

由系统生成，列出所有知识文件的清单。是索引功能的持久化形态——相当于 `ls --bare` 的快照，供人类浏览和版本控制追踪。

---

## 5. 叙事：理解上下文

索引告诉你在哪，叙事告诉你为什么。

### 5.1 `cat`

```bash
enjoyknowledge cat gotchas/export.md
```

路径相对于 `.enjoyknowledge/`，自动补全前缀。输出完整文件内容。

### 5.2 `##` 段作为叙事单元

正文用 `##` 二级标题分界。`grep` 匹配到某个 `##` 段时，输出段标题和上下文 snippet。使用者据此判断是否需要 `cat` 完整文件。

### 5.3 索引→叙事的标准路径

```
1. ls / AGENTS.md 推送块              → 索引：知道有 gotchas/export.md
2. grep "OOM" --type gotchas          → 索引+叙事：确认相关段存在
3. cat gotchas/export.md              → 叙事：读完整因果链条
```

三步用的都是 Unix 动词，AI 不需要学新概念。

---

## 6. 触达：推送 + 拉取双通道

这是 enjoyknowledge 最核心的架构机制。AI 不会主动发现知识——知识必须被放在它面前。两条通道解决两个不同的问题：

```
推送通道：启动即加载，零成本感知
  AGENTS.md 内嵌 enjoyknowledge ls 的输出
  → AI 启动时自然读到知识库全貌
  → 解决"不知道知识库存在"的问题

拉取通道：有方向后深入探查
  enjoyknowledge ls / grep / cat
  → 根据推送通道提供的方向，按需获取具体内容
  → 解决"知道了方向后怎么找"的问题
```

### 6.1 AGENTS.md

AGENTS.md 是推送通道的载体。它不是知识文件的副本，而是知识库的索引视图：

```markdown
# AGENTS.md

本项目使用 enjoyknowledge 管理 AI 编程的上下文知识。

## 知识库概况

<!-- enjoyknowledge_LS_START -->
architecture/
  overview.md           — 项目整体架构、模块划分、部署拓扑
  tech-stack.md         — Rust + React + PostgreSQL 技术栈选型
gotchas/
  export.md             — 导出超时、OOM、status字段缺失（3条）
  auth.md               — Token刷新失效、权限缓存不一致（2条）
<!-- enjoyknowledge_LS_END -->

以上是当前知识库的所有内容。开始任何编码任务之前，先看这里判断是否有相关知识。

## 命令参考

| 命令 | 为什么不用系统替代品 |
|---|---|
| `enjoyknowledge ls` | 系统 `ls` 看不到 description，等你 cat 完才发现不相关 |
| `enjoyknowledge grep <pattern>` | 系统 `grep` 不理解 `##` 段结构，输出乱序行号 |
| `enjoyknowledge cat <path>` | 路径自动补全 `.enjoyknowledge/` 前缀 |
| `enjoyknowledge add <path> "..."` | 保证 OKF 格式合规，系统 echo 不更新 frontmatter |
```

**维护规则**：

- 由 `enjoyknowledge init` 生成，摘要块由 `add` 自动同步，`doctor` 校验一致性
- 摘要只含 `description`，不含正文——控制上下文预算
- 不要求 AI 工具做特殊处理，它只是项目根目录下的一个普通 Markdown 文件

### 6.2 上下文预算

AI 的上下文窗口有限且昂贵。AGENTS.md 的摘要只占最少的 token——一行 `description` 传达方向，正文通过拉取通道按需获取，不预加载。

### 6.3 标准任务流

```
1. [AI 启动，读到 AGENTS.md 中的知识摘要]           ← 推送，零成本
2. [判断：修导出 bug，gotchas/export.md 相关]       ← 基于 description
3. enjoyknowledge cat gotchas/export.md                ← 拉取，按需
4. [编码...]
5. enjoyknowledge add gotchas/export.md "## 新踩的坑"  ← 回写，自动更新摘要
```

### 6.4 AI 工具入口适配

不同 AI 工具的上下文加载机制不同，enjoyknowledge 在 `init` 时生成对应的入口文件：

| AI 工具 | 入口 | `init --ai` 产物 |
|---|---|---|
| Claude Code / 通用 | 自动加载 AGENTS.md | AGENTS.md（默认） |
| Cursor | `.cursor/rules/*.mdc` | 规则文件 |
| Copilot | `.github/copilot-instructions.md` | 追加指令块 |
| Windsurf | `.windsurf/rules/enjoyknowledge.md` | Markdown |
| Cline | `.clinerules/enjoyknowledge.md` | Markdown |
| Codex | `.codex/prompts/enjoyknowledge.md` | Markdown |
| Trae | `.trae/rules/enjoyknowledge.md` | Markdown |
| Gemini CLI | `GEMINI.md` | 追加 Markdown 块 |
| 通用 | 项目级 prompt 文件 | `enjoyknowledge.md` |

`init --ai generic` 生成一份工具无关的 `enjoyknowledge.md`，人类或 AI 自行引用。`init --ai auto` 自动检测当前环境中的 AI 工具。适配文件的核心内容不变：知识库在哪、有什么、怎么看、怎么记。

---

## 7. 维护纪律

### 7.1 doctor 检查

`enjoyknowledge doctor` 是知识文件的 linter，7 项检查：

| 检查项 | 说明 |
|---|---|
| 缺 frontmatter | 每个 `.md` 有可解析 YAML frontmatter |
| 缺 description | 无 description → 索引通道无法展示摘要 |
| 超出预算 | 单文件超过 20 条 `##` 条目，建议拆分 |
| 疑似重复 | 同 tag 组内条目相似度 > 70% |
| 深度超标 | 目录超过 2 层 |
| AGENTS.md 过期 | `ls` 输出与 AGENTS.md 摘要不一致 |
| 待归档任务 | `knowledge-tasks/` 下已完成未归档的任务目录 |

### 7.2 `fix` — 自动修复

`enjoyknowledge fix` 自动修复可程序化处理的合规问题：

| 可修复 | 修复方式 |
|---|---|
| 缺 `description` | 从正文首段提取或填入模板 |
| AGENTS.md 过期 | 重新生成摘要块 |
| 超出预算 | 将最早 `##` 条目移到归档文件 |
| 待归档任务 | 提取 `knowledge-tasks/<REQ-ID>/` 可复用条目到 `.enjoyknowledge/` |

不可自动修复：缺 frontmatter（需手动补元数据）、深度超标（需重组织目录）、疑似重复（需人工判断）。

### 7.3 归档机制

归档针对已完成任务的 `knowledge-tasks/<REQ-ID>/` 目录：

- `doctor` 检测到已完成的任务目录 → 提示归档
- `fix` 将 `knowledge-tasks/<REQ-ID>/` 下可复用条目提取到 `.enjoyknowledge/` 对应分类，原目录标记为已归档
- `grep --archive` 搜索范围包含已归档的任务材料
- 归档后，任务目录不进入 AGENTS.md 推送块

### 7.4 何时不记录

不是所有知识都值得进入知识库。每一条都在增加维护负担和搜索噪声。以下情况不记录：

- 信息在代码注释中已足够清晰
- 信息会在两个迭代内失效
- 信息可通过 lint 或类型系统自动校验
- 因"完备性"而非"必要性"而记录

doctor 不检查"缺少记录"——策展靠人的判断，不靠工具催促。

### 7.5 知识生命周期

```
碎片记录 → 格式化 → 链接 → 发现 → 归档
```

1. **碎片记录**：编码间隙，`add` 一行写入
2. **格式化**：任务边界，补全 frontmatter、标签、时间戳
3. **链接**：关联相关条目或系统文件
4. **发现**：doctor 检查重复、过期、格式不一致
5. **归档**：任务完成后通过 `fix` 归档



## 8. 设计决策

本节记录影响系统形态的关键工程决策。每一项都是一个具体的权衡结果。

### 8.1 文件系统即索引

当前选择用文件系统承载索引功能。目录名是分类，文件名是标题，路径是 ID。这个选择的前提是文件系统的浏览和搜索已经足够好。如果未来出现更好的索引形式（语义索引、向量检索），这个选择可以换——知识引擎不绑定到文件系统。

### 8.2 聚合而非原子

一个文件聚合一个主题（`gotchas/export.md` 包含导出相关的所有条目），而非一条目一文件。原子粒度在条目少时更精确，但条目线性增长时 `ls` 输出迅速失控。聚合保持 `ls` 始终可浏览，且同主题上下文天然在一起。上限 20 条是经验阈值，以 `##` 标题数计。

### 8.3 深度不超过 2 层

```
2 层: ls gotchas/ → 直接看到 export.md           (1 次操作)
3 层: ls gotchas/ → 看到 export/ → ls export/     (2 次操作)
```

2 层是全局可见性和组织力之间的权衡点。

### 8.4 复用 Unix 动词

AI 已经理解 `ls`、`cat`、`grep`、`tree`。用它们承载知识操作，AI 零学习成本。enjoyknowledge 在这些动词上增加了"理解 Markdown 结构"的能力（`grep` 定位到 `##` 段），动词本身不变。

### 8.5 AGENTS.md ≠ index.md

| | AGENTS.md | index.md |
|---|---|---|
| 位置 | 项目根目录 | `.enjoyknowledge/` 内 |
| 内容 | `ls` 输出 + 命令参考 | 目录的目录 |
| 读者 | AI 编程工具 | 人 |
| 维护 | `add` 自动同步 | 系统生成 |


---

## 9. 扩展点

以下入口对开发者开放。每处都是设计时故意留的接口，不是事后补丁。

### 9.1 模板系统：定义新的知识分类

```
~/.enjoyknowledge/templates/
  legal/          ← 法务场景
  gamedev/        ← 游戏开发
  data-eng/       ← 数据工程
```

```bash
enjoyknowledge init --template legal
```

开发者建一个新目录，放上文件骨架，就是一个新模板。引擎看到 `contracts/` 目录推导分类为 Contract，看到 `regulations/` 推导分类为 Regulation——不需要注册、不需要配置、不需要改引擎代码。这条来自 §8.1"文件系统即索引"：目录名承载语义，引擎只做推导，不写死分类。

模板加载优先级：
1. `.enjoyknowledge/templates/<name>/`（项目级）
2. `~/.enjoyknowledge/templates/<name>/`（用户级）

### 9.2 AI 工具适配：接入新的 AI 编程工具

```bash
enjoyknowledge init --ai generic      # 生成 enjoyknowledge.md
enjoyknowledge init --ai cursor       # 生成 .cursor/rules/enjoyknowledge.mdc
```

新增 AI 工具不需要改引擎。`init --ai generic` 生成标准 Markdown，新工具的任何自定义指令文件都可以引用它。社区贡献的新工具适配只需增加一份模板映射——改的是 `init` 的参数表，不动知识引擎的任何命令。这条来自 §6.1：AGENTS.md 就是文件，不要求工具做特殊处理。

### 9.3 消费者无关：任何工具都能读这些知识

`.enjoyknowledge/` 里就是标准 Markdown + YAML frontmatter。CI 管线做 `doctor`，文档生成器遍历 `decisions/` 和 `architecture/` 生成文档站，AI 代码审查在 PR 时拉取 `business/` 做合规校验——都不需要了解 enjoyknowledge CLI，读文件就行。这条来自 §1：引擎不假设消费者是谁。



---


### 9.4 跨项目知识源：引用远程知识库

```bash
enjoyknowledge init --link ../team-knowledge
```

一个项目可以引用另一个 `.enjoyknowledge/` 目录作为上游知识源。引擎透明地合并本项目和上游的索引——`ls` 同时列出两边的文件，`doctor` 标注知识归属（本项目 / 上游）。

远程（Git URL）是远期的统一路径，但底层不区分来源——因为知识文件就是标准 Markdown，不管是本地路径还是 clone 下来的远程仓库，格式完全一样。引擎只看 `.enjoyknowledge/` 目录，不关心它来自哪里。这一条来自 §2.1：OKF 兼容格式 + 文件系统即索引，让"远程"退化为一个传输问题，不需要引擎理解网络协议。

OKF 的 `resource` 字段用于标注知识出处（空 = 本项目原创，非空 = 源仓库 URL）——不需要 enjoyknowledge 新增字段。


---

## 附录 A：CLI 命令全景

| 命令 | 功能 | 所在章节 |
|---|---|---|
| `init [--ai <tool>] [--template <name>] [--link <path>]` | 初始化知识库骨架 + AI 入口文件 | §6.4, §9.1 |
| `ls [path] [--bare]` | 浏览目录，默认带 description | §4.1 |
| `tree [--bare]` | 递归浏览，含 description | §4.1 |
| `grep <pattern> [--type] [--tags] [--archive]` | 结构感知搜索 | §4.2 |
| `cat <path>` | 读取完整文件 | §5.1 |
| `add <path> <content>` | 追加条目，自动补全元数据 | §3.1 |
| `doctor` | 健康检查 | §7.1 |
| `fix` | 自动修复可处理问题 | §7.2 |

## 附录 B：源码映射

| 源码路径 | 对应章节 |
|---|---|
| `src/cli/init.rs` | §6.4, §9.1 |
| `src/cli/doctor.rs` | §7.1 |
| `src/cli/add.rs` | §3.1 |
| `src/cli/grep.rs` | §4.2 |
| `src/format/` | §2.1 |
| `src/knowledge/` | §4, §5 |
| `src/init/` | §6 |