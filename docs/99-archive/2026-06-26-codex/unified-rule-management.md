# 统一规则管理架构（Unified Rule Management Architecture）

**Task**: t_d413c776 — Design unified rule management architecture
**Upstream**: `docs/research/common-concepts-divergences.md` (t_2ee39db5 — 词汇表)
**Source of vocabulary**: §1 of upstream — Rule · Scope · Location · InclusionPolicy · MergeStrategy
**Scope**: Cursor · Claude Code · Codex CLI · Trae
**Consumer**: t_3193c881 (sync 实现) · t_afd1cfd8 (doctor / VCS 校验)
**信度标签** 继承自上游: [OK] / [V] / [ND]

---

## 0. 核心结论

`.enjoyknowledge/rules/<id>.md` 是规则**唯一真值源**（SoT）。`enjoyknowledge rules sync` 命令从 SoT **单向**生成各 AI 工具的原生规则文件，生成器只读取 SoT、不反向写回。工具专有产物带 `<!-- generated from enjoyknowledge rule: <id> -->` 标记,`doctor` 通过该标记做"是否被手改"与"增量 sync"判定。SoT 内是**可移植的 Markdown**——不包含任何工具专有 frontmatter,只用 §2 的统一字段表达。

四件事本文档定下来:
1. **SoT 形态** = Markdown 文件 + OKF frontmatter,字段集合见 §2
2. **生成器形态** = 一次读全部 SoT + 按 tool-adapter 模板输出,见 §3
3. **冲突策略** = additive concat 默认 + 能力降级到目标工具的最强公共子集,见 §4
4. **Scope 分流** = project 进仓库、user 进用户家目录、team 走管理通道(不直接生成文件),见 §6

下游 task t_3193c881 实现 sync、t_afd1cfd8 实现 doctor 校验,本文档是它们的接口合约。

---

## 1. 设计原则

按重要性排序。每条只回答"为什么这样",**不**回答"具体字段是什么"——字段在 §2。

### 1.1 SoT 单一,工具多产物(One Source, Many Targets)

**为什么**: 四个工具的规则机制在概念上收敛到同一组抽象(见上游 §1)。如果每个工具各持一份 SoT,变更要同步改 4 处,任何一处遗漏都导致行为分歧;反过来,工具专有格式在版本升级时变化(YAML frontmatter 新增字段、Cursor 改文件位置等),把 SoT 放在工具侧意味着每次升级要改 SoT 解析器。SoT 必须用工具无关的 Markdown 表达,工具侧产物是**生成物**而非**真值**。

### 1.2 单向生成,不反向同步(Unidirectional Generation)

**为什么**: 工具侧产物(`.cursorrules`、`CLAUDE.md` 等)可能被开发者在 IDE 里手动调过(改个标题、加一行注释)。反向同步会把人工调整灌回 SoT,丢掉所有非规则元数据。`rules sync` 只读 SoT,写工具文件;**从不读工具文件回填 SoT**。`doctor` 负责检测"生成物被手改"并警告(§5.2),不自动 revert。

### 1.3 词汇表锁死,不发明新抽象(Vocabulary Locked)

**为什么**: 上游 t_2ee39db5 已经从四个工具的能力矩阵抽出 5 个一等公民概念(Rule / Scope / Location / InclusionPolicy / MergeStrategy)。新增抽象(`Preset` / `Profile` / `ContextBundle` 等)会引入和工具侧不对齐的语义,后面排查"为什么这条规则在 Cursor 里没生效"时多一层翻译。**新增字段必须能映射回这 5 个概念之一**,否则就是范围漂移。

### 1.4 能力降级,失败不阻断(Downgrade, Don't Fail)

**为什么**: Cursor `.mdc` 有 `globs` 表达力,Trae `.traerules` 没有。SoT 写 `policy:glob:src/**/*.rs`,sync 到 Trae 时如果硬失败(拒绝写),Trae 用户的规则集就少一条;如果静默丢字段,doctor 不知道哪里有信息丢失。**降级是默认值**——sync 写到 Trae 时把 `policy:glob` 降为 `always`、在生成的 `.traerules` 段尾追加 warning 注释、并在 CLI stderr 提示。**不阻断**整个 sync 流程。

### 1.5 用户/团队/项目 三层显式分流(Explicit Scope Routing)

**为什么**: 开发者个人习惯(自己 IDE 的 keybind、写代码时的小段子)不该进项目仓库,团队组织级安全规则(禁用某些命令)不该落到开发者机器上。SoT 用 `scope:` tag 显式标注,sync 按 scope 路由到不同位置(项目仓库 / 用户家目录 / 组织管理通道)。**没有 `scope` tag 的 SoT 默认是 `project`**——但 sync 必须 warn。

---

## 2. SoT 格式

### 2.1 位置

```
<project_root>/.enjoyknowledge/rules/<id>.md
```

- **目录**固定为 `.enjoyknowledge/rules/`(沿用 `docs/ROADMAP.md:175` 的命名约定)。
- **文件名** = `<id>.md`,`id` 是稳定 slug(`[a-z0-9-]+`,全小写,连字符分隔),**整个项目唯一**——sync 用它做去重 key 和生成物注释头。
- **深度**: 不嵌套子目录。所有 rule 平铺在 `rules/` 下;子分组用 `tags` 而非子目录——避免 sync 跑两遍 glob。

### 2.2 字段表(OKF frontmatter)

| 字段 | 类型 | 必需 | 约束 / 语义 | 映射到 §1 哪个概念 |
|---|---|---|---|---|
| `title` | string | 否 | 人类可读标题(出现于 Cursor `description` 字段、UI 列表面板) | Rule (1.1) |
| `description` | string | 强烈推荐 | ≤200 字符,一行摘要。Cursor 透传为 `.mdc` 的 `description`;其他工具 drop | Rule (1.1) |
| `id` | string | **是** | 稳定 slug,== 文件名去后缀 ==。sync 用它做去重 key 和生成物注释头 | Rule (1.1) |
| `tags` | string[] | 否 | 命名空间 tag 集合(见 §2.3) | 全部 5 概念 |
| `body` | markdown | **是** | 文件 frontmatter 之后的所有内容。`## H2` 段是**条目单元**(doctor 按 `##` 计数;和现有 `enjoyknowledge grep` 一致) | Rule body |

> `body` 不写在 frontmatter 里——frontmatter 之后的所有内容就是 body,沿用现有 OKF 解析(参见 `docs/INTERFACE-SPEC.md:74-81` 的 "每个 `##` 为一个条目")。

### 2.3 `tags` 命名空间(三个轴,正交)

```
tags: [scope:<team|user|project>, policy:<always|glob|description|subdir>, tools:<cursor|claude|codex|trae>, ...free_form...]
```

| 命名空间 | 取值 | 缺省 | 语义 |
|---|---|---|---|
| `scope:` | `team` / `user` / `project` | `project` | §6 的路由判定键;sync 看见 `user` 写到 `~/.enjoyknowledge/rules/`、看见 `team` 走 `--emit-org-manifest`(只产出 manifest 不直接写文件,见 §6) |
| `policy:` | `always` / `glob:<pat>` / `description:<text>` / `subdir:<path>` | `always` | 共同概念 §1.4 InclusionPolicy 的统一表达。sync 降级到目标工具的最强公共子集(见 §4) |
| `tools:` | 子集(逗号分隔)= `cursor,claude,codex,trae` | 四个都发 | sync 的白名单;**没列出的工具不写**。如果 `tools` 缺省 = 发所有工具 |

> **缺省值警告**: `scope` 缺省 = `project`(不是 silent fallback),sync 必须 warn "rule `<id>` has no `scope:` tag, defaulting to `scope:project`"——避免个人规则误进仓库。

### 2.4 一个完整 SoT 文件示例

```markdown
---
title: 不要在外层 unwrap Result
description: 库代码一律 `?`,bin 边界才 unwrap 并打 error
id: no-unwrap-result
tags: [scope:project, policy:glob:src/**/*.rs, tools:cursor,claude,codex,trae]
---

## Rationale

库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；只在 binary 入口
（src/main.rs、src/bin/*）的 `fn main` 内允许 `.unwrap()`/`.expect()`，且必须紧跟
`.context()` 或 `eprintln!` 说明失败原因。CI 拒绝任何新增的库内 `unwrap()`。
```

### 2.5 不允许的形态

- **禁止**在 SoT 里写工具专有 frontmatter(如 `globs:`、`alwaysApply:`、`@import:`)——这是 sync 的输入,不是输出。SoT 用 `policy:glob:<pat>` 统一表达。
- **禁止**同一 `id` 出现在两个 SoT 文件——sync 用 `id` 做去重,冲突 = 错误退出(doctor 的硬错)。
- **禁止**在 SoT 的 frontmatter 写 `id` 之外的身份字段(比如 `uuid`、内部 hash)——`id` 已经是稳定身份。

---

## 3. Sync / 生成器流水线

### 3.1 命令形状

```
enjoyknowledge rules sync [--tools <csv>] [--scope <team|user|project>] [--dry-run] [--check]
```

| 标志 | 行为 |
|---|---|
| `--tools` | 覆盖 SoT 的 `tools:` 白名单(临时只发给某些工具) |
| `--scope` | 临时覆盖 `scope` 路由(调试用) |
| `--dry-run` | 打印将写出的文件清单和 diff,不实际写盘 |
| `--check` | 不写盘;只比对"当前生成物"和"按当前 SoT 重新生成的内容",exit 0 = 一致,exit 1 = 漂移(CI 用) |

### 3.2 流水线总览

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Discover                                                 │
│    scan .enjoyknowledge/rules/*.md                         │
│    parse OKF frontmatter → Vec<Rule>                       │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Validate (per Rule)                                      │
│    - id == filename                                         │
│    - id unique across all rules                             │
│    - scope: tag present (warn if missing)                   │
│    - policy: tag value is one of always|glob|desc|subdir    │
│    - body has at least one ## H2 (warn if empty)            │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Scope route                                              │
│    - scope:user   → user home dir rules instance            │
│    - scope:team   → emit org manifest, do NOT write files   │
│    - scope:project→ current project root output             │
│    (default = project + warn)                               │
└────────────────┬────────────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. For each ToolAdapter in target_tools:                    │
│    - filter rules by Rule.tools whitelist                   │
│    - for each rule, downgrade policy to adapter's express  │
│    - emit file(s) with generated marker                    │
└─────────────────────────────────────────────────────────────┘
```

### 3.3 ToolAdapter 接口(给 t_3193c881 用)

```rust
pub trait ToolAdapter {
    /// Stable name used in CLI flag and SoT `tools:` tag
    fn name(&self) -> &'static str;

    /// Files this adapter will write under `project_root`
    /// (relative paths; parent dirs auto-created)
    fn target_paths(&self, project_root: &Path) -> Vec<PathBuf>;

    /// The single-file convention if adapter uses append-mode
    /// (Claude/Codex/Trae all append to a single root file)
    fn is_append_only(&self) -> bool;

    /// For each Rule, emit the per-tool content
    /// `policy` has been pre-downgraded to what this tool supports
    fn render_rule(&self, rule: &Rule) -> AdapterOutput;

    /// Render the per-tool header (the file's preamble, before any rules)
    /// Used only when is_append_only() == false OR the file is being created
    fn render_header(&self) -> &'static str;
}

pub struct AdapterOutput {
    /// Path relative to project_root
    pub path: PathBuf,
    /// Content to write (or to append if adapter.is_append_only())
    pub content: String,
    /// For downgraded policy — shown as warning comment at end of segment
    pub downgrade_warnings: Vec<String>,
}
```

> `is_append_only` 决定是**整文件覆盖**(Cursor `.mdc` 目录)还是**段追加**(Claude `CLAUDE.md` / Codex `AGENTS.md` / Trae `.traerules`)。两种模式都用 `<!-- generated from enjoyknowledge rule: <id> -->` 标记,只是 append 模式多一层"段边界"概念(§5.2 解释)。

### 3.4 四个 ToolAdapter 目标位置

| Tool | Adapter 写到哪里 | 模式 | 备注 |
|---|---|---|---|
| Cursor | `.cursor/rules/<id>.mdc`(每个 rule 一文件) | 整文件覆盖 | `<id>` 和 SoT 同名;`.mdc` 后缀 |
| Claude Code | `CLAUDE.md`(段追加) | append-only | 用 `### <title>` 切分各 rule 段;段头有 `generated` 标记 |
| Codex CLI | `AGENTS.md`(段追加) | append-only | 同 Claude 模式,`#### <title>` 切分 |
| Trae | `.traerules`(段追加) | append-only | 同 Claude 模式,`### <title>` 切分;Trae 没有 glob 表达力(降级见 §4) |

> 沿用现有 `src/init/ai_tools.rs` 的目录约定(参见 `E:\codes\code2enjoyflow\src\init\ai_tools.rs:70-127` 的 `generate_tool_files`)。**区别**: `init --ai` 是 init-time 一次性写一份 "enjoyknowledge 简介" 模板;`rules sync` 是**增量、持续、按 SoT 真实内容**写。

---

## 4. 冲突解决与降级策略

### 4.1 默认策略 = Additive Concatenation

四个工具对**多源规则**的共同默认行为是把所有生效规则按固定顺序拼到系统 prompt 后(**上游 §1.5**)。sync 沿用这个默认:多个 SoT rule → 在目标工具文件里**按 id 字典序**拼成连续段,不互相覆盖。

**为什么默认就是 concat**:
- 工具自身的 concat 默认已经验证过稳定(四个工具都是这个默认)。
- 如果 sync 主动 override,会和工具自身 concat 打架,行为不可预测。
- **`override:` 标签**只有 SoT 显式声明时才走 override 分支(§4.4)。

### 4.2 InclusionPolicy 降级表

当 SoT 写了一条 `policy:` 但目标工具不支持该表达力时,sync **降级**到目标工具支持的最近公共子集,**不阻断**。降级规则如下(横轴 = 工具,纵轴 = SoT 写的 policy):

| SoT policy | Cursor | Claude Code | Codex CLI | Trae |
|---|---|---|---|---|
| `policy:always` | alwaysApply: true (透传) | append 段 | append 段 | append 段 |
| `policy:glob:<pat>` | globs: `<pat>` (透传) | subdir placement* | subdir placement* | **降级**为 always + warning |
| `policy:description:<text>` | description: `<text>` (透传) | **降级**为 always + warning | **降级**为 always + warning | **降级**为 always + warning |
| `policy:subdir:<path>` | alwaysApply: true + 文档注释 | append 段到 `<path>/CLAUDE.md` | append 段到 `<path>/AGENTS.md` | **降级**为 always + warning |

*subdir placement 的实现细节: sync 把这条 rule 写到 `<path>/.claude/CLAUDE.md` / `<path>/.codex/AGENTS.md`,**而不是** root——让工具的 subdir walk 自然加载。详细规格在 t_3193c881 的实现任务里。

**降级 warning 的产物形态**(以 Trae 接受 `policy:glob:src/**/*.rs` 为例):

```markdown
### 不要在外层 unwrap Result
<!-- generated from enjoyknowledge rule: no-unwrap-result -->
<!-- policy downgraded: glob:src/**/*.rs → always (Trae 不支持 glob) -->
库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；...
```

同时,sync 在 stderr 打印一条人类可读 warning(便于 CI log 阅读):

```
WARN  rule 'no-unwrap-result': policy 'glob:src/**/*.rs' not supported by Trae — downgraded to 'always'. Consider splitting .traerules manually or using a different scope.
```

### 4.3 字段降级

| SoT 字段 | Cursor | Claude | Codex | Trae |
|---|---|---|---|---|
| `title` | .mdc description | 段标题 (###) | 段标题 (####) | 段标题 (###) |
| `description` | .mdc description | drop(段标题已够) | drop | drop |
| `body` | 透传 | 透传 | 透传 | 透传 |
| `tags` | drop(只 SoT 内部用) | drop | drop | drop |
| `id` | 生成物注释头 | 注释头 | 注释头 | 注释头 |

### 4.4 Override 显式声明(默认禁用)

SoT 可以在 frontmatter 写 `override: <key>=<value>` 来声明"这条规则对工具的某字段强制覆盖"。**默认 sync 不认这个标签**——只有当 SoT 显式写 `override:enabled: true` 时,sync 才走 override 分支。

override 分支只对**单一字段**生效(典型: Cursor Team rules 的 `permissions.deny`、Claude `settings.json` 的 `permissions`),不试图覆盖工具的整个规则体。override 字段写入工具专有的 config 文件(`.cursor/hooks/*`、`settings.json`、`config.toml`),不写规则文件本身——和 §1.2 的"sync 只写规则文件"略冲突,**但 override 走单独通道**(和 init 一样),不污染主 sync 流程。

> **当前 v0 范围不实现 override**——本文档定义接口边界,具体实现由 t_3193c781(sync 任务)决定是否做,默认 false。

---

## 5. 版本控制(VCS)友好性

### 5.1 什么进仓库

- **进仓库(committed)**: `.enjoyknowledge/rules/*.md`——所有 SoT。
- **进仓库(generated, 可选 commit)**: 工具专有规则文件——技术上 generated,但**通常 commit 进去**,原因是:
  - 让没有装 `enjoyknowledge` 的 contributor / CI 也能拿到规则(零工具依赖)。
  - git diff 工具文件 = 看"规则有没有变"的最快方式。
  - PR review 能在 PR UI 里看到规则变更。
- **不进仓库(gitignored)**: 用户家目录的 rule 副本(§6.2)——个人偏好不该污染团队仓库。

### 5.2 Generated 标记与漂移检测

每个生成物开头都带:

```
<!-- generated from enjoyknowledge rule: <id> -->
```

对 append-only 工具(Claude / Codex / Trae),**整段**前后都有标记:

```markdown
<!-- BEGIN generated from enjoyknowledge rule: no-unwrap-result -->
### 不要在外层 unwrap Result
库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；...
<!-- END generated from enjoyknowledge rule: no-unwrap-result -->
```

`doctor` 怎么用这个标记:
- **漂移检测**: 对每个 `<!-- BEGIN -->` 块,比对当前内容 vs 重新 sync 出的内容。不一致 = "工具文件被手改"。`doctor` 报告,但不自动 revert(尊重人工调整)。
- **增量 sync**: append-only 工具文件里,`doctor` 列出所有 `<!-- BEGIN -->` 块,和 SoT 列表做交叉差集——`SoT 有 / 工具文件没有` = 需要 append,`SoT 没有 / 工具文件有` = 需要删段。
- **手改保留模式**(未来选项,不在 v0): `sync --preserve-edits` 会保留块外的手改内容——但**默认不安全**,v0 走"全量覆盖整块"。

### 5.3 .gitignore 建议

`enjoyknowledge init` 应该把以下放进 `.gitignore`:

```
# enjoyknowledge: user-scope rules live in HOME, not in repo
# (only when user-scope rules exist; doctor will tell you to add this line)
```

**默认不主动 gitignore 工具文件**——见 §5.1,工具文件通常 commit;只有团队明确"工具文件不应该 commit"时,doctor 才提示加:

```
.cursor/rules/
.claude/
.codex/
.traerules
CLAUDE.md
AGENTS.md
.cursorrules
```

### 5.4 Git Hook(预告,t_3193c781 之外)

`docs/ROADMAP.md:176` 已经规划了"commit 时提示相关知识条目"。本架构不直接实现,但 sync 输出应该给 hook 提供可消费的 metadata——建议 t_3193c781 输出 `.enjoyknowledge/.sync-manifest.json`(SoT 路径 + 工具文件路径 + 上次 sync 时间),hook 读它做"diff 提示"。

---

## 6. Team vs Personal(Scope) 分离

### 6.1 三个 Scope 的语义

| Scope | SoT 位置 | Sync 写到 | 例子 |
|---|---|---|---|
| `team` | 仓库内 `.enjoyknowledge/rules/<id>.md` | **不直接写文件**——产出 `org-manifest.json` 描述"组织级规则应包含哪些 id",由 Cursor Team dashboard / Claude Enterprise policy / Codex managed config 各工具的 admin 通道消费 | "禁用 `rm -rf`"、"必须用公司内部 NPM registry" |
| `user` | `~/.enjoyknowledge/rules/<id>.md`(用户家目录) | 同样生成工具文件,但写到 `~/.config/<tool>/...` 或工具识别的 user 层位置 | "我写 Rust 喜欢 snake_case"、"我个人偏好 4 空格缩进" |
| `project` | 仓库内 `.enjoyknowledge/rules/<id>.md` | 项目根的工具文件(见 §3.4 表格) | "本项目用 `?` 传播 Result"、"本项目 commit msg 用 Conventional Commits" |

### 6.2 user 层的细节

`user` scope 的 SoT **不进仓库**——用户在多台机器 / 多个项目之间共享自己的偏好。sync 的处理:

1. 扫描两处 SoT: `<project>/.enjoyknowledge/rules/*.md` **和** `~/.enjoyknowledge/rules/*.md`。
2. 合并两边的 rule(用户的 rule 全局生效,项目 rule 在该项目生效)。
3. **冲突**: `user` 和 `project` 都有同 `id` 的 rule,走**用户优先**(个人偏好 > 团队规则,因为这是"我的工作环境"的偏好)——和 Cursor 自己的 "User rules > Project rules" 行为一致(参见上游 §1.2)。

实现上: sync 的"用户路径"按工具写到对应位置:
- Cursor: 写 IDE profile 配置目录(具体路径 [V],实现时查 IDE 设置面板的"Open User Rules Folder"按钮)——[ND] 不进 git。
- Claude Code: `~/.claude/CLAUDE.md`(append 一段 + 标记)。
- Codex CLI: `~/.codex/AGENTS.md`(append 一段 + 标记)。
- Trae: in-panel 字段(工具 API 限制,只能引导用户手动复制)——sync 输出待复制文本到 stdout,不直接调 IDE。

### 6.3 team 层的细节(管理员通道)

`team` scope 的 SoT **留在仓库**(.enjoyknowledge/rules/ 内),但 sync 不直接生成各工具的"组织级"规则文件——因为:

- Cursor Team rules 走 Cursor admin dashboard,不在仓库。
- Claude Code Enterprise policy 走 Anthropic 控制台,不在仓库。
- Codex managed config 走 OpenAI 管理通道,不在仓库。
- Trae 没有 team tier。

sync 的行为:
1. `team` scope 的 rule 收集到 `.enjoyknowledge/.sync-manifest.team.json`:
   ```json
   {
     "version": 1,
     "rules": [
       {"id": "no-rm-rf", "body_excerpt": "禁用 rm -rf", "tools": ["cursor","claude","codex","trae"]}
     ]
   }
   ```
2. **管理员/集成脚本**读这个 manifest 调各工具的 admin API 把规则 push 上去。
3. sync 在 stderr 打印提示 "N team-scope rules emitted to manifest. Push to your org admin console before merging."

> 当前 v0 范围:**不实现 admin API 集成**——只产出 manifest。t_3193c781 决定是否做 manifest,或推迟到 v0.1。

### 6.4 决策表(给用户 / 文档作者参考)

| 场景 | Scope 选 | 例子 |
|---|---|---|
| "本项目禁止 `unwrap()`" | `project` | 跨团队守则,其他贡献者也该看到 |
| "我写代码时偏好 4 空格" | `user` | 个人审美,不该约束同事 |
| "公司代码必须经过内部 security review 才能 merge" | `team` | 组织级,工具 admin 通道分发 |
| "本项目用 Conventional Commits" | `project` | 项目级约定 |
| "我个人的 commit msg 习惯加 emoji 前缀" | `user` | 个人怪癖 |

---

## 7. 完整示例:一条 Rule → 四个工具产物

SoT 文件 `.enjoyknowledge/rules/no-unwrap-result.md`(已经在 §2.4 出现过,这里完整重列):

```yaml
---
title: 不要在外层 unwrap Result
description: 库代码一律 `?`，bin 边界才 unwrap 并打 error
id: no-unwrap-result
tags: [scope:project, policy:glob:src/**/*.rs, tools:cursor,claude,codex,trae]
---

## Rationale

库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；只在 binary 入口
（src/main.rs、src/bin/*）的 `fn main` 内允许 `.unwrap()`/`.expect()`，且必须紧跟
`.context()` 或 `eprintln!` 说明失败原因。CI 拒绝任何新增的库内 `unwrap()`。
```

`enjoyknowledge rules sync` 跑完后,仓库内新增 / 更新以下四个文件。

### 7.1 Cursor → `.cursor/rules/no-unwrap-result.mdc`

```markdown
---
description: 库代码一律 `?`，bin 边界才 unwrap 并打 error
globs: src/**/*.rs
alwaysApply: false
---

<!-- generated from enjoyknowledge rule: no-unwrap-result -->

## Rationale

库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；只在 binary 入口
（src/main.rs、src/bin/*）的 `fn main` 内允许 `.unwrap()`/`.expect()`，且必须紧跟
`.context()` 或 `eprintln!` 说明失败原因。CI 拒绝任何新增的库内 `unwrap()`。
```

- **policy 透传**: `glob:src/**/*.rs` → `globs: src/**/*.rs`(完整保留)。
- `description` 从 SoT `description` 透传。
- `alwaysApply: false` = 因为有 globs,仅在匹配路径时加载。

### 7.2 Claude Code → `CLAUDE.md` 追加一个段

```markdown
<!-- BEGIN generated from enjoyknowledge rule: no-unwrap-result -->
<!-- policy downgraded: glob:src/**/*.rs → subdir placement @ src/.claude/CLAUDE.md (Claude 无 globs 原生表达) -->
### 不要在外层 unwrap Result

库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；只在 binary 入口
（src/main.rs、src/bin/*）的 `fn main` 内允许 `.unwrap()`/`.expect()`，且必须紧跟
`.context()` 或 `eprintln!` 说明失败原因。CI 拒绝任何新增的库内 `unwrap()`。
<!-- END generated from enjoyknowledge rule: no-unwrap-result -->
```

- **policy 降级**: glob 走 subdir placement——sync 在 `src/.claude/CLAUDE.md` 也追加一份同样内容,Claude 的 ancestor/subdir walk 加载时等同于"只在 src/ 下生效"。
- 根 `CLAUDE.md` 这份也保留,作为"目录里所有文件都看到"的兜底段。

### 7.3 Codex CLI → `AGENTS.md` 追加一个段

```markdown
<!-- BEGIN generated from enjoyknowledge rule: no-unwrap-result -->
<!-- policy downgraded: glob:src/**/*.rs → subdir placement @ src/AGENTS.md (Codex 无 globs 原生表达) -->
#### 不要在外层 unwrap Result

库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；只在 binary 入口
（src/main.rs、src/bin/*）的 `fn main` 内允许 `.unwrap()`/`.expect()`，且必须紧跟
`.context()` 或 `eprintln!` 说明失败原因。CI 拒绝任何新增的库内 `unwrap()`。
<!-- END generated from enjoyknowledge rule: no-unwrap-result -->
```

- Codex 同样无 globs,降级策略和 Claude 一致: subdir placement(写到 `src/AGENTS.md`)。
- 段标题用 `####`(4 级)而非 `###`(3 级),避免和 Claude 的 `###` 段冲突——Codex 的 AGENTS.md 经常和 Claude 的 CLAUDE.md 在同一项目,差异化让 grep 不会撞名。

### 7.4 Trae → `.traerules` 追加一个段

```markdown
<!-- BEGIN generated from enjoyknowledge rule: no-unwrap-result -->
<!-- policy downgraded: glob:src/**/*.rs → always (Trae 不支持 glob 也不支持 subdir placement) -->
### 不要在外层 unwrap Result

库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；只在 binary 入口
（src/main.rs、src/bin/*）的 `fn main` 内允许 `.unwrap()`/`.expect()`，且必须紧跟
`.context()` 或 `eprintln!` 说明失败原因。CI 拒绝任何新增的库内 `unwrap()`。
<!-- END generated from enjoyknowledge rule: no-unwrap-result -->
```

- Trae 是最弱表达力,直接降级到 `always`——每次 AI 看到这条 rule 都生效,粒度损失靠**手工**在 `.traerules` 里分文件 / 用注释拆段来近似(文档建议)。
- sync 在 stderr 打 warn: `WARN rule 'no-unwrap-result': glob downgraded to 'always' for Trae; consider manual segmenting`.

### 7.5 同时输出的 manifest

`.enjoyknowledge/.sync-manifest.json`(给 git hook / CI 消费):

```json
{
  "version": 1,
  "synced_at": "2026-06-26T18:00:00Z",
  "tools": ["cursor", "claude", "codex", "trae"],
  "rules": [
    {
      "id": "no-unwrap-result",
      "scope": "project",
      "outputs": {
        "cursor": ".cursor/rules/no-unwrap-result.mdc",
        "claude": ["CLAUDE.md", "src/.claude/CLAUDE.md"],
        "codex": ["AGENTS.md", "src/AGENTS.md"],
        "trae": ".traerules"
      },
      "downgrades": [
        {"tool": "claude", "from": "policy:glob:src/**/*.rs", "to": "subdir placement @ src/.claude/CLAUDE.md"},
        {"tool": "codex", "from": "policy:glob:src/**/*.rs", "to": "subdir placement @ src/AGENTS.md"},
        {"tool": "trae",  "from": "policy:glob:src/**/*.rs", "to": "policy:always"}
      ]
    }
  ]
}
```

> 包含 `downgrades` 数组,让 CI / doctor 能直接看到"哪些信息被丢了",不靠人工读 diff。

---

## 8. 与现有代码 / 文档的对齐

### 8.1 沿用现有约定

| 来源 | 沿用内容 |
|---|---|
| `docs/INTERFACE-SPEC.md:38-47` | `.enjoyknowledge/` 不超过 2 层(`category/file.md`)。`rules/` 是 1 层子目录,符合 |
| `docs/INTERFACE-SPEC.md:74-81` | "每个 `## H2` 为一个条目"。SoT body 沿用此约定 |
| `docs/INTERFACE-SPEC.md:280-289` | 现有 `init --ai` 表格(9 个工具的位置)。本架构不重写 init,但 `rules sync` 的目标路径和 init 表格保持一致 |
| `docs/ROADMAP.md:175-176` | 规则目录约定 + git commit hook 规划 |
| `docs/ROADMAP.md:262` | `rules sync` 命令名,本架构实现它 |
| `src/init/ai_tools.rs:70-127` | `generate_tool_files` 函数的目录约定(`.cursor/rules/`、`.claude/skills/`、`.codex/prompts/`、`.trae/rules/` 等) |

### 8.2 和 `init --ai` 的关系

| 维度 | `init --ai` (现有) | `rules sync` (本架构) |
|---|---|---|
| 触发时机 | 项目初始化时一次 | 任何 SoT 变更后 |
| 写什么 | "enjoyknowledge 是什么 + 怎么用"的固定介绍 | 当前 `.enjoyknowledge/rules/*.md` 的真实内容 |
| 写到哪里 | `.cursor/rules/enjoyknowledge.mdc` 等固定名 | `<id>.mdc` / 段追加到 `CLAUDE.md` 等 |
| 内容来源 | 硬编码字符串 | 从 SoT 动态生成 |

**两者互补不冲突**:`init --ai` 写"如何使用 enjoyknowledge"的 meta-rule;`rules sync` 写用户的真实业务规则。两者都用 `<!-- generated from enjoyknowledge ... -->` 标记,`doctor` 一并管理。

### 8.3 已识别的偏差(本架构在 t_3193c781 实现时需要确认)

- **`.cursor/rules/` 的目录前缀**: 上游 §1.1 写 Cursor 是 `.cursor/rules/*.mdc`;本架构沿用。`init --ai cursor` 现有代码是 `.cursor/rules/enjoyknowledge.mdc`,对齐。
- **Codex subdir walk 的发现顺序**: [V] 标记——subdir placement 依赖"AGENTS.md 在子目录里被加载"的假设。t_3193c781 实现时必须跑一次**真实 e2e 测试**(在临时仓库里建 src/AGENTS.md 跑 Codex 验证)再提交。
- **Trae 的 `.traerules` 路径**: [V] 标记——本文档假设 workspace 根。t_3193c781 实现时确认官方文档路径。
- **任何工具的 token 限制**: [ND]——sync 不做截断,`doctor` 报"单文件超过 N tokens"作为 warning(不阻断)。N 值的选取在 doctor 任务里。

---

## 9. 给下游任务的接口合约

### 9.1 给 t_3193c781(sync 实现)

1. **命令** `enjoyknowledge rules sync`,支持 `--tools` / `--scope` / `--dry-run` / `--check`(§3.1)。
2. **ToolAdapter trait** 接口如 §3.3,4 个具体实现: Cursor / Claude / Codex / Trae。
3. **输出路径** 严格按 §3.4 表格。
4. **降级策略** 按 §4.2 / §4.3 表格实现,warning 注释插入对应位置。
5. **Manifest** 输出到 `.enjoyknowledge/.sync-manifest.json`(§7.5 schema)。
6. **错误退出码**:
   - 0 = 全部 ok
   - 1 = SoT 解析错(frontmatter 缺 `id`、id 不唯一)
   - 2 = 部分降级(--check 模式用:有降级但不是错)
   - 3 = 工具文件被手改 + --check 模式(和现有 `doctor --ci` 的 3 对齐)

### 9.2 给 t_afd1cfd8(doctor / VCS 校验)

1. **检测项**:
   - 每个 `.enjoyknowledge/rules/*.md` 有合法 frontmatter 且 `id` == filename 去后缀。
   - 没有重复 `id`。
   - 没有工具文件"被手改"(对照 `<!-- generated -->` 标记里的当前生成内容)。
   - `.enjoyknowledge/.sync-manifest.json` 的 `synced_at` 在 `git log -1 --format=%ct` 之后(即"git 改了 SoT 但没 sync")。
2. **不实现**:
   - 内容语义检查(规则写得对不对)——超出 doctor 范围。
   - 自动修复手改——只报告。
3. **输出格式**: 沿用 `doctor` 现有 `enjoyknowledge doctor` 的输出,新增子命令 `enjoyknowledge rules doctor`(或作为 `doctor` 的新检查维度)。

### 9.3 给 future 任务

- **MCP server 把 rules 暴露为 resource**(已在 v1.0 路线图): 走 `cursor/claude/codex` 各自的 MCP rules resource,Trae 暂不支持,放弃。
- **规则版本号**: 当前 SoT 没有 `version` 字段——医生/sync 不知道"我是不是过期了"。v0.1 加 `version: 1.0.0` 字段,doctor 检 `synced_at` 和 SoT `timestamp` 的差距。
- **规则依赖 / 引用**: 当前 SoT 不支持"rule A 引用 rule B"。如果未来需要,在 frontmatter 加 `requires: [<id>, ...]`,sync 按拓扑序排序 concat。

---

## 10. 验证状态与遗留问题

- **本文件未引入新事实**——所有"工具支持/不支持"的声明继承自 `docs/ai-tools-rules-comparison.md` 和 `docs/research/common-concepts-divergences.md`,标签 `[OK]` / `[V]` / `[ND]` 不变。
- **本架构未在 v0 实现**——是给 t_3193c781 和 t_afd1cfd8 的合约文档,不是实现报告。
- **降级表 §4.2 有一处策略选择**:`policy:glob` 在 Claude/Codex 走 subdir placement(产生两个产物,根 + 子目录)还是只在根写、靠人工拆?本文档选 subdir placement(更自动化),t_3193c781 实现时可改为"只在根 + stderr 建议人工拆"——权衡:**subdir placement 实现成本高(要管理两份文件),但用户体验好;根 + 警告实现成本低,但用户得手动操作**。
- **本任务范围内不做**: URL 验证(继承自上游)——§3.4 路径的 [V] 项需要在 t_3193c781 实现时真实 e2e 验证。
- **未覆盖**:规则之间冲突(SoT 内两个 rule 互斥)——目前 SoT 不表达互斥关系,冲突由作者在 body 里用文字说明。v0.1 可以加 `conflicts: [<id>]` tag,本文档不定义。

---

## 11. 给 Enjoy 的快速摘要(可跳过细节)

1. **SoT = `.enjoyknowledge/rules/<id>.md`**,统一 frontmatter(`title` / `description` / `id` / `tags`),body 是普通 Markdown。
2. **tags 三轴**:`scope:`(team/user/project) + `policy:`(always/glob/desc/subdir) + `tools:`(白名单)。缺省值是 project + always + 全部工具,缺省时 sync 报警。
3. **sync 是单向**:`enjoyknowledge rules sync` 读 SoT → 写工具文件,不反向。
4. **工具文件位置**: Cursor `.cursor/rules/<id>.mdc`(整文件),Claude/Codex/Trae 追加段到 `CLAUDE.md`/`AGENTS.md`/`.traerules`。
5. **冲突 = 降级到最强公共子集**: Cursor 的 `globs` / `description` 是高表达力,Trae 啥都没;降级不阻断,但写 warning 注释 + stderr 提示。
6. **generated 标记** = `<!-- generated from enjoyknowledge rule: <id> -->`,doctor 用它做漂移检测 + 增量 sync。
7. **scope 分流**:`project` 进仓库,`user` 进 `~/.enjoyknowledge/`,`team` 只产 manifest(给 admin dashboard 消费)。
8. **下两个任务**:t_3193c781 写 sync、t_afd1cfd8 写 doctor;本文档是它们的接口合约。

---

*Generated by kanban worker run on task t_d413c776, 2026-06-26. Workspace: `E:\codes\code2enjoyflow`. Vocabulary inherited from t_2ee39db5; no new abstractions introduced.*
