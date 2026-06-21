# EnjoyFlow 术语表

> 版本 1.0 | 2026-06-21
>
> 所有文档共享的统一术语定义。按字母序排列。

---

## A

**AGENTS.md** — Linux 基金会 AI Agent 基金会采纳的通用 AI 编程工具配置文件标准。EnjoyFlow 始终生成此文件作为主入口，20+ 工具原生支持。

**ADR (Architecture Decision Record)** — 架构决策记录。当前生效的存于 A8，被替代的归档到 C3。

---

## C

**class** — 知识文档 frontmatter 中的分类字段。格式为 `<字母><数字>_<描述>`（如 `C1_gotchas`）。用户通过语义标签（`--class gotchas`）而非 class ID 交互。

**ContextFlow** — EnjoyFlow 的核心知识流模型。AI 通过 `search` 获取上下文，通过 `record` 写回发现，形成 search → act → record 闭环。不是独立引擎。

---

## D

**doctor** — CLI 命令，诊断知识库健康度（缺 class/tag、文件超标、重复条目、索引不一致）。

---

## E

**EnjoyFlow** — 工程团队的人机共享任务上下文层。提供 CLI 工具 + 知识组织结构，让 AI 编码时知道项目的架构、规范、踩坑、业务规则和当前进度。

---

## F

**fix** — CLI 命令，自动修复 doctor 发现的问题。

**frontmatter** — 知识文档开头的 YAML 块，包含 `class`、`tags`、`last_modified` 字段。

---

## G

**GOTCHAS (C1)** — 踩坑清单。AI 和开发者发现的陷阱，通过 `enjoyflow record gotcha` 追加。

---

## I

**init** — CLI 命令，初始化 EnjoyFlow 项目（生成 `.enjoyflow/` + AGENTS.md + AI 工具配置文件）。

---

## L

**L1/L2/L3 留痕** — 全程留痕的三个层次：L1 任务轨迹（自动）、L2 决策轨迹（AI 记录技术取舍）、L3 经验轨迹（AI 记录踩坑和教训）。

---

## K

**知识源 (Knowledge Source)** — search 的数据来源。可以是本地文件（`.enjoyflow/knowledge-base/`）、Obsidian vault、远程 git 仓库、MCP 服务器。

---

## P

**PATTERNS (C2)** — 最佳实践清单。通过 `enjoyflow record pattern` 追加。

**preset** — 已废弃的概念。旧设计中场景模板的维度预设（如 `bug_fix → [A1, A2, C1, C5]`），现由 AI 自主决定搜什么。

---

## R

**record** — CLI 命令，追加知识到知识库。三种类型：gotcha / pattern / decision。只追加不修改，不审批。

---

## S

**search** — CLI 命令，在知识源中检索知识。返回段级摘要。支持 `--class`、`--tag`、`--archive` 过滤。

**语义标签** — class ID 的人可读别名（如 `gotchas` 对应 `C1_gotchas`）。`--class` 参数接受语义标签，CLI 内部做映射。

**源适配器 (Source Adapter)** — 接入不同知识源的适配层。同统一接口 `search / read / append / listFiles`。

---

## 数字

**.enjoyflow/** — EnjoyFlow 的所有文件所在的隐藏目录。包含 knowledge-base/、lifecycle/、snapshots/、config.yaml。

**三层合约** — 产品合约（A6，做什么）、设计合约（A7，长什么样）、技术合约（A1-A5+A10，怎么建）。团队流程维护，框架只读取和供给。

**32 类** — ABCD 四类知识共 32 子类的完整分类体系。详见 KNOWLEDGE-ARCHITECTURE.md。

**四通道** — Define（合约组装）、Build（执行）、Verify（独立验证）、Learn（知识归档）。逻辑模型，不直接映射到 CLI 命令。

---

*文档版本: 1.0 | 最后更新: 2026-06-21*
