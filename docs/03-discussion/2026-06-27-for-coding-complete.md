# v4 整合：for Coding 完整设计（基于 v1-v3 + B 站借鉴 + 项目现状修正）

**整合日期**：2026-06-27（同一会话第四轮）
**触发**：用户指出"现有项目还没开始用，完全可以改"——v1-v3 默认假设现状合理需修正
**关键发现**：现有项目用 **9 个 AI 工具的 skills/mdc/rules 目录**（不是 AGENTS.md 单一入口）
**关键借鉴**：B 站 bili-fe-workflow 的 **Harness Engineering** 哲学根 + **工作流 = 元数据驱动** 模式
**关键否定**：不照搬 B 站的"硬编码文件名 + 固定工作流"

---

## §1 设计哲学（4 条不可违反）

### 1.1 SoT 单一 + 入口多元

**核心原则**：
- **唯一真值源（SoT）** = `.enjoyknowledge/` 目录（rule / template / knowledge / workflow 元数据）
- **多入口分发** = 9 个 AI 工具各自的入口文件（skills / mdc / rules / instructions）

**为什么这样**：
- B 站 `.workflow/AGENTS.md` 单入口对单团队够用，但 enjoyknowledge 适配 9 个工具——单入口不可能
- v1-v3 整合的"managed section 划界"在单文件才有效，多工具入口需要更轻的"sync"机制
- **关键洞察**：SoT 的"唯一"是逻辑上的（用户只改一处），"多入口"是物理上的（每个 AI 工具读自己格式）

### 1.2 元数据驱动，不是引擎驱动

**核心原则**：
- 工作流 = YAML/JSON 元数据文件，描述"读哪些 SoT 文件 → 做什么 → 输出什么"
- 不在 Rust 里实现工作流引擎
- AI 工具读工作流元数据 = 跑工作流

**为什么这样**（vs B 站）：
- B 站硬编码 3 个工作流（`prd-preprocess` / `dev-workflow-plan` / `archive`），扩展要写新命令
- 元数据驱动：用户加 YAML 文件 = 加新工作流（无需重新编译）
- **关键洞察**：**元数据 = schema 也是规则的延续**——这是 Karpathy "schema 是宪法" 哲学的延伸

### 1.3 显式失败，不静默降级

**核心原则**：
- SoT 缺失 → 报错（不是用空模板）
- 工具适配器未实现 → 报错（不是 fallback 到 Generic）
- 工作流引用了不存在的文件 → 报错（不是用空内容继续）

**为什么这样**（v1-v3 没强调的）：
- B 站文章没提失败处理——但实战中这是"知识腐烂"的最大来源
- 静默降级会让用户"以为对了，实际错了"——3 个月后才发现
- **关键洞察**：**显式失败 = "我无法独立决定" 的具体化**（C6 风险意识原则在工程上的体现）

### 1.4 工具特性保留，不强制统一

**核心原则**：
- Cursor 的 `.mdc` frontmatter（`globs` + `alwaysApply`）**保留**
- Claude 的 `@path` import **保留**
- Codex 的 `AGENTS.md` 多文件层级 **保留**
- 不强制所有工具"必须支持同一 frontmatter 字段"

**为什么这样**（v1-v3 共识 "路径即 ID" 的延伸）：
- 工具特性 = 不同 AI 工具的"工作方式差异"，强制统一会降低质量
- SoT 表达**项目知识**，工具入口表达**工具偏好**——两者解耦
- **关键洞察**：**抽象的代价 = 抹平差异**——只在值得抽象的地方抽象

---

## §2 总体架构（5 层）

```
┌──────────────────────────────────────────────────────────┐
│ Layer 1: 用户/AI 工具入口                                  │
│   - 9 个 AI 工具的入口文件（skills/mdc/rules/instructions）│
│   - 每个工具按自己的格式读取                                │
│   - 入口文件 = 路由表（指向 SoT 文件，不复制内容）            │
└──────────────────────────────────────────────────────────┘
                            ↓ sync（v4.1）
┌──────────────────────────────────────────────────────────┐
│ Layer 2: SoT 元数据（for Coding Profile）                  │
│   - `.enjoyknowledge/rules/*.md`（rule 单一源）            │
│   - `.enjoyknowledge/templates/*.md`（代码模板）          │
│   - `.enjoyknowledge/knowledge/**/*.md`（描述性知识）      │
│   - `.enjoyknowledge/workflows/*.yaml`（工作流元数据）      │
└──────────────────────────────────────────────────────────┘
                            ↓ core 命令
┌──────────────────────────────────────────────────────────┐
│ Layer 3: Core 引擎（Rust 实现，跨 Profile 共享）            │
│   - OKF 格式解析 + frontmatter 校验                        │
│   - `add` / `search` / `doctor` / `sync` / `workflow`      │
│   - Profile trait 适配 for-coding / for-design / for-...  │
└──────────────────────────────────────────────────────────┘
                            ↓ 已有
┌──────────────────────────────────────────────────────────┐
│ Layer 4: 文件系统层                                         │
│   - markdown 文件 + YAML frontmatter                       │
│   - git 管理版本（人类协作 + 历史回溯）                     │
│   - 工具配置文件由各工具自己读取                            │
└──────────────────────────────────────────────────────────┘
```

**关键设计选择**：
- **Layer 1 入口文件不复制 SoT 内容**（v1-v3 错位）——只做"路由表"（"for architecture 看 `.enjoyknowledge/knowledge/architecture/`"）
- **Layer 2 SoT 是 Profile 特定的**——`for-coding` profile 有自己的目录，切换到 `for-design` profile 换一套目录
- **Layer 3 Core 不假设 SoT 是什么**——Profile trait 提供 `directories()` + `seed_files()`

---

## §3 SoT 详细设计（.enjoyknowledge/ 目录）

### 3.1 8 类知识（v3.1 修订 6+2 落地版）

```
.enjoyknowledge/
├── rules/                          # 约束性（同步到 9 工具）
│   ├── coding-style.md
│   ├── naming.md
│   ├── no-unwrap.md                # 必含 trigger 字段
│   └── archive/                    # doctor 引导归档
│
├── templates/                      # 范式性（同步到 9 工具）
│   ├── rust-builder-pattern.md
│   ├── error-handling.md
│   └── archive/
│
├── knowledge/                      # 描述性（AI 按需读，不 sync）
│   ├── architecture/               # 系统结构 + 约束（不是教程）
│   │   ├── overview.md
│   │   ├── api-layer.md            # 必含"为什么不"
│   │   └── data-flow.md
│   ├── gotchas/                    # IF-THEN 触发器（必含 trigger 字段）
│   │   ├── react-useeffect-loop.md
│   │   └── db-migration-types.md
│   ├── patterns/                   # 仅本项目特有（不存通用模式）
│   │   ├── auth-flow.md
│   │   └── error-propagation.md
│   ├── decisions/                  # ADR（必含 reversible + "为什么"）
│   │   ├── 001-monorepo-vs-polyrepo.md
│   │   └── 002-pg-vs-mysql.md
│   ├── business/                   # 业务规则（最稳定）
│   │   └── modules.md
│   ├── contracts/                  # 跨模块接口契约（v3 新增）
│   │   └── api-user-endpoint.md
│   ├── conventions/                # 命名/目录/commit 格式（v3 新增 codex 建议）
│   │   ├── file-naming.md
│   │   └── commit-format.md
│   └── context/                    # 运行时：env 变量/端口/服务依赖（v3 新增 claude 建议）
│       ├── env-vars.md
│       └── local-dev.md
│
├── workflows/                      # 工作流元数据（v4 新增 ⭐）
│   ├── onboard.yaml
│   ├── prd-preprocess.yaml
│   ├── preflight.yaml
│   ├── capture.yaml
│   └── sync.yaml
│
├── tasks/                          # 待补充知识清单（v0.2 重命名 knowledge-tasks/）
│   └── pending.md
│
└── index.md                        # 内容目录（AI 入口路由）
```

### 3.2 知识类型依赖图（v3 + v4 修订）

```
business/                  ← 顶层约束（最稳定）
    ↓ 约束
architecture/              ← 静态结构
    ↓ 产生
contracts/ + patterns/ + gotchas/    ← 具体化
    ↓ 影响
decisions/                 ← 上下文
    ↓ 派生
context/ + conventions/    ← 底层（被所有层引用）
    ↓ 调度
workflows/                 ← 元数据驱动（v4 新增）
```

### 3.3 知识 frontmatter 必填字段

| 类型 | 必含字段 | 选填字段 |
|---|---|---|
| **rule** | `id`, `applies_to` | `priority`, `tags`, `tools` |
| **template** | `id`, `applies_to` | `tags` |
| **gotcha** | `id`, **`trigger`** | `severity`, `tags` |
| **decision** | `id`, **`reversible`**, `decided_at` | `alternatives` |
| **architecture** | `id` | `last_reviewed` |
| **pattern** | `id`, `applies_to` | `tags` |
| **contract** | `id`, `applies_to` | `breaking_change_since` |
| **convention** | `id` | `enforced_by` (linter/formatter/manual) |
| **context** | `id`, `env` | `last_verified` |

**核心约束**：
- `id` 全项目唯一（路径即 ID + `id` 字段一致）
- `applies_to` 支持 `glob` 模式（"`**/*.rs`"）和语言标记（"`lang:rust`"）
- `trigger` 是 gotcha 的灵魂字段——缺它 doctor 直接报错

---

## §4 工作流设计（v4 元数据驱动版）

### 4.1 5 个核心工作流（YAML 元数据）

**示例：onboard.yaml**

```yaml
# .enjoyknowledge/workflows/onboard.yaml
name: onboard
description: AI 工具首次进入仓库时建立项目心智模型
trigger:  # 触发条件
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
    filter: "severity:critical"
    required: true

  - id: read_active_decisions
    action: grep
    target: .enjoyknowledge/knowledge/decisions/
    filter: "reversible:true"
    required: false

output:
  type: ai_context
  description: AI 工具已建立项目心智模型
  max_words: 4000  # 强约束：单次 onboard 输出 ≤ 4000 词
```

**示例：preflight.yaml**

```yaml
# .enjoyknowledge/workflows/preflight.yaml
name: preflight
description: PR 提交前/AI 大改前检查冲突
trigger:
  - on_pre_commit
  - on_pr_open
  - manual: /ek preflight

input:  # 工作流输入
  type: git_diff
  source: "git diff --name-only HEAD~1"

steps:
  - id: find_related_gotchas
    action: grep
    target: .enjoyknowledge/knowledge/gotchas/
    filter:
      trigger_file_match: "{{input.files}}"
    required: true

  - id: find_related_decisions
    action: grep
    target: .enjoyknowledge/knowledge/decisions/
    filter:
      applies_to: "{{input.files}}"
    required: true

  - id: find_relevant_rules
    action: grep
    target: .enjoyknowledge/rules/
    filter:
      applies_to: "{{input.files}}"
    required: true

output:
  type: report
  format: markdown
  sections:
    - block:  # ⛔ 阻塞
      - violates rule (R-Code 同步检测)
      - contradicts decision
    - warn:  # ⚠️ 警告
      - matches gotcha
    - info:  # ℹ️ 参考
      - related architecture
```

### 4.2 工作流元数据 schema

```yaml
# 工作流 YAML 的标准字段
name: <string>                    # 工作流名（kebab-case）
description: <string>             # 用途描述
trigger: <list>                   # 触发条件
input: <object?>                  # 输入定义
steps: <list>                     # 步骤序列
output: <object>                  # 输出定义
```

**step 标准字段**：
```yaml
- id: <string>                    # 步骤 ID（唯一）
  action: <enum>                  # cat / grep / ls / doctor / ai_decide
  target: <path glob>             # 目标路径
  filter: <object?>               # 过滤条件
  required: <bool>                 # 失败是否阻塞
  timeout_seconds: <int?>         # 超时
```

**action 枚举**：
- `cat` - 读全文
- `grep` - 全文搜索
- `ls` - 列目录
- `doctor` - 跑 doctor 检查
- `ai_decide` - 让 AI 工具判断（用 AI 上下文）
- `ask_human` - 中断流程，问用户
- `record` - 把结果写回 SoT

### 4.3 主动 vs 被动（v3 共识保留）

| 工作流 | 触发模式 | 原因 |
|---|---|---|
| **onboard** | 被动（AI 工具启动时） | AI 工具应该自己发现项目结构 |
| **prd-preprocess** | 主动（用户输入需求） | 需要自然语言理解 |
| **preflight** | 主动（git hook） | 必须在代码提交前执行 |
| **capture** | 主动（用户/AI 工具建议） | 写入必须有意图 |
| **sync** | 主动（用户命令 / 工具切换） | 跨工具同步 |

**原则**：**"读"走被动（让 AI 工具发现）；"写"走主动（必须有意图）**——这是 v3 共识保留。

### 4.4 工作流依赖图（v3 保留 + v4 修正）

```
onboard (W1) ──→ 唯一前置依赖
    ↓
    ├──→ prd-preprocess (W2)  ← 每次新需求
    │       ↓
    │       └──→ preflight (W3) ← 每次变更前
    │               ↓
    │               └──→ capture (W4) ← 每次有价值的产出
    │
    └──→ sync (W5) ← AI 工具切换时（低频）
```

**v4 修正**：W1 在元数据驱动下不再是"AI 工具读 AGENTS.md 启动"，而是"AI 工具读 `.enjoyknowledge/index.md` 启动"——**入口文件变了**。

---

## §5 9 工具入口文件设计

### 5.1 现状（v4 修正）

| 工具 | 实际入口 | 现状 |
|---|---|---|
| Cursor | `.cursor/rules/enjoyknowledge.mdc` | ✅ 已实现 |
| Claude | `.claude/skills/enjoyknowledge.md` | ✅ 已实现 |
| Copilot | `.github/copilot-instructions.md` | ✅ 已实现（append 模式）|
| Windsurf | `.windsurf/rules/enjoyknowledge.md` | ✅ 已实现 |
| Cline | `.clinerules/enjoyknowledge.md` | ✅ 已实现 |
| Codex | `.codex/prompts/enjoyknowledge.md` | ✅ 已实现 |
| Trae | `.trae/rules/enjoyknowledge.md` | ✅ 已实现 |
| Gemini | `GEMINI.md` | ✅ 已实现（append 模式）|
| Generic | 不写 | ✅（AGENTS.md 单独） |

**v4 关键修正**：
- **v1-v3 的"managed section 划界"只对 AGENTS.md / GEMINI.md 有效**（单文件）
- **其他 7 个工具是 skills/mdc/rules 目录**（多文件 + 各自格式）——**managed section 不适用**
- **v4 改为"入口文件 = 路由表"模式**（不复制 SoT 内容）

### 5.2 入口文件标准格式（v4 引入）

**通用模板**（每个工具的入口文件都长这样）：

```markdown
# enjoyknowledge knowledge base

> Auto-generated by `enjoyknowledge sync`. Do not edit manually.

## Quick links

- **Index**: `.enjoyknowledge/index.md` — start here
- **Rules**: `.enjoyknowledge/rules/` — constraints (must follow)
- **Templates**: `.enjoyknowledge/templates/` — patterns (should use)
- **Knowledge**: `.enjoyknowledge/knowledge/` — context (reference)
- **Workflows**: `.enjoyknowledge/workflows/` — task sequences

## Top 3 things to know

1. **Architecture**: [link to architecture/overview.md]
2. **Critical gotchas**: [link to gotchas/ with severity:critical]
3. **Active decisions**: [link to decisions/ with reversible:true]

## How to use

- **New to project?** → Read [link to architecture/overview.md] + run `/ek onboard`
- **Making changes?** → Run `/ek preflight` before commit
- **Captured a gotcha?** → Use `/ek capture "description"`

## Sync status

- Last sync: {timestamp}
- Source SHA: {git_sha}
- Profile: for-coding
```

**关键**：**这个文件只有链接，没有实际内容**——保证小（< 100 行），AI 工具读完入口后去读具体 SoT。

### 5.3 工具特定的额外配置

**Cursor 特殊处理**（`.mdc` 支持 frontmatter）：

```yaml
---
description: "enjoyknowledge knowledge base - project context for AI"
globs: ["**/*"]
alwaysApply: true
---
```

[Markdown content from §5.2 template]
```

**Claude Skill 特殊处理**（`.claude/skills/enjoyknowledge.md`）：

```markdown
---
name: enjoyknowledge
description: "Access project knowledge base. Use when you need to understand project architecture, gotchas, or conventions before making changes."
---

[Markdown content from §5.2 template]
```

**Codex 特殊处理**（`.codex/prompts/enjoyknowledge.md`）—— Codex 原生支持 `$file` include：

```markdown
# enjoyknowledge knowledge base

See `$file:.enjoyknowledge/index.md` for project knowledge.

## Profile

- Profile: for-coding
- Last sync: {timestamp}
```

**关键洞察**：**Codex 可以直接用 `$file:` 语法 include SoT 文件**——这比 v1-v3 的"managed section 划界"更优雅。**每个工具的特性要保留**（v4 设计哲学 §1.4）。

---

## §6 与现有项目的衔接（v4 关键修正）

### 6.1 现有实现 vs v4 设计

| 维度 | 现有实现 | v4 设计 | 差距 |
|---|---|---|---|
| **Profile trait** | 已有（`src/core/profile.rs`） | 保留 | 0 |
| **9 工具适配器** | 已有（`src/init/ai_tools.rs`） | 保留 | 0 |
| **种子文件** | 硬编码字符串（`CURSOR_RULES`） | 改为引用 SoT | 需重构 |
| **AGENTS.md 单一入口** | v1-v3 默认假设 | ❌ 实际是 skills/mdc/rules | **架构修正** |
| **工作流引擎** | 不存在 | YAML 元数据驱动 | 新增 |
| **sync 命令** | 不存在（只有 init 一次性） | 新增 | 新增 |
| **doctor for Coding 专属检查** | 只有通用 check | 加 6 项专属 | 新增 |

### 6.2 v4 重构路径（按 80/20）

**P0 必改**（v0.1 内）：
1. 入口文件改为 §5.2 路由表模板（去掉"复制 SoT 内容"）
2. 入口文件大小硬上限 ≤ 100 行（不是 30-50 行——v4 放宽，因为多文件）
3. 入口文件加 `Last sync: {timestamp}` 标记

**P1 应改**（v0.2 内）：
4. 加 `enjoyknowledge sync --tool <name>` 命令（动态 sync 替代 init 一次性）
5. 工作流 YAML 元数据 schema 化
6. 加 `enjoyknowledge workflow run <name>` 命令

**P2 可改**（v0.3+）：
7. 加 for-Coding 专属 doctor 检查（R-Code 同步 / 知识腐烂 / 体积约束）
8. 工作流可视化（输出依赖图）
9. 元数据驱动 vs 引擎驱动的实现对比

### 6.3 不该改的（保留现状）

- ✅ Profile trait 抽象（v4 完全沿用）
- ✅ 9 工具适配器（v4 保留为入口）
- ✅ OKF 格式（v4 不变）
- ✅ 体积硬上限 4000 词 / 100 词单条（v4 沿用 v1-v3 共识）

---

## §7 B 站借鉴（具体落地）

### 7.1 直接借鉴（v4 采纳）

| B 站做法 | v4 落地 |
|---|---|
| `.workflow` SoT 显式声明 | `.enjoyknowledge/AGENTS.md`（or `index.md`）首段加 "**This is the single source of truth for the project's AI context.**" |
| `knowledge/` + `rules/` + `template/` 三层 | v4 沿用：`.enjoyknowledge/knowledge/` + `rules/` + `templates/` |
| 工作流命令读不同文件 | v4 沿用：工作流 YAML 描述读哪些文件 |
| Harness Engineering 哲学根 | v4 哲学 §1 采用："显式失败，不静默降级" |

### 7.2 不照搬（v4 明确否定）

| B 站做法 | v4 不照搬 |
|---|---|
| 硬编码文件名到命令 | v4 用 `id` + `applies_to` 路由 |
| 固定 3 个工作流 | v4 用 YAML 元数据驱动，用户可加 |
| 工作流 = 编译到命令的逻辑 | v4 工作流 = 描述性元数据 |
| 不区分 9 工具 | v4 保留 9 工具特性差异 |

### 7.3 B 站没给但 v4 必有的

| 缺失能力 | v4 设计 |
|---|---|
| **R-Code 同步检测** | doctor 加 R-Code 一致性检查（v3 F9）|
| **体积约束** | 4000 词 / 100 词单条（v1-v3 共识）|
| **失败显式化** | 显式失败，不静默降级（v4 §1.3）|
| **多入口协同** | 9 工具入口统一模板 + 工具特定 frontmatter（v4 §5）|
| **元数据 schema 化** | 工作流 YAML / 知识 frontmatter 强 schema（v4 §3.3 + §4.2）|

---

## §8 MVP 边界（v4 修订）

### 8.1 必含 6 项（v3 共识保留）

1. `add` + 路由（自动判断 gotcha/pattern/decision/...）
2. `search` + frontmatter filter
3. **9 工具入口生成**（v4 修正：从 init 改 sync）
4. `doctor` 3 项基础（frontmatter 有效 / 体积上限 / 链接完整）
5. 4000 词硬上限 + 100 词单条
6. frontmatter 必填校验（含 `trigger` for gotcha, `reversible` for decision）

### 8.2 应包含但可简单做（5 项，v3 保留 + v4 调整）

1. `capture` 基础版（手动 `/remember`）
2. `preflight` 基础版（只做路径匹配）
3. 9 工具 sync 基础版（v4 修正：先支持 2-3 种工具，不用全 9 个）
4. `pattern` 全文搜索 + tags filter
5. **入口文件路由表模板**（v4 新增：§5.2 模板）

### 8.3 延后到 v0.2/v0.3（5 项，v3 保留 + v4 调整）

1. 语义级 preflight（embedding）
2. 跨项目知识共享
3. AI 工具主动建议 capture
4. 知识过期自动检测
5. **工作流 YAML 元数据 schema 化**（v4 新增，从 v0.1 必含降到 v0.2）

### 8.4 永不做 / 靠生态（5 项，v3 保留 + v4 调整）

1. 在线知识库托管
2. 协作编辑 / 实时同步
3. **AI 自动生成 gotcha**（违反 F7 信号纯度）
4. 知识"质量评分"
5. **LLM 扩写 knowledge**（违反 100 词约束）
6. **工作流引擎**（v4 新增：保持元数据驱动，不做引擎）

---

## §9 关键风险（v4 特别强调）

### R7：9 工具入口文件"长得一样但实际不同"的同步风险

**问题**：v4 §5.2 的"通用模板"对 9 个工具都一样，但 v4 §5.3 的"工具特定处理"每个不同——**容易出现"用 Cursor 模板生成 Claude 入口" 的 bug**。

**缓解**：
- `sync` 命令必须按工具 ID 走不同路径
- 工具入口生成器单文件，**不允许 generic fallback**
- doctor 加 "工具入口格式校验"（按工具 spec 校验 frontmatter / section 结构）

### R8：工作流 YAML 化 = "用户写 YAML" = 学习成本

**问题**：B 站硬编码 3 个工作流是为了降低用户负担；v4 元数据驱动是把"定义工作流"的负担转嫁给用户。

**缓解**：
- 提供 5 个**模板工作流**（onboard / preflight / capture / sync / 1 个空的）作为 starter
- 文档化 YAML schema（v4 §4.2）
- `enjoyknowledge workflow init <name>` 命令自动生成 YAML 骨架

### R9：v4 推翻 v1-v3 入口假设 = 文档可能误导

**问题**：v1-v3 大量"managed section 划界" / "AGENTS.md 中心"的描述，v4 修正后这些建议**部分失效**。

**缓解**：
- v4 在文档里明确标记 v1-v3 的"哪些结论保留 / 哪些被修正"
- v4 §6 详细列出"现有项目 v4 重构路径"——让读者看 1 个章节就能知道怎么改

### R10：v4 工作流元数据 vs 现有 init 命令的语义冲突

**问题**：现有 `init` 是"一次性生成入口文件"，v4 改为"sync 动态"——**两个命令的语义部分重叠**。

**缓解**：
- `init` 改为"首次生成 SoT + 入口文件"
- `sync` 改为"更新入口文件（不重新生成 SoT）"
- `init` 在 v0.1 保留，但 v0.2 标记为 deprecated，v0.3 移除

---

## §10 v4 vs v1-v3 修正清单

| 维度 | v1-v3 设计 | v4 修正 |
|---|---|---|
| **入口文件** | AGENTS.md 单一 | 9 工具各自的 skills/mdc/rules 目录 |
| **入口文件内容** | 含 managed section + 完整内容 | **只做路由表**（不复制 SoT）|
| **入口文件大小** | 30-50 行（v3）| ≤ 100 行（v4 放宽，因多文件）|
| **managed section 划界** | 必要（v1-v3）| **不再必要**（路由表模式）|
| **工作流实现** | 待设计 | YAML 元数据驱动（v4 §4）|
| **sync 命令** | 替换 init（v1-v3 假设）| 保留 init + 加 sync（v4 兼容）|
| **工具特定 frontmatter** | 抽象统一（v1-v3 倾向）| **保留差异**（v4 §1.4）|
| **B 站借鉴** | 已知 | **明确借鉴 + 明确否定**（v4 §7）|

**关键洞察**：v1-v3 的"managed section 划界"是 v1 时**不知道现状**的设计假设，v4 修正后这个机制**变得不必要**——路由表天然支持多文件 + 工具差异。

---

## 📂 v4 完整记录

- **整合文档**：`C:\Users\jay\Documents\why-workspace\daily\2026-06-27-enjoyknowledge-rule-design-integration.md`
- **新增节**：v4 for Coding 完整设计（10 节）
- **Codex v3 log**：`C:\Users\jay\AppData\Local\Temp\ek-codex-fc.log`
- **Claude v3 log**：`C:\Users\jay\AppData\Local\Temp\ek-claude-fc.log`

---

## 🔗 v4 下一步

1. **接受 v4 修正**——把 v1-v3 中"AGENTS.md 中心" / "managed section 划界" 的描述加上 [v4 修正] 标记
2. **补 `docs/architecture/for-coding-design.md`**——直接用 v4 内容作为单一可信源
3. **重构入口文件生成**（v4 §6.2 P0 必改）——把硬编码字符串改为路由表模板
4. **写工作流元数据 schema 文档**（v4 §4.2）
5. **更新 memory**——v4 是 for Coding 设计的"当前基线"
