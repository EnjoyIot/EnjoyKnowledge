# AI 工具规则机制 — 共同概念模型 & 工具专属扩展

**Task**: t_2ee39db5 — Identify shared concepts and divergences across tools
**Input**: `docs/ai-tools-rules-comparison.md` (t_c1586b65 的产出)
**Scope**: Cursor · Claude Code · Codex CLI · Trae
**Consumer**: t_d413c776 (unified rule management architecture) — 本文件是它的词汇表

**信度标签**（继承自上游比较矩阵）：
- **[OK]** = 训练时文档支持 + 稳定 doc URL
- **[V]** = 训练时合理但本轮未 URL 验证
- **[ND]** = 未公开 / 无数字限制

---

## 0. 核心结论

四个工具的"自定义规则/指令"机制在概念上**收敛到同一组抽象**，差异只在**实现细节和表达能力**。任何上层架构（统一管理、跨工具同步）都应该用这套抽象作为词汇表，不要被工具名牵走。

下游架构任务（t_d413c776）的 SoT 命名约定是 `.enjoyknowledge/rules/`（工具无关 Markdown），同步引擎单向生成各工具的原生文件（参见 `docs/ROADMAP.md:175` 和 `docs/INTERFACE-SPEC.md:280-289`）。本文件的术语直接对应该流水线。

---

## 1. 共同概念（共享抽象）— 5 个一等公民

下面 5 个概念在四个工具中**全部存在**。任何跨工具模型都至少要有这 5 个实体。

### 1.1 `Rule`（规则条目）

- **定义**：开发者或团队为 AI 工具编写的一条指令/上下文/约束，可以是散文、要点列表、或带 frontmatter 的结构化块。
- **跨工具映射**：
  - Cursor: `.cursorrules` 的一行/一段 / `.cursor/rules/*.mdc` 的单个文件
  - Claude Code: `CLAUDE.md` 的一个章节 / 通过 `@import` 引入的独立 .md
  - Codex CLI: `AGENTS.md` 的一个章节 / `instructions.md` 的等价段
  - Trae: `.traerules` 的一个章节
- **统一层**: `.enjoyknowledge/rules/*.md` 的单文件
- **最小属性**: `id`（稳定标识，用于同步去重）、`body`（实际指令文本）、`scope_tags`（项目/用户/团队 — 见 §1.2）

### 1.2 `Scope`（作用范围）

- **定义**：规则在哪些上下文中生效。三个层级在四个工具中**都有等价物**：
  | 层级 | Cursor | Claude Code | Codex CLI | Trae |
  |---|---|---|---|---|
  | **team / org / managed** | Team rules (admin) | Enterprise policy | — [ND] | — [V] |
  | **user / personal** | User rules (IDE profile) | `~/.claude/CLAUDE.md` | `~/.codex/AGENTS.md` | IDE settings panel |
  | **project / repo** | `.cursorrules` / `.cursor/rules/*.mdc` | `CLAUDE.md` (root/subdir) | `AGENTS.md` (root/subdir) | `.traerules` |
- **统一层抽象**: `Scope = { team, user, project }`，但需要标注 `[V]`/`[ND]` 标记的工具不支持某些层级。
- **设计含义**: 同步器必须**按 scope 分流**——`user` 层级不应进入仓库的 `.enjoyknowledge/rules/`（个人偏好不进版本控制），`project` 层级才进。

### 1.3 `Location`（文件位置 + 路径约定）

- **定义**：规则文件存放在文件系统的哪个位置，触发了"被该工具加载"的隐式行为。
- **共同模式**: 每个工具用**目录结构或文件命名约定**作为隐式配置（不是显式注册表）。
  - 单文件约定: `CLAUDE.md`、`AGENTS.md`、`.cursorrules`、`.traerules` ——文件名即注册。
  - 目录约定: `.cursor/rules/`、`.codex/prompts/` (project) ——目录扫描即注册。
- **统一层抽象**: `Location = (root, glob_pattern, format_hint)`，例如 `(repo_root, "**/rules/*.md", "okf-frontmatter")`。
- **设计含义**: 同步器知道每个工具扫什么路径，就能精确写回，而不会把生成物放到工具不读的位置。

### 1.4 `InclusionPolicy`（包含条件）

- **定义**：决定一条规则是否在**当前会话/当前文件**被加载。
- **共同子维度**:
  1. **always-on**: 无条件加载（绝大多数规则默认）。
  2. **path-conditional**: 只在 AI 操作匹配某 glob 时加载。Cursor 的 `globs` frontmatter 显式支持；Claude Code 的 ancestor/subdir CLAUDE.md 隐式支持；Codex 的 subdir AGENTS.md 隐式支持；Trae [V] 无显式机制。
  3. **content-conditional** [V]: Cursor 的 `description` frontmatter（语义触发）— 多数工具无等价物。
- **统一层抽象**: `InclusionPolicy = always | glob:<pattern> | description:<text> | subdir:<path>`。
- **设计含义**: 同步器在向工具 X 写入时，需要把通用 policy **降级**到工具 X 支持的最强表达（最弱公共子集 = `always`）。

### 1.5 `MergeStrategy`（多源合并策略）

- **定义**：当同一时刻多条规则（或多源规则）都生效时，工具如何把它们组合。
- **观察到的两种策略**:
  - **Additive concatenation**（主策略，四个工具都用）: 所有生效规则按固定顺序拼到系统 prompt 后面，不互相覆盖。
  - **Override hierarchy**（仅 Claude Code 的 `settings.json` + Cursor 的 Team 规则有）: 高优先级源**替换**低优先级源的同名字段。
- **统一层抽象**: `MergeStrategy ∈ { concatenate, override:<key>=<value> }`。`overridable_keys` 是个白名单（settings.json 的 `model`、`permissions`、Cursor 的 Team rules 全集）。
- **设计含义**: 同步器不需要重新发明合并算法——绝大多数情况下把所有规则按 SoT 顺序 concat 即可；只有当用户的 SoT 明确标了"这个键要 override"才走 override 分支。

---

## 2. 工具专属扩展（Divergences）

下面这些能力**不是四个工具的公共子集**——它们只在部分工具中存在。同步器遇到这些能力时必须**降级**或**不写回**。

### 2.1 仅 Cursor 有

- **`.mdc` YAML frontmatter**（`description` / `globs` / `alwaysApply`）——结构化规则。Claude Code/Codex/Trae 无等价 frontmatter。
- **Team rules**（dashboard 管理，强制覆盖 User/Project）——唯一的"组织级 override 通道"。
- **`AGENTS.md` 自动拼接**（Cursor 2025 起也认 AGENTS.md 作为项目上下文，与 Cursor 自身规则一起 concat）——多入口发现。

### 2.2 仅 Claude Code 有

- **`@relative/path.md` import**（在 CLAUDE.md 内部嵌入其他 .md）——模块化规则的唯一表达。
- **`settings.json` 真正 override 层级**（`local > project > user`，per-key deep-merge）——唯一支持结构化配置的合并。
- **`.mcp.json` / `~/.mcp.json` MCP 服务器注册**（独立于 rules 的 config slot）。
- **`.claude/commands/`、`.claude/agents/`**（slash command / sub-agent 定义）——正交于 rules。
- **Enterprise/managed policy**（admin 部署）—— settings 的最外层 override。

### 2.3 仅 Codex CLI 有

- **`AGENTS.md` 的 ancestor walk + subdir walk 双重发现**（同时支持向上和向下）——单文件约定的最强覆盖。
- **`CODEX_HOME` 环境变量**（重定位整个用户配置目录）——其他工具不支持。
- **`config.toml` TOML sidecar**（model / shell / sandbox / notice）——结构化配置，但不是 rules 本身。
- **`instructions.md` legacy 别名**（与 `AGENTS.md` 同槽位，旧项目兼容）——历史包袱。

### 2.4 仅 Trae 有

- **单文件 `.traerules` 强制**（不支持多文件规则目录）——最简单的模型，但也最弱。
- **in-panel "Custom Instructions" / "System Prompt" 字段**（per-chat 临时覆盖）——UI 层 override，不在磁盘。
- 没有 team tier、没有 frontmatter、没有 import、没有 path condition。

### 2.5 共同缺失（**任何工具都没有**）

- 没有一个工具公开**字节/行数/token 上限**。实际限制都是"模型上下文窗口减去其他内容" [ND]。
- 没有一个工具有**真正的"规则版本控制"**——规则文件在仓库里靠 git，工具不感知 schema 版本。
- 没有一个工具有**规则健康检查**——SoT 写错了、引用了不存在的文件，工具照样加载直到运行时报错。

---

## 3. 统一抽象到 SoT 的字段映射

把 §1 的 5 个概念落到 `.enjoyknowledge/rules/<id>.md` 的 frontmatter 形态（OKF 兼容，参见 `docs/GLOSSARY.md:62`）：

```yaml
---
title: <rule title>
description: <one-line summary, ≤200 chars>
tags: [scope:<team|user|project>, policy:<always|glob|description|subdir>, tools:<cursor|claude|codex|trae>]
id: <stable slug, used by sync to dedupe>
---
# <rule body — Markdown>
```

字段 → 共同概念映射：

| frontmatter 字段 | 来自哪个概念 | 同步器怎么用 |
|---|---|---|
| `id` | Rule (1.1) | 生成各工具文件时用作注释头（`<!-- generated from enjoyknowledge rule: <id> -->`），便于 doctor 反查和增量 sync。 |
| `description` | Rule (1.1) + InclusionPolicy (1.4) | Cursor `.mdc` 的 `description` 字段直接透传；其他工具无对应，drop。 |
| `tags: scope:project` | Scope (1.2) | 决定写哪个目录：project → 仓库内，user → 用户家目录（不进 git），team → 文档/管理通道（不直接生成文件）。 |
| `tags: policy:glob:<pat>` | InclusionPolicy (1.4) | Cursor 写 `globs: <pat>` frontmatter；Claude Code/Codex 通过 subdir placement 实现；Trae 降级为 `always`。 |
| `tags: policy:description:<text>` | InclusionPolicy (1.4) | 仅 Cursor 透传，其他工具降级为 `always`（写一个 warning）。 |
| `tags: tools:...` | 显式选择子集 | 同步器按这个白名单过滤，不写未列出的工具。 |
| body | Rule body | 原样 concat（共同 MergeStrategy §1.5）。 |

---

## 4. 一条规则 → 四个工具的小例子

SoT 文件 `.enjoyknowledge/rules/no-unwrap-result.md`：

```yaml
---
title: 不要在外层 unwrap Result
description: 库代码一律 `?`，bin 边界才 unwrap 并打 error
tags: [scope:project, policy:glob:src/**/*.rs, tools:cursor,claude,codex,trae]
id: no-unwrap-result
---
库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；只在 binary 入口
（src/main.rs、src/bin/*）的 `fn main` 内允许 `.unwrap()`/`.expect()`，且必须紧跟
`.context()` 或 `eprintln!` 说明失败原因。CI 拒绝任何新增的库内 `unwrap()`。
```

四个工具的生成产物（仅关键差异，省略共同 header）：

**Cursor → `.cursor/rules/no-unwrap-result.mdc`**
```markdown
---
description: 库代码一律 `?`，bin 边界才 unwrap 并打 error
globs: src/**/*.rs
alwaysApply: false
---
<!-- generated from enjoyknowledge rule: no-unwrap-result -->
库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；...
```

**Claude Code → `CLAUDE.md`（追加一个章节，因为 Claude 没有目录化规则文件的概念）**
```markdown
<!-- generated from enjoyknowledge rule: no-unwrap-result -->
### 不要在外层 unwrap Result
库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；...
```
（`@import` 没法用，因为 SoT 不在仓库的规则目录里——子模块切分留给用户用 `@./.enjoyknowledge/rules/no-unwrap-result.md`，由 Claude Code 加载时内联。）

**Codex CLI → `AGENTS.md`（追加章节）**
```markdown
<!-- generated from enjoyknowledge rule: no-unwrap-result -->
#### 不要在外层 unwrap Result
库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；...
```
（Codex 的 subdir walk 等价实现了 globs——把这条规则放到 `src/.enjoyknowledge-rules/no-unwrap-result.md` 或类似的"子目录 AGENTS.md"里，对应 `globs` 行为；这是 sync 的一个实现选择，doc 同步消费端不需要关心。）

**Trae → `.traerules`（追加章节）**
```markdown
<!-- generated from enjoyknowledge rule: no-unwrap-result -->
### 不要在外层 unwrap Result
库代码（src/lib.rs 及子模块）一律用 `?` 传播 Result；...
```
（Trae 没有 glob，sync 写 warning：`policy:glob:src/**/*.rs` 降级为 `always`，建议人工拆分 .traerules。）

注意所有四个产物都有 `<!-- generated from enjoyknowledge rule: <id> -->` 标记——这是 `doctor` 检测"规则文件是否被手动改过"和"增量 sync 该重写哪一段"的钩子。

---

## 5. 边界：什么**不**算"规则"（避免范围漂移）

这些是四个工具都有的相邻概念，但**不是规则**——sync 不应碰它们：

| 概念 | 例子 | 为什么不算 rule |
|---|---|---|
| **MCP 服务器注册** | Claude `.mcp.json`、Cursor MCP config | 是 tool 扩展，不是指令 |
| **Slash command / sub-agent 定义** | Claude `.claude/commands/`、`.claude/agents/` | 是调用入口，不是 prompt 注入 |
| **Lifecycle hooks** | Cursor `.cursor/hooks/` | 是事件回调，不是静态 prompt |
| **Settings / config（model、permissions、sandbox）** | Claude `settings.json`、Codex `config.toml` | 是行为配置，不是指令文本 |
| **Per-chat "System Prompt" UI 字段** | Trae in-panel | 是临时覆盖，不是持久化文件 |

架构任务 t_d413c776 应当把 sync 的 scope **严格限定在"静态 prompt 注入"的规则文件**——其余的 hooks / MCP / commands 各自有独立的兼容层（见 `docs/INTERFACE-SPEC.md:280-289` 的 `init --ai <tool>` 表格，那些是 init 命令的产物，不是 sync 命令的产物）。

---

## 6. 给下游架构任务（t_d413c776）的要点清单

1. **词汇表用 §1 的 5 个概念**（Rule、Scope、Location、InclusionPolicy、MergeStrategy）——不要发明 `Preset`、`Profile`、`ContextBundle` 之类的新抽象。
2. **冲突解决默认走"concat + 降级"**：高表达力特性（Cursor frontmatter、Claude @import、Codex subdir walk）映射到低表达力工具时降级为 `always` + warning，不阻断 sync。
3. **`user` scope 不进仓库**——`.enjoyknowledge/rules/` 只放 `project` 和 `team`；`user` 层规则走用户家目录的同名引擎实例。
4. **每个生成物都有 `generated from enjoyknowledge rule: <id>` 标记**——给 `doctor` 和增量 sync 用。
5. **`Override` 仅在 SoT 显式声明时启用**——默认所有规则都是 additive concat，避免和工具的"concat 默认行为"打架。
6. **架构任务应当把"如何生成"和"如何校验"分成两阶段**：先做 sync，再做 doctor 检查；不要在一个 PR 里同时引入生成器和校验器（参考 `docs/ROADMAP.md:175` 的 split）。

---

## 7. 验证状态与遗留问题

- **本文件未引入新事实**——所有"共同/差异"声明都来自 `docs/ai-tools-rules-comparison.md` 矩阵，每行可上溯到上游 comment。
- **仍待验证（继承自上游）**：
  - Trae 的 `.traerules` 真实文件路径、是否支持多文件 [V]
  - Cursor Team rules 的覆盖语义是否真的"完全 override" [V]
  - Codex `AGENTS.md` subdir walk 的具体发现顺序细节 [V]
  - 任何工具的 token/byte/count 限制 [ND]
- **本任务范围内不做**：URL 验证（需要带 web_fetch 的 worker 重做 t_c1586b65）。架构任务 t_d413c776 应当把"sync 时遇到 [V] 能力的降级策略"作为降级规则落地，不强求每个细节都验证过。

---

*Generated by kanban worker run on task t_2ee39db5, 2026-06-26. Workspace: `E:\codes\code2enjoyflow`. Vocabulary locked for downstream task t_d413c776.*
