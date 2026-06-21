# AI 工程化生态市场调研 - 二轮追加 (2026-06-19)

> **本文件是 `MARKET-RESEARCH-2026Q2.md` 的追加**，不是替代。
>
> 第一轮调研关注"spec-driven / context engineering / skills framework"等大方向。
> 二轮调研**深挖"AI 编码 agent 持久记忆"这条赛道**——因为 EnjoyFlow v3 定位的"双向翻译"绕不过它。
>
> **关键修正**：第一轮调研结论"AI Memory 是空白带"——**错了**。本文件记录事实证据。

---

## 0. 元信息

| 项 | 值 |
|---|---|
| 日期 | 2026-06-19 |
| 触发原因 | 第一轮调研后定位 v3（"双向翻译"），用户问"继续调研类似工具" |
| 调研范围 | "AI coding agent persistent memory" 赛道全扫描 + 双向翻译实际案例 + awesome list 路径 |
| 方法 | HN Algolia 搜索 + GitHub API + 各项目一手 README |
| 不下结论 | 只记录事实，定位判断留给用户 |

---

## 1. 二轮调研颠覆的第一轮判断

### 1.1 第一轮说错了的话

| 第一轮判断 | 二轮发现 |
|---|---|
| "AI Memory for Engineering Teams 是空白带（< 300 stars）" | ❌ **严重错**。实际有 21.8 万 + 6 万 + 2.3 万 + 1.4 万 + 9K stars 等多个项目 |
| "双向翻译（human readable ↔ machine readable）是空白" | ❌ **部分错**。basic-memory 3,265 stars 已做"human + AI 共写 Markdown"，obsidian-mind 3,037 stars 已做"Obsidian → AI memory" |

### 1.2 第一轮判断对的

| 第一轮判断 | 二轮验证 |
|---|---|
| "spec-driven 是红海" | ✅ 仍对，11.4 万 + 5.5 万 stars |
| "Plugin 市场 + npm 形态对标 Superpowers/OpenSpec" | ✅ 仍对，但实际门槛被 ECC 21.8 万星抬高 |
| "Breunig 共识语言是技术圈暗号" | ✅ 仍对 |
| "EnjoyFlow 现有机制对应 4 失败 + 5 修复" | ✅ 仍对，但没人在 EnjoyFlow 这条线 |

---

## 2. 二轮新发现：AI Coding Agent Memory 全景

### 2.1 头部（10K+ stars）

| # | 项目 | Stars | 定位（一手） |
|---|---|---|---|
| 1 | **affaan-m/ECC** | **218,278** | "**The harness-native operator system** for agentic work. Built from real-world multi-harness engineering workflows" |
| 2 | **ruvnet/ruflo** | 60,379 | "🌊 The leading **agent meta-harness for Claude**. Deploy intelligent multi-agent swarms" |
| 3 | **Egonex-AI/Understand-Anything** | 64,059 | "Graphs that teach > graphs that impress. **Turn any code into an interactive knowledge**..." |
| 4 | **zhayujie/CowAgent** | 45,471 | "Open-source super AI assistant & **Agent Harness**. Plans tasks, runs tools and skills" |
| 5 | **rohitg00/agentmemory** | 23,463 | "**Persistent memory for AI coding agents** based on real-world benchmarks" |
| 6 | **NevaMind-AI/memU** | 13,890 | "**File System as Memory**, Memory Shapes the Agent" |

### 2.2 腰部（2K-10K stars）

| # | 项目 | Stars | 定位（一手） |
|---|---|---|---|
| 7 | **MemTensor/MemOS** | 9,928 | "🎯 **Self-evolving memory OS** for LLM & AI Agents"（**有 ArXiv 论文** 2507.03724）|
| 8 | **EverMind-AI/EverOS** | 7,965 | "Self-evolving memory across Agent and platform. The **one portable memory layer**" |
| 9 | **Dammyjay93/interface-design** | 5,091 | "Design engineering for Claude Code. **Craft, memory, and enforcement** for consistency" |
| 10 | **CaviraOSS/OpenMemory** | 4,248 | "Local persistent memory store for LLM applications including claude desktop" |
| 11 | **basicmachines-co/basic-memory** | 3,265 | "**AI conversations that actually remember**. Never re-explain your project to your AI" |
| 12 | **breferrari/obsidian-mind** | 3,037 | "An **Obsidian vault** that gives AI coding agents persistent memory" |
| 13 | **letta-ai/letta-code** | 2,749 | "Stateful agents with **memory, identity**, and the ability to..." |

### 2.3 关键数据点

- **AI coding agent memory 是真红海**，5 个 10K+ + 7 个 2K-10K stars
- 头部 ECC **21.8 万 stars** 是 Superpowers（23.3 万）量级
- **MemOS 有 ArXiv 论文**，是学术 + 工程并重
- **MemOS 直接对接 Hermes Agent / OpenClaw 生态**（`memos-local-plugin 2.0`）

---

## 3. 双向翻译（Human ↔ Machine）实际案例

### 3.1 basic-memory —— 3,265 stars —— **最接近 EnjoyFlow v3 定位的现成项目**

**一手描述**（来自 README）：

> "Your knowledge lives as **Markdown files that both you and your AI can read, write, and search**"
>
> "**Two-way.** AI and humans write to the same files; sync keeps them in step"
>
> "A **real knowledge graph**"
>
> "MCP-native. Works with every major AI client and IDE"
>
> "Local-first. Plain text on your disk. Forever."

**商业模式**:
- $15/月 SaaS（"locked in for life"）
- 7 天免费试用
- 团队版（共享云 workspace）
- 开源 AGPL-3.0

**技术栈**:
- Python 3.12+
- MCP Server
- 通过 DeepWiki 索引

**跟 EnjoyFlow 的差异**:
| 维度 | basic-memory | EnjoyFlow v3 |
|---|---|---|
| 双向翻译 | ✅ 核心卖点 | ✅ 核心卖点 |
| Markdown 文件 | ✅ 唯一存储 | ✅ 主要存储 |
| 知识图谱 | ✅ 内建 | ❌ 没明确 |
| 制品层（PRD/Contract/Design Plan） | ❌ 无 | ✅ 强调 |
| 跨项目 shared/ 共享层 | ❌ 无 | ✅ 强调 |
| 团队协作 | ✅ 团队版 | ⚠️ 4 人角色契约（设计阶段）|
| 商业化 | ✅ 已上线 | ❌ 无 |

### 3.2 obsidian-mind —— 3,037 stars —— "Obsidian vault + AI memory"

**一手描述**:
> "An Obsidian vault that gives AI coding agents persistent memory"
>
> "Works with **Claude Code (full support), Codex CLI, and Gemini CLI** — same hooks, same commands, same vault"
>
> "Install via `shardmind install` or `git clone`"

**核心命令**（来自 README）:
- `/om-standup` — 读 North Star, active projects, recent memories
- `/om-dump` — 把对话沉淀到 vault

**跟 EnjoyFlow 的差异**:
- obsidian-mind 把 Obsidian 当 vault（用户已有知识库）
- EnjoyFlow 是新建 `knowledge-base/` 目录
- 都没有显式的"制品层"（PRD/Contract）

### 3.3 memU —— 13,890 stars —— 文件系统当记忆

**目录结构**:
```
memory/
├── INDEX.md              ← 地图
├── MEMORY.md             ← profile/preferences/goals
└── skill/
    └── {skill_name}/SKILL.md
```

**跟 EnjoyFlow `knowledge-base/` 结构对比**:
| memU | EnjoyFlow |
|---|---|
| 3 文件 | 6 类 |
| INDEX/MEMORY/skill | project/business/development/testing/deployment/contract |
| 单层 | shared/ 跨项目层 + inherits: 继承 |

**思路同源，EnjoyFlow 多了一层跨项目共享**。

### 3.4 letta-code —— 2,749 stars —— "Stateful agents"

**一手描述**:
> "Letta Code is a **stateful agent harness** for creating agents that are more like people than tools"
>
> "MemFS — All context (including memory blocks) is **tracked via git**"
>
> "Skills — Loads global skills (`~/.letta`), project-scoped skills (`.agents/skills`)"

**关键技术点**:
- **MemFS 用 git 跟踪记忆**（跟 EnjoyFlow 的"Git-friendly 制品"思路一致）
- **Skills 三层加载**：global / project-scoped / agent-scoped（跟 EnjoyFlow shared/ + project/ + business/ 思路接近）

### 3.5 MemOS —— 9,928 stars —— "Self-evolving memory OS"

**一手描述**:
> "🎯 +43.70% Accuracy vs. OpenAI Memory"
>
> "**Self-evolving memory**: L1 trace, L2 policy, L3 world model, and crystallized Skills driven by feedback"
>
> "memos-local-plugin 2.0 — **One local-first memory core for Hermes Agent and OpenClaw**"

**学术背书**: ArXiv 2507.03724

**层级模型**（跟 EnjoyFlow 可以对照）:
| MemOS | EnjoyFlow |
|---|---|
| L1 trace | 活文档（PRD/Contract/进度）|
| L2 policy | 记忆资产（knowledge-base/）|
| L3 world model | 共享资产（knowledge-base/shared/）|
| Crystallized Skills | skills + SKILL.md 沉淀 |

**几乎是同构的**。

---

## 4. ECC 详解（最直接竞争对手）

### 4.1 数据

| 指标 | 数值 |
|---|---|
| Stars | **218,278**（GitHub 自报 211.9K+）|
| Forks | 32.5K+ |
| Contributors | 230+ |
| Language ecosystems | 12+（Shell/TypeScript/Python/Go/Java/Perl/Markdown/...）|
| 周下载（npm）| ecc-universal 显著（badge 显示）|
| 14 天 git clones | 106K |
| 商业网站 | ecc.tools |

### 4.2 自我定位

> "**The harness-native operator system** for agentic work. Built from real-world multi-harness engineering workflows"
>
> "Not just configs. A complete system: skills, instincts, **memory optimization**, continuous learning, security scanning, and research-first development"

### 4.3 跟 EnjoyFlow v3 对位

| 维度 | ECC | EnjoyFlow v3 |
|---|---|---|
| Skills | ✅ | ✅ |
| Memory | ✅ 优化 | ✅ 双向翻译 |
| 跨 AI 工具 | ✅ Multi-harness | ✅ 计划 ≥4 工具 |
| Security | ✅ | ❌ |
| 商业化 | ✅ 完整（npm + App + 网站）| ❌ 无 |
| 社区 | ✅ 230+ contributors | ❌ 无 |
| 制品层（PRD/Contract）| ❌ | ✅ 核心 |
| 双向翻译（人读 ↔ AI 记忆）| ⚠️ 部分 | ✅ 核心 |
| 跨项目 shared/ 共享层 | ❌ | ✅ 核心 |
| 中文支持 | ✅ 简体中文 README | ❌ 无 |

### 4.4 对 EnjoyFlow 的含义

**ECC 是 EnjoyFlow 最可怕的对手**：
- 21.8 万 stars = 已被市场验证
- 覆盖范围跟 EnjoyFlow v3 大幅重合
- 但 ECC 没做"制品层 + 双向翻译 + 跨项目共享"——这三块是 EnjoyFlow 真正的差异化锚点

**享受 Flow 该不该直接竞争 ECC？**
- 如果是，意味着需要 18 个月从 0 追到 21.8 万 stars —— 不现实
- 如果不是，应该明确"我们做 ECC 没做的事"，而不是做 ECC 在做的事的子集

---

## 5. awesome list 路径（传播路径）

### 5.1 头部 awesome lists

| 项目 | Stars | 涵盖 |
|---|---|---|
| **hesreallyhim/awesome-claude-code** | 46,866 | "A curated list of awesome **skills, hooks, slash-commands, agent orchestrators**, applications" |
| **sickn33/antigravity-awesome-skills** | 41,152 | "**Installable GitHub library of 1,500+ agentic skills** for Claude Code, Cursor, Codex CLI, Gemini..." |
| **VoltAgent/awesome-claude-code-subagents** | 22,122 | 100+ specialized Claude Code subagents |
| **travisvn/awesome-claude-skills** | 13,589 | Claude Skills 资源 |
| **alirezarezvani/claude-skills** | 18,557 | 337 skills, 30+ agents, 70+ commands |
| **davepoon/buildwithclaude** | 3,087 | Claude Skills / Agents / Commands / Hooks / Plugins / Marketplace 单 hub |
| **jeremylongshore/claude-code-plugins-plus-skills** | 2,397 | 425 plugins, 2810 skills, 200 agents |

### 5.2 含义

- **进 awesome-claude-code (46,866 stars) 是 2026 标准传播路径**
- 但享受 Flow 需要先**自己做成 plugin 形态**才能被收录（不是文档仓库）
- enjoyflow 当前是"仓库 + 文档"形态，**不在 awesome list 收录范围**

---

## 6. Karpathy "LLM Wiki" 模式溯源

### 6.1 发现

agentmemory README 提到：
> "The gist extends **Karpathy's LLM Wiki pattern** with confidence scoring, lifecycle, knowledge graphs, and hybrid search"

**Karpathy 早就提出过 "LLM Wiki" 模式** —— 这是 EnjoyFlow `knowledge-base/` 章节索引分片加载的**理论先驱**。

### 6.2 未完成调研

- Karpathy 原 blog / tweet 时间
- LLM Wiki 模式具体定义
- 是否被广泛采纳

（这些留待后续调研。本节先标记存在）

---

## 7. v3 定位需要重新评估的 4 件事

### 7.1 原定位

> "EnjoyFlow 是 AI 时代的项目文档与记忆管理框架——同一份文档既让人读、也让 AI 读，并能**自动从人读文档生成 AI 可用的项目记忆**"

### 7.2 需要重新评估的点

| # | 原判断 | 现实证据 | 评估方向 |
|---|---|---|---|
| 1 | "双向翻译是空白带" | basic-memory 3,265 stars 已做"human + AI 共写 Markdown" | 双向翻译**整体不是空白**，但 EnjoyFlow 强调"自动从人读文档生成 AI 记忆"这条具体路径**可能仍有空间** |
| 2 | "差异化锚点是双向翻译" | ECC 21.8 万 + agentmemory 2.3 万已涵盖"AI memory" | 双向翻译**不是差异化**，差异化需要重新挖 |
| 3 | "差异化锚点是跨项目 shared/" | ECC/basic-memory/obsidian-mind 都没做跨项目 | ✅ **这点是真实差异化** |
| 4 | "差异化锚点是制品层（PRD/Contract）" | ECC/basic-memory/obsidian-mind 都没强调制品层 | ✅ **这点是真实差异化** |

### 7.3 新结论（仅作观察，不下判断）

**真实差异化锚点可能是**（待你确认）：
1. **制品层 + 双向翻译的组合**（PRD/Contract 是 AI 的契约输入，不是记忆）
2. **跨项目 shared/ 共享层**（其他项目都没做）
3. **中文场景**（ECC/basic-memory 都有简体中文 README 但 EnjoyFlow 是中国团队）

**真正空白带可能不是"AI memory"赛道整体**，而是"**AI memory + 项目制品层 + 跨项目共享 + 中文场景**"这个交集。

---

## 8. 数据源

### 8.1 GitHub API（一手）

- https://api.github.com/repos/affaan-m/ECC
- https://api.github.com/repos/rohitg00/agentmemory
- https://api.github.com/repos/basicmachines-co/basic-memory
- https://api.github.com/repos/breferrari/obsidian-mind
- https://api.github.com/repos/NevaMind-AI/memU
- https://api.github.com/repos/MemTensor/MemOS
- https://api.github.com/repos/letta-ai/letta-code
- https://api.github.com/repos/ruvnet/ruflo
- https://api.github.com/search/repositories?q=claude+memory+OR+gpt+memory+OR+gemini+memory
- https://api.github.com/search/repositories?q=claude+skill+plugin+marketplace

### 8.2 HN Algolia API（一手）

- https://hn.algolia.com/api/v1/search?query=claude%20memory%20%22long-term%22
- https://hn.algolia.com/api/v1/search?query=chatgpt%20memory%20%22long-term%22
- https://hn.algolia.com/api/v1/search?query=%22LLM%20wiki%22

### 8.3 论文

- MemOS ArXiv: https://arxiv.org/abs/2507.03724

---

## 9. 下一步建议（不动手，等 Jay 决定）

| 选项 | 内容 |
|---|---|
| A | 重新评估 v3 定位稿（基于二轮调研修正差异化锚点）|
| B | 继续挖 Karpathy LLM Wiki 模式 + Manus / Drew Breunig 实践 |
| C | 跟 basic-memory / obsidian-mind 做直接对比，找出 EnjoyFlow 还能立足的角度 |
| D | 暂停调研，开始做 demo 验证双向翻译机制是否真的成立 |
| E | 其他（Jay 指定）|

---

*文档版本: v1.0 | 创建日期: 2026-06-19 | 关联文件: MARKET-RESEARCH-2026Q2.md / POSITIONING.md / DISCUSSION-LOG-2026-06-19.md*
