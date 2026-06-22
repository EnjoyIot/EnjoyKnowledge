# AI 工程化生态市场调研 - 三轮追加 (2026-06-19)

> **本文件是 `MARKET-RESEARCH-2026Q2.md` 和 `MARKET-RESEARCH-ADDENDUM-2026-06-19.md` 的第二次追加**。
>
> 一轮：spec-driven / context engineering / skills framework 大方向
> 二轮：AI coding agent persistent memory 赛道深挖
> **三轮：接口协议哲学的参照系（MCP / OpenAPI / LSP / Agent Skills 等成功案例）**

---

## 0. 元信息

| 项 | 值 |
|---|---|
| 日期 | 2026-06-19 |
| 触发原因 | Jay 把定位升级为 v4 "接口规范 + 默认实现"哲学，需要找真实参照系 |
| 调研范围 | 已成功的接口协议案例 + 2026 年 AI 领域 interface-driven 项目 |
| 方法 | HN Algolia + GitHub API + 各项目一手 README + 规范站 |
| 不下结论 | 只记录事实，定位判断留给用户 |

---

## 1. 历史级接口协议成功案例（已验证的"接口哲学"）

| 协议 | Stars | 创建 | 现状 |
|---|---|---|---|
| **OpenAPI Specification** | 31,039 | 2014-03 | **12 年**，REST API 事实标准 |
| **Language Server Protocol (LSP)** | 12,884 | 2015-09 | **11 年**，VSCode/IDE 标配 |
| **Model Context Protocol (MCP)** | 8,438 + **87,458 servers** | 2024-09 | **不到 2 年**，AI 工具标配 |

**关键观察**：
- 接口协议的成功模式都是：**少数人定义规范 → 社区大规模实现**
- MCP **2 年**就做出 OpenAPI **12 年**的影响力——AI 时代的接口协议速度更快
- EnjoyFlow v4 的"接口规范 + 默认实现"哲学**完全对标**这个成功模式

---

## 2. MCP 详解（最直接对位）

### 2.1 数据

| 指标 | 数值 |
|---|---|
| **MCP 协议本体** | 8,438 stars |
| **MCP Servers（实现集）** | **87,458 stars** |
| 创建日期 | 2024-09-24 |
| HN 高信号 | "Mcp-Agent" 80 点 / "Will Anthropic's MCP succeed" 4 cmt |

### 2.2 成功模式

- **Anthropic 出接口规范** → 社区出 **87K stars** 的实现
- 不到 2 年成为 AI 工具**事实标准**
- OpenAI / Google / IDE 厂商全在跟

### 2.3 EnjoyFlow v4 跟 MCP 的对位

| 维度 | MCP | EnjoyFlow v4 |
|---|---|---|
| 接口规范 | modelcontextprotocol.io | docs/POSITIONING.md + INTERFACE-SPEC.md (待写) |
| 实现集 | mcp-servers 87K stars | adapters/ (待设计) |
| Plugin 市场 | `/plugin install xxx@marketplace` | 计划同上 |
| 跨工具 | 各 AI 工具原生支持 | 计划 ≥4 AI 工具 |
| 文档站 | modelcontextprotocol.io | 待建 |

**关键**：MCP 是 EnjoyFlow v4 哲学的**最直接对位真实案例**。Anthropic 已经把这条路跑通了。

---

## 3. Agent Skills 规范（最相关参照系）

### 3.1 数据

| 指标 | 数值 |
|---|---|
| **anthropics/skills 仓库** | **152,856 stars** |
| 规范站 | **agentskills.io** |
| Plugin marketplace | `/plugin marketplace add anthropics/skills` |

### 3.2 规范结构

```
skill-name/
├── SKILL.md              # 必须：metadata + instructions
├── scripts/              # 可选：可执行代码
├── references/           # 可选：文档
├── assets/               # 可选：模板、资源
└── ...

SKILL.md:
---
name: <必填，≤64 chars>
description: <必填>
license: <可选>
compatibility: <可选>
metadata: <可选>
allowed-tools: <可选>
---

# Markdown body...
```

### 3.3 核心设计理念

- **Progressive disclosure**（渐进披露）—— Anthropic 的核心设计理念
- **YAML frontmatter + Markdown body** —— 既是机器可解析，也是人可读
- **Plugin 市场一键装** —— `/plugin install xxx@marketplace`

### 3.4 跟EnjoyFlow v4 的对位

| 维度 | Agent Skills | EnjoyFlow v4 |
|---|---|---|
| 接口规范 | agentskills.io/specification | INTERFACE-SPEC.md (待写) |
| 单文件 | SKILL.md | 应该是 enjoyflow.yaml |
| Frontmatter | YAML | 同上 |
| Progressive disclosure | ✅ 核心 | 待设计 |
| 官方示例 | anthropics/skills 仓库 | 默认实现 |
| Plugin 市场 | Claude Code 官方 | 计划同上 |

**关键**：EnjoyFlow v4 应该**直接采用** Agent Skills 的渐进披露设计。

---

## 4. 其他成功接口协议 / 格式规范

### 4.1 AGENTS.md —— 22,348 stars

**一手描述**:
> "AGENTS.md is a simple, open format for guiding coding agents"
>
> "Think of AGENTS.md as a **README for agents**: a dedicated, predictable place to provide context and instructions"

**真实形态**：
```markdown
# Sample AGENTS.md file

## Dev environment tips
- Use `pnpm dlx turbo run where <project_name>`...

## Testing instructions
- Find the CI plan in the .github/workflows folder...

## PR instructions
- Title format: [<project_name>] <Title>
```

**跟EnjoyFlow v4 的对位**：
- AGENTS.md 是**单文件**接口规范
- EnjoyFlow v4 是**多 API**（6 类）
- EnjoyFlow 的差异化：覆盖**文档、记忆、制品、翻译器**多个抽象，不只是 agent instructions

### 4.2 DESIGN.md —— 16,000 stars（Google Labs）

**一手描述**:
> "A format specification for describing a visual identity to coding agents. DESIGN.md gives agents a persistent, structured understanding of a design system"

**真实形态**：
```md
---
name: Heritage
colors:
  primary: "#1A1C1E"
  secondary: "#6C7278"
  tertiary: "#B8422E"
typography:
  h1:
    fontFamily: Public Sans
    fontSize: 3rem
---

# Markdown prose...
```

**核心创新**：
> "A DESIGN.md file combines **machine-readable design tokens (YAML front matter) with human-readable design rationale (markdown prose)**. Tokens give agents exact values. Prose tells them *why* those values exist and how to apply them."

**EnjoyFlow v3 "双向翻译"哲学的真实范式**：
- YAML frontmatter = 机器视图
- Markdown body = 人读视图
- 一个文件，两种呈现

**EnjoyFlow 可以直接采用这个模式**——把 PRD/Contract/Design Plan 都设计成 YAML frontmatter + Markdown body 双层结构。

### 4.3 planning-with-files —— 23,631 stars

**一手描述**:
> "Persistent file-based planning for AI coding agents and long-running agentic tasks"
>
> "It keeps `task_plan.md`, `findings.md`, and `progress.md` on disk"
>
> "It installs across **60+ agents via the SKILL.md standard**"

**跟EnjoyFlow 的对位**：
- planning-with-files 用 SKILL.md 跨 60+ agent 装 = EnjoyFlow v4 的 plugin 机制原型
- progress.md 跟EnjoyFlow 的进度账本同源

---

## 5. Memory Interface 赛道扫描

### 5.1 数据

| 项目 | Stars | 定位 |
|---|---|---|
| **mem0ai/mem0** | **58,941** | "Universal memory layer for AI Agents" |
| **volcengine/OpenViking** | 25,830 | "open-source context database designed specifically for AI Agents" |
| **deepset-ai/haystack** | 25,614 | "context-engineered, production-ready LLM apps" |
| **letta-ai/letta** | 23,416 | "stateful agents: AI with advanced memory" |
| **cognee** | 17,910 | "open-source AI memory platform for agents" |
| **MemTensor/MemOS** | 9,928 | "Self-evolving memory OS" |

### 5.2 mem0 = "Universal memory layer"（关键发现）

**mem0 自称 "Universal memory layer"**——这就是EnjoyFlow v4 想要的 MemoryAPI 的**现成参照**。

**关键洞察**：
- mem0 是**接口**（universal memory layer），不是 single memory backend
- 跟EnjoyFlow v4 的 MemoryAPI **同构**
- EnjoyFlow 不需要做 universal memory layer——但需要**接** mem0 作为可选适配器

### 5.3 haystack / letta / cognee / MemOS

这些是**接口层下的实现**——EnjoyFlow 可以作为它们的上游接口规范。

---

## 6. Document / Artifact Spec 空白带

### 6.1 搜索结果

`document spec AI agent` 搜索：
- 最高 36 stars（OpenSpec - 企业文档生成，跟接口规范无关）
- 0 个 protocol / interface / spec 类项目

`artifact spec AI agent` 搜索：
- 最高 34 stars（ara-registry/spec - AI 制品 registry）
- 0 个 protocol / interface 类项目

### 6.2 关键发现

**两个EnjoyFlow v4 真正独有的空白**：

| 接口 | 谁在做 | EnjoyFlow 的机会 |
|---|---|---|
| **DocAPI** | ❌ 没人做 | ✅ 真实空白 |
| **ArtifactAPI** | ❌ 没人做 | ✅ 真实空白 |
| MemoryAPI | mem0 在做 | 接 mem0 |
| SyncAPI | 没人做明确接口 | ✅ 部分空白 |
| HookAPI | Claude Code 自有 | 接各 AI 工具 |
| PluginAPI | MCP/Agent Skills 在做 | 接它们 |

---

## 7. EnjoyFlow 生态位（基于三轮调研）

### 7.1 EnjoyFlow 在大生态里的位置

```
┌─────────────────────────────────────────────────────┐
│              已被验证的接口协议                       │
│  - MCP (87K)              - OpenAPI (31K)         │
│  - LSP (13K)              - Agent Skills (153K)   │
│  - DESIGN.md (16K)        - AGENTS.md (22K)       │
└─────────────────────────────────────────────────────┘
                       │
                       │ 这些都没管"项目文档与记忆"
                       │
                       ▼
┌─────────────────────────────────────────────────────┐
│           EnjoyFlow v4 占领的生态位                   │
│  - DocAPI        ← 真实空白                          │
│  - ArtifactAPI   ← 真实空白                          │
│  - MemoryAPI     ← mem0 占，enjoyflow 接            │
│  - SyncAPI       ← 部分空白                          │
│  - HookAPI       ← 各 AI 工具自有                    │
│  - PluginAPI     ← 接 MCP / Agent Skills             │
└─────────────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────┐
│           被 enjoyflow 作为适配器调用的实现           │
│  - Notion / Obsidian / Confluence (文档后端)        │
│  - mem0 / MemOS / cognee (记忆后端)                  │
│  - spec-kit / OpenSpec / GSD Core (制品规范)        │
│  - basic-memory / obsidian-mind (翻译器)             │
│  - ECC / Superpowers / Anthropic Skills (skill源)   │
└─────────────────────────────────────────────────────┘
```

### 7.2 EnjoyFlow v4 的真正空白

| 接口 | 空白程度 | EnjoyFlow 的机会 |
|---|---|---|
| **DocAPI** | ★★★★★ | 没人做，EnjoyFlow 可定义 |
| **ArtifactAPI** | ★★★★★ | 没人做，EnjoyFlow 可定义 |
| **SyncAPI** | ★★★★☆ | 部分空白，EnjoyFlow 可定义 |
| **MemoryAPI** | ★★☆☆☆ | mem0 已在做，EnjoyFlow 应接它 |
| **HookAPI** | ★☆☆☆☆ | 各 AI 工具自有，EnjoyFlow 应接它们 |
| **PluginAPI** | ★★☆☆☆ | MCP/Agent Skills 已在做，EnjoyFlow 应接它们 |

### 7.3 EnjoyFlow 真正能赢的策略

**不要做 universal memory layer（mem0 已在做）**——而是做：
1. **DocAPI** —— 项目文档的接口规范（没人做）
2. **ArtifactAPI** —— 项目制品的接口规范（没人做）
3. **接 mem0 / 接 Agent Skills / 接 MCP** —— 不重新发明轮子

**这是 v4 哲学的真正落地策略**。

---

## 8. Jay 工具栈相关发现

### 8.1 Hermes Agent

- **NousResearch/hermes-agent** —— **197,655 stars**
- Jay 用的工具
- EnjoyFlow 是 Jay 开发的——enjoyflow 跟 Hermes 是**同一个生态**

### 8.2 MemOS

- **MemTensor/MemOS** —— **9,928 stars**
- 自家 plugin: **memos-local-plugin 2.0** 直接对接 Hermes Agent 和 OpenClaw
- EnjoyFlow v4 可以作为 MemOS 的上游接口

### 8.3 openclaw

- **openclaw/openclaw** —— **379,529 stars**（"lobster way"）
- 跟 MemOS 紧密相关

### 8.4 含义

- EnjoyFlow 在 **197K stars 的 Hermes Agent 生态**里有现成用户基础
- 应该把 **Hermes / OpenClaw 作为 HookAPI 的核心适配目标**

---

## 9. 数据源

### 9.1 接口协议成功案例

- https://github.com/modelcontextprotocol/modelcontextprotocol
- https://github.com/modelcontextprotocol/servers
- https://github.com/microsoft/language-server-protocol
- https://github.com/OAI/OpenAPI-Specification

### 9.2 2026 AI 接口哲学项目

- https://github.com/anthropics/skills （152,856 stars）
- https://github.com/agentsmd/agents.md （22,348 stars）
- https://github.com/google-labs-code/design.md （16,000 stars）
- https://github.com/OthmanAdi/planning-with-files （23,631 stars）

### 9.3 Memory interface

- https://github.com/mem0ai/mem0 （58,941 stars）
- https://github.com/volcengine/OpenViking
- https://github.com/deepset-ai/haystack
- https://github.com/letta-ai/letta
- https://github.com/topoteretes/cognee

### 9.4 Jay 工具栈相关

- https://github.com/NousResearch/hermes-agent （197,655 stars）
- https://github.com/MemTensor/MemOS
- https://github.com/openclaw/openclaw （379,529 stars）

### 9.5 规范站

- https://modelcontextprotocol.io
- https://agentskills.io/specification

---

## 10. EnjoyFlow v4 的真实成功路径（基于三轮调研推断）

### 10.1 不要做的事

| ❌ 不要做 | 原因 |
|---|---|
| 做 Universal Memory Layer | mem0 58K stars 已占 |
| 做 Plugin Marketplace | Claude Code 官方市场已被 Anthropic Skills 占 |
| 做 AI 行为纪律 | Superpowers 233K stars 已占 |
| 做 Spec-driven 流程 | spec-kit 114K stars 已占 |

### 10.2 应该做的事

| ✅ 应该做 | 依据 |
|---|---|
| 定义 **DocAPI**（项目文档接口规范）| 没人做，是真实空白 |
| 定义 **ArtifactAPI**（项目制品接口规范）| 没人做，是真实空白 |
| 直接采用 **Agent Skills SKILL.md 模式**作为 enjoyflow.yaml 格式 | Anthropic 已验证 |
| 直接采用 **DESIGN.md YAML frontmatter 模式**作为双向翻译的默认实现 | Google Labs 已验证 |
| 把 **MCP / Agent Skills / MEM0 / Hermes / Claude Code** 作为默认适配器 | 不重新发明轮子 |
| 把 **mem0 作为 MemoryAPI 的参考实现** | 借用 mem0 的 universal layer 概念 |
| 把EnjoyFlow 作为 **doc + artifact spec 的接口规范**，跟 spec-kit/OpenSpec 互补而非竞争 | EnjoyFlow 不做 spec-driven 流程，做 spec-driven 的接口 |

### 10.3 流行性路径

| 阶段 | 行动 | 参照 |
|---|---|---|
| 短期 | 发布 INTERFACE-SPEC.md + 1 个 DocAPI + 1 个 ArtifactAPI 规范 | 复制 Agent Skills 早期 |
| 中期 | 出 3-5 个适配器（Obsidian / Notion / spec-kit / mem0）| 复制 MCP 早期 |
| 长期 | 把EnjoyFlow 的 DocAPI/ArtifactAPI 推为事实标准 | 复制 OpenAPI 路径 |

---

*文档版本: v1.0 | 创建日期: 2026-06-19 | 关联文件: MARKET-RESEARCH-2026Q2.md / MARKET-RESEARCH-ADDENDUM-2026-06-19.md / POSITIONING.md / DISCUSSION-LOG-2026-06-19.md*
