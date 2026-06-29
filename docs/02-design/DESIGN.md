# enjoyknowledge 设计

> v0.4.10 | 2026-06-29
>
> 整体架构入口。详细设计见子文档。

---

## 1. 产品定位

**enjoyknowledge = AI 编程工具的共享上下文层**。给知识文件加工程化治理（格式校验、过期检测、结构化检索、自动化推送）。

详细定位见 [POSITIONING.md](../00-vision/POSITIONING.md)。

---

## 2. 两层架构

```
enjoyknowledge Core        ← 通用知识引擎
  ↓ profile
enjoyknowledge for Coding   ← 领域应用（默认）
```

| 层 | 职责 | 当前内容 |
|---|---|---|
| **Core** | 知识文件的格式、索引、检索、写入、健康检查 | 14 个 CLI 命令 |
| **for Coding** | AI 编程场景的目录、任务暂存区、AI 入口 | 11 类知识资产 + stage + export |

---

## 3. 目录结构

```
项目根/
├── AGENTS.md                      # AI 入口（内嵌 ls 摘要）
├── .enjoyknowledge/               # 长期知识 SoT（人类编辑/审核）
│   ├── _meta/kinds.md              # kind 注册表（Markdown 表格）
│   ├── skills/                    # 工作流 skill 文件
│   ├── architecture/
│   ├── gotcha/
│   ├── pattern/
│   ├── rule/
│   ├── decision/
│   ├── business/
│   ├── contract/
│   ├── convention/
│   ├── context/
│   ├── command/
│   ├── template/
│   ├── index.md
│   └── AGENTS.md                   # KB 写入规则
└── .enjoyknowledge_stage/          # 任务暂存区（AI 自动写）
    ├── _meta/stage-defaults.md       # 默认目录清单（用户可编辑）
    ├── tasks/<task-id>/            # 8 文件
    ├── drafts/                     # 待 promote 草稿
    ├── .archive/                   # TTL 过期（180 天）
    └── AGENTS.md                   # 任务写入规范
```

详细目录设计理由见 [directory-design-rationale.md](../00-vision/directory-design-rationale.md)。

---

## 4. 设计原则

| 原则 | 含义 |
|---|---|
| **SoT 单一** | `.enjoyknowledge/` 是唯一真值源 |
| **入口多元** | export 生成 Claude + Cursor 各自的入口文件 |
| **元数据驱动** | kind 注册表 = Markdown 表格，加新 kind 不需要改 Rust 代码 |
| **kind = dir** | 目录名即 kind 名，无 "s" 派生 |
| **物理分离** | stage（AI 写）与 KB（人类写）用物理目录区分 |

---

## 5. Kind 注册表（v0.4.3）

v0.4.3 引入 `.enjoyknowledge/_meta/kinds.md` —— Markdown 表格格式的 kind 清单。

```
| kind | required | summary |
|------|----------|---------|
| gotcha | trigger | Pitfall / anti-pattern |
| decision | reversible, decided_at | Decision with reversibility |
| pattern | applies_to | Reusable code/design pattern |
| rule | applies_to | Project-wide rule |
| business | applies_to | Domain concept |
| architecture | applies_to | System architecture |
| contract | applies_to | API / data contract |
| convention | applies_to | Team convention |
| context | applies_to | Runtime context |
| template | applies_to | Reusable template |
| command | applies_to | CLI command documentation |
```

**设计原则**：
- **单一真相源**：`kinds::all()` 解析 `kinds.md`，所有 kind→dir 映射从此派生
- **kind = dir**：`kinds::dir_for("gotcha")` = `"gotcha"`（无 "s" 派生）
- **人类可编辑**：Markdown 格式，任何人都能读懂、修改
- **doctor 校验**：检查 `kinds.md` 与代码注册表一致

---

## 5. CLI 命令（13 个）

| 命令 | 用途 |
|---|---|
| `init` | 初始化知识库骨架 |
| `ls` | 列表（带 description） |
| `tree` | 递归目录树 |
| `cat` | 读取文件 |
| `grep` | 结构化搜索（定位到 `##` 段） |
| `add` | 追加/创建知识条目 |
| `doctor` | 5 项健康检查 |
| `fix` | 自动修复可处理问题 |
| `export` | 生成 AI 工具入口文件（Claude + Cursor） |
| `kind` | 管理知识种类（add/rm/list，v0.4.5） |
| `promote` | draft → KB（v0.4） |
| `stage clean` | TTL 清理（v0.4） |

完整接口合约见 [INTERFACE-SPEC.md](./INTERFACE-SPEC.md)。

---

## 6. 详细设计入口

| 文档 | 内容 |
|---|---|
| [architecture/knowledge-types.md](./architecture/knowledge-types.md) | 11 类知识资产 + 必填字段 |
| [architecture/rule-system.md](./architecture/rule-system.md) | 规则系统 + export 行为 |
| [architecture/for-coding-design.md](./architecture/for-coding-design.md) | for Coding 完整设计 |
| [INTERFACE-SPEC.md](./INTERFACE-SPEC.md) | CLI 行为合约 + 错误码 |
| [../01-philosophy/GLOSSARY.md](../01-philosophy/GLOSSARY.md) | 统一术语 |
| [../01-philosophy/DESIGN-PHILOSOPHY.md](../01-philosophy/DESIGN-PHILOSOPHY.md) | 设计哲学 |
| [../00-vision/POSITIONING.md](../00-vision/POSITIONING.md) | 定位宣言 |

---

*关联文档：[ROADMAP.md](../00-vision/ROADMAP.md) · [CHANGELOG.md](../04-changelog/CHANGELOG.md)*
