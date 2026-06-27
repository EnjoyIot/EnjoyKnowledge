# 规则系统设计

> 版本: 1.0 | 2026-06-27 | 来源: v1+v2 整合
>
> **本文件覆盖 rule 单一 SoT + ACID sync + managed section 划界**。for Coding 整体设计见 [for-coding-design.md](./for-coding-design.md)

---

## 1. 核心问题

LLM 工具分散：每个 AI 工具（Cursor/Claude/Codex/...）的 rule 格式不同。如果每个工具各持一份 SoT，变更要同步改 9 处。任何遗漏都导致行为分歧。

## 2. 解决方案

**单一 SoT + 多入口同步**：
- SoT = `.enjoyknowledge/rules/<id>.md`（每个 rule 一文件）
- 9 工具入口由 `enjoyknowledge sync` 自动生成（不是手工维护）

## 3. Rule 文件结构

```yaml
---
id: rust-no-unwrap
applies_to: [rust, "glob:**/*.rs"]
priority: 4              # 1-5，1=低 5=高
type: gotcha              # convention / gotcha / invariant
tags: [rust, error-handling]
scope: project             # project / user / team
---

# 不要在生产代码用 .unwrap()
.unwrap() 只在 prototype 和测试中允许。
生产代码用 ? 或 expect("reason")。
```

**字段说明**：
- `id`：稳定 slug，全项目唯一
- `applies_to`：适用文件 glob 或语言（**最强过滤字段**）
- `priority`：冲突时谁赢（默认 3）
- `type`：rule 类型分类
- `scope`：作用范围
- `tags`：辅助搜索

## 4. 关键约束（v1 共识）

| 约束 | 阈值 | 处理 |
|---|---|---|
| 单条 rule 长度 | > 100 词 | warning |
| 总 ruleset 长度 | > 4000 词 | warning + 建议归档 |
| Rule 数量 | > 8 条 | warning（AI 记不住）|
| 渲染幂等性 | 二次渲染 diff 非空 | error（renderer bug） |

**`type` 字段 v1 共识**：
- convention（编码规范）
- gotcha（陷阱）
- invariant（不可违反约束）

**`priority` v1 共识**：1-5 整数，**5 = 最高优先级**（违反就出生产事故）。

## 5. 9 工具 sync 行为

| 工具 | 同步目标 | 文件格式 |
|---|---|---|
| Cursor | `.cursor/rules/<id>.mdc` | frontmatter (globs, alwaysApply) + markdown |
| Claude | `.claude/skills/<id>.md` | frontmatter (description) + markdown |
| Copilot | `.github/copilot-instructions.md` | append 到文件末尾 |
| Windsurf | `.windsurf/rules/<id>.md` | markdown |
| Cline | `.clinerules/<id>.md` | markdown |
| Codex | `.codex/prompts/<id>.md` | `$file:` 引用或 markdown |
| Trae | `.trae/rules/<id>.md` | markdown |
| Gemini | `GEMINI.md` | append 到文件末尾 |
| Generic | AGENTS.md | markdown |

**sync 命令**：
```bash
enjoyknowledge rule sync --tool claude      # 单工具
enjoyknowledge rule sync --tool all         # 全 9 工具
enjoyknowledge rule sync --dry-run          # 预览不写
```

## 6. Managed Section 划界（Copilot / Gemini / Generic）

这 3 个工具用"append 到已有文件"模式，需要划界：

```markdown
<!-- ek:managed:start -->
<!-- 以下由 enjoyknowledge rule sync 自动生成，请勿手改 -->
[渲染的 rule 内容]
<!-- ek:managed:end -->
```

**策略**：
- 段外：保留用户手改
- 段内：sync 时覆盖

## 7. 失败显式化（v4 哲学 #3）

| 情况 | 行为 |
|---|---|
| SoT 缺失 | **报错**——不用空模板 |
| 工具适配器未实现 | **报错**——不 fallback 到 Generic |
| 渲染输出 > 100 行 | **报错**——说明 rule 太多或太长 |
| 渲染幂等性失败 | **error**——renderer bug |

## 8. Rule-Code 同步检测（v3 F9 致命反模式）

**问题**：Rule 说"API 返回 `{data, error}`"但代码混用 `{result, msg}`——AI 按 rule 生成不一致的代码。

**缓解**：
- doctor 加 R-Code 一致性检查
- 检测方式：扫描 rule 提到的 API 格式 + grep 代码实际格式
- 失败 = warning（不能 hard fail，因为不是所有 rule 都能静态检查）

## 9. 与 Template/Knowledge 的关系

```
rule    → 约束（"禁止/必须"）
template → 范式（"怎么写"）—— rule 的具体化
knowledge → 上下文（"为什么"）—— 解释 rule 存在的原因
```

三者**不能互相包含**：
- rule 不写"为什么"（留给 knowledge）
- knowledge 不写"禁止"（留给 rule）
- template 不写"约束"（留给 rule）

如果发现 `rules/no-unwrap.md` 写了 API 错误类型 → 该拆进 `knowledge/api-design.md`。
如果发现 `knowledge/api-design.md` 写了"禁止在 handler 做 DB 查询" → 该拆进 `rules/`。

---

**关联文档**：
- [for-coding-design.md §3 3 机制协同](./for-coding-design.md)
- [knowledge-types.md](./knowledge-types.md)
- [workflows.md §4 步骤的 filter 用法](./workflows.md)
- [INTERFACE-SPEC.md](../INTERFACE-SPEC.md)
