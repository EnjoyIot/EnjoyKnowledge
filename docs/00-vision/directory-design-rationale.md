# 目录设计理由

> v0.4.10 | 2026-06-29

## 设计原则

| 原则 | 含义 |
|---|---|
| **目录名即分类** | 文件所在目录决定其概念类型，不依赖 frontmatter 重复声明 |
| **kind = dir** | 目录名 = kind 名，无 "s" 派生（v0.4.3 回归 v0.1 哲学） |
| **文件名自解释** | 文件名表达主题，不在文件名中重复目录名 |
| **深度 ≤ 2 层** | `.enjoyknowledge/` 内 `kind/file.md`，不嵌套子目录 |
| **物理分离** | `.enjoyknowledge/`（人类写）与 `.enjoyknowledge_stage/`（AI 写）严格物理分离 |

---

## `.enjoyknowledge/` 目录结构

```
.enjoyknowledge/
├── _meta/
│   └── kinds.md         # kind 注册表（单一真相源，用户可编辑）
├── AGENTS.md            # KB 写入规则（Hermes skill 格式，用户拥有）
├── index.md             # 索引文件
├── skills/              # 工作流 skill 文件（coding/research/review/design）
├── architecture/        # 系统结构
├── business/            # 业务规则
├── command/             # CLI 命令文档
├── context/             # 项目背景/运行时
├── contract/            # 接口契约
├── convention/          # 命名/格式约定
├── decision/            # 架构决策记录
├── gotcha/              # 踩坑记录
├── pattern/             # 最佳实践
├── rule/                # 强制规则
└── template/            # 范式模板
```

**11 类知识资产 = 11 个目录**，目录名 = kind 名（v0.4.3 去 "s" 复数）。

---

## `.enjoyknowledge_stage/` 目录结构

```
.enjoyknowledge_stage/
├── _meta/
│   └── stage-defaults.md   # 默认 stage 目录清单（用户可编辑）
├── AGENTS.md               # 任务写入规范（Hermes skill 格式，用户拥有）
├── tasks/_template/        # 8 文件模板
│   ├── summary.md
│   ├── requirements.md
│   ├── design.md
│   ├── plan.md
│   ├── changes.md
│   ├── tests.md
│   ├── delivery.md
│   └── review.md
├── drafts/                 # AI 草稿，人类 promote 后落地
└── .archive/               # TTL 过期（默认 180 天）
```

---

## 物理分离：KB vs Stage

v0.4 核心设计决策——用物理目录区分两类内容：

| 目录 | 写入者 | 审核 | 生命周期 |
|---|---|---|---|
| `.enjoyknowledge/` | 人类（或人类显式要求 AI） | 人类 | 长期 |
| `.enjoyknowledge_stage/` | AI 自动 | 人类 promote 后落地 | 短期（180 天 TTL） |

**为什么不靠 frontmatter 字段区分？** 物理分离比状态字段更直观——AI 一看路径就知道能不能写，不需要读 YAML 再判断。

---

## kind 注册表（kinds.md）

`.enjoyknowledge/_meta/kinds.md` 是 kind → dir 映射的单一真相源。Markdown 表格格式，人类可读可编辑。

**v0.4.10 关键设计**：代码不再硬编码任何 kind 名或必填字段。`ek doctor` 的 `check_required_fields` 动态从 kinds.md 读取，用户添加/修改 required 字段后 doctor 自动校验新规则。

管理命令：`ek kind add/rm/list`（v0.4.5）。

---

*关联文档：[DESIGN.md](../02-design/DESIGN.md) · [knowledge-types.md](../02-design/architecture/knowledge-types.md) · [GLOSSARY.md](../01-philosophy/GLOSSARY.md)*
