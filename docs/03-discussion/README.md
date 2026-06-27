# 讨论历史

本目录是 enjoyknowledge 设计过程的讨论记录。按时间归档。

| 日期 | 主题 | 来源 |
|---|---|---|
| 2026-06-27 | rule 核心设计（v1） | [rule-core.md](./2026-06-27-rule-core.md) |
| 2026-06-27 | 3 机制协同设计（v2）| [3mechanisms.md](./2026-06-27-3mechanisms.md) |
| 2026-06-27 | for Coding 深度设计（v3）| [for-coding-deep.md](./2026-06-27-for-coding-deep.md) |
| 2026-06-27 | for Coding 完整设计（v4）| [for-coding-complete.md](./2026-06-27-for-coding-complete.md) |
| 2026-06-27 | 文档整理（本文档）| 整理记录 |

## 演进关系

```
v1 (rule 核心)     →  v2 (3 机制协同)     →  v3 (for Coding 深度)     →  v4 (完整设计)
       │                    │                       │                       │
       └─ rule 单一 SoT      └─ rule/template/        └─ 6 场景 + 5 工作流      └─ 修正入口假设
          + ACID sync         knowledge 三角          + 8 类知识               + 9 工具适配
          + managed section                            + MVP 边界              + YAML 元数据
```

v4 是当前基线（2026-06-27）。
