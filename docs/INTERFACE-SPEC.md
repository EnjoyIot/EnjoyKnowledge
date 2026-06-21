# EnjoyFlow 接口规范

> 版本 1.0 | 2026-06-21
>
> EnjoyFlow 兼容实现的最小接口合约。第三方适配器、工具生成器、AI 工具集成的唯一参考。
> 不解释为什么，只定义是什么。

---

## 1. 兼容性级别

| 级别 | 要求 | 标识 |
|---|---|---|
| **L1 完整兼容** | 实现本文档全部合约 | `enjoyflow/v1` |
| **L2 格式兼容** | 读/写 EnjoyFlow 格式的文件，不实现 CLI | `enjoyflow-format/v1` |
| **L3 源兼容** | 实现 Source Adapter 接口，接入 search | `enjoyflow-source/v1` |

---

## 2. 配置文件

### 2.1 config.yaml

```yaml
# .enjoyflow/config.yaml — 完整 schema
# 所有字段可为空时使用默认值

sources:                        # 必填，至少 1 个
  - type: filesystem            # "filesystem" | "git" | "mcp"
    path: .enjoyflow/knowledge-base/  # filesystem 必填
    # git 专用:
    # url: <git-url>
    # branch: main
    # mcp 专用:
    # server: <mcp-server-name>

class_labels:                   # 可选，未列出时用内置默认映射
  A1: architecture
  A2: code_standards
  # ... 见 §7 完整注册表

ai_tool: auto                   # auto | cursor | claude | copilot | windsurf | cline | codex | gemini
```

### 2.2 默认值

| 字段 | 默认值 |
|---|---|
| `sources` | `[{type: filesystem, path: .enjoyflow/knowledge-base/}]` |
| `class_labels` | §7 内置注册表 |
| `ai_tool` | `auto` |

---

## 3. 知识文档格式

所有 EnjoyFlow 知识文件的必须格式：

### 3.1 Frontmatter

```yaml
---
class: C1_gotchas              # 必填，来自 §7 注册表
tags:                          # 必填，≥1 个
  - export
  - excel
last_modified: 2026-06-20      # 必填，ISO 8601 日期
description: "导出 Excel 功能的踩坑记录"  # 可选，search 匹配用
---
```

| 字段 | 类型 | 必填 | 约束 |
|---|---|---|---|
| `class` | string | 是 | §7 注册表中的 ID |
| `tags` | string[] | 是 | ≥1 个，纯小写字母+连字符 |
| `last_modified` | date | 是 | ISO 8601 `YYYY-MM-DD` |
| `description` | string | 否 | ≤200 字符，search 匹配 |

### 3.2 正文

```markdown
# 标题（唯一，一级标题）

## 段标题（二级标题，search 段界）
正文内容...

### 子段标题（三级标题，可嵌套）
...
```

| 规则 | 要求 |
|---|---|
| 一级标题 `#` | 文件唯一，作为文件名等价物 |
| 二级标题 `##` | search 的段界——每条结果定位到 `##` |
| 三级标题 `###` | 可选，辅助组织 |
| tags 出现位置 | 只在 frontmatter，不在正文 |

---

## 4. search 输出格式

### 4.1 命令行

```bash
enjoyflow search <query>           # 自由文本
enjoyflow search <query> --class gotchas   # 按 class 过滤
enjoyflow search <query> --tag excel       # 按 tag 过滤
enjoyflow search <query> --archive         # 含归档
```

多个 `--class` / `--tag` 之间为 AND 逻辑。

### 4.2 输出格式

每行一条命中记录：

```
<文件路径>##<段标题>
  <匹配行周围上下文>
```

```
# 示例输出
.enjoyflow/knowledge-base/development/GOTCHAS.md##导出
  t_export_record 无 status 字段，用 create_time 判断
.enjoyflow/knowledge-base/business/water-billing.md##导出规则
  单次最多 10 万行，超限分批，格式 xlsx/csv
```

### 4.3 实现要求

| 要求 | 说明 |
|---|---|
| 匹配范围 | frontmatter（class, tags, description）+ 标题 + 正文 |
| 大小写 | 不区分 |
| 排序 | 按文件路径字母序 |
| 归档搜索 | 默认跳过 `archive/` 目录，`--archive` 包含 |
| 多源搜索 | 并行查询所有源，合并输出 |

---

## 5. record 写入合约

### 5.1 命令行

```bash
enjoyflow record gotcha --tag <tag> --content "<文本>"
enjoyflow record pattern --tag <tag> --content "<文本>"
enjoyflow record decision --task <REQ-ID> --content "<文本>"
```

### 5.2 目标文件路由

| record 类型 | 写入文件 | 写入方式 |
|---|---|---|
| `gotcha` | `.enjoyflow/knowledge-base/development/GOTCHAS.md` | 追加条目到末尾 |
| `pattern` | `.enjoyflow/knowledge-base/development/PATTERNS.md` | 追加条目到末尾 |
| `decision` | `.enjoyflow/lifecycle/active-sprint/<REQ-ID>.md` | 追加到 ## Decisions 段 |

### 5.3 追加格式

```markdown
- <tag>: <content>（<REQ-ID>, <YYYY-MM-DD>）
```

**约束**：只追加，不修改已有条目。追加后更新 `last_modified`。

---

## 6. 源适配器接口

### 6.1 语言无关接口

```
KnowledgeSource {
    search(query: SearchQuery) → [SearchResult]
    read(path: string) → string
    append(path: string, content: string) → void
    listFiles() → [string]
}

SearchQuery {
    text: string              // 自由文本
    class: string | null      // 可选 class 过滤
    tags: string[] | null     // 可选 tag 过滤（AND）
    includeArchive: bool      // 默认 false
}

SearchResult {
    file: string              // 文件路径
    section: string           // ## 标题
    snippet: string           // 匹配行 ± 3 行上下文
}
```

### 6.2 源类型

| type | 接入方式 | 实现要求 |
|---|---|---|
| `filesystem` | 直接读写本地文件 | `search` 用 grep / 索引 |
| `git` | `git clone --depth 1` + filesystem 逻辑 | 只读 |
| `mcp` | MCP 客户端协议 | 实现 MCP tools/resources |

### 6.3 多源合并

实现方调用所有源的 `search`，按 `file` 字母序合并 `SearchResult`。重复文件去重（保留第一个）。

---

## 7. class 字段

class 是 frontmatter 中的必填字符串，标识知识文档的类别。格式为 `<字母><数字>_<描述>`，如 `C1_gotchas`。

`--class` 参数接受 class ID（如 `C1_gotchas`）或语义标签（如 `gotchas`）——实现方在 `config.yaml` 的 `class_labels` 中维护映射。

| class_labels 内置默认 | ID |
|---|---|
| architecture | A1 |
| code_standards | A2 |
| api_contract | A3 |
| data_model | A4 |
| api_specs | A5 |
| requirements | A6 |
| design_spec | A7 |
| adr | A8 |
| test_report | A9 |
| dependencies | A10 |
| environments | A11 |
| releases | A12 |
| glossary | B1 |
| business_rules | B2 |
| business_flow | B3 |
| constraints | B4 |
| cases | B5 |
| gotchas | C1 |
| patterns | C2 |
| decisions | C3 |
| test_strategy | C4 |
| known_issues | C5 |
| team_convention | C6 |
| review_checklist | C7 |
| deployment | C8 |
| task_progress | D1 |
| session_log | D2 |
| decision_history | D3 |
| contract_sync | D4 |
| ai_call_chain | D5 |
| contextflow | D6 |
| failure_modes | D7 |

完整定义（每个 class 的含义、典型内容、物理路径）见 [KNOWLEDGE-ARCHITECTURE.md §2](KNOWLEDGE-ARCHITECTURE.md)。

自定义 class 可在 `config.yaml` 的 `class_labels` 中追加映射，使用 A-Z 之外的字母类别（如 `X1_my_domain`）。

---

## 8. CLI 合约

兼容实现必须提供的命令及其行为：

| 命令 | 输入 | 输出 | 副作用 |
|---|---|---|---|
| `enjoyflow search <query> [--class] [--tag] [--archive]` | §4 | §4.2 格式，stdout | 无 |
| `enjoyflow record <type> --tag <t> --content "<c>"` | §5 | `✓` 或错误信息，stderr | 追加到目标文件 |
| `enjoyflow init [path] [--ai <t>] [--link <p>]` | §2 | 生成目录 + AGENTS.md | 创建文件和目录 |
| `enjoyflow doctor` | 无 | 问题清单，stdout | 无 |
| `enjoyflow fix` | 无 | 修复结果，stdout | 修改文件 |

### 8.1 错误码

| 码 | 含义 |
|---|---|
| 0 | 成功 |
| 1 | 输入参数错误 |
| 2 | 文件/路径不存在 |
| 3 | 格式校验失败 |
| 4 | 源不可达 |

### 8.2 AI 工具文件生成

`enjoyflow init --ai <tool>` 必须生成：

| --ai | 生成的文件 | 格式 |
|---|---|---|
| 任意 / 默认 | `AGENTS.md` | Markdown |
| `cursor` | `.cursor/rules/enjoyflow.mdc` | YAML frontmatter + Markdown |
| `claude` | `.claude/skills/enjoyflow.md` | Markdown |
| `copilot` | `.github/copilot-instructions.md` | 追加 Markdown 块 |
| `windsurf` | `.windsurf/rules/enjoyflow.md` | Markdown |
| `cline` | `.clinerules/enjoyflow.md` | Markdown |
| `codex` | `.codex/prompts/enjoyflow.md` | Markdown |
| `gemini` | `GEMINI.md` | 追加 Markdown 块 |

---

## 9. 版本化

`config.yaml` 中的 `apiVersion: enjoyflow/v1` 标识本文档版本。实现方应检查此字段：

- 主版本变更 → 拒绝加载，提示升级
- 次版本 → 向后兼容，警告
- 未知字段 → 忽略（forward compatibility）

---

## 10. 测试套件

兼容性验证用指定输入和预期输出验证：

```
enjoyflow-test/
├── fixtures/
│   ├── minimal-project/        # 最小项目
│   │   └── .enjoyflow/
│   │       ├── config.yaml
│   │       └── knowledge-base/...
│   └── full-project/           # 多源项目
├── cases/
│   ├── search-basic.txt        # 输入 → 预期输出
│   ├── search-class-filter.txt
│   ├── search-multi-source.txt
│   └── record-gotcha.txt
└── validate.sh                 # 跑全部用例
```

用例格式：

```
# cases/search-basic.txt
INPUT:  search "导出"
EXPECT: .enjoyflow/knowledge-base/development/GOTCHAS.md##导出
EXPECT: t_export_record 无 status 字段
```

通过全部用例即为 L1 兼容。

---

*文档版本: 1.0 | apiVersion: enjoyflow/v1*
