# 工作流系统

> v0.4.2 | 2026-06-28
>
> 定义 v0.2 的 2 个核心工作流 + YAML schema。

---

## 1. 核心思想

**工作流 = YAML 元数据驱动**。加新工作流 = 加 YAML 文件，不需要懂 Rust。

| 旧范式 | 新范式 |
|---|---|
| 工作流 = 编译到命令的 Rust 代码 | 工作流 = YAML 文件 |
| 状态/上下文在代码里 | 状态完全 stateless，每次重读 SoT |

---

## 2. 2 个核心工作流

| # | 工作流 | 触发 | 主动/被动 |
|---|---|---|---|
| W1 | **onboard** | AI 工具启动 | 被动（入口文件触发） |
| W2 | **capture** | 用户/AI 主动 | 主动 |

**原则**："读"走被动（让 AI 自己发现），"写"走主动（必须有意图）。

---

## 3. YAML Schema

### 3.1 工作流文件

```yaml
# .enjoyknowledge_stage/workflow/<name>.yaml
name: <string>              # kebab-case
description: <string>       # 一句话用途
trigger: <list>             # 触发条件
input: <object?>            # 输入定义
steps: <list>               # 步骤序列
output: <object>            # 输出定义
```

### 3.2 步骤字段

```yaml
- id: <string>              # 步骤 ID（唯一）
  action: <enum>            # cat / grep / ls / doctor / ai_decide / ask_human / record
  target: <path glob>       # 目标路径
  filter: <object?>         # 过滤条件
  required: <bool>          # 失败是否阻塞（默认 true）
  timeout_seconds: <int?>
```

### 3.3 action 枚举

| action | 含义 |
|---|---|
| `cat` | 读全文 |
| `grep` | 全文搜索 |
| `ls` | 列目录 |
| `doctor` | 跑 doctor 检查 |
| `ai_decide` | 让 AI 判断 |
| `ask_human` | 中断流程，问用户 |
| `record` | 把结果写回 SoT |

---

## 4. onboard.yaml

```yaml
name: onboard
description: AI 工具首次进入仓库时建立项目心智模型
trigger:
  - on_ai_session_start
  - manual: enjoyknowledge workflow onboard

steps:
  - id: read_index
    action: cat
    target: .enjoyknowledge/index.md
    required: true

  - id: read_architecture
    action: cat
    target: .enjoyknowledge/architecture/architecture.md
    required: true

  - id: read_gotchas
    action: grep
    target: .enjoyknowledge/gotchas/
    required: true

  - id: read_active_decisions
    action: grep
    target: .enjoyknowledge/decisions/
    filter: { frontmatter_reversible: true }
    required: false

output:
  type: ai_context
  max_words: 4000
```

---

## 5. capture.yaml

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
    classify_to: [gotcha, decision, pattern, rule, business, architecture, contract, convention, context, template]
    required: true

  - id: check_required
    action: doctor
    target: .enjoyknowledge/
    check: required_fields
    required: true

  - id: write_kb
    action: record
    target: ".enjoyknowledge/{{classified_to}}/{{generated_id}}.md"
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

---

## 6. 失败处理

| 失败模式 | 行为 |
|---|---|
| `required: true` 步骤失败 | 工作流整体 fail |
| `required: false` 步骤失败 | 工作流继续，warning 记录 |
| AI 工具不支持某 action | fall back 到 `ai_decide` |
| 步骤超时 | fail + timeout 信息记录 |
| render 失败 | fail，**不写文件** |

---

*关联文档：[rule-system.md](./rule-system.md) · [knowledge-types.md](./knowledge-types.md) · [for-coding-design.md](./for-coding-design.md)*
