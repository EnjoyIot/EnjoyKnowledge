# 目录设计理由

> v0.4.2 | 2026-06-28

## 设计原则

| 原则 | 含义 |
|---|---|
| **目录名即分类** | 文件所在目录决定其概念类型，不依赖 frontmatter 重复声明 |
| **文件名自解释** | 文件名表达主题，不在文件名中重复目录名 |
| **深度 ≤ 2 层** | `.enjoyknowledge/` 内 `category/file.md`，不嵌套子目录 |
| **profile 可扩展** | 不同 profile（for-coding / for-design）可定义不同目录集 |

---

## `.enjoyknowledge/` 目录结构

```
.enjoyknowledge/
├── AGENTS.md          # KB 写入规则（人类写，AI 读）
├── index.md           # 索引文件
├── log.md             # 操作日志
├── architecture/      # 系统结构
├── gotchas/           # 踩坑记录
├── patterns/          # 最佳实践
├── rules/             # 强制规则
├── decisions/         # 架构决策记录
├── business/          # 业务规则
├── contracts/         # 接口契约
├── conventions/       # 命名/格式约定
├── context/           # 项目背景/运行时
└── templates/         # 范式模板
```

**10 类知识资产 = 10 个目录**，一一对应。

---

## `.enjoyknowledge_stage/` 目录结构

```
.enjoyknowledge_stage/
├── AGENTS.md          # 任务写入规范（AI 读）
├── tasks/_template/   # 8 文件模板
│   ├── requirements.md
│   ├── design.md
│   ├── plan.md
│   ├── changes.md
│   ├── tests.md
│   ├── delivery.md
│   ├── summary.md
│   └── review.md
├── drafts/            # AI 草稿，人类 promote 后落地
├── .archive/          # TTL 过期（默认 180 天）
└── workflow/          # v0.2 工作流 YAML（onboard.yaml / capture.yaml）
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

## kind → 目录映射

`default_path_for_kind`（`src/cli/workflow.rs:381`）负责 kind 到目录+文件的映射：

| kind | 目录 | 文件名 |
|---|---|---|
| gotcha | gotchas/ | gotchas.md |
| decision | decisions/ | decisions.md |
| pattern | patterns/ | patterns.md |
| rule | rules/ | rules.md |
| architecture | architecture/ | architecture.md |
| business | business/ | business.md |
| contract | contracts/ | contracts.md |
| convention | conventions/ | conventions.md |
| context | context/ | context.md |
| template | templates/ | templates.md |

**注意**：前 4 类 kind 名与目录名不同（gotcha→gotchas, decision→decisions, pattern→patterns, rule→rules），后 6 类 kind 名 = 目录名。

---

*关联文档：[DESIGN.md](../02-design/DESIGN.md) · [knowledge-types.md](../02-design/architecture/knowledge-types.md) · [GLOSSARY.md](../01-philosophy/GLOSSARY.md)*
