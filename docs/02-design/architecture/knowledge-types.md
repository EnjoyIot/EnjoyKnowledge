# 知识类型设计

> 版本: 1.0 | 2026-06-27 | 来源: v3 §3 + v4 修订
>
> **本文件定义 10 类知识资产 + 判断标准 + 依赖图**。
>
> 说明：2 类 sync 资产（rules + templates）+ 8 类 knowledge 子目录 = 10 类知识资产。

---

## 1. 10 类知识资产

| # | 目录 | 本质 | 必含字段 | 例子 |
|---|---|---|---|---|
| 1 | `rules/` | 约束（sync）| `id`, `applies_to` | "禁止 .unwrap()" |
| 2 | `templates/` | 范式（sync）| `id`, `applies_to` | "Builder 模式骨架" |
| 3 | `knowledge/architecture/` | 静态结构 | `id`, `last_reviewed` | "API 层用 axum + tower" |
| 4 | `knowledge/gotchas/` | 触发器 | `id`, **`trigger`** | "IF 用 useEffect THEN 检查依赖" |
| 5 | `knowledge/patterns/` | 项目特有模式 | `id`, `applies_to` | "本项目错误流：Result<T, AppError>" |
| 6 | `knowledge/decisions/` | 决策（ADR）| `id`, **`reversible`**, `decided_at` | "为什么选 PostgreSQL" |
| 7 | `knowledge/business/` | 业务规则 | `id` | "用户模块字段定义" |
| 8 | `knowledge/contracts/` | 接口契约 | `id`, `applies_to` | "/api/user/:id 的 avatar 可能是 null" |
| 9 | `knowledge/conventions/` | 命名约定 | `id`, `enforced_by` | "组件 PascalCase，工具函数 camelCase" |
| 10 | `knowledge/context/` | 运行时 | `id`, `env` | "DATABASE_URL 在本地是..." |

## 2. 判断标准（放哪？）

**关键问题**："这条知识 AI 在什么时候消费？"

| 消费时机 | 放哪 |
|---|---|
| AI 启动时自动加载 | rules/ + templates/ |
| AI 写代码时按 glob 触发 | rules/（带 `applies_to`）|
| AI 写代码时按"如果...那么..."触发 | gotchas/（带 `trigger`）|
| AI 写代码时需要"本项目怎么写" | patterns/ |
| AI 写代码前需要"为什么这么设计" | decisions/ + architecture/ |
| AI 写代码前需要"环境是什么" | context/ |
| AI 改代码时需要"哪些字段要同步" | contracts/ |
| AI 改代码时需要"命名/格式规范" | conventions/ |

**判断流程**：
1. 含 "禁止/必须/不能/应当"？→ rules/
2. 是可复用代码/文档骨架？→ templates/
3. 是"如果 X 触发，则 Y 解决"？→ gotchas/（必含 trigger）
4. 是"我们做 X 因为 Y"？→ decisions/（必含 reversible）
5. 是项目模块结构？→ architecture/
6. 是 API 跨模块约束？→ contracts/
7. 是命名/目录/commit 格式？→ conventions/
8. 是 env 变量/端口/服务依赖？→ context/
9. 是可复用的"本项目特有"模式？→ patterns/
10. 是业务领域规则？→ business/

## 3. 知识类型依赖图

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
```

**关键洞察**：`gotchas/` 是**被派生出来的**——一个 gotcha 之所以是 gotcha，是因为它发生在特定的 architecture + pattern + dependency 组合下。所以 gotcha 条目应该能反向链接到它依赖的 architecture 条目。

## 4. 必填字段（frontmatter schema）

> **重要原则**：frontmatter **不强制**——纯 markdown 也能用，只是失去 `applies_to` / `trigger` 过滤能力。v4 决策"不发明新抽象"（不引入 `priority` 之类的复杂概念）。`id` 推荐（路径即 ID 时可省）；`applies_to` 仅 rule/template 必填；`trigger` 仅 gotcha 必填；`reversible` + `decided_at` 仅 decision 必填。

```yaml
# rule（必含 applies_to，无 frontmatter 时无法 sync）
id: rust-no-unwrap                  # 推荐（路径即 ID 可省）
applies_to: [rust, "glob:**/*.rs"]  # 必填，glob 或语言
tags: [rust, error-handling]        # 选填，跨分类标签

# template（必含 applies_to）
id: builder-pattern-skeleton
applies_to: ["lang:rust"]           # 必填
tags: [pattern, scaffold]           # 选填

# gotcha（必含 trigger）
id: useeffect-deps-must-be-stable
trigger: "use useEffect"            # 必填，触发条件
severity: 1-5                       # 选填
tags: [react, hooks]                # 选填

# decision（必含 reversible + decided_at）
id: postgresql-over-mysql
reversible: false                   # 必填
decided_at: 2026-04-15              # 必填
alternatives: [mysql, sqlite]       # 选填

# architecture（推荐 id + last_reviewed）
id: api-layer-axum-tower
last_reviewed: 2026-06-20           # 推荐，>90 天未审 doctor 警告
tags: [api, backend]                # 选填

# pattern（推荐 id + applies_to）
id: error-flow-result-app-error
applies_to: ["src/**/*.rs"]         # 推荐
tags: [error-handling]              # 选填

# contract（推荐 id + applies_to + breaking_change_since）
id: api-user-avatar-can-be-null
applies_to: ["src/api/user.ts"]     # 推荐
breaking_change_since: null         # 选填

# convention（推荐 id + enforced_by）
id: pascalcase-components
enforced_by: lint                   # 选填，lint/formatter/manual

# context（推荐 id + env + last_verified）
id: database-url-local
env: DATABASE_URL=postgres://...    # 推荐
last_verified: 2026-06-15           # 选填

# business（推荐 id）
id: user-module-fields
tags: [user, domain]                # 选填
```

## 5. 体积约束

| 约束 | 阈值 | 处理 |
|---|---|---|
| 单条长度 | > 100 词 | warning |
| 总 SoT 长度 | > 4000 词 | warning + 建议归档 |

**例外**：rule 可以超 100 词（"API 错误码体系"是 20+ 错误码）。但**单条决策/单条 gotcha ≤ 100 词**。

## 6. 命名约定

**文件名 = kebab-case slug**：
- ✅ `rust-no-unwrap.md` / `api-design.md` / `monorepo-vs-polyrepo.md`
- ❌ `Rust_NoUnwrap.md` / `api design.md` / `001-monorepo.md`

**目录即分类**（无嵌套子目录）：
- ✅ `knowledge/architecture/overview.md`
- ❌ `knowledge/architecture/system/overview.md`

## 7. 关系：path 即 ID

`id` 字段 = 文件名（不含后缀）。

**`E:\codes\code2enjoyflow\.enjoyknowledge\rules\rust-no-unwrap.md` 的 id = `rust-no-unwrap`**。

这保证：
- 跨项目引用不冲突（路径天然 namespace）
- 文件改名 = id 改名 = 全局一致
- git diff 精确到单条

## 8. 强制 / 禁止

**强制**（Core `add` 命令 + doctor 校验）：
- gotcha 必含 `trigger` 字段（缺它 doctor 报错）
- decision 必含 `reversible` + `decided_at`
- 体积上限 100 词单条 / 4000 词总

**禁止**：
- 在 gotchas/ 放"注意性能"类空洞条目（`doctor` 检测到无 `trigger` 字段打回）
- 在 rules/ 写代码风格（留给 formatter/linter）
- 直接删 gotcha（必须先走 doctor 确认无引用）

---

**关联文档**：
- [for-coding-design.md §4 10 类知识目录](./for-coding-design.md)
- [rule-system.md](./rule-system.md)
- [workflows.md §4 步骤的 filter 用法](./workflows.md)
