# AI 工程化生态市场调研 - 四轮追加 (2026-06-19)

> **本文件是三轮调研的第四次追加**。
>
> 一轮：spec-driven / context engineering / skills framework 大方向
> 二轮：AI coding agent persistent memory 赛道深挖
> 三轮：接口协议哲学的参照系（MCP / OpenAPI / LSP / Agent Skills 等成功案例）
> **四轮：接口规范的"代码生成"和"企业级文档标准"维度**

---

## 0. 元信息

| 项 | 值 |
|---|---|
| 日期 | 2026-06-19 |
| 触发原因 | 三轮调研后定位 v4 接口规范哲学，需要找"代码生成"和"企业级"维度的参照系 |
| 调研范围 | OpenAPI Generator / DITA / Backstage / ADR / TOON / 文档 API / SDK 模式 |
| 方法 | GitHub API + 一手 README |
| 不下结论 | 只记录事实，定位判断留给用户 |

---

## 1. 接口协议 + 代码生成：OpenAPI 的成功秘密

### 1.1 OpenAPI Generator — 26,427 stars

**一手描述**：
> "OpenAPI Generator allows generation of **API client libraries (SDK generation), server stubs**, ..."

**关键洞察**：
- OpenAPI 不只是接口规范，还有 **26K stars 的代码生成器**
- 这是 OpenAPI 能成为 REST API 事实标准的真正原因：**不只定义接口，还能自动生成代码**
- 享受 Flow v4 如果只做接口规范而不做生成器，**会被边缘化**

### 1.2 接口规范成功的双轮模式

```
┌──────────────────────────────────────────────────┐
│  接口规范层（schema / specification）              │
│  - OpenAPI / LSP / MCP / Agent Skills           │
│  - 描述"应该是什么"                              │
└─────────────────┬────────────────────────────────┘
                  │
                  ▼ 自动生成
┌──────────────────────────────────────────────────┐
│  代码生成层（generator / SDK / tooling）           │
│  - openapi-generator (26K)                       │
│  - quicktype (13K)                               │
│  - swift-openapi-generator (1.9K)                │
│  - 把规范变成"能用的代码"                         │
└──────────────────────────────────────────────────┘
```

**享受 Flow v4 的策略选项**：
- **A. 只做规范层**：让第三方写生成器（参照 LSP 模式）
- **B. 规范 + 自带生成器**：参照 OpenAPI Generator 模式
- **C. 规范 + 自带生成器 + 适配器生态**：参照 OpenAPI 完整生态

---

## 2. 企业级文档 / 制品标准

### 2.1 Backstage — 33,655 stars（CNCF Incubating）

**一手描述**：
> "Backstage is an open framework for building developer portals. Powered by a centralized software catalog, Backstage restores order to your microservices and infrastructure"
>
> "**TechDocs** for making it easy to create, maintain, find, and use technical documentation, using a 'docs like code' approach"
>
> "Created by **Spotify** but is now hosted by the **Cloud Native Computing Foundation (CNCF)** as an **Incubation level project**"

**Backstage 的核心组件**：
- **Software Catalog** — 管理所有软件（microservices, libraries, ML models）
- **Software Templates** — 标准化新项目创建
- **TechDocs** — "docs like code" 方法
- **158 个 plugins**（生态规模）

**跟享受 Flow v4 的对位**：
- Backstage = 企业级"开发者门户"框架
- 享受 Flow = AI 时代的"项目文档与记忆"接口
- Backstage 已被 CNCF 收编 = 基础设施级背书
- **享受 Flow 不应跟 Backstage 竞争，应作为 Backstage 的适配器**

### 2.2 DITA — 企业级技术文档标准（445 stars）

**DITA Open Toolkit** —— 来自 IBM 的技术文档标准
- 现实使用：航空、汽车、医疗器械、硬件行业的标准
- AI 时代：**几乎没人包装它**

**享受 Flow v4 的策略选项**：
- 可以提供 DITA adapter，让传统企业用户的 DITA 文档能被 AI 读
- 或明确"享受 Flow 不做 DITA 兼容"

### 2.3 ADR — 16,145 stars（制品层最成熟标准）

| 项目 | Stars | 角色 |
|---|---|---|
| **architecture-decision-record/architecture-decision-record** | **16,145** | "ADR examples for software planning, IT leadership" |
| **npryce/adr-tools** | 5,525 | ADR CLI 工具 |
| **adr/madr** | 2,275 | "**Markdown Architectural Decision Records**" |

**关键观察**：
- ADR 是**制品层**的真实成熟标准（16K stars）
- **享受 Flow v4 的 ArtifactAPI 应该接 ADR，不重新发明轮子**
- 享受 Flow 自己的 `development/decisions/` 目录可以采用 ADR 规范

### 2.4 Architecture as Code

| 项目 | Stars | 角色 |
|---|---|---|
| **finos/architecture-as-code** | 341 | "Architecture as Code (AasC)" |
| **SlavaVedernikov/C4InterFlow** | 245 | "Architecture as Code (AaC) framework using C4 model" |
| **inherd/forming** | 26 | "轻量级架构即代码语言" |
| **sruja-ai/sruja** | 22 | "**Context Engineering and Architecture Intelligence for the AI Era**. Machine-readable, continuous..." |

**关键观察**：
- 架构即代码方向**真实存在**但没人做出强势品牌（最高 341 stars）
- sruja 22 stars 提出 **"Context Engineering and Architecture Intelligence for the AI Era"**——跟享受 Flow v4 直接撞概念
- **享受 Flow v4 有机会占领"AI 时代的架构即代码"空白**

---

## 3. AI 文档 / Spec 模板生成（最直接对位空白）

### 3.1 cogeet-io/ai-development-specifications — 34 stars

**唯一直接对位的项目**：
> "Complete specifications for AI-assisted development: **Spec as Code, Testing as Code, Documentation...**"

**关键观察**：
- AI 开发规范领域**只有 34 stars**——**真实空白**
- 享受 Flow v4 的 DocAPI + ArtifactAPI **就是这个位置**

### 3.2 其他 AI spec 相关项目

| 项目 | Stars | 角色 |
|---|---|---|
| `sruja-ai/sruja` | 22 | "Architecture Intelligence for the AI Era" |
| `JiuNian3219/architext` | 14 | "Document-driven development for AI coding assistants. **Spec and plan first**, then code" |
| `SpillwaveSolutions/document-specialist-skill` | 30 | "Claude Code Skill. AI-powered software documentation automation" |
| `sakitA/spec-kit-optimize` | 7 | "Spec-Kit extension that audits and optimizes AI governance documents" |

**整体观察**：
- "AI 时代的 spec / 文档接口规范" 是**真实空白**
- 但**碎片化严重**——没人统一

---

## 4. TOON — 24,612 stars（LLM 时代的新数据格式）

### 4.1 一手描述

> "🎒 **Token-Oriented Object Notation (TOON)** – Compact, human-readable, **schema-aware JSON for LLM**"

### 4.2 关键观察

- TOON 是 2026 年 LLM 数据传输格式的**新爆款**
- 设计目标：**比 JSON 节省 token + 仍是 schema-aware + 仍可读**
- **享受 Flow v4 可以考虑用 TOON 作为默认传输格式**（而不是 JSON 或 YAML）

### 4.3 享受 Flow 的传输格式选项

| 格式 | 优点 | 缺点 | 推荐 |
|---|---|---|---|
| JSON | 工具支持最广 | LLM 解析 token 多 | ❌ |
| YAML | 人可读 | 缩进敏感 | ✅ 默认 |
| TOML | 配置友好 | LLM 不熟 | ❌ 配置用 |
| **TOON** | LLM 友好 + schema-aware | 工具链少 | ⚠️ 评估中 |
| Markdown + frontmatter | 人 + AI 都友好 | 不适合复杂结构 | ✅ 文档用 |

---

## 5. Markdown frontmatter 验证工具（真实空白）

### 5.1 搜索结果

`schema markdown frontmatter validate` 搜索：全部 < 100 stars
- `remark-lint-frontmatter-schema` 77 stars
- `markdownlint/markdownlint` 2,057 stars
- `maxchang3/newmd` 15 stars
- 其他 < 30 stars

### 5.2 关键观察

**Markdown frontmatter schema 验证是真实空白**——享受 Flow v4 的 DocAPI 可以**定义 frontmatter schema + 验证工具**，填补这块。

---

## 6. SDK 维度（产品形态决策）

### 6.1 已有的 Agent SDK

| 项目 | Stars | 角色 |
|---|---|---|
| **anthropics/claude-agent-sdk-python** | 7,362 | Anthropic 官方的 agent SDK |
| **coze-dev/coze-studio** | 21,004 | "AI agent development platform" |
| **openai/openai-cs-agents-demo** | 6,409 | OpenAI Agents SDK demo |
| **google/adk-samples** | 9,694 | Google Agent Development Kit |
| **AgentOps-AI/agentops** | 5,644 | "Python SDK for AI agent monitoring" |
| **anthropics/claude-agent-sdk-python** | 7,362 | Claude Agent SDK |

### 6.2 享受 Flow v4 是否需要做 SDK？

| 选项 | 内容 | 参照 |
|---|---|---|
| **A. 只做规范** | 不做 SDK，让 AI 工具团队自己适配 | LSP 模式 |
| **B. 出 TypeScript/Python SDK** | 让用户写 enjoyflow adapter 时有 SDK | MCP 模式（87K servers 自带 SDK）|
| **C. 出 CLI + SDK** | 同时出 CLI（npx @enjoyorg/enjoyflow）和 SDK | OpenAPI Generator 模式 |
| **D. 出 Schema + Generator + SDK** | 完整三件套 | OpenAPI 完整生态模式 |

---

## 7. 享受 Flow v4 接口规范的参照系全景

### 7.1 已成熟接口协议（应接不重做）

| 协议 | 状态 | 享受 Flow 对位 |
|---|---|---|
| **MCP** (Anthropic) | 8K + 87K servers | 接作为 HookAPI 的一部分 |
| **Agent Skills SKILL.md** (Anthropic) | 152K stars | enjoyflow.yaml 应直接采用 SKILL.md 模式 |
| **AGENTS.md** | 22K stars | 接作为 AgentAPI 的一种实现 |
| **DESIGN.md** (Google Labs) | 16K stars | 接作为 ArtifactAPI 的一种实现（YAML frontmatter）|
| **OpenAPI** | 31K stars + 26K generator | 接口规范的元标准，享受 Flow 借用其方法论 |
| **LSP** | 13K stars | 借其"接口+实现"成功模式 |
| **ADR / MADR** | 16K + 2K stars | ArtifactAPI 接 ADR，不重做决策记录 |
| **TOON** | 24K stars | 评估作为默认传输格式 |
| **SBOM / SPDX / CycloneDX** | 多家生态 | artifact 安全标准的接口，享受 Flow 可接 |

### 7.2 真实空白（享受 Flow v4 应该占）

| 空白 | 程度 | 享受 Flow 的机会 |
|---|---|---|
| **DocAPI**（项目文档接口规范）| ★★★★★ 真实空白 | 直接定义 |
| **ArtifactAPI**（项目制品接口规范）| ★★★★★ 真实空白 | 直接定义 |
| **Markdown frontmatter schema 验证** | ★★★★☆ | 提供 schema 规范 + 验证工具 |
| **AI 时代的"架构即代码"规范** | ★★★★☆ | 占领 sruja 22 stars 的空白 |
| **AI development specifications** | ★★★★★ 真实空白 | cogeet-io 34 stars 已在 |

### 7.3 已成熟但享受 Flow 应主动接

| 项目 | Stars | 享受 Flow 的接法 |
|---|---|---|
| **backstage/backstage** | 33,655 | DocAPI 适配器 |
| **OpenAPI Generator** | 26,427 | 享受 Flow 应自带类似工具（or 接） |
| **mem0** | 58,941 | MemoryAPI 适配器 |
| **AppFlowy / AFFiNE** | 72K / 69K | DocAPI 适配器 |
| **Notion / Confluence / Obsidian** | 已成熟产品 | DocAPI 适配器 |

---

## 8. 享受 Flow v4 接口规范的设计建议

### 8.1 应该采用的现成模式

1. **enjoyflow.yaml 格式 = SKILL.md 格式**（Anthropic 已验证）
   - YAML frontmatter + Markdown body
   - 渐进披露（progressive disclosure）

2. **双向翻译默认实现 = DESIGN.md 模式**（Google Labs 已验证）
   - YAML frontmatter = 机器视图
   - Markdown body = 人读视图

3. **接口哲学 = LSP/MCP 模式**
   - 接口规范 + 默认实现 + 适配器生态

4. **制品层 = ADR / MADR 模式**
   - 16K stars 成熟标准，不重做

### 8.2 享受 Flow 应该填补的空白

1. **DocAPI 正式规范**
   - 项目文档（PRD/Contract/Design Plan）的接口契约
   - Markdown frontmatter schema 验证
   - 章节索引、标签、引用关系的元数据标准

2. **ArtifactAPI 正式规范**
   - 制品的接口契约
   - 接 ADR / MADR
   - 接 SBOM（如有需要）

3. **附带的代码生成器**（如果资源允许）
   - 从 enjoyflow.yaml 生成 .claude/skills/、.cursor/rules/ 等适配器
   - 参照 OpenAPI Generator 26K stars 的成功模式

### 8.3 不应该做的事

| ❌ 不做 | 原因 |
|---|---|
| 重新发明 AI memory 接口 | mem0 58K stars 已占 |
| 重新发明 plugin 市场 | Claude Code 官方市场已被 Anthropic Skills 占 |
| 做 SDK 跟 agent SDK 竞争 | Anthropic / OpenAI / Google 已各自做 SDK |
| 重新发明 ADR | 16K stars 已成熟 |

---

## 9. 流行性参照系数据汇总

| 维度 | 标杆项目 | 标杆数据 |
|---|---|---|
| **接口协议** | MCP | 8K + **87K servers** / 不到 2 年 |
| **Skill 标准** | anthropics/skills | **152K stars** + agentskills.io 规范站 |
| **文档门户** | backstage/backstage | **33,655 stars** + CNCF Incubating + 158 plugins |
| **决策记录** | ADR | **16K stars** + npryce/adr-tools 5.5K |
| **LLM 数据格式** | TOON | **24K stars** |
| **OpenAPI 配套** | openapi-generator | **26K stars**（代码生成器）|
| **universal memory** | mem0 | **58,941 stars** |
| **AGENTS.md** | agentsmd/agents.md | **22K stars** |
| **DESIGN.md** | google-labs-code/design.md | **16K stars** |
| **planning-with-files** | OthmanAdi | **23,631 stars** |

---

## 10. 数据源

### 10.1 OpenAPI / 代码生成

- https://github.com/OpenAPITools/openapi-generator （26K）
- https://github.com/OAI/OpenAPI-Specification （31K）
- https://github.com/glideapps/quicktype （13K）

### 10.2 企业级文档

- https://github.com/backstage/backstage （33K，CNCF）
- https://github.com/dita-ot/dita-ot （445）

### 10.3 ADR / 制品层

- https://github.com/architecture-decision-record/architecture-decision-record （16K）
- https://github.com/npryce/adr-tools （5.5K）
- https://github.com/adr/madr （2.2K）

### 10.4 Architecture as Code

- https://github.com/finos/architecture-as-code （341）
- https://github.com/SlavaVedernikov/C4InterFlow （245）
- https://github.com/sruja-ai/sruja （22，AI 时代直接对位）

### 10.5 AI spec / 文档生成空白

- https://github.com/cogeet-io/ai-development-specifications （34，真实空白）
- https://github.com/JiuNian3219/architext （14）

### 10.6 Markdown frontmatter 验证空白

- https://github.com/JulianCataldo/remark-lint-frontmatter-schema （77）
- https://github.com/markdownlint/markdownlint （2K）

### 10.7 LLM 数据格式

- https://github.com/toon-format/toon （24K）

### 10.8 Agent SDK

- https://github.com/anthropics/claude-agent-sdk-python （7.3K）
- https://github.com/google/adk-samples （9.7K）

---

*文档版本: v1.0 | 创建日期: 2026-06-19 | 关联文件: MARKET-RESEARCH-2026Q2.md / MARKET-RESEARCH-ADDENDUM-2026-06-19.md / MARKET-RESEARCH-ADDENDUM-2-2026-06-19.md / POSITIONING.md / DISCUSSION-LOG-2026-06-19.md*
