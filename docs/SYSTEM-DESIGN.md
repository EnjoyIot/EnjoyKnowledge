# EnjoyFlow 系统设计

> 版本 1.0-draft | 2026-06-21
>
> 回答 EnjoyFlow 内部怎么运作——模块划分、数据流、关键算法、组件边界。
> 本文档不涉及用户体验（PRODUCT-DESIGN.md）、不涉及设计哲学（DESIGN-PHILOSOPHY.md）。

---

## 1. 系统边界

EnjoyFlow 不做的事和要做的事一样重要：

```
┌─────────────────────────────────────────────┐
│  不归 EnjoyFlow 管                          │
│  ┌─────────┐ ┌──────────┐ ┌─────────────┐  │
│  │AI 怎么写 │ │合约怎么定 │ │任务怎么调度  │  │
│  │代码     │ │（团队流程）│ │（项目管理）  │  │
│  └─────────┘ └──────────┘ └─────────────┘  │
│                                             │
│  归 EnjoyFlow 管                             │
│  ┌─────────────┐ ┌──────────────────────┐   │
│  │知识在哪     │ │知识怎么检索           │   │
│  │（多源接入）  │ │（search）            │   │
│  └─────────────┘ └──────────────────────┘   │
│  ┌─────────────┐ ┌──────────────────────┐   │
│  │知识怎么写回 │ │知识库健康度           │   │
│  │（record）   │ │（doctor/fix）         │   │
│  └─────────────┘ └──────────────────────┘   │
└─────────────────────────────────────────────┘
```

---

## 2. 整体架构

```
                    ┌──────────────────┐
                    │   AI 编码工具     │
                    │ (Cursor/Copilot/  │
                    │  Claude Code/...) │
                    └────────┬─────────┘
                             │ 读 AGENTS.md
                             │ 调 CLI 命令
                             ▼
              ┌──────────────────────────┐
              │       AGENTS.md          │
              │  "想搜架构 → 运行         │
              │   enjoyflow search ..."  │
              └──────────────────────────┘
                             │
                             ▼
┌─────────────────── enjoyflow CLI ──────────────────┐
│                                                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌────────┐│
│  │  init   │  │ search  │  │ record  │  │ doctor ││
│  │初始化项目│  │检索知识 │  │写入知识 │  │诊断修复││
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬───┘│
│       │            │            │            │     │
│       ▼            ▼            ▼            ▼     │
│  ┌─────────────────────────────────────────────┐  │
│  │              源适配器层 (Source Adapters)      │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────┐ │  │
│  │  │filesystem│ │   git    │ │     mcp      │ │  │
│  │  │ adapter  │ │ adapter  │ │   adapter    │ │  │
│  │  └──────────┘ └──────────┘ └──────────────┘ │  │
│  └─────────────────────────────────────────────┘  │
│                                                     │
└─────────────────────────────────────────────────────┘
                             │
                             ▼
              ┌──────────────────────────┐
              │       知识源              │
              │  ┌──.enjoyflow/knowledge─│
              │  ├──~/Obsidian/vault     │
              │  ├──git repos            │
              │  └──MCP servers          │
              └──────────────────────────┘
```

四个模块四套职责，互不调用。共享的是源适配器层——所有模块都通过适配器读写知识。

---

## 3. CLI 模块

### 3.1 init

```
enjoyflow init [path]             # 默认初始化
enjoyflow init --scan [path]      # 扫描已有项目
enjoyflow init --describe "..."   # 自然语言描述
enjoyflow init --link <path>      # 引用已有知识库
enjoyflow init --ai <tool>        # 指定 AI 工具（默认 auto 检测）
```

**内部流程**：

```
[1] 确定目标路径
    - 指定 path → 使用 path
    - --link → 不创建目录，只写工具配置

[2] 生成目录骨架
    .enjoyflow/
    ├── config.yaml
    ├── knowledge-base/
    │   ├── project/
    │   │   ├── ARCHITECTURE.md      # 最小模板
    │   │   └── CODE-STANDARDS.md
    │   └── development/
    │       └── GOTCHAS.md
    ├── lifecycle/
    │   └── active-sprint/
    ├── snapshots/
    └── .index.json                  # 惰性索引

[3] 生成 AI 工具文件
    - AGENTS.md（始终）
    - .cursor/rules/enjoyflow.mdc（--ai cursor）
    - .claude/skills/enjoyflow.md（--ai claude）
    - ...

[4] 写 .gitignore
    追加: .enjoyflow/snapshots/
    追加: .enjoyflow/.doctor-cache
```

`--scan` 变体：先检测项目中的 package.json / pom.xml / requirements.txt 等，提取技术栈信息，填入 ARCHITECTURE.md 和 CODE-STANDARDS.md 的初始模板。

`--describe` 变体：将自然语言描述发给 LLM，生成初始 proposal 供用户确认。

### 3.2 search

```bash
enjoyflow search <query>           # 自由文本搜索（正文 + 标题）
enjoyflow search <query> --class gotchas  # 限定知识类别
enjoyflow search <query> --tag excel      # 限定 tag
enjoyflow search <query> --class gotchas --tag excel  # AND
enjoyflow search <query> --archive        # 含归档
```

**内部流程**：

```
输入: "导出", class=gotchas, tag=excel
        │
        ▼
[1] 索引查找（如有 .index.json）
    命中 class=gotchas → 文件列表: [GOTCHAS.md, archive/GOTCHAS-2025Q4.md]
    命中 tag=excel    → 进一步过滤
    无索引 → 遍历源
        │
        ▼
[2] 内容匹配（按源适配器）
    filesystem: grep 标题 + YAML frontmatter + 正文首段
    git: git grep
    mcp: 调 MCP resources/tools
        │
        ▼
[3] 组合条件（AND 逻辑）
    保留同时满足所有 --class / --tag / query 的结果
        │
        ▼
[4] 提取摘要
    每条命中返回: 文件路径 + 标题锚点 + 匹配行周围 3 行
        │
        ▼
输出:
.enjoyflow/knowledge-base/development/GOTCHAS.md##导出
  t_export_record 无 status 字段
  → 用 create_time + status=1 判断是否完成
.enjoyflow/knowledge-base/business/water-billing.md##导出规则
  单次上限 10 万行, 超限分批
```

**匹配规则**：

| 条件 | 匹配方式 | 范围 |
|---|---|---|
| `query`（自由文本） | grep（不区分大小写） | 标题 + 正文的前 200 字符 + frontmatter description |
| `--class` | 精确匹配 frontmatter `class` 字段 | YAML frontmatter |
| `--tag` | 精确匹配 frontmatter `tags` 数组 | YAML frontmatter |
| 多个条件 | AND | — |

**返回粒度**：段级——`文件路径##标题锚点` + 匹配行周围的一两句上下文。不是路径级（太粗），不是全段落（太碎），而是"给 AI 足够判断是否值得读完整段的线索"。

AI 拿到结果后自己决定读哪些文件的完整内容，EnjoyFlow 不替它决定。

### 3.3 record

```bash
enjoyflow record gotcha --tag excel --content "t_export_record 表无 status 字段"
enjoyflow record pattern --tag api --content "异步导出用 @Async + 进度回调"
enjoyflow record decision --task REQ-042 --content "临时用同步导出，等 REQ-005 改异步"
```

**内部流程**：

```
输入: gotcha, tag=excel, content="...", task=REQ-042
        │
        ▼
[1] 路由到目标文件
    gotcha   → .enjoyflow/knowledge-base/development/GOTCHAS.md
    pattern  → .enjoyflow/knowledge-base/development/PATTERNS.md
    decision → .enjoyflow/lifecycle/active-sprint/REQ-042.md
        │
        ▼
[2] 格式化条目
    - excel: t_export_record 表无 status 字段（REQ-042, 2026-06-21）
        │
        ▼
[3] 追加到文件末尾
    - 不改动已有内容
    - 更新 frontmatter 的 last_modified 日期
        │
        ▼
[4] 增量更新索引
    .index.json: 追加 "excel" → GOTCHAS.md 映射（如新增）
```

**约束**：
- 只追加，不修改已有条目
- 不审批，零摩擦写入——噪音成本低（3 行），漏记成本高（重复踩坑）
- 修改结构知识（ARCHITECTURE.md、CODE-STANDARDS.md 等）必须走正常 PR 流程

### 3.4 doctor / fix

```bash
enjoyflow doctor       # 诊断
enjoyflow fix          # 自动修复
enjoyflow fix --full   # 全量修复（默认增量）
```

**五个检查项**：

| # | 检查 | 修复动作 |
|---|---|---|
| 1 | 缺 `class` 字段 | AI 读内容，建议 class |
| 2 | 缺 `tags` 字段 | AI 读内容，建议 tags |
| 3 | 文件超出预算 | 归档最老的一半条目 |
| 4 | 疑似重复条目 | LLM 语义比对，合并 |
| 5 | 索引与文件不一致 | 重建 .index.json |

**增量策略**：

```
.enjoyflow/.doctor-cache 记录上次 doctor 时间戳
运行时：
  - 默认：只扫描 last_modified > 上次时间戳 的文件
  - --full：全量扫描
```

**去重算法**：

```
[1] L1 过滤: 只比较 tag 有交集的条目对（tag 不同 → 肯定不重复）
[2] L2 语义: 把 L1 筛选出的候选对，一次 LLM 调用批量判断
    prompt: "以下条目中，哪些是描述同一件事的重复？合并保留更详细的版本。"
[3] 合并: doctor 标记，fix 执行删除 + 保留
```

**预算与归档**：

| 文件 | 上限 | 超标时 |
|---|---|---|
| GOTCHAS.md | 100 行 | 最老的一半 → `archive/GOTCHAS-YYYYQQ.md` |
| PATTERNS.md | 100 行 | 同上 |
| ARCHITECTURE.md | 150 行 | 旧内容移到 decisions 归档 |
| .enjoyflow/snapshots/*.md | 50 行 | 超过重新生成摘要 |

归档时自动生成摘要索引：

```
archive/INDEX.md
- [2026Q2] excel: 内存溢出排查步骤（GOTCHAS-2026Q2.md#L12）
- [2025Q4] export: 大数据量超时（GOTCHAS-2025Q4.md#L3）
```

---

## 4. 知识源抽象

### 4.1 统一接口

所有源实现相同的 trait：

```rust
trait KnowledgeSource {
    fn search(&self, query: &SearchQuery) -> Vec<SearchResult>;
    fn read_file(&self, path: &str) -> String;
    fn append_to_file(&self, path: &str, content: &str);
    fn list_files(&self) -> Vec<FilePath>;
}
```

`search` 命令不关心源的类型——遍历所有源，并行查询，合并结果。

### 4.2 源类型

| 类型 | MVP | 实现方式 |
|---|---|---|
| **filesystem** | ✅ | 直接读写本地文件 |
| **git** | 后期 | `git clone --depth 1` + filesystem 逻辑 |
| **mcp** | 后期 | MCP 客户端 → MCP server 的 resources/tools |

### 4.3 配置

```yaml
# .enjoyflow/config.yaml
sources:
  - type: filesystem
    path: .enjoyflow/knowledge-base/

  - type: filesystem
    path: ~/Obsidian/vault/projects/water-iot

  - type: git
    url: git@github.com:team/backend-docs.git
    branch: main

  - type: mcp
    server: mem0-knowledge
```

```bash
enjoyflow source add <path|url>
enjoyflow source remove <name>
enjoyflow source list
```

### 4.4 Obsidian 兼容性

Obsidian vault 天然是 markdown + YAML frontmatter——与 EnjoyFlow 格式一致。class 字段可选——没有 class 的 Obsidian 笔记仍可通过 tag 和关键词检索，只是无法按 class 过滤。

```yaml
# Obsidian 笔记中的典型 frontmatter
---
tags: [export, excel, performance]
date: 2026-06-15
---
# Excel 导出踩坑
...
```

`search --class gotchas` 会跳过没有 `class` 字段的笔记（不报错），但 `search --tag export` 仍能命中。

---

## 5. 惰性索引

### 5.1 索引结构

```json
// .enjoyflow/.index.json
{
  "by_class": {
    "C1_gotchas": [
      ".enjoyflow/knowledge-base/development/GOTCHAS.md",
      ".enjoyflow/knowledge-base/development/archive/GOTCHAS-2025Q4.md"
    ],
    "B2_business_rules": [
      ".enjoyflow/knowledge-base/business/water-billing.md"
    ]
  },
  "by_tag": {
    "export": [
      ".enjoyflow/knowledge-base/development/GOTCHAS.md",
      ".enjoyflow/knowledge-base/business/water-billing.md"
    ],
    "excel": [
      ".enjoyflow/knowledge-base/development/GOTCHAS.md"
    ]
  },
  "last_rebuilt": "2026-06-21T14:30:00"
}
```

### 5.2 索引生命周期

```
init    → 全量生成 .index.json
record  → 增量更新（追加 class→file, tag→file 映射）
doctor  → fix 后重建（保证一致）
search  → 首先查索引，无索引时遍历文件
```

索引缺失时 search 退化为直接遍历文件——索引是优化，不是依赖。

---

## 6. 场景模板（降格）

8 个场景模板不进入 CLI 逻辑。它们是 AGENTS.md 里的自然语言指南：

```markdown
# AGENTS.md

## 场景建议

| 场景 | 建议先查 |
|---|---|
| 修 Bug | `--class gotchas --class architecture --class known_issues` |
| 新功能 | `--class architecture --class code_standards --class patterns` |
| 重构 | `--class architecture --class gotchas --class decisions` |
| Hotfix | `--class architecture --class gotchas --class known_issues` |
| 架构决策 | `--class architecture --class decisions` |
| 发版部署 | `--class environments --class deployment` |
| 代码审查 | `--class code_standards --class review_checklist` |
| 监控响应 | `--class architecture --class known_issues --class failure_modes` |

这不是强制预设——AI 根据实际情况自己判断需要搜什么。
```

系统不维护场景模板的独立存储结构。未来如果需要，可以在 `.enjoyflow/scenarios/` 下放 YAML 文件作为社区扩展入口，但 MVP 不做。

---

## 7. 文档格式规范

所有知识文档统一格式：

```markdown
---
class: C1_gotchas
tags:
  - export
  - excel
  - backend
last_modified: 2026-06-20
---

# 导出功能踩坑清单

## 数据库

### t_export_record 无 status 字段
- **影响**: 无法直接判断导出状态
- **当前方案**: 用 create_time + 其他字段组合判断
...
```

**约束**：
- `class` 必填（EnjoyFlow 创建的文件）。外部源（Obsidian 等）可为空
- `tags` 必填，≥1 个
- 正文用 `##` 标题划分段落——search 以 `##` 为段界
- `last_modified`：record / fix 自动更新

class 代号（如 `C1_gotchas`）只在前端 matter 和索引中使用，用户和 AI 通过语义标签（`--class gotchas`）交互——CLI 内部做映射。

---

## 8. 四通道的系统落地

四通道出自设计哲学，在系统层面不直接映射到模块：

| 通道 | 系统角色 |
|---|---|
| **Define** | 不在系统中。合约由团队流程维护。search 让 AI 能找到当前合约 |
| **Build** | 不在系统中。AI 工具执行。search 提供上下文 |
| **Verify** | `enjoyflow search` + 约定：AI 读验证相关维度（test_reports, review_checklist）。不负责任务调度 |
| **Learn** | `enjoyflow record`——AI 发现什么就写回知识库 |

---

## 9. 配置总览

```yaml
# .enjoyflow/config.yaml
project:
  name: "IoT 水务平台"
  tech_stack: ["vue3", "java", "spring-boot"]

ai_tool: cursor

sources:
  - type: filesystem
    path: .enjoyflow/knowledge-base/

class_labels:           # 将 class ID 映射为语义标签
  A1: architecture
  A2: code_standards
  A3: api_contract
  A4: data_model
  C1: gotchas
  C2: patterns
  C4: test_strategy
  C5: known_issues
  C6: team_convention
  C7: review_checklist
  C8: deployment
  D1: task_progress
  D7: failure_modes
  # ... 完整 32 类映射，search --class 参数用语义标签匹配
```

---

## 10. 数据流速览

### 10.1 search 全链路

```
AI 运行: enjoyflow search "导出" --class gotchas --tag excel
                │
    ┌───────────┼───────────┐
    ▼           ▼           ▼
 filesystem    git         mcp
  adapter    adapter     adapter
    │           │           │
    ▼           ▼           ▼
 .enjoyflow/  远程仓库    MCP server
 knowledge-
 base/
    │           │           │
    └───────────┼───────────┘
                ▼
           合并结果
                │
                ▼
        段级摘要输出
```

### 10.2 record 全链路

```
AI 运行: enjoyflow record gotcha --tag excel --content "..."
                │
                ▼
         路由 → GOTCHAS.md
                │
                ▼
         追加条目（不改已有）
                │
                ▼
         增量更新 .index.json
                │
                ▼
         ✓ 已记录
```

---

## 11. MVP 范围

| 模块 | MVP | 备注 |
|---|---|---|
| init | ✅ | 含 --scan, --describe, --link, --ai |
| search | ✅ | 含 --class, --tag, query, --archive |
| record | ✅ | gotcha / pattern / decision |
| doctor / fix | ✅ | 含去重、预算、归档 |
| 源: filesystem | ✅ | |
| 源: git | 后期 | |
| 源: mcp | 后期 | |
| 索引 | ✅ | .index.json，惰性生成 |
| 场景模板 | ✅ | 仅 AGENTS.md 文本，不进 CLI |
| search 语义排序 | 不做 | 按文件路径排序，AI 自己判断相关性 |
| 全文嵌入检索 | 不做 | grep + 索引够用 |

---

*文档版本: 1.0-draft | 最后更新: 2026-06-21*
