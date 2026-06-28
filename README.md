# enjoyknowledge
未发布，产品讨论和设计阶段，当前代码全是demo.

<p align="center">
  <strong>通用知识资产引擎 + AI 编程知识应用</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/enjoyknowledge"><img src="https://img.shields.io/crates/v/enjoyknowledge" alt="Crates.io"></a>
  <a href="https://docs.rs/enjoyknowledge"><img src="https://img.shields.io/docsrs/enjoyknowledge" alt="Docs"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
</p>

enjoyknowledge 分为两层：

- **enjoyknowledge Core**：通用知识资产引擎，提供 OKF 兼容格式、`ls`/`grep`/`cat`/`add`/`doctor` 等知识管理原语，不绑定具体领域。
- **enjoyknowledge for Coding**：基于 Core 的第一个领域应用，面向 AI 编程场景，内置架构、踩坑、模式、业务规则、决策和任务暂存区。

当前 README 介绍的是 **Core + for Coding 默认应用**。后续可以在同一个 Core 上扩展 `for support`、`for research`、`for sales` 等其他应用。

## 一句话定位

> enjoyknowledge Core 让知识文件可被工程化管理；enjoyknowledge for Coding 让 AI 编程工具和开发者用同一份工程知识干活。

## 快速开始

```bash
# 安装
cargo install enjoyknowledge

# 在当前项目初始化
enjoyknowledge init

# 浏览知识库概况
enjoyknowledge ls

# 结构化搜索
enjoyknowledge grep "导出" --type Gotcha --tags excel

# 记录踩坑
enjoyknowledge add gotchas/export.md "## t_export_record 无 status 字段"

# 诊断知识库健康度
enjoyknowledge doctor
```

## 核心理念

| 问题 | enjoyknowledge 的答案 |
|---|---|
| AI 不知道项目架构 | AGENTS.md 推送 `ls` 摘要，`cat` 按需读取架构文件 |
| AI 重复踩已知坑 | `enjoyknowledge add` 沉淀踩坑，`grep` 定位到相关 `##` 段 |
| 换 AI 工具知识丢失 | 知识在文件系统，工具无关 |
| 团队知识孤岛 | `.enjoyknowledge/` 进 git，团队共享 |

## 架构

```
AI 编码工具 (Cursor/Copilot/Claude Code)
         │
         ▼
    AGENTS.md  ← 内嵌 enjoyknowledge ls 摘要
         │
         ▼
┌─ enjoyknowledge Core CLI ───────────────┐
│  init │ ls │ grep │ cat │ add │ doctor │ promote │ stage-clean │
└────────────────────────────────────┘
         │
         ▼
    for Coding 应用结构 (v0.4)
    项目根目录/
    ├── .enjoyknowledge/           ← 长期知识 SoT (人类编辑/审核)
    │   ├── architecture/          # 架构知识
    │   ├── gotchas/               # 踩坑记录
    │   ├── patterns/              # 最佳实践
    │   ├── business/              # 业务规则
    │   ├── decisions/             # 架构决策记录
    │   ├── index.md
    │   ├── log.md
    │   └── AGENTS.md              ← AI 入口 (人类写, AI 读)
    └── .enjoyknowledge_stage/     ← 短期任务工作区 (AI 自动写, 人类审核)
        ├── tasks/<task-id>/       # 8 文件: requirements/design/plan/changes/tests/delivery/summary/review
        ├── drafts/                # 待 promote 草稿
        ├── workflow/              # 工作流定义 (v0.2 保留)
        ├── .archive/              # TTL 过期 (默认 180 天)
        └── AGENTS.md              ← AI 任务写入规范 (AI 读)
```

## 文档

| 文档 | 内容 |
|---|---|
| 文档 | 内容 |
|---|---|
| [AGENTS.md](./AGENTS.md) | AI 入口（30 秒读懂项目）|
| [POSITIONING.md](./docs/00-vision/POSITIONING.md) | 定位宣言、竞品对比 |
| [DESIGN-PHILOSOPHY.md](./docs/01-philosophy/DESIGN-PHILOSOPHY.md) | 设计哲学：为什么存在、取舍原则 |
| [DESIGN.md](./docs/02-design/DESIGN.md) | 整体架构 + 4 个子文档入口 |
| [GLOSSARY.md](./docs/01-philosophy/GLOSSARY.md) | 术语表 |

## 贡献

欢迎贡献！详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 许可

MIT © enjoyknowledge Team
