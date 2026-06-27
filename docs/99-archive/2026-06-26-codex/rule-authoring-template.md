# 规则文件撰写模板与命名约定

**Task**: t_a461c915 — Define rule authoring template and naming convention
**Input**: `docs/research/common-concepts-divergences.md` (t_2ee39db5 的 5 概念词汇表)
**Consumer**: t_d413c776 (unified rule management architecture) + t_00c6331b / t_3193c881 (下游 sync / doctor 实现)
**Scope**: `.enjoyknowledge/rules/` 目录下的 SoT 规则文件
**Spec 合规**: OKF v0.1 frontmatter + `docs/INTERFACE-SPEC.md` §2.2 / §3 / §7 的目录与 doctor 约束

---

## 0. 一句话结论

每个规则 = **一个 `.md` 文件**，路径 `.enjoyknowledge/rules/<category>/<id>.md`。frontmatter 是 OKF 基础 4 字段 + 5 个 rules 专属字段（`id` / `scope` / `policy` / `tools` / `priority`）。body 必有 4 个 `##` 段：`Title`、`Applies To`、`Priority & Scope`、`Rule`。category 决定子目录、文件名 slug 决定 id，二者必须语义一致。

下游 sync 引擎看到的就是这个模板；tool-specific 降级策略走 §4。

---

## 1. 路径与命名约定

### 1.1 目录位置

```
.enjoyknowledge/
└── rules/                    # 规则 SoT 根目录
    ├── coding-style/         # 编码风格类
    ├── architectural/        # 架构约束类
    ├── security/             # 安全类
    ├── tool-usage/           # AI 工具使用类
    ├── business/             # 业务规则类
    └── workflow/             # 开发流程类（git、PR、release）
```

- **根目录固定**：`.enjoyknowledge/rules/`（GLOSSARY §R 已定义，ROADMAP §175 已承诺）
- **第二层是 category**：对应 §3 的 6 类规则；新增类别先在 §3 表里登记再写文件
- **不允许 3 层**：`a/b/c.md` 一律违规（继承 `INTERFACE-SPEC.md` §2.2 的 2 层深度规则）

### 1.2 文件命名

`<id>.md`，**全小写、连字符分隔、动词或名词短语、最长 50 字符**。

| 规则 | 反例 | 正例 |
|---|---|---|
| 描述"做什么"或"不做什么" | `notes.md` `rule1.md` `temp.md` | `no-unwrap-result.md` `use-thiserror.md` |
| 名词在前 + 限定词在后 | `wrap_dont.md` `UsingThisError.md` | `prefer-thiserror.md` |
| 不重复 category 名 | `coding-style/no-unwrap-result.md`（id 含 `coding`）✓ 但目录已表达 | `no-unwrap-result.md` |
| 不带日期/版本号 | `rule-2026.md` `rule-v2.md` | `no-unwrap-result.md` |
| 单词数 2–5 个 | `u.md` `the-thing-we-discussed-last-week-about-export.md` | `batch-export-over-100k.md` |

### 1.3 `<id>` 的稳定性

`id` = 文件名去掉 `.md`。**id 是 sync 引擎去重的唯一键**，所以：

- **不要在文件名里塞时间戳、PR 号、作者名**（它们会让 sync 把"同一规则"识别成"不同规则"）
- **重命名是 breaking change**——sync 引擎会把它当新规则重新生成，旧文件留着等 doctor 检测漂移
- **id 必须全局唯一**——不允许 `coding-style/use-error.md` 和 `tool-usage/use-error.md` 共存；doctor 会报 duplicate-id

---

## 2. Frontmatter Schema

### 2.1 完整模板

```yaml
---
title: <一句话标题, ≤80 字符, 必填>
description: <AI 用的摘要, ≤200 字符, 必填>
id: <stable-slug, 必填, 等于文件名去掉 .md>
category: <coding-style|architectural|security|tool-usage|business|workflow, 必填>
scope: <project|user|team, 必填>
policy: <always|glob:<pattern>|description:<text>|subdir:<path>, 必填>
tools: [<cursor|claude|codex|trae|copilot|windsurf|cline|gemini>, 必填, 可空数组]
priority: <P0|P1|P2, 必填>
tags: [<lowercase-hyphen>, 可选]
timestamp: <YYYY-MM-DD, 必填>
---
```

### 2.2 字段定义与约束

| 字段 | 类型 | 必需 | 约束 | 同步器怎么用 |
|---|---|---|---|---|
| `title` | string | ✓ | ≤80 字符，可中文，不可空 | 写入 Cursor `.mdc` 的 description 之外的人类标题；同步到 CLAUDE.md / AGENTS.md 作 `##` 章节标题 |
| `description` | string | ✓ | ≤200 字符，一行 | Cursor `.mdc` 的 `description:` 字段直接透传；其他工具 drop（写入时打 warning） |
| `id` | string | ✓ | 必须等于文件名去掉 `.md`；kebab-case | sync 用作生成产物的注释头：`<!-- generated from enjoyknowledge rule: <id> -->`；doctor 用作漂移检测 key |
| `category` | enum | ✓ | 必须等于文件所在目录名 | 决定目录归属；sync 用作章节前缀（如 `## [coding-style] <title>`） |
| `scope` | enum | ✓ | `project` / `user` / `team` | 决定写哪个目录：`project` → 仓库内；`user` → 用户家目录（不进 git）；`team` → 仅文档，不直接生成文件 |
| `policy` | enum | ✓ | 见 §2.3 | 决定何时加载；sync 按工具能力降级（详见 §4） |
| `tools` | string[] | ✓ | 元素必须在 §4.1 白名单内；可写 `[]`（=不生成） | sync 按此白名单过滤；空数组 = 仅 doctor 校验用 |
| `priority` | enum | ✓ | `P0`（必须）/ `P1`（重要）/ `P2`（建议） | 决定 sync 顺序（同 scope 内 P0 先写）；doctor 把 P0 漂移列为 error（CI 必断） |
| `tags` | string[] | — | 纯小写+连字符 | 跨目录过滤；不参与 sync |
| `timestamp` | date | ✓ | ISO 8601 `YYYY-MM-DD` | doctor 检测陈旧（>180 天）规则；不参与 sync |

### 2.3 `policy` 子类型

| 值 | 含义 | 同步降级 |
|---|---|---|
| `always` | 无条件加载 | 全部工具支持，原样透传 |
| `glob:<pattern>` | 只在 AI 操作匹配 glob 时加载 | Cursor 写 `globs:`；Claude Code/Codex 用 subdir placement；Trae/Windsurf/Cline/Copilot/Gemini 降级为 `always` + warning |
| `description:<text>` | 语义触发（Cursor `description` frontmatter） | **仅 Cursor 透传**；其他工具一律降级为 `always` + warning |
| `subdir:<path>` | 仅在指定子目录的 AI 会话加载 | Codex 通过 subdir walk 实现；其他工具降级为 `always` |

> **设计约束**：规则作者可以自由表达意图，但要知道大部分工具只能理解 `always`。如果你想"只对前端代码生效"，写 `policy: glob:src/frontend/**/*` 是对的方向——sync 会把 Trae 那份降级为 `always` 并打 warning，这是 by design，不是 bug。

### 2.4 字段示例对照表

| 字段 | OKF base（INTERFACE-SPEC §3） | rules 扩展（本文） |
|---|---|---|
| `title` | 可选 | **必填** |
| `description` | 强烈推荐 | **必填**（sync 依赖） |
| `tags` | 可选 | 可选（cross-cut 用） |
| `timestamp` | 可选 | **必填**（陈旧检测） |
| `id` | — | **新增必填** |
| `category` | — | **新增必填** |
| `scope` | — | **新增必填** |
| `policy` | — | **新增必填** |
| `tools` | — | **新增必填** |
| `priority` | — | **新增必填** |

继承自上游 t_2ee39db5 §1 的 5 概念（Rule / Scope / Location / InclusionPolicy / MergeStrategy）映射到本表：`scope`→Scope、`policy`→InclusionPolicy、`tools`+目录→Location、`title`/`description`→Rule 标识、`body`→Rule body、`MergeStrategy` 不暴露给作者（默认 additive concat，符合 §1.5 共同结论）。

---

## 3. 规则分类（category）白名单

| category | 子目录 | 适用场景 | 典型例子 |
|---|---|---|---|
| `coding-style` | `rules/coding-style/` | 命名、注释、函数长度、错误处理风格 | `no-unwrap-result.md`、`prefer-thiserror.md`、`max-fn-50-lines.md` |
| `architectural` | `rules/architectural/` | 模块边界、依赖方向、分层、禁止跨层调用 | `no-ui-import-in-domain.md`、`cli-only-imports-core.md` |
| `security` | `rules/security/` | 凭据、注入、SSRF、加密、审计 | `no-secret-in-commit.md`、`ssrf-allowlist-required.md` |
| `tool-usage` | `rules/tool-usage/` | 工具开关、提示词习惯、推荐的 flag | `claude-use-imports.md`、`codex-enable-cache.md` |
| `business` | `rules/business/` | 业务语义约束、计费、合规、领域不变量 | `water-tier-pricing.md`、`export-status-required.md` |
| `workflow` | `rules/workflow/` | git、PR、release、CI/CD 流程 | `pr-needs-knowledge-ref.md`、`commit-msg-conventional.md` |

### 3.1 category 选择决策树

```
这条规则约束的是什么？
├─ 写的代码长什么样 → coding-style
├─ 代码不能跨越哪些模块 → architectural
├─ 不能泄露/注入/越权 → security
├─ 怎么用某个 AI 工具 → tool-usage
├─ 业务上"必须"或"必须不"如何 → business
└─ 流程/协作约定 → workflow
```

### 3.2 边界规则（避免重复或混淆）

- **跨层调用**归 `architectural`，不归 `coding-style`（前者是模块依赖图约束，后者是单文件内风格）
- **"禁止在 main 分支 force push"**归 `workflow`，不归 `security`（流程而非安全威胁）
- **"API 必须用 HTTPS"**归 `security`，不归 `architectural`（安全域而非架构域）
- **"用 thiserror 而非 anyhow!"**归 `coding-style`，不归 `tool-usage`（不是工具使用，是错误处理风格）

---

## 4. Body 结构（4 个必填 `##` 段）

### 4.1 模板骨架

```markdown
# <id>

## Title
<一句话陈述规则，可与 frontmatter title 不同但应同义；同步器优先用 frontmatter title 作为章节标题>

## Applies To
- **Tools**: <展开 frontmatter tools 数组的人类说明>
- **Policy**: <展开 frontmatter policy 的人类说明>
- **Scope**: <展开 frontmatter scope 的人类说明>
- **Priority**: <frontmatter priority + 一句为什么>

## Priority & Scope
<为什么是这个优先级？这条规则会影响哪些人/哪些场景？什么时候可以临时违反？>
<P0 写"never break"，P1 写"should"，P2 写"prefer">

## Rule
<规则正文——具体的、可执行的指令。要求：>
- 用祈使句（"Do X" / "Don't X" / "禁止 Y"）
- 给反例（❌）和正例（✅）
- 引用具体的文件路径或代码片段
- 写清楚"违反后的可见后果"（CI 报错 / 运行时崩溃 / 安全告警）

## Examples
<可选，但强烈推荐——2-4 个 code block>

## References
<可选——相关的其他 rule id、ADR、文档链接>
```

### 4.2 各段的硬要求

| 段 | 必需 | 长度约束 | 风格 |
|---|---|---|---|
| `Title` | ✓ | ≤30 词 | 与 frontmatter `title` 语义一致；可用祈使句 |
| `Applies To` | ✓ | 4-8 行 | 纯列表，展开 frontmatter 的元数据给人类读 |
| `Priority & Scope` | ✓ | 3-6 句 | 解释**为什么**是这个优先级；P0 必须说明"什么情况下是 hard fail" |
| `Rule` | ✓ | 5-20 行 | 至少 1 个 ❌ 反例 + 1 个 ✅ 正例；P0 必须给出违反后的可见后果 |
| `Examples` | — | 0-50 行 | code fence；带文件名注释（` ```rust ` 之外用 ` ```ts ` 等） |
| `References` | — | 1-5 行 | Markdown 链接 |

### 4.3 单文件条数预算

继承 `INTERFACE-SPEC.md` §3.3 / §4.7 的"`##` 条目数 ≤ 20" 约束。本模板的 6 个段是固定骨架，不计入预算；只有 `###` 三级标题和 `Examples` 内的多组对比计入。**如果你写一条规则超过 20 个 `###`，说明它承载了多个独立约束——拆成多个文件，每个文件一个 id**。

---

## 5. 三个示例（覆盖不同 category）

> 示例存放在本文件 §5.1–§5.3，**实际写规则时把内容搬到 `.enjoyknowledge/rules/<category>/<id>.md`**。这里的代码 fence 用 ` ```yaml ` / ` ```markdown ` 是为了展示模板，**真实规则文件只用 ` ```markdown ` 一个外层 fence**。

### 5.1 示例一：`coding-style/no-unwrap-result.md`

**文件路径**: `.enjoyknowledge/rules/coding-style/no-unwrap-result.md`

```markdown
---
title: 库代码禁止 unwrap Result
description: lib/ 下一律 ? 传播错误；只在 bin/main 边界 unwrap 且必须带 context
id: no-unwrap-result
category: coding-style
scope: project
policy: glob:src/**/*.rs
tools: [cursor, claude, codex, trae]
priority: P0
tags: [error-handling, rust]
timestamp: 2026-06-26
---

# no-unwrap-result

## Title
库代码一律用 `?` 传播 Result；只在 binary 入口允许 `.unwrap()` / `.expect()`。

## Applies To
- **Tools**: 全部 4 个目标工具都会收到这条规则
- **Policy**: glob 匹配 `src/**/*.rs`——非 Rust 代码或非 src 路径不受约束
- **Scope**: project 级；不进用户家目录
- **Priority**: P0；CI 会拒绝新增的库内 unwrap

## Priority & Scope
**P0 — never break**。本规则失败时 Cargo build 不报错，但运行时 panic 风险显著上升。本规则影响的场景：所有 PR 修改 `src/lib.rs` / `src/<module>/` 下任何文件。临时违反需在 PR 描述里写明原因。

## Rule
- `src/lib.rs` 及子模块内的**库代码**（被其他 crate 调用的函数）一律用 `?` 传播 `Result`
- 仅 `src/main.rs` 和 `src/bin/*.rs` 的 `fn main` 内部允许 `.unwrap()` / `.expect()`
- 任何 `.unwrap()` / `.expect()` 必须紧跟 `.context(...)` 或 `eprintln!` 说明失败原因
- 违反后：CI lint job 报错；运行时 panic 概率提升；用户报告难以定位

### ❌ 反例

```rust
// src/lib.rs
pub fn parse_config(path: &Path) -> Config {
    let raw = std::fs::read_to_string(path).unwrap();  // 库内 unwrap
    serde_yaml::from_str(&raw).unwrap()                 // 库内 unwrap
}
```

### ✅ 正例

```rust
// src/lib.rs
pub fn parse_config(path: &Path) -> Result<Config, ConfigError> {
    let raw = std::fs::read_to_string(path)
        .map_err(ConfigError::Read)?;
    serde_yaml::from_str(&raw)
        .map_err(ConfigError::Parse)
}

// src/main.rs
fn main() -> anyhow::Result<()> {
    let cfg = parse_config(&args.config)
        .context("loading config")?;
    // ...
}
```

## References
- 相关：`workflow/pr-needs-knowledge-ref.md`（PR 修改 lib 时自动提示本规则）
```

### 5.2 示例二：`security/no-secret-in-commit.md`

**文件路径**: `.enjoyknowledge/rules/security/no-secret-in-commit.md`

```markdown
---
title: 禁止把密钥提交到仓库
description: .env / *.pem / id_rsa 等敏感文件必须 gitignore，pre-commit hook 兜底扫描
id: no-secret-in-commit
category: security
scope: project
policy: always
tools: [cursor, claude, codex, trae, copilot]
priority: P0
tags: [secret, git, ci]
timestamp: 2026-06-26
---

# no-secret-in-commit

## Title
禁止把任何密钥、token、私钥提交到 git 历史；pre-commit hook 兜底扫描。

## Applies To
- **Tools**: 全部 5 个目标工具；trae 的 `policy:always` 降级生效
- **Policy**: always-on——所有 AI 操作都受约束（写代码、改配置、commit message 建议）
- **Scope**: project 级；不进用户家目录
- **Priority**: P0；泄露后必须立即 rotate 并清理历史

## Priority & Scope
**P0 — never break**。密钥泄露后：(1) GitHub 自动扫描告警；(2) 必须立即 revoke + rotate；(3) 历史重写用 `git filter-repo`；(4) 通知所有团队成员。影响：所有人；不可临时违反。

## Rule
- `.env`、`.env.*`、`.envrc` 必须出现在 `.gitignore`
- `*.pem`、`*.key`、`id_rsa`、`id_ed25519`、`*.p12` 永远不进入仓库
- API token / 数据库连接字符串必须通过环境变量或密钥管理服务（Vault / AWS Secrets Manager）
- pre-commit hook 启用 `gitleaks` 或 `trufflehog`，扫描失败阻断 commit
- CI 流水线二次扫描（pre-commit hook 可能被绕过）
- 违反后：立即 revoke 该密钥；git filter-repo 重写历史；强制全员 rotate

### ❌ 反例

```bash
# .env（被提交）
DATABASE_URL=postgres://prod_user:s3cret_p@ss@db.example.com/prod
STRIPE_SECRET_KEY=sk_live_51Hx...

# config.yaml（被提交）
aws:
  access_key_id: AKIAIOSFODNN7EXAMPLE
  secret_access_key: wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

### ✅ 正例

```bash
# .env（gitignore 里）
DATABASE_URL=postgres://prod_user:${DB_PASS}@db.example.com/prod
STRIPE_SECRET_KEY=${STRIPE_SECRET_KEY}

# config.yaml
aws:
  access_key_id: ${AWS_ACCESS_KEY_ID}
  secret_access_key: ${AWS_SECRET_ACCESS_KEY}
```

```gitignore
# .gitignore
.env
.env.*
*.pem
*.key
id_rsa
id_ed25519
*.p12
```

## References
- 相关：`workflow/commit-msg-conventional.md`（commit message 中禁止贴密钥全文）
- 工具：[gitleaks](https://github.com/gitleaks/gitleaks) / [trufflehog](https://github.com/trufflehog/trufflehog)
```

### 5.3 示例三：`architectural/cli-only-imports-core.md`

**文件路径**: `.enjoyknowledge/rules/architectural/cli-only-imports-core.md`

```markdown
---
title: CLI 层只能依赖 Core 层
description: src/cli/* 禁止反向 import src/init、src/doctor 等领域应用模块；依赖方向单向
id: cli-only-imports-core
category: architectural
scope: project
policy: glob:src/cli/**/*.rs
tools: [cursor, claude, codex]
priority: P1
tags: [layering, dependency, three-layer]
timestamp: 2026-06-26
---

# cli-only-imports-core

## Title
`src/cli/*` 只能依赖 `src/core/*`；禁止反向依赖 `src/init/`、`src/doctor/`、`src/add/` 等领域应用模块。

## Applies To
- **Tools**: 3 个支持依赖分析的 LLM 工具
- **Policy**: glob `src/cli/**/*.rs`——AI 修改 CLI 代码时加载
- **Scope**: project 级
- **Priority**: P1；违反后编译能过但循环依赖会逐渐累积

## Priority & Scope
**P1 — should**。本规则失败时编译能过，但依赖图逐渐变成网状结构，三层分离原则被破坏。影响：所有重构工作；可临时违反但需在 PR 描述说明。

## Rule
- 依赖方向：`cli → core`（单向）
- 禁止：`cli → init` / `cli → doctor` / `cli → add` / `cli → knowledge`（除非该模块是 Core 的薄别名）
- 禁止：`init → cli` / `doctor → cli` / `add → cli`（应用层不应反向耦合入口）
- 例外：`init --ai <tool>` 在 init 模块里调用 cli 子命令是允许的（通过 `Command::new` 子进程），但**不能**用 Rust import
- 违反后：循环依赖警告；后续重构成本指数增长

### ❌ 反例

```rust
// src/cli/init.rs
use crate::init::templates;  // ❌ CLI 直接 import 领域应用

pub fn handle_init(args: InitArgs) -> Result<()> {
    templates::render_default(args.template)?;  // 反向耦合
    Ok(())
}
```

### ✅ 正例

```rust
// src/cli/init.rs
use crate::core::render;  // ✅ CLI 只依赖 Core
use crate::core::template::Template;  // 通过 Core 的抽象

pub fn handle_init(args: InitArgs) -> Result<()> {
    let tpl = Template::load(&args.template)?;
    render::apply(tpl, &args.target)?;
    Ok(())
}

// src/init/mod.rs（领域应用层）独立持有自己的逻辑
pub fn init_default(target: &Path) -> Result<()> {
    // ...
}
```

## References
- 相关：`docs/DESIGN-V3.md` 三层分离章节
- 相关：`workflow/pr-needs-knowledge-ref.md`（PR 改 cli 时自动提示）
```

---

## 6. 与现有 INTERFACE-SPEC 的差异

| 项 | INTERFACE-SPEC §3（OKF 通用） | 本文（rules 专属） |
|---|---|---|
| `title` | 可选 | **必填** |
| `description` | 强烈推荐 | **必填** |
| `timestamp` | 可选 | **必填** |
| `id` / `category` / `scope` / `policy` / `tools` / `priority` | — | **新增必填** |
| `##` 段结构 | 自由 | **4 个必填段** + 2 个可选段 |
| 路径 | `.enjoyknowledge/<category>/<file>.md`（2 层） | `.enjoyknowledge/rules/<category>/<id>.md`（2 层，加 `rules/` 固定前缀） |
| 条目预算 | ≤ 20 个 `##` / 文件 | 同上；本模板的 4 必填段不计 |

**`docs/INTERFACE-SPEC.md` 需要补一节 `§3.5 规则文件 schema`**（任务范围内不做；交给 t_d413c776 决定是否合并到主 spec 或单独成文）。本文档是同步的事实标准，下游 sync 引擎按本模板实现。

---

## 7. 校验清单（写完一条规则后过一遍）

- [ ] 文件路径：`.enjoyknowledge/rules/<category>/<id>.md`，三层无
- [ ] 文件名与 `id` 完全一致
- [ ] `category` 与目录名一致
- [ ] `policy` 值是 §2.3 的合法形式（`always` / `glob:` / `description:` / `subdir:`）
- [ ] `tools` 数组元素都在 §4.1 白名单内（来自 INTERFACE-SPEC §6 的 `--ai` 取值）
- [ ] `priority` 是 `P0` / `P1` / `P2` 之一
- [ ] `timestamp` 是 `YYYY-MM-DD`
- [ ] body 有 4 个必填 `##` 段：`Title` / `Applies To` / `Priority & Scope` / `Rule`
- [ ] `Rule` 段至少 1 个 ❌ 反例 + 1 个 ✅ 正例
- [ ] P0 规则的 `Priority & Scope` 段说明了 hard fail 场景
- [ ] 没有 `##` 总数超过 20

---

## 8. 给下游任务（t_d413c776 架构 / t_00c6331b sync / t_3193c881 doctor）的要点

1. **frontmatter 解析器按 §2.2 字段表实现**——`id` / `category` / `scope` / `policy` / `tools` / `priority` 是必填；缺任一项 → 错误码 3（参考 INTERFACE-SPEC §8）
2. **sync 按 `tools` 数组过滤**——`policy` 降级策略见 §2.3 表的"同步降级"列
3. **`id` 是 dedup key**——sync 用 `<!-- generated from enjoyknowledge rule: <id> -->` 注释头 + 文件名双重比对
4. **doctor 必须新增 3 项检查**（相对 INTERFACE-SPEC §7）：
   - rules 文件缺 4 个必填段（任一缺失 → error，CI 必断）
   - `policy` 值不合法 → error
   - `priority` 为 `P0` 的规则在工具专属产物里缺失 → error
5. **`category` ↔ 目录名一致性**是 doctor 必查项——这是继承 INTERFACE-SPEC §2.2 的"目录名即分类"在 rules 子树下的具体化
6. **命名约定的反例（`notes.md` / `rule1.md` / `temp.md`）应作为 doctor 的可选 warning**——不要 hard fail，鼓励而非强制

---

## 9. 验证状态与遗留问题

- **本文件未引入新事实**——所有字段约束、分类白名单、`policy` 子类型都从以下来源派生：
  - t_2ee39db5 §1（5 概念词汇表）
  - INTERFACE-SPEC.md §2.2 / §3 / §6 / §7（目录、frontmatter、AI 工具白名单、doctor）
  - ROADMAP.md §175 / §262（rules SoT + sync 引擎）
  - GLOSSARY.md §R（rules/ 术语定义）
- **本任务范围内不做**：
  - 实际写一个完整项目级 rules 目录（那是 t_3193c881 的活）
  - sync 引擎实现（t_00c6331b）
  - 把本模板合并到 INTERFACE-SPEC.md §3.5（架构任务 t_d413c776 决定）
- **可能的争议点（标记 `[V]` 给架构任务裁决）**：
  - `tools` 数组是否允许空（"仅 doctor 用，不 sync"）——本规范允许，但 sync 引擎需要决定空数组的语义
  - `priority: P0` 的失败是否真要 CI 阻断——取决于团队对"never break"的容忍度
  - `policy: description:<text>` 是否真的只有 Cursor 支持——继承自 t_2ee39db5 §1.4 的 [V] 标记，本规范保守地"仅 Cursor"

---

*Generated by kanban worker run on task t_a461c915, 2026-06-26. Workspace: `E:\codes\code2enjoyflow`. Template locked for downstream sync (t_00c6331b) and doctor (t_3193c881) tasks.*