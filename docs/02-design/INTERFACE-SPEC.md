# enjoyknowledge 接口规范

> v0.4.10 | 2026-06-29
>
> CLI 行为合约。第三方适配器、工具生成器、AI 工具集成的唯一参考。

---

## 1. 目录结构

### 1.1 for Coding 默认结构

```
<项目根>/
├── AGENTS.md                      # AI 入口（内嵌 ls 摘要）
├── .enjoyknowledge/               # 长期知识 SoT（人类编辑/审核）
│   ├── _meta/kinds.md              # kind 注册表（Markdown 表格）
│   ├── architecture/               # 架构知识
│   ├── gotcha/                     # 踩坑记录
│   ├── pattern/                    # 最佳实践
│   ├── rule/                       # 强制规则
│   ├── decision/                   # 架构决策
│   ├── business/                   # 业务规则
│   ├── contract/                   # 接口契约
│   ├── convention/                 # 命名/格式约定
│   ├── context/                    # 项目背景/运行时
│   ├── command/                    # CLI 命令
│   ├── template/                   # 范式模板
│   ├── index.md                    # 索引
│   └── AGENTS.md                   # KB 写入规则
└── .enjoyknowledge_stage/          # 任务暂存区（AI 自动写）
    ├── _meta/
    │   └── stage-defaults.md        # 默认 stage 目录（用户可编辑）
    ├── tasks/<task-id>/            # 8 文件
    ├── drafts/                     # 待 promote 草稿
    ├── .archive/                   # TTL 过期（180 天）
    └── AGENTS.md                   # 任务写入规范（Hermes skill 格式，用户拥有）
```

### 1.2 结构约束

| 规则 | 约束 |
|---|---|
| 深度 | `.enjoyknowledge/` 内 ≤ 2 层（`category/file.md`） |
| 目录名即分类 | 文件目录决定类型，不在 frontmatter 重复声明 |
| kind = dir | directory name = kind name，无 "s" 派生 |
| 物理分离 | `.enjoyknowledge_stage/`（AI 写）与 `.enjoyknowledge/`（人类写）严格物理分离 |
| frontmatter 极简 | KB 文件仅 4 字段：id / kind / created / author |
| 任务暂存区 | `tasks/<task-id>/` 不进入长期知识索引 |
| kind 注册表 | `.enjoyknowledge/_meta/kinds.md` 为 kind → dir 埋点的单一真相源 |

---

## 2. 知识文档格式

### 2.1 Frontmatter

```yaml
---
title: 导出功能踩坑
description: 超过10万行时接口超时
tags: [export, excel]
timestamp: 2026-06-21
---
```

### 2.2 字段约束

| 字段 | 类型 | 必需 | 约束 |
|---|---|---|---|
| `title` | string | 否 | 人类可读标题 |
| `description` | string | 强烈推荐 | ≤ 200 字符，出现在 `ls`/`tree`/AGENTS.md 中 |
| `tags` | string[] | 否 | 纯小写+连字符 |
| `timestamp` | string | 否 | ISO 8601 `YYYY-MM-DD` |

### 2.3 正文

自由 Markdown。每个 `##` 二级标题为一个条目（entry）：
- `grep` 输出定位到 `##` 段
- `doctor` 以 `##` 标题数统计条目量（> 20 条建议拆分）
- `add` 追加内容以 `##` 为开头

### 2.4 必填 frontmatter 字段（v0.4 文档层规范）

| kind | 必填字段 |
|---|---|
| gotcha | `trigger` |
| decision | `reversible` + `decided_at` |
| pattern / rule / business / architecture / contract / convention / context / template / command | `applies_to` |

### 2.5 保留文件名

| 文件 | 位置 | 作用 |
|---|---|---|
| `index.md` | 任何目录下（可选）| 该目录的目录 |
| `AGENTS.md` | `.enjoyknowledge/` + `.enjoyknowledge_stage/` | AI 入口 + 写入规范，均为 Hermes skill 格式，用户拥有（init 不覆盖）|

---

## 3. CLI 命令

### 3.1 命令总览

| 命令 | 语义 | 输出 |
|---|---|---|
| `init [--ai <tool>] [--template <name>] [--link <path>] [--profile <name>]` | 初始化知识库 | 目录骨架 + AGENTS.md |
| `ls [path] [--bare]` | 列出目录/文件 | 文件列表，默认带 description |
| `tree [--bare]` | 递归目录树 | 目录树，默认带 description |
| `cat <path>` | 查看文件内容 | 文件全文（stdout） |
| `grep <pattern> [--type <dir>] [--tags <tag>] [--path <path>] [--archive] [--req <REQ-ID>]` | 结构化搜索 | 定位到 `##` 段 + 上下文 snippet |
| `add <path> <content>` | 新增/追加知识 | 确认消息（stderr） |
| `doctor [--ci]` | 5 项健康检查 | 问题清单；`--ci` warning 也非零退出 |
| `fix [--req <REQ-ID>]` | 自动修复 | 修复结果 |
| `export --tool <cursor/claude/auto> [--dry-run]` | 生成 AI 工具入口 | `.cursor/rules/*.mdc` 或 `.claude/skills/*.md` |
| `onboard` | 建立项目心智模型 | AGENTS.md + 定位文档 + 关键 gotchas + 活跃决策 |
| `promote <draft_file> --to <kind> [--id <id>] [--author <name>]` | draft → KB | KB 文件 + frontmatter（kind=dir，无 "s" 派生） |
| `kind add <name> [--required <csv>] [--summary <text>] [--yes]` | 新增知识种类 | 更新 kinds.md + 创建目录 + seed 文件 |
| `kind rm <name> [--force] [--yes]` | 删除知识种类 | 从 kinds.md 移除 + --force 删除目录 |
| `kind list` | 列出知识种类 | kinds.md 表格（kind / required / summary） |
| `stage clean [--dry-run] [--force] [--older-than <days>]` | TTL 清理 | 清理 .archive/ 过期文件 |

### 3.2 `init`

```
enjoyknowledge init [--ai <tool>] [--template <name>] [--link <path>] [--profile <name>]
```

- 默认 profile = `for-coding`
- 创建 `.enjoyknowledge/`（11 类目录 + _meta/kinds.md + AGENTS.md + index.md）
- 创建 `.enjoyknowledge_stage/`（tasks/_template/ 8 文件 + drafts/ + .archive/ + AGENTS.md + _meta/stage-defaults.md）
- v0.4.4+: stage AGENTS.md 和 _meta/stage-defaults.md 为用户所有，init **不覆盖**已存在文件
- v0.4.6+: `.enjoyknowledge/AGENTS.md` 也为用户所有，init **不覆盖**已存在文件；采用 Hermes skill 格式（frontmatter + body，与 stage AGENTS.md 一致）
- `--ai <tool>` 同时生成工具入口文件（支持 9 工具）
- `--link <path>` 引用外部知识库，不创建目录

### 3.3 `ls`

```
enjoyknowledge ls [path] [--bare]
```

```
$ enjoyknowledge ls
architecture/
  overview.md           — 项目整体架构
gotchas/
  export.md             — 导出超时、OOM（3 条）
patterns/
  batch-processing.md   — 分批处理模式
```

`--bare`：只列文件名。

### 3.4 `tree`

```
enjoyknowledge tree [--bare]
```

递归目录树。默认带 description。`--bare` 去掉 description。

### 3.5 `cat`

```
enjoyknowledge cat <path>
```

路径相对于 `.enjoyknowledge/`。输出文件全文。

### 3.6 `grep`

```
enjoyknowledge grep <pattern> [--type <dir>] [--tags <tag>] [--path <path>] [--archive] [--req <REQ-ID>]
```

```
$ enjoyknowledge grep "导出超时"
gotchas/export.md##大数据量超时
  - 超过 10 万行时接口超时
  - 当前方案：分批导出，单次最多 10 万行
```

| 要求 | 说明 |
|---|---|
| 匹配范围 | 正文（`##` 段），不搜 frontmatter |
| 大小写 | 不区分 |
| 排序 | 按匹配段的知识密度（段越长越靠前） |
| 段界定位 | 每个匹配行定位到最近的 `##` 标题 |

### 3.7 `add`

```
enjoyknowledge add <path> <content>
```

- 文件存在 → 追加到末尾
- 文件不存在 → 创建（生成 frontmatter 模板），写入内容
- 目录不存在 → 自动创建
- 追加后自动更新 AGENTS.md 中的 `ls` 摘要块

### 3.8 `doctor`

```
enjoyknowledge doctor [--ci]
```

5 项检查（v0.4.10：必填字段动态从 kind registry 读取，不再硬编码）：

| 检查 | 内容 |
|---|---|
| check_frontmatter | 所有 .md 有有效 YAML frontmatter |
| check_required_fields | 动态从 kinds.md 读取每个 kind 的必填字段并校验 |
| check_sot_staleness | timestamp > 180 天 → warning |
| check_export_consistency | export 生成文件与 SoT 一致 |
| check_kinds_md | kinds.md 存在 + 可解析 + 与代码 registry 一致 |

`--ci` 模式：warning 也返回非零退出码。

### 3.9 `fix`

```
enjoyknowledge fix [--req <REQ-ID>]
```

可自动修复：

| 问题 | 修复方式 |
|---|---|
| 缺 `description` | 从正文首段提取或填入占位 |
| AGENTS.md 过期 | 重新生成 `ls` 摘要块 |
| 超出预算（> 20 条目）| 最早条目移至归档 |
| 待归档任务 | 提取可复用条目到 `.enjoyknowledge/` |

不可自动修复：缺 frontmatter。

### 3.10 `export`

```
enjoyknowledge export --tool <cursor|claude|auto> [--dry-run]
```

| `--tool` | 目标文件 |
|---|---|
| `cursor` | `.cursor/rules/enjoyknowledge.mdc` |
| `claude` | `.claude/skills/enjoyknowledge.md` |
| `auto` | 自动检测当前 AI 工具 |

- `--dry-run`：预览不写文件
- v0.2 首发 2 工具（cursor + claude），其他 7 工具架构保留

### 3.11 `onboard`

```
enjoyknowledge onboard
```

- 建立项目心智模型（AGENTS.md + 定位文档 + 关键 gotchas + 活跃决策）
- 无参数，自动扫描 `.enjoyknowledge/` 目录

### 3.12 `promote`（v0.4）

```
enjoyknowledge promote <draft_file> --to <kind> [--id <id>] [--author <name>]
```

- 把 `.enjoyknowledge_stage/drafts/<name>.md` 落地到 `.enjoyknowledge/<kind>/<name>.md`（kind=dir，无 "s" 派生）
- 自动生成 4 字段 frontmatter（id / kind / created / author）
- 默认 author = `enjoy`
- 原 draft 保留（加 `[PROMOTED]` 标记）
- 必须人类手动执行

### 3.13 `stage clean`（v0.4）

```
enjoyknowledge stage clean [--dry-run] [--force] [--older-than <days>]
```

- 清理 `.enjoyknowledge_stage/.archive/` 过期文件
- 默认 TTL = 180 天
- `--dry-run`：列出将清理的文件，不删除
- `--force`：跳过确认
- `--older-than <days>`：覆盖默认天数

### 3.14 `kind`（v0.4.5）

```
enjoyknowledge kind add <name> [--required <csv>] [--summary <text>] [--yes]
enjoyknowledge kind rm <name> [--force] [--yes]
enjoyknowledge kind list
```

管理知识种类（kind registry）。所有操作修改 `.enjoyknowledge/_meta/kinds.md`（Markdown 表格，单一真相源）。

**`kind add <name>`**：
- 校验名称（alphanumeric/underscore/dash）
- 在 kinds.md 表格追加一行
- 创建 `.enjoyknowledge/<name>/` 目录
- 创建 seed file `<name>.md`
- `--required <csv>`：逗号分隔的必填 frontmatter 字段
- `--summary <text>`：人类可读简述
- `--yes`：跳过确认提示

**`kind rm <name>`**：
- 从 kinds.md 表格移除对应行
- 若目录有内容且未传 `--force`，报错阻止删除
- `--force`：同时删除目录及其内容
- `--yes`：跳过确认提示

**`kind list`**：
- 解析 kinds.md 表格，以表格形式输出

---

## 4. AGENTS.md 维护

`init` 生成 AGENTS.md（项目根），内嵌 `<!-- enjoyknowledge_LS_START -->` 块。`add` 时自动更新此块，保持与 `ls` 输出一致。

---

## 5. 模板系统

```
enjoyknowledge init --template <name>
enjoyknowledge init --template list   # 列出可用模板
```

模板加载优先级：
1. `.enjoyknowledge/templates/<name>/`（项目级）
2. `~/.enjoyknowledge/templates/<name>/`（用户级）

---

## 6. 错误码

| 码 | 含义 |
|---|---|
| 0 | 成功 |
| 1 | 输入参数错误 |
| 2 | 文件/路径不存在 |
| 3 | 格式校验失败（frontmatter 不可解析等） |
| 4 | 文件不可读写 |

---

*关联文档：[DESIGN.md](./DESIGN.md) · [GLOSSARY.md](../01-philosophy/GLOSSARY.md)*
