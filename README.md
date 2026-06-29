# enjoyknowledge

> **项目知识 SoT + AI 工具共享上下文层**。一份 markdown 写一次，多个 AI 工具使用。

---

## 快速开始

```bash
# 从 Git 仓库直接安装（尚未发布到 crates.io）
cargo install --git <repo-url>

# 在当前项目初始化知识库
enjoyknowledge init
```

`ek init` 会创建 `.enjoyknowledge/`（知识库）和 `.enjoyknowledge_stage/`（任务暂存区）。

---

## 目录结构

```
项目根/
├── AGENTS.md                      # AI 入口（内嵌 ls 摘要，30 秒读懂项目）
├── .enjoyknowledge/               # 长期知识 SoT（人类写，AI 读）
│   ├── _meta/kinds.md              # kind 注册表 ← 改 kinds 改这里
│   ├── skills/                    # 工作流 skill 文件 ← 自定义工作流放这里
│   ├── architecture/              # 系统结构
│   ├── business/                  # 业务规则
│   ├── command/                   # CLI 命令文档
│   ├── context/                   # 项目背景/运行时
│   ├── contract/                  # 接口契约
│   ├── convention/                # 命名/格式约定
│   ├── decision/                  # 架构决策（ADR）
│   ├── gotcha/                    # 踩坑记录
│   ├── pattern/                   # 最佳实践
│   ├── rule/                      # 强制规则
│   ├── template/                  # 范式模板
│   ├── index.md                   # 索引
│   └── AGENTS.md                  # KB 写入规则（用户拥有，init 不覆盖）
└── .enjoyknowledge_stage/         # 任务暂存区（AI 自动写，人类审核）
    ├── _meta/stage-defaults.md     # 默认 stage 目录清单（用户可编辑）
    ├── tasks/<task-id>/           # 8 文件模板
    ├── drafts/                    # AI 草稿，人类 promote 后落地
    ├── .archive/                  # TTL 过期（默认 180 天）
    └── AGENTS.md                  # 任务写入规范（用户拥有，init 不覆盖）
```

---

## 自定义知识种类（kinds）

### 查看当前 kinds

```bash
ek kind list
```

输出 kinds.md 表格：kind 名、必填字段、简述。

### 添加新 kind

```bash
# 基础用法：加一个 "api" kind
ek kind add api --summary "API 设计文档"

# 带必填字段
ek kind add api --required "endpoint,method" --summary "API 设计文档"

# 跳过确认
ek kind add api --summary "API 设计文档" --yes
```

这会自动完成 3 件事：
1. 在 `.enjoyknowledge/_meta/kinds.md` 追加一行
2. 创建 `.enjoyknowledge/api/` 目录
3. 创建 seed 文件 `.enjoyknowledge/api/api.md`

### 删除 kind

```bash
ek kind rm api           # 只从 kinds.md 移除（目录有内容时报错）
ek kind rm api --force   # 同时删除目录及其内容
```

### 修改已有 kind 的必填字段

直接编辑 `.enjoyknowledge/_meta/kinds.md`，修改 `required` 列：

```markdown
| kind | required | summary |
|------|----------|---------|
| api | endpoint, method, auth | API 设计文档 |
```

保存后 `ek doctor` 会自动校验新规则。**不需要改任何代码**。

### 修改 seed 模板内容

`ek init` 生成的默认文件内容来自 `src/fixtures/seeds/{kind}.md`。修改后重新编译即可生效。

---

## 自定义 stage 结构

### 修改 stage 目录清单

编辑 `.enjoyknowledge_stage/_meta/stage-defaults.md`，修改 `Default Directories` 表格：

```markdown
| Directory | Purpose |
|-----------|---------|
| tasks     | 任务工作目录 |
| drafts    | 待 promote 草稿 |
| .archive  | TTL 过期归档 |
| designs   | 设计稿暂存    ← 自定义
```

`ek init` 会按这个表格创建目录。**用户拥有，init 不覆盖**。

### 修改 stage 任务模板

`.enjoyknowledge_stage/tasks/_template/` 下有 8 个模板文件。直接编辑即可，`ek init` 不覆盖已存在的文件。

---

## 自定义工作流

### 内置 4 个工作流

`.enjoyknowledge/skills/` 下 4 个 skill 文件定义 AI 的工作流：

| 文件 | 场景 | 触发词 |
|------|------|--------|
| `coding.md` | AI 接编码任务 | "实现"、"修 bug"、"开始任务" |
| `research.md` | AI 查现有知识 | "查"、"找"、"调研" |
| `review.md` | AI 完成任务后沉淀 | "复盘"、"总结"、"沉淀" |
| `design.md` | AI 设计新功能/新 kind | "加新分类"、"设计" |

### 修改现有工作流

直接编辑对应的 `skills/{flow}.md`。每个文件格式：

```markdown
---
name: enjoyknowledge-flow-{name}
description: 一句话描述
version: 1.0.0
---

# 工作流名称

## Purpose
这个工作流的目标

## When to use
- 触发条件 1
- 触发条件 2

## Step-by-step
1. 步骤 1
2. 步骤 2

## File Reading Order
1. 先读什么
2. 再读什么

## File Writing Order
1. 先写什么
2. 再写什么

## Common Patterns
- 常见模式 1
- 常见模式 2

## Pitfalls
- ❌ 不要做什么
```

### 添加新工作流

在 `.enjoyknowledge/skills/` 下创建新的 `.md` 文件，按上述格式编写即可。`ek init` 不覆盖用户创建的文件。

---

## 日常命令参考

### 查阅知识

```bash
ek ls              # 列出所有 kind 及其文件（带 description）
ek ls --bare       # 只列文件名
ek tree            # 递归目录树
ek cat gotcha/export.md        # 查看文件全文
ek grep "超时"                  # 全文搜索，定位到 ## 段
ek grep "超时" --type gotcha   # 限定 kind
ek grep "api" --tags backend   # 按 tag 过滤
```

### 记录知识

```bash
# 追加到已有文件（文件不存在则创建）
ek add gotcha/export.md "## 大数据量超时

- **Instance**: 导出超过 10 万行时接口超时
- **Workaround**: 分批导出，单次最多 1 万行"
```

### 任务暂存区（AI 编码流程）

```bash
# AI 写草稿到 stage
# → 写入 .enjoyknowledge_stage/drafts/{task}.md

# 人类审核后 promote 到知识库
ek promote drafts/{task}.md --to gotcha

# 清理过期归档
ek stage clean --dry-run     # 预览
ek stage clean --force       # 执行清理
ek stage clean --older-than 90  # 自定义 TTL
```

### 健康检查

```bash
ek doctor        # 5 项检查：frontmatter / 必填字段 / SoT 过期 / export 一致性 / kinds.md
ek doctor --ci   # CI 模式：warning 也非零退出
ek fix           # 自动修复可处理的问题
```

### 导出到 AI 工具

```bash
ek export --tool claude    # → .claude/skills/enjoyknowledge.md
ek export --tool cursor    # → .cursor/rules/enjoyknowledge.mdc
ek export --tool auto      # 自动检测
ek export --dry-run        # 预览不写
```

### 建立项目心智模型

```bash
ek onboard    # AI 首次进入项目时运行，输出项目概况
```

---

## 哲学

1. **filesystem 是唯一真相源** — 代码只提供 seed 默认值，用户改文件即改行为
2. **kind = dir** — 目录名即类型名，无需在 frontmatter 重复声明
3. **物理分离** — `.enjoyknowledge/`（人类写） vs `.enjoyknowledge_stage/`（AI 写）
4. **AGENTS.md > frontmatter** — AI 读 markdown 比解析 YAML 字段更直接
5. **简单 > 完整** — 能用的工具 > 完美的设计

---

## 文档索引

| 文档 | 内容 |
|---|---|
| [GLOSSARY.md](./docs/01-philosophy/GLOSSARY.md) | 术语表 |
| [DESIGN-PHILOSOPHY.md](./docs/01-philosophy/DESIGN-PHILOSOPHY.md) | 设计哲学 |
| [DESIGN.md](./docs/02-design/DESIGN.md) | 整体架构 |
| [INTERFACE-SPEC.md](./docs/02-design/INTERFACE-SPEC.md) | CLI 行为合约 |
| [ROADMAP.md](./docs/00-vision/ROADMAP.md) | 版本路线图 |
| [CHANGELOG.md](./docs/04-changelog/CHANGELOG.md) | 变更记录 |

## 许可

MIT
