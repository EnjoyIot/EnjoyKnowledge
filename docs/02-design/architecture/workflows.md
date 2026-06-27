# 工作流系统

> 版本: 1.0 | 2026-06-27 | 来源: v4 §4
>
> **本文件定义 v0.2 2 个核心工作流 + YAML schema + 主动/被动分层**。

---

## 1. 核心思想

**工作流 = 元数据驱动，不实现引擎**。

| 旧范式（引擎驱动） | 新范式（元数据驱动）|
|---|---|
| 工作流 = 编译到命令的 Rust 代码 | 工作流 = YAML 文件 |
| 加新工作流 = 改代码 + 重新编译 | 加新工作流 = 加 YAML 文件 |
| 状态/上下文/中间产物都存在代码里 | 状态全部 stateless，每次重读 SoT |

**关键好处**：用户加工作流不需要懂 Rust 也不需要重新编译——**vibe-coding friendly**。

## 2. v0.2 2 个核心工作流

```
onboard (W1) ──── 唯一前置依赖
    ↓
    └──→ capture (W2) ← 每次有价值的产出
```

| # | 工作流 | 触发 | 主动/被动 |
|---|---|---|---|
| W1 | **onboard** | AI 工具启动 | 被动（入口文件触发）|
| W2 | **capture** | 用户/AI 主动 | 主动 |

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

## 4. v0.2 2 个工作流定义（YAML 示例）

### 4.1 onboard.yaml

```yaml
name: onboard
description: AI 工具首次进入仓库时建立项目心智模型
trigger:
  - on_ai_session_start
  - manual: enjoyknowledge onboard

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

### 4.2 capture.yaml

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

### 4.3 export.yaml

> **v0.2 重命名** = sync → export（1 工具时 sync 撒谎，export 诚实）

```yaml
name: export
description: 把 SoT 导出到 v0.2 已启用的 AI 工具（Claude + Cursor）
trigger:
  - manual: enjoyknowledge export
  - on_post_record

steps:
  - id: list_enabled_tools
    action: grep
    target: .enjoyknowledge/.config/tools.yaml
    required: true

  - id: render_each_tool
    action: cat
    target: .enjoyknowledge/index.md
    per_tool: "{{enabled_tools}}"
    template: per_tool_template
    required: true

  - id: validate_idempotency
    action: doctor
    target: .enjoyknowledge/
    check: export_idempotent
    required: true

output:
  type: log
  log_to: .enjoyknowledge/.log/export.log
```

## 5. 失败处理

| 失败模式 | 行为 |
|---|---|
| `required: true` 步骤失败 | 工作流整体 fail，输出错误 |
| `required: false` 步骤失败 | 工作流继续，warning 记录 |
| AI 工具不支持某 action | fall back 到 `ai_decide`（用 AI 工具自身能力） |
| 步骤超时 | fail，timeout 信息记录 |
| render 失败 | fail，**不写文件**（不部分写入） |

---

## 7. filter 语法定义

> **v0.2 收缩后**——v0.2 仅 2 个工作流 YAML（onboard + capture）都用了 `filter`。本节给出 v1 规范。

### 7.1 通用形式

```yaml
filter: <object>      # 一个对象 = AND 所有 key
filter: <list>        # 一个列表 = OR 所有项
```

### 7.2 5 种 filter key

#### ① `frontmatter_<field>` — 按 frontmatter 字段

```yaml
filter: { frontmatter_severity: 4-5 }                    # 范围字符串（4 ≤ x ≤ 5）
filter: { frontmatter_severity: [4, 5] }                 # 枚举列表
filter: { frontmatter_reversible: true }                 # 布尔
filter: { frontmatter_kind: "gotcha" }                   # 字符串精确匹配
filter: { frontmatter_tags: ["utf8", "encoding"] }      # 列表包含任一
```

- 字段名格式：`frontmatter_<yaml-field-name>`（不写 `frontmatter:` 前缀的反面是 `applies_to` / `trigger`）
- 值类型：string / number / bool / list
- 范围字符串 `<a>-<b>`：仅用于**数字字段**，`a ≤ x ≤ b` 闭区间

#### ② `trigger_file_match` — gotcha 触发文件

```yaml
filter: { trigger_file_match: "src/cli/args.rs" }        # 精确路径
filter: { trigger_file_match: "src/cli/**" }             # glob
filter: { trigger_file_match: ["src/cli/**", "src/init/**"] }  # 任一匹配
```

匹配 `gotcha.frontmatter.trigger`（glob 模式）。**gotcha 专属**。

#### ③ `applies_to` — rule / decision 适用范围

```yaml
filter: { applies_to: "*.rs" }                          # glob
filter: { applies_to: ["*.rs", "*.toml"] }               # 任一匹配
```

匹配 `rule.frontmatter.applies_to` 或 `decision.frontmatter.applies_to`。**rule / decision 专属**。

#### ④ `content_match` — 内容搜索

```yaml
filter: { content_match: "UTF-8 编码" }                  # substring
filter: { content_match: "/utf-?8/i" }                  # 正则（用 /.../ 包裹 + 可选 flag）
```

- 字符串 = substring 匹配
- `/.../` 包裹 = 正则；末尾可加 `i`（大小写不敏感）

#### ⑤ `frontmatter_reversible` — decision 可逆性

```yaml
filter: { frontmatter_reversible: true }                 # 仅可逆
filter: { frontmatter_reversible: false }                # 仅不可逆
```

`①` 的语法糖。为 decision 工作流便利而设。

### 7.3 AND vs OR 语义

```yaml
# 单个对象 = AND 所有 key
filter: { frontmatter_kind: "gotcha", frontmatter_severity: 4-5 }
# = kind=gotcha AND severity in [4,5]

# 列表 = OR 所有元素
filter:
  - { frontmatter_kind: "gotcha" }
  - { frontmatter_kind: "decision" }
# = kind=gotcha OR kind=decision

# 嵌套：外层 AND，内层 OR（少见但允许）
filter:
  frontmatter_kind: "gotcha"
  frontmatter_severity: 4-5
```

### 7.4 模板变量

filter 值支持 `{{...}}` 模板变量（来自 `input` 字段）：

```yaml
filter: { trigger_file_match: "{{input.files}}" }       # 多文件
filter: { content_match: "{{input}}" }                   # 自由文本
filter: { applies_to: "{{inferred_modules}}" }          # AI 推断
```

### 7.5 校验规则

- 未知 `frontmatter_xxx` 字段 → `doctor` warning（不阻塞）
- 范围字符串用在非数字字段 → 编译/加载失败（阻塞）
- glob 语法错误 → 编译/加载失败
- 模板变量在 `input` 中找不到 → 加载时 warning + 运行时 fail

## 6. 与 B 站的差异

| 维度 | B 站 bili-fe-workflow | enjoyknowledge v4 |
|---|---|---|
| 工作流数量 | 3 个硬编码 | 2 个 + 用户可加（v0.2 收缩）|
| 工作流定义 | 编译到命令 | YAML 元数据 |
| 加新工作流 | 改 Rust 代码 | 加 YAML 文件 |
| 工具无关性 | 内部命令 | 多工具无关（v0.2 首发 2 工具，架构保留 9 工具 adapter）|
| 状态管理 | 部分 stateful | 完全 stateless |

---

**关联文档**：
- [for-coding-design.md §7 v0.2 2 个工作流依赖图](./for-coding-design.md)
- [rule-system.md §5 v0.2 2 工具 export](./rule-system.md)
- [knowledge-types.md §4 必填字段](./knowledge-types.md)
