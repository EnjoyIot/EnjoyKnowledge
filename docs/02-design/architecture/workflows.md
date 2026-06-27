# 工作流系统

> 版本: 1.0 | 2026-06-27 | 来源: v4 §4
>
> **本文件定义 5 个核心工作流 + YAML schema + 主动/被动分层**。

---

## 1. 核心思想

**工作流 = 元数据驱动，不实现引擎**。

| 旧范式（引擎驱动） | 新范式（元数据驱动）|
|---|---|
| 工作流 = 编译到命令的 Rust 代码 | 工作流 = YAML 文件 |
| 加新工作流 = 改代码 + 重新编译 | 加新工作流 = 加 YAML 文件 |
| 状态/上下文/中间产物都存在代码里 | 状态全部 stateless，每次重读 SoT |

**关键好处**：用户加工作流不需要懂 Rust 也不需要重新编译——**vibe-coding friendly**。

## 2. 5 个核心工作流

```
onboard (W1) ──── 唯一前置依赖
    ↓
    ├──→ prd-preprocess (W2)  ← 每次新需求
    │       ↓
    │       └──→ preflight (W3) ← 每次变更前
    │               ↓
    │               └──→ capture (W4) ← 每次有价值的产出
    │
    └──→ sync (W5) ← AI 工具切换时（低频）
```

| # | 工作流 | 触发 | 主动/被动 |
|---|---|---|---|
| W1 | **onboard** | AI 工具启动 | 被动（入口文件触发）|
| W2 | **prd-preprocess** | 用户输入需求 | 主动 |
| W3 | **preflight** | git commit/PR 前 | 主动 |
| W4 | **capture** | 用户/AI 主动 | 主动 |
| W5 | **sync** | AI 工具切换 | 主动 |

**原则**：**"读"走被动（让 AI 工具自己发现）；"写"走主动（必须有意图）**

## 3. YAML Schema

### 3.1 工作流文件

```yaml
# 文件名：.enjoyknowledge/workflows/<name>.yaml
name: <string>                    # kebab-case
description: <string>             # 一句话用途
trigger: <list>                   # 触发条件
input: <object?>                  # 输入定义
steps: <list>                     # 步骤序列
output: <object>                  # 输出定义
```

### 3.2 步骤标准字段

```yaml
- id: <string>                    # 步骤 ID（唯一）
  action: <enum>                  # cat / grep / ls / doctor / ai_decide / ask_human / record
  target: <path glob>             # 目标路径
  filter: <object?>               # 过滤条件
  required: <bool>                 # 失败是否阻塞（默认 true）
  timeout_seconds: <int?>
```

### 3.3 action 枚举

| action | 含义 | 例子 |
|---|---|---|
| `cat` | 读全文 | `target: .enjoyknowledge/knowledge/architecture/overview.md` |
| `grep` | 全文搜索 | `filter: { frontmatter_field: value }` |
| `ls` | 列目录 | `target: .enjoyknowledge/knowledge/gotchas/` |
| `doctor` | 跑 doctor 检查 | `target: .enjoyknowledge/` |
| `ai_decide` | 让 AI 工具判断 | （无 target）|
| `ask_human` | 中断流程，问用户 | （无 target）|
| `record` | 把结果写回 SoT | `target: .enjoyknowledge/knowledge/gotchas/<id>.md` |

## 4. 5 个工作流定义（YAML 示例）

### 4.1 onboard.yaml

```yaml
name: onboard
description: AI 工具首次进入仓库时建立项目心智模型
trigger:
  - on_ai_session_start
  - manual: /ek onboard

steps:
  - id: read_index
    action: cat
    target: .enjoyknowledge/index.md
    required: true

  - id: read_architecture
    action: cat
    target: .enjoyknowledge/knowledge/architecture/overview.md
    required: true

  - id: read_gotchas_critical
    action: grep
    target: .enjoyknowledge/knowledge/gotchas/
    filter: { frontmatter_severity: 4-5 }
    required: true

  - id: read_active_decisions
    action: grep
    target: .enjoyknowledge/knowledge/decisions/
    filter: { frontmatter_reversible: true }
    required: false

output:
  type: ai_context
  max_words: 4000
```

### 4.2 preflight.yaml

```yaml
name: preflight
description: PR 提交前/AI 大改前检查冲突
trigger:
  - on_pre_commit
  - on_pr_open
  - manual: /ek preflight

input:
  type: git_diff
  source: "git diff --name-only HEAD~1"

steps:
  - id: find_related_gotchas
    action: grep
    target: .enjoyknowledge/knowledge/gotchas/
    filter: { trigger_file_match: "{{input.files}}" }
    required: true

  - id: find_related_decisions
    action: grep
    target: .enjoyknowledge/knowledge/decisions/
    filter: { applies_to: "{{input.files}}" }
    required: true

  - id: find_relevant_rules
    action: grep
    target: .enjoyknowledge/rules/
    filter: { applies_to: "{{input.files}}" }
    required: true

  - id: rcode_sync_check
    action: doctor
    target: .enjoyknowledge/
    check: rule_code_sync
    required: false

output:
  type: report
  format: markdown
  sections:
    - block: ["violates rule", "contradicts decision"]
    - warn: ["matches gotcha"]
    - info: ["related architecture"]
```

### 4.3 capture.yaml

```yaml
name: capture
description: 把对话中发现的隐性知识沉淀到 SoT
trigger:
  - manual: /remember "description"
  - on_ai_suggestion

input:
  type: free_text
  source: "{{user_input}}"

steps:
  - id: classify
    action: ai_decide
    classify_to: [gotcha, decision, pattern, rule]
    required: true

  - id: check_trigger_field
    action: doctor
    target: .enjoyknowledge/knowledge/gotchas/
    check: trigger_field_required
    only_if: { classified_as: gotcha }
    required: true

  - id: write_so
    action: record
    target: ".enjoyknowledge/knowledge/{{classified_to}}/{{generated_id}}.md"
    template: default
    required: true

  - id: update_index
    action: record
    target: .enjoyknowledge/index.md
    append: true
    required: true

output:
  type: confirmation
  message: "已记录到 {{classified_to}}/{{generated_id}}"
```

### 4.4 sync.yaml

```yaml
name: sync
description: 把 SoT 同步到 9 个 AI 工具的入口文件
trigger:
  - manual: /ek sync
  - on_post_record

steps:
  - id: list_target_tools
    action: grep
    target: .enjoyknowledge/.config/tools.yaml
    required: true

  - id: render_each_tool
    action: cat
    target: .enjoyknowledge/index.md
    per_tool: "{{target_tools}}"
    template: per_tool_template
    required: true

  - id: validate_idempotency
    action: doctor
    target: .enjoyknowledge/
    check: sync_idempotent
    required: true

output:
  type: log
  log_to: .enjoyknowledge/.log/sync.log
```

### 4.5 prd-preprocess.yaml

```yaml
name: prd-preprocess
description: 把用户需求转化为结构化任务清单
trigger:
  - manual: /ek prd "requirement"

input:
  type: free_text
  source: "{{user_input}}"

steps:
  - id: find_related_business
    action: grep
    target: .enjoyknowledge/knowledge/business/
    filter: { content_match: "{{input}}" }
    required: true

  - id: find_related_architecture
    action: grep
    target: .enjoyknowledge/knowledge/architecture/
    filter: { content_match: "{{input}}" }
    required: true

  - id: find_related_gotchas
    action: grep
    target: .enjoyknowledge/knowledge/gotchas/
    filter: { trigger_file_match: "{{inferred_modules}}" }
    required: false

  - id: generate_task_list
    action: ai_decide
    output_format: markdown_table
    columns: [task, modules, known_risks, related_decisions]
    required: true

output:
  type: markdown
  max_words: 2000
```

## 5. 失败处理

| 失败模式 | 行为 |
|---|---|
| `required: true` 步骤失败 | 工作流整体 fail，输出错误 |
| `required: false` 步骤失败 | 工作流继续，warning 记录 |
| AI 工具不支持某 action | fall back 到 `ai_decide`（用 AI 工具自身能力） |
| 步骤超时 | fail，timeout 信息记录 |
| render 失败 | fail，**不写文件**（不部分写入） |

## 6. 与 B 站的差异

| 维度 | B 站 bili-fe-workflow | enjoyknowledge v4 |
|---|---|---|
| 工作流数量 | 3 个硬编码 | 5 个 + 用户可加 |
| 工作流定义 | 编译到命令 | YAML 元数据 |
| 加新工作流 | 改 Rust 代码 | 加 YAML 文件 |
| 工具无关性 | 内部命令 | 9 工具无关 |
| 状态管理 | 部分 stateful | 完全 stateless |

---

**关联文档**：
- [for-coding-design.md §7 5 个工作流依赖图](./for-coding-design.md)
- [rule-system.md §5 9 工具 sync](./rule-system.md)
- [knowledge-types.md §4 必填字段](./knowledge-types.md)
