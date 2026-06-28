# AI 工程化生态市场调研观察 (2026 Q2)

> **本文件不是定位稿**——是定位稿背后的市场证据库。
>
> 用途：
> 1. 写对外 PR / 博客 / README 时取用具体数据
> 2. 团队讨论 EnjoyFlow 定位时引用
> 3. 未来重新评估定位时复查"市场还在不在、对手还在不在"
>
> **采集时间**: 2026-06-19
> **方法**: HN Algolia 搜索 + GitHub API + 各项目官方 README
> **不做评判**：只记录事实和已经成立的共识

---

## 1. 关键数据快照（2026-06-19）

| 项目 | Stars | Forks | 创建日期 | 类别 |
|---|---|---|---|---|
| **obra/superpowers** | 233,323 | - | 2025-10-09 | Agentic skills 框架 |
| **github/spec-kit** | 114,151 | - | 2025-08-21 | Spec-driven 开发（GitHub 官方） |
| **cline/cline** | 63,538 | - | - | Autonomous coding agent |
| **Fission-AI/OpenSpec** | 55,690 | 3,902 | 2025-08-05 | Spec-driven for AI coding |
| **RooCodeInc/Roo-Code** | 24,246 | - | - | AI agents in editor |
| **open-gsd/gsd-core** | 4,542 | 278 | 2026-05-22 | Context engineering + spec-driven |
| **Pimzino/spec-workflow-mcp** | 4,238 | - | - | MCP-based spec workflow |
| **ai-boost/awesome-harness-engineering** | 1,939 | - | - | 资源列表：harness engineering |
| **Gentleman-Programming/agent-teams-lite** | 1,230 | - | - | Spec-driven + AI sub-agents |
| **GD-Agentic-Skills** | 261 | - | - | Godot 游戏的 AI 长期记忆 |

### 1.1 AI Memory/记忆 类（EnjoyFlow 最直接的生态位）

| 项目 | Stars | 定位 |
|---|---|---|
| **zero8dotdev/smriti** | 33 | Shared memory for AI-powered engineering teams |
| **keeprlabs/keepr** | 30 | AI memory layer for engineering teams, local-first desktop |
| **engineering-os/engineering-os** | 34 | Your AI finally remembers your codebase |
| **roberto-mello/lavra** | 47 | Plugin with compound engineering workflows and memory for AI coding agents |
| **rullerzhou-afk/memoria-chat** | 35 | OSS AI chat client with structured memory |
| **thedivergentai/GD-Agentic-Skills** | 261 | Long-term memory for Godot 4.5+ AI agents |

**观察**: 这条赛道**全部 < 300 stars**，没有强势品牌。是真正的空白带。

---

## 2. 概念与共识语言（2026 年已经成立的）

### 2.1 Context Engineering（已被命名 + 共识）

- **HN 主帖**: "The new skill in AI is not prompting, it's context engineering" (Phil Schmid, 2025-06-30) — **518 评论 / 915 点**
- **核心定义** (Schmid): *"the art of providing all the context for the task to be plausibly solvable by the LLM."*
- **核心论断** (Schmid): *"Most agent failures are not model failures anymore, they are context failures."*
- **Anthropic 官方**: 将 **memory** 列为 augmented LLM 的核心 augmentation 之一（与 retrieval、tools 并列）—— https://www.anthropic.com/engineering/building-effective-agents

### 2.2 Context Rot 的 4 种失败模式 (Drew Breunig, 2025-06-22)

> https://www.dbreunig.com/2025/06/22/how-contexts-fail-and-how-to-fix-them.html

| 失败 | 定义 | 例子 |
|---|---|---|
| **Context Poisoning** | 幻觉进入 context 被反复引用 | Gemini 玩 Pokemon 时产生"不可能达成"的目标并持续追求 |
| **Context Distraction** | context 过长，模型忽略训练所学 | Gemini >100k tokens 时倾向重复历史动作而非综合推理 |
| **Context Confusion** | 多余 context 干扰响应 | 工具描述重叠导致选择错误 |
| **Context Clash** | context 各部分互相矛盾 | 同一 agent 内多目标冲突 |

### 2.3 Context Rot 的 5+1 种修复机制 (Breunig, 2025-06-26)

| 修复 | 定义 |
|---|---|
| **Context Quarantine** | 把不同 context 隔离在独立线程 |
| **Context Pruning** | 删除 context 中不相关信息 |
| **Context Summarization** | 把累积 context 浓缩成摘要 |
| **Context Offloading** | 信息外存到 LLM context 之外（工具管理） |
| **Tool Loadout** | 像游戏装弹一样按任务选择相关工具（>30 工具后必须） |
| **RAG** | 选择性注入相关文档（不在 Breunig 主文，但被反复引用） |

### 2.4 其他被命名的现象

- **Context Rot** — 上下文腐化（GSD Core 直接用作核心痛点词）
- **Vibe Coding** — 凭直觉让 AI 写（spec-kit 自我定位就是反对这个）
- **Spec-Driven Development (SDD)** — spec 优先 / spec 可执行（spec-kit 主推）
- **Harness Engineering** — Agent harness 设计工程（awesome-harness-engineering 1939 stars，词刚被命名）
- **Subagent-Driven Development** — 多 agent 流水线（Superpowers 主推）

---

## 3. HN 上的第一手用户痛点语料（2025-2026）

### 3.1 高信号帖子

| 帖子 | 点数 | 评论 | 信号 |
|---|---|---|---|
| "Daemons – we pivoted from building agents to cleaning up after them" | 70 | 31 | **"帮 AI 擦屁股"成赛道** |
| "Context engineering is sleeping on the humble hyperlink" | 177 | 68 | context 工程不只是 prompt |
| "Effective context engineering for AI agents" (Manus) | 148 | 32 | 生产级 agent 经验 |
| "Context Engineering for AI Agents: Lessons from Building Manus" | 120 | 4 | 一手生产经验 |
| "Flathub prohibits AI-generated code" | 21 | 9 | 社区对 AI 生成物的**敌意** |
| "Show HN: Cartograph – AI-generated code documentation" | 6 | 2 | AI 生成文档是公认痛点 |

### 3.2 真实抱怨原话节选

> **"AI 写出来的东西越来越多，但团队越来越看不懂、越来越接不住、会话一换就清零"** — 多次重复出现

> **"It is probably 6-7 months ago I used ChatGPT for 'vibe coding', and my main complaint was that the model eventually started moving away too far from its intended goal... I had to fire up a new model, and feed all the context I had, and continue."** — context rot 真实体验

> **"Building Agents is less about the code you write or framework you use. The difference between a cheap demo and a 'magical' agent is about the quality of the context you provide."** — Schmid

> **"If you treat your context like a junk drawer, the junk will influence your response."** — Breunig

---

## 4. 直接对标：GSD Core（最像 EnjoyFlow 的物种）

> https://github.com/open-gsd/gsd-core
> 主页: https://www.opengsd.net/

### 4.1 一手自述

> *"A light-weight meta-prompting, context engineering, and spec-driven development system for Claude Code, OpenCode, Gemini CLI, Kimi CLI, Kilo, Codex, Copilot, Cursor, Windsurf, and more."*

> *"It solves context rot — the quality degradation that accumulates as an AI fills its context window — by running all heavy research, planning, and execution work in fresh-context subagents while keeping your main session lean."*

### 4.2 5 阶段循环

1. **Discuss** — 实现前捕获决策
2. **Plan** — 研究、分解、验证计划适配 fresh context
3. **Execute** — 计划并行波次执行，每个 executor 200k clean context
4. **Verify** — 走查、诊断、修复
5. **Ship** — PR / 归档 / 下一轮

### 4.3 EnjoyFlow vs GSD Core 一手对比

| 维度 | GSD Core | EnjoyFlow |
|---|---|---|
| 入口 | `npx @opengsd/gsd-core@latest` | 仓库 + 脚本 |
| 适配工具 | 9 个 | 4 个 |
| spec-driven 5 阶段 | ✅ 核心 | ✅ 核心 |
| Context rot 对位 | ✅ **明文写** | ⚠️ 隐性 |
| **跨项目知识共享** | ❌ | ✅ shared/ + inherits: |
| **文档资产分层** | ❌ 代码优先 | ✅ 三层一体 |
| **防腐化机制** | 弱（fresh-context 规避） | 强（progress + sync_memory + shared 准入） |
| 团队协作 | 单人 | 4 人角色契约 |
| Breunig 共识语言 | 部分使用 | **未使用** |

### 4.4 关键观察

**GSD Core 已经在用 Breunig 共识语言（context rot, fresh-context subagents），而 EnjoyFlow 没有**。这是传播层面的隐性劣势——同样一个机制，GSD Core 用了圈内有共识的词，EnjoyFlow 用自己的术语，传播成本高。

---

## 5. 其他直接竞品

### 5.1 Superpowers (obra/superpowers)

- **定位**: "An agentic skills framework & software development methodology that works"
- **数据**: 233,323 stars（8 个月内）
- **机制**: 可组合 skills + session-start hook + subagent-driven-development + 红绿 TDD + YAGNI/DRY
- **商业**: Primera Radian 公司全职维护 + 招社区工程师
- **短板**: 不管项目知识怎么组织、跨项目怎么共享

### 5.2 GitHub spec-kit (github/spec-kit)

- **定位**: "💫 Toolkit to help you get started with Spec-Driven Development"
- **数据**: 114,151 stars
- **机制**: `specify init` + Constitution/Specify/Plan/Tasks/Implement 流程 + 多 AI 工具适配
- **关键**: **GitHub 官方**在做 → spec-driven 已是**平台共识**
- **短板**: spec 制品本身，没管知识沉淀和跨项目

### 5.3 OpenSpec (Fission-AI/OpenSpec)

- **定位**: "The most loved spec framework"
- **数据**: 55,690 stars / 3,902 forks / 10 个月
- **机制**: `/opsx:propose → /opsx:apply → /opsx:archive` 三步
- **哲学**: fluid not rigid / iterative not waterfall / brownfield-first
- **短板**: spec 模板工具，不管知识共享和防腐化

### 5.4 Cline / Roo Code

- **定位**: IDE 内的 autonomous coding agent
- **数据**: Cline 63,538 stars / Roo 24,246 stars
- **短板**: 单 agent 工具，不是项目管理框架

---

## 6. 生态位总结

### 6.1 拥挤的红海（不建议直接撞）

- **Spec-driven**：spec-kit 11.4 万 + OpenSpec 5.5 万 + spec-workflow-mcp 4K + N 个小厂
- **Single agent**：Cline / Roo / Cursor / Continue 等
- **Skills 框架**：Superpowers 23 万 + Claude Code Skills + Cursor Skills

### 6.2 空白带（真正的机会）

- **AI Memory for Engineering Teams** — 全部 < 300 stars，没有强势品牌
- **Harness Engineering** — 词刚被命名（awesome-harness-engineering 1939 stars）
- **跨项目知识共享机制** — Superpowers/spec-kit/OpenSpec 都不做

### 6.3 EnjoyFlow 占哪块

| EnjoyFlow 现有机制 | 对应 Breunig 共识语言 | 对应市场空白 |
|---|---|---|
| progress.md 进度账本 | Context Offloading | AI Memory 赛道 |
| 六类知识分离 + 分片加载 | Context Quarantine + Pruning | Harness Engineering |
| `inherits:` + shared 准入 | Tool Loadout（按任务选） | 跨项目知识共享 |
| sync_memory.py 四段巡检 | Context Pruning（自动化） | AI Memory 赛道 |
| 独立验证 | 防 Context Poisoning | 验证纪律 |
| active-sprint 物理隔离 | Context Quarantine | Harness Engineering |

**结论**：EnjoyFlow 的 6 大机制**完全覆盖** Breunig 命名的核心修复机制 + AI Memory 空白带，但**没用共识语言**包装自己，**没占领任何明确赛道**。

---

## 7. 流行性预判（产品定义阶段）

### 7.1 借鉴 Superpowers 怎么爆的

| Superpowers 做了什么 | 解释 |
|---|---|
| **早期挂在 Claude Code 官方 plugin 市场** | 进入门槛 0 |
| **取了一个"自嘲但鲜明"的名字** | "superpowers" 暗示"超级能力"，传播有钩子 |
| **明确单一价值主张** | "skills framework"——一句话能说清 |
| **README 头部有自陈哲学** | "step back and asks you what you're really trying to do"——情感钩子 |
| **公司化运营** | Primera Radian 招人 + 商业支持 |
| **跨多 AI 工具** | Claude Code / Codex / Cursor / Gemini CLI / Pi 等 |

### 7.2 EnjoyFlow 现状 vs Superpowers 路径

| 维度 | Superpowers | EnjoyFlow |
|---|---|---|
| 进入门槛 | 0（plugin 一键） | 高（仓库 + 纪律） |
| 名字钩子 | "superpowers" 自嘲鲜明 | "enjoyflow" 无钩子（且 Flow 含义未明）|
| 一句话价值 | "skills framework" | 还在变 |
| README 钩子 | "step back" 哲学陈述 | 文档完整但**无情感钩子** |
| 跨工具 | 已 9 个工具 | 已 4 个 |
| 商业化 | 公司化运营 | 无 |

### 7.3 流行性缺口（关键）

1. **名字钩子弱** — "enjoyflow" 没有认知抓手；"Flow" 含义未明
2. **进入门槛高** — clone 仓库 + 部署 .mdc + 维护纪律 vs 一键 plugin
3. **共识语言未对接** — Breunig / Schmid / Anthropic 命名了 context engineering 整套语言，EnjoyFlow 没用
4. **缺 demo 案例** — Superpowers/GSD/spec-kit 都有"5 分钟跑通"示例
5. **README 缺情感钩子** — 详细但**没有"为什么我要停下来读这段"的钩子**

---

## 8. 待办（如果 EnjoyFlow 想抓住 2026 这个窗口）

按重要性排：

| # | 优先级 | 事 |
|---|---|---|
| 1 | P0 | **用 Breunig/Schmid 共识语言重写对外文案**——context rot / context quarantine / context pruning / context offloading / tool loadout —— 现有机制直接对应 |
| 2 | P0 | **明确占领"AI Memory for Engineering Teams"赛道**——把 shared/ + 知识库分层定位成 AI 项目的"记忆层" |
| 3 | P0 | **降低进入门槛**——出 `npx @enjoyorg/enjoyflow` 一键 bootstrap，参考 GSD Core 形态 |
| 4 | P1 | **起一个 Hook 名字**（"enjoyflow" 这个名字传播性弱，或定义 Flow，或起个新名） |
| 5 | P1 | **做 1-2 个 5 分钟 demo**——Superpowers 早期靠 demo 病毒传播 |
| 6 | P1 | **出 README 顶部钩子段**——参考 Superpowers "step back and asks you" 哲学钩子 |
| 7 | P2 | **5 阶段流程可能不合理**——GSD 是 Discuss/Plan/Execute/Verify/Ship，更"敏捷"；EnjoyFlow 是 需求/规约/设计/实现/验证，更"瀑布"，需评估 |
| 8 | P2 | **知识库六分类可能过细**——小团队可能用不到，需要"最简三分类"起步套件 |
| 9 | P2 | **跨 AI 工具适配**目前仅 4 个，需扩到 6-8 个 |

---

## 9. EnjoyFlow 当前定位问题（产品定义阶段）

### 9.1 现状

| 维度 | 现状 | 问题 |
|---|---|---|
| 一句话 | "AI 驱动全生命周期工程化框架" | 抽象，没差异化 |
| 生态位 | 无明确赛道 | 跟 spec-kit/OpenSpec/GSD 都撞赛道 |
| 共识语言 | 自有术语（progress.md、active-sprint、shared/） | 不接 Breunig/Schmid 暗号 |
| 名字钩子 | "EnjoyFlow" | Flow 含义未明 |
| 流行性路径 | 无 | 没 plugin 市场 / 没 demo / 没情感钩子 |

### 9.2 该重新决定的根本问题

1. **EnjoyFlow 是 spec-driven 框架吗？** 如果是，直接撞 spec-kit 11.4 万 + OpenSpec 5.5 万，没胜算
2. **EnjoyFlow 是 AI Memory for Engineering Teams 吗？** 如果是，占领真正的空白带，但需重写定位
3. **EnjoyFlow 是 Harness Engineering 框架吗？** 如果是，跟刚被命名的赛道同跑，可能赢得定义权
4. **EnjoyFlow 到底是什么？** 这件事必须先回答，后面才有传播策略

---

## 10. 数据源

- HN Algolia API: https://hn.algolia.com/api
- GitHub REST API: https://api.github.com
- 各项目官方 README（一手）
- Drew Breunig 原文: https://www.dbreunig.com/2025/06/22/how-contexts-fail-and-how-to-fix-them.html
- Drew Breunig 修复篇: https://www.dbreunig.com/2025/06/26/how-to-fix-your-context.html
- Phil Schmid 原文: https://www.philschmid.de/context-engineering
- Anthropic 官方: https://www.anthropic.com/engineering/building-effective-agents

---

*文档版本: v1.0 | 创建日期: 2026-06-19 | 维护者: 项目负责人*


---

# 附录：MARKET-RESEARCH-ADDENDUM-1-2026-06-19


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
| 9 | **DammyEnjoy93/interface-design** | 5,091 | "Design engineering for Claude Code. **Craft, memory, and enforcement** for consistency" |
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

**EnjoyFlow 该不该直接竞争 ECC？**
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
- 但EnjoyFlow 需要先**自己做成 plugin 形态**才能被收录（不是文档仓库）
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

## 9. 下一步建议（不动手，等 Enjoy 决定）

| 选项 | 内容 |
|---|---|
| A | 重新评估 v3 定位稿（基于二轮调研修正差异化锚点）|
| B | 继续挖 Karpathy LLM Wiki 模式 + Manus / Drew Breunig 实践 |
| C | 跟 basic-memory / obsidian-mind 做直接对比，找出 EnjoyFlow 还能立足的角度 |
| D | 暂停调研，开始做 demo 验证双向翻译机制是否真的成立 |
| E | 其他（Enjoy 指定）|

---

*文档版本: v1.0 | 创建日期: 2026-06-19 | 关联文件: MARKET-RESEARCH-2026Q2.md / POSITIONING.md / DISCUSSION-LOG-2026-06-19.md*


---

# 附录：MARKET-RESEARCH-ADDENDUM-2-2026-06-19


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
| 触发原因 | Enjoy 把定位升级为 v4 "接口规范 + 默认实现"哲学，需要找真实参照系 |
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

## 8. Enjoy 工具栈相关发现

### 8.1 Hermes Agent

- **NousResearch/hermes-agent** —— **197,655 stars**
- Enjoy 用的工具
- EnjoyFlow 是 Enjoy 开发的——enjoyflow 跟 Hermes 是**同一个生态**

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

### 9.4 Enjoy 工具栈相关

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


---

# 附录：MARKET-RESEARCH-ADDENDUM-3-2026-06-19


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
- EnjoyFlow v4 如果只做接口规范而不做生成器，**会被边缘化**

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

**EnjoyFlow v4 的策略选项**：
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

**跟EnjoyFlow v4 的对位**：
- Backstage = 企业级"开发者门户"框架
- EnjoyFlow = AI 时代的"项目文档与记忆"接口
- Backstage 已被 CNCF 收编 = 基础设施级背书
- **EnjoyFlow 不应跟 Backstage 竞争，应作为 Backstage 的适配器**

### 2.2 DITA — 企业级技术文档标准（445 stars）

**DITA Open Toolkit** —— 来自 IBM 的技术文档标准
- 现实使用：航空、汽车、医疗器械、硬件行业的标准
- AI 时代：**几乎没人包装它**

**EnjoyFlow v4 的策略选项**：
- 可以提供 DITA adapter，让传统企业用户的 DITA 文档能被 AI 读
- 或明确"EnjoyFlow 不做 DITA 兼容"

### 2.3 ADR — 16,145 stars（制品层最成熟标准）

| 项目 | Stars | 角色 |
|---|---|---|
| **architecture-decision-record/architecture-decision-record** | **16,145** | "ADR examples for software planning, IT leadership" |
| **npryce/adr-tools** | 5,525 | ADR CLI 工具 |
| **adr/madr** | 2,275 | "**Markdown Architectural Decision Records**" |

**关键观察**：
- ADR 是**制品层**的真实成熟标准（16K stars）
- **EnjoyFlow v4 的 ArtifactAPI 应该接 ADR，不重新发明轮子**
- EnjoyFlow 自己的 `development/decisions/` 目录可以采用 ADR 规范

### 2.4 Architecture as Code

| 项目 | Stars | 角色 |
|---|---|---|
| **finos/architecture-as-code** | 341 | "Architecture as Code (AasC)" |
| **SlavaVedernikov/C4InterFlow** | 245 | "Architecture as Code (AaC) framework using C4 model" |
| **inherd/forming** | 26 | "轻量级架构即代码语言" |
| **sruja-ai/sruja** | 22 | "**Context Engineering and Architecture Intelligence for the AI Era**. Machine-readable, continuous..." |

**关键观察**：
- 架构即代码方向**真实存在**但没人做出强势品牌（最高 341 stars）
- sruja 22 stars 提出 **"Context Engineering and Architecture Intelligence for the AI Era"**——跟EnjoyFlow v4 直接撞概念
- **EnjoyFlow v4 有机会占领"AI 时代的架构即代码"空白**

---

## 3. AI 文档 / Spec 模板生成（最直接对位空白）

### 3.1 cogeet-io/ai-development-specifications — 34 stars

**唯一直接对位的项目**：
> "Complete specifications for AI-assisted development: **Spec as Code, Testing as Code, Documentation...**"

**关键观察**：
- AI 开发规范领域**只有 34 stars**——**真实空白**
- EnjoyFlow v4 的 DocAPI + ArtifactAPI **就是这个位置**

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
- **EnjoyFlow v4 可以考虑用 TOON 作为默认传输格式**（而不是 JSON 或 YAML）

### 4.3 EnjoyFlow 的传输格式选项

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

**Markdown frontmatter schema 验证是真实空白**——EnjoyFlow v4 的 DocAPI 可以**定义 frontmatter schema + 验证工具**，填补这块。

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

### 6.2 EnjoyFlow v4 是否需要做 SDK？

| 选项 | 内容 | 参照 |
|---|---|---|
| **A. 只做规范** | 不做 SDK，让 AI 工具团队自己适配 | LSP 模式 |
| **B. 出 TypeScript/Python SDK** | 让用户写 enjoyflow adapter 时有 SDK | MCP 模式（87K servers 自带 SDK）|
| **C. 出 CLI + SDK** | 同时出 CLI（npx @enjoyorg/enjoyflow）和 SDK | OpenAPI Generator 模式 |
| **D. 出 Schema + Generator + SDK** | 完整三件套 | OpenAPI 完整生态模式 |

---

## 7. EnjoyFlow v4 接口规范的参照系全景

### 7.1 已成熟接口协议（应接不重做）

| 协议 | 状态 | EnjoyFlow 对位 |
|---|---|---|
| **MCP** (Anthropic) | 8K + 87K servers | 接作为 HookAPI 的一部分 |
| **Agent Skills SKILL.md** (Anthropic) | 152K stars | enjoyflow.yaml 应直接采用 SKILL.md 模式 |
| **AGENTS.md** | 22K stars | 接作为 AgentAPI 的一种实现 |
| **DESIGN.md** (Google Labs) | 16K stars | 接作为 ArtifactAPI 的一种实现（YAML frontmatter）|
| **OpenAPI** | 31K stars + 26K generator | 接口规范的元标准，EnjoyFlow 借用其方法论 |
| **LSP** | 13K stars | 借其"接口+实现"成功模式 |
| **ADR / MADR** | 16K + 2K stars | ArtifactAPI 接 ADR，不重做决策记录 |
| **TOON** | 24K stars | 评估作为默认传输格式 |
| **SBOM / SPDX / CycloneDX** | 多家生态 | artifact 安全标准的接口，EnjoyFlow 可接 |

### 7.2 真实空白（EnjoyFlow v4 应该占）

| 空白 | 程度 | EnjoyFlow 的机会 |
|---|---|---|
| **DocAPI**（项目文档接口规范）| ★★★★★ 真实空白 | 直接定义 |
| **ArtifactAPI**（项目制品接口规范）| ★★★★★ 真实空白 | 直接定义 |
| **Markdown frontmatter schema 验证** | ★★★★☆ | 提供 schema 规范 + 验证工具 |
| **AI 时代的"架构即代码"规范** | ★★★★☆ | 占领 sruja 22 stars 的空白 |
| **AI development specifications** | ★★★★★ 真实空白 | cogeet-io 34 stars 已在 |

### 7.3 已成熟但EnjoyFlow 应主动接

| 项目 | Stars | EnjoyFlow 的接法 |
|---|---|---|
| **backstage/backstage** | 33,655 | DocAPI 适配器 |
| **OpenAPI Generator** | 26,427 | EnjoyFlow 应自带类似工具（or 接） |
| **mem0** | 58,941 | MemoryAPI 适配器 |
| **AppFlowy / AFFiNE** | 72K / 69K | DocAPI 适配器 |
| **Notion / Confluence / Obsidian** | 已成熟产品 | DocAPI 适配器 |

---

## 8. EnjoyFlow v4 接口规范的设计建议

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

### 8.2 EnjoyFlow 应该填补的空白

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
