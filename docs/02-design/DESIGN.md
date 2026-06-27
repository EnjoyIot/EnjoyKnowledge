# enjoyknowledge 设计 — 整体架构

> 版本: 1.0 | 2026-06-27
>
> **本文件是整体设计的入口**。详细设计见各子目录：
>
> - [architecture/for-coding-design.md](./architecture/for-coding-design.md) — for Coding 完整设计（v4）
> - [architecture/rule-system.md](./architecture/rule-system.md) — 规则系统（v1+v2）
> - [architecture/knowledge-types.md](./architecture/knowledge-types.md) — 知识类型（v3）
> - [architecture/workflows.md](./architecture/workflows.md) — 工作流（v4）
> - [INTERFACE-SPEC.md](./INTERFACE-SPEC.md) — 接口规范
>
|> **v3 内容已合并到本文**。v3/PRODUCT-DESIGN 原始版本已归档（见 git history `5632dd4`）。

---

## 1. 产品愿景

**enjoyknowledge** = AI 时代的工程知识资产管理层。给知识文件加上工程化治理（格式校验、过期检测、结构化检索、自动化推送）。

**3 个关键词**：
| 关键词 | 含义 | 排除了什么 |
|---|---|---|
| **知识资产** | 架构、决策、踩坑、业务规则——代码之外的结构化知识 | 不管理代码本身 |
| **工程化** | 格式校验、过期检测、结构化检索、自动化推送 | 不是手写 Markdown 的松散约定 |
| **管理层** | 在文件系统之上提供索引、检索、健康检查 | 不是取代 Git / 编辑器 / AI 工具 |

详细定位见 [00-vision/POSITIONING.md](../00-vision/POSITIONING.md)。

## 2. 两层架构

```
enjoyknowledge Core        ← 通用知识引擎
  ↓ profile
enjoyknowledge for X       ← 领域应用（for Coding 是默认）
```

| 层 | 职责 | 当前内容 |
|---|---|---|
| **Core** | 知识文件的格式、索引、检索、写入、健康检查 | OKF frontmatter、`ls`、`grep`、`cat`、`add`、`doctor` |
| **for Coding** | AI 编程场景的目录、任务暂存区、AI 入口 | `architecture/`、`gotchas/`、`patterns/`、`business/`、`decisions/`、`contracts/`、`conventions/`、`context/`、AGENTS.md |

## 3. 3 机制协同（核心架构）

```
        Rule（约束层）—— 必须遵守
            ↓ 规定边界
        Template（范式层）—— 推荐方式
            ↓ 演示怎么用
        Knowledge（上下文层）—— 项目事实
            ↓ 解释为什么用
```

详细规则见 [architecture/rule-system.md](./architecture/rule-system.md)。

## 4. 设计原则

| 原则 | 含义 |
|---|---|
| **SoT 单一** | `.enjoyknowledge/` 是唯一真值源 |
| **入口多元** | v0.2 首发 2 个 AI 工具各自的入口文件（Claude skills + Cursor mdc） |
| **元数据驱动** | 工作流 = YAML 文件，不实现引擎 |
| **显式失败** | 不静默降级——缺失/未实现直接报错 |
| **工具特性保留** | Cursor `.mdc` / Claude `@path` / Codex `$file` 不强制统一 |

## 5. v0.2 2 个工作流（onboard + capture）

| 工作流 | 触发 | 价值 |
|---|---|---|
| **onboard** | AI 工具启动 | 30 秒建立项目心智模型 |
| **capture** | 用户/AI 主动 | 沉淀知识 |
| **export** | AI 工具切换 | export 到 v0.2 首发 2 工具 |

详细见 [architecture/workflows.md](./architecture/workflows.md)。

## 6. v0.2 多工具支持（首发 2 工具 = Claude + Cursor）

> **v0.2 决定**：首发 2 工具（Claude + Cursor）；架构上保留 9 工具 adapter trait（Codex / Copilot / Windsurf / Cline / Trae / Gemini / Generic），v0.3+ 渐进。详细见 [architecture/rule-system.md §5](./architecture/rule-system.md) 和 [architecture/for-coding-design.md](./architecture/for-coding-design.md)。

| 工具 | 入口 |
|---|---|
| Cursor | `.cursor/rules/enjoyknowledge.mdc` |
| Claude | `.claude/skills/enjoyknowledge.md` |
| Copilot | `.github/copilot-instructions.md` (append) |
| Windsurf | `.windsurf/rules/enjoyknowledge.md` |
| Cline | `.clinerules/enjoyknowledge.md` |
| Codex | `.codex/prompts/enjoyknowledge.md` |
| Trae | `.trae/rules/enjoyknowledge.md` |
| Gemini | `GEMINI.md` (append) |
| Generic | AGENTS.md |

详细见 [INTERFACE-SPEC.md](./INTERFACE-SPEC.md)。

## 7. MVP 边界（v0.1）

**必含 6 项**：
1. `add` + 路由（自动判断 gotcha/pattern/decision/...）
2. `search` + frontmatter filter
3. **多工具入口生成**（v0.2 首发 2 工具 = Claude + Cursor；架构上保留 9 工具 adapter trait，v0.3+ 渐进）
4. `doctor` 3 项基础（frontmatter 有效 / 体积上限 / 链接完整）
5. 4000 词硬上限 + 100 词单条
6. frontmatter 必填校验

**应包含但可简单做 5 项**：见 [architecture/for-coding-design.md §8.2](./architecture/for-coding-design.md)

**永不做 5 项**：
1. AI 自动生成 gotcha
2. LLM 扩写 knowledge
3. 知识"质量评分"
4. 在线知识库托管
5. 协作编辑 / 实时同步（Git 解决）

## 8. 详细设计

- **for Coding** → [architecture/for-coding-design.md](./architecture/for-coding-design.md)
- **规则系统** → [architecture/rule-system.md](./architecture/rule-system.md)
- **知识类型** → [architecture/knowledge-types.md](./architecture/knowledge-types.md)
- **工作流** → [architecture/workflows.md](./architecture/workflows.md)
- **接口规范** → [INTERFACE-SPEC.md](./INTERFACE-SPEC.md)
- **历史讨论** → [../03-discussion/](../03-discussion/)

## 9. 演进历史

| 版本 | 日期 | 关键变化 |
|---|---|---|
| v0.1.0 | 2026-06-22 | 初始 5 命令（init/search/record/doctor/fix） |
| v0.1.1 | 2026-06-26 | codex 副作用文档（已归档） |
| v1.0 设计基线 | 2026-06-27 | 4 轮讨论整合（rule/template/knowledge 协同 + 9 工具入口 + 元数据驱动工作流） |

完整讨论记录见 [03-discussion/](../03-discussion/)。
