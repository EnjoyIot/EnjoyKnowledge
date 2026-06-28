# enjoyknowledge 定位

> v0.4.2 | 2026-06-28

## 一句话定位

> **enjoyknowledge = 项目知识 SoT + AI 工具共享上下文层**。一份 markdown 写一次，多个 AI 工具使用。

| 关键词 | 含义 | 排除了什么 |
|---|---|---|
| **项目知识 SoT** | 架构、规则、决策、踩坑——项目级知识的唯一真值源 | 不管理个人笔记 / Notion / 通用 wiki |
| **AI 工具共享** | 一份 `.enjoyknowledge/` 供多个 AI 工具消费 | 不是单工具原生（Cursor rules / Claude skills 各自为政）|

**它不是什么**：
- 不是 AI 编码工具——Cursor / Copilot / Claude Code 是消费方
- 不是通用知识库——Notion / Confluence / Obsidian 是给人看的
- 不是 AI memory 系统——mem0 / MemOS 是 agent 内部记忆
- 不是 Spec 框架——spec-kit / OpenSpec 是项目规范流程

**它是什么**：
- 项目知识的工程化管理层——给 markdown 加 schema / doctor / fix
- 跨 AI 工具的路由层——一份 markdown → 多个 AI 工具入口
- 人类与 AI 的分工界面——KB 人类写、stage AI 写，物理目录分离

---

## 解决的 3 个痛点

| 痛点 | 表现 | enjoyknowledge 答案 |
|---|---|---|
| **AI 不知道项目架构** | AI 猜架构、重复踩已知坑 | AGENTS.md 推送 `ls` 摘要，`cat` 按需读取 |
| **换 AI 工具知识丢失** | `.cursor/rules` 和 `CLAUDE.md` 80% 重复 | 知识在 `.enjoyknowledge/` 文件系统，工具无关 |
| **任务临时文件无归处** | AI 写的设计/计划散落各处 | `.enjoyknowledge_stage/` 8 文件结构，`promote` 沉淀 |

---

## 差异化

| 维度 | enjoyknowledge | ECC (222K stars) | ai-rules-sync (124 stars) |
|---|---|---|---|
| **定位** | 项目知识 SoT + 多工具路由 | Agent harness 性能优化 | Rule 同步 |
| **工具支持** | 多工具（export 到 Claude + Cursor） | 部分 | 3-4 工具 |
| **知识 schema** | 10 类知识 + 必填字段校验 | 无 | 无 |
| **任务暂存区** | `.enjoyknowledge_stage/` 8 文件 + promote | 无 | 无 |
| **人类/AI 分离** | 物理目录分离（KB vs stage） | Agent 中心 | Rule 中心 |

---

## v0.4.2 极简上下文层

v0.4 在 v0.2 "一份 markdown 多个 AI 工具"之上，加 1 层"任务工作区"：

```
项目根/
├── .enjoyknowledge/         ← 长期知识 SoT（人类写，AI 只读）
└── .enjoyknowledge_stage/   ← 任务暂存区（AI 自动写，人类审核）
    ├── tasks/<task-id>/     # 8 文件：requirements/design/plan/changes/tests/delivery/summary/review
    ├── drafts/              # AI 草稿，人类 `ek promote` 后落地
    └── .archive/            # TTL 过期（默认 180 天）
```

**4 极简原则**（6 次反馈沉淀）：

1. **人类是 authority anchor** — KB 内容人类手动 / 显式让 AI 写
2. **物理分离 > 状态字段** — `.enjoyknowledge_stage/`（AI 写）vs `.enjoyknowledge/`（人类写）
3. **AGENTS.md > frontmatter** — AI 读 markdown 内容比 YAML 字段更直接
4. **简单 > 完整** — 能用的工具 > 完美的设计

**v0.4.2 砍掉的能力**（不预设 v0.5 恢复）：

- C10 trust 体系（confidence/source/last_verified/feedback_count）
- C11 lifecycle 4 状态机（draft/active/deprecated/archived）
- C12 sync 检测（3 类冲突 + 3 级频率）
- frontmatter 6 字段扩展
- `ek capture --from-commit` 提议门
- 独立 `workflow/` 目录（已并入 `stage/`）

---

## 与竞品的关系

enjoyknowledge 不替代任何 AI 工具。它定义项目知识纪律——像 CI 配置、lint 规则一样，成为项目基础设施的一部分。任何 AI 工具进入项目，知识库质量已被保障。

---

*关联文档：[ROADMAP.md](./ROADMAP.md) · [DESIGN-PHILOSOPHY.md](../01-philosophy/DESIGN-PHILOSOPHY.md) · [DESIGN.md](../02-design/DESIGN.md)*
