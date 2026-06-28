# enjoyknowledge

> AI 时代的工程知识资产管理层 | 当前阶段: **v0.2 收尾**（2026-06-27，文档层面就绪，代码 export 命令待实施）

## Quick links

| 用途 | 文档 |
|---|---|
| 了解产品是什么 | [00-vision/POSITIONING.md](./docs/00-vision/POSITIONING.md) |
| 了解产品方向 | [00-vision/ROADMAP.md](./docs/00-vision/ROADMAP.md) |
| 了解设计原则 | [01-philosophy/DESIGN-PHILOSOPHY.md](./docs/01-philosophy/DESIGN-PHILOSOPHY.md) |
| 了解术语 | [01-philosophy/GLOSSARY.md](./docs/01-philosophy/GLOSSARY.md) |
| 了解整体设计 | [02-design/DESIGN.md](./docs/02-design/DESIGN.md) |
| 了解 for Coding 设计 | [02-design/architecture/for-coding-design.md](./docs/02-design/architecture/for-coding-design.md) |
| 了解规则系统 | [02-design/architecture/rule-system.md](./docs/02-design/architecture/rule-system.md) |
| 了解知识类型 | [02-design/architecture/knowledge-types.md](./docs/02-design/architecture/knowledge-types.md) |
| 了解工作流 | [02-design/architecture/workflows.md](./docs/02-design/architecture/workflows.md) |
| 了解接口规范 | [02-design/INTERFACE-SPEC.md](./docs/02-design/INTERFACE-SPEC.md) |
| 了解历史决策 | [03-discussion/](./docs/03-discussion/) |
| 了解变更 | [04-changelog/CHANGELOG.md](./docs/04-changelog/CHANGELOG.md) |

## Top 3 things to know

1. **3 机制协同**: rule（约束）+ template（范式）+ knowledge（上下文）—— 详见 [for-coding-design.md §3](./docs/02-design/architecture/for-coding-design.md)
2. **v0.2 多工具入口**（首发 Claude + Cursor）—— 详见 [DESIGN.md §6](./docs/02-design/DESIGN.md)
3. **元数据驱动工作流**: 工作流 = YAML 文件，不实现引擎 —— 详见 [workflows.md](./docs/02-design/architecture/workflows.md)

## Current status

- **阶段**: v0.2 收尾 + v0.2.1 工作流落地 + doctor 4 项重写（设计基线 v4.2，2026-06-27；workflow 实现 2026-06-28）
- **代码**: Rust CLI，10 命令（init/ls/tree/grep/cat/add/doctor/fix/export/workflow）—— 9 个已实现 + workflow 已实现 onboard + capture + doctor 4 项新 check
- **下次重点**: 任务 C（修老债 P0-P3：UTF-8 编码 / from_str 测试 / ls.rs 拆分）

## How to use this repo

- **新 AI 接手**: 读 [DESIGN.md](./docs/02-design/DESIGN.md) + [for-coding-design.md](./docs/02-design/architecture/for-coding-design.md)
- **新开发者**: 读 [POSITIONING.md](./docs/00-vision/POSITIONING.md) + [DESIGN-PHILOSOPHY.md](./docs/01-philosophy/DESIGN-PHILOSOPHY.md)
- **回顾决策**: 读 [03-discussion/](./docs/03-discussion/)（4 轮讨论历史）
- **历史包袱**: 读 [99-archive/](./docs/99-archive/)（codex 副作用归档）
