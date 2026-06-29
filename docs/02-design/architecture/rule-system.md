# 规则系统

> v0.4.10 | 2026-06-29
>
> 定义 rule 的单一 SoT + export 行为。

---

## 1. 核心问题

LLM 工具分散：Cursor / Claude / Codex 的 rule 格式不同。如果每个工具各持一份 SoT，变更要同步改多处，遗漏即行为分歧。

## 2. 解决方案

**单一 SoT + export**：
- SoT = `.enjoyknowledge/rule/<id>.md`（v0.4.3：kind=dir，无 "s" 派生）
- v0.2 首发 Claude + Cursor 入口由 `enjoyknowledge export` 自动生成
- 其他工具 adapter trait 架构保留，v0.3+ 渐进

---

## 3. Rule 文件格式

```yaml
---
id: rust-no-unwrap
applies_to: ["*.rs"]
tags: [rust, error-handling]
---

# 不要在生产代码用 .unwrap()
.unwrap() 只在 prototype 和测试中允许。
生产代码用 ? 或 expect("reason")。
```

| 字段 | 必需 | 说明 |
|---|---|---|
| `id` | 推荐 | 稳定 slug，全项目唯一 |
| `applies_to` | **必填** | glob 或语言，指定适用范围 |
| `tags` | 选填 | 辅助搜索 |

---

## 4. 约束

| 约束 | 阈值 | 处理 |
|---|---|---|
| 单条长度 | > 100 词 | warning |
| 总 ruleset | > 4000 词 | warning + 建议归档 |
| Rule 数量 | > 8 条 | warning（AI 记不住） |

---

## 5. export 行为

```bash
enjoyknowledge export --tool claude     # → .claude/skills/enjoyknowledge.md
enjoyknowledge export --tool cursor    # → .cursor/rules/enjoyknowledge.mdc
enjoyknowledge export --tool auto      # 自动检测
enjoyknowledge export --dry-run        # 预览不写
```

| 工具 | 目标文件 | 格式 |
|---|---|---|
| Claude | `.claude/skills/enjoyknowledge.md` | frontmatter (description) + Markdown |
| Cursor | `.cursor/rules/enjoyknowledge.mdc` | frontmatter (globs, alwaysApply) + Markdown |

其他 7 工具（Copilot / Windsurf / Cline / Codex / Trae / Gemini / Generic）：adapter trait 架构保留，v0.3+ 渐进。

---

## 6. 失败显式化

| 情况 | 行为 |
|---|---|
| SoT 缺失 | 报错，不用空模板 |
| 工具适配器未实现 | 报错，不 fallback 到 Generic |
| 渲染输出 > 100 行 | 报错 |

---

## 7. 与 knowledge 的关系

```
rule      → 约束（"禁止/必须"）
knowledge → 上下文（"为什么"）—— 解释 rule 存在的原因
```

两者不互相包含：rule 不写"为什么"，knowledge 不写"禁止"。

---

*关联文档：[knowledge-types.md](./knowledge-types.md) · [INTERFACE-SPEC.md](../INTERFACE-SPEC.md)*
