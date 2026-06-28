# enjoyknowledge 定位宣言

> 回答：enjoyknowledge 在 AI 编程知识管理生态里占什么位，Core 和 for Coding 怎么分工，为什么开发者应该选它。
>
> **v4.2 调研 + 双 AI 验证（2026-06-27）**——
> - v4.1 = GitHub 90+ 竞品 + 3 痛点社区证据 + POSITIONING 调研驱动重写
> - v4.2 = codex + claude 双 AI 验证砍功能计划，3 处修订：
>   1. **v0.2 工具 9→2**（非 9→1，2 AI 一致：1 工具 = 杀 thesis）
>   2. **首发工具 = Claude**（非 Cursor，原因：Claude 适配更简单 + 社区更活跃 r/ClaudeCode 90+ 评论 + Jay 工具栈）
>   3. **命令名 sync → export**（1 工具时 sync 撒谎；export 暗示单向导出，留名给未来真 sync）

---

## 1. 一句话定位

> **enjoyknowledge = AI 时代项目的"知识 SoT + 多工具路由层"**。一份 markdown 写一次，多个 AI 工具使用。

不是 AI 工具。不是知识库。**是 AI 工具之间共享的项目知识层**。

| 关键词 | 排除了什么 |
|---|---|
| **项目知识 SoT** | 不管理个人笔记 / 团队聊天 / Notion 文档。专注"项目级"——架构、规则、决策、踩坑 |
| **多工具路由** | 不是单工具原生（Cursor rules 只服务 Cursor，Claude skills 只服务 Claude）。做多个 AI 工具都消费的层 |
| **首发 2 工具** | **v0.2 首发 Claude + Cursor**——Claude 社区更活跃（r/ClaudeCode 90+ 评论"AGENTS.MD standard"）+ 适配更简单（CLAUDE.md 追加 vs .mdc frontmatter）|

**对用户的说法**：
- "我同时用 Claude 写代码 + Cursor 审 PR——这 2 个工具都该知道我们项目用什么架构 / 不许用 .unwrap() / 为什么用 PostgreSQL。**我只想写一次**。"
- "我的 .cursor/rules 和 CLAUDE.md 内容 80% 重复——**维护成本太高**。"
- "我新加一条 rule，**手动复制到 2 个工具入口文件**——任何遗漏就行为分歧。"

**它不是**：
- ❌ AI 编码工具（Cursor / Copilot / Claude Code）——它们是消费方
- ❌ 通用知识库（Notion / Confluence / Obsidian）——那些是给人看的
- ❌ 通用 AI memory（mem0 / MemOS / agentmemory）——那些是"agent 内部记忆"
- ❌ Spec 框架（spec-kit / OpenSpec）——那是"项目规范流程"
- ❌ 项目管理工具（Jira / Linear）——那是"任务追踪"

**它是**：
- ✅ **项目知识的工程化层**——给 markdown 加 schema / doctor / sync
- ✅ **跨 AI 工具的路由层**——一份 markdown → 多个 AI 工具入口
- ✅ **AGENTS.md 标准的实现参考**——r/ClaudeCode 90+ 评论共识

---

## 2. 根本命题（社区验证的 3 大痛点）

| 痛点 | 社区证据 | 普遍度 | enjoyknowledge 答 |
|---|---|---|---|
| **A. 跨工具同步成本** | r/vibecoding 14 答案"switching AI tools kills flow" / r/GithubCopilot 3 答案"copying same AI instruction files into every repo" | 7-8/10 | 1 份 markdown export 到多个工具 |
| **B. AI 不知道项目架构** | r/ClaudeCode 90 评论"AGENTS.MD standard" / r/cursor 10 评论 40%→92% 合规率 | **9/10** | frontmatter + schema + AGENTS.md 路由 |
| **C. 任务临时文件无归处** | ECC 222K ★ "memory + sessions" 二分模型 / mattpocock 147K ★ "Skills for Real Engineers" | 5-6/10 | `.enjoyknowledge/` 长期 + `knowledge-tasks/` 短期 |

**关键洞察**：
- 痛点 B 是行业共识（AGENTS.MD 标准化趋势）——窗口期
- 痛点 A + C 已被巨头验证概念（ECC 222K / mattpocock 147K / multica-karpathy 183K）
- 现有竞品**都没做到 3 维组合**（多工具 + 项目级知识 SoT + YAML 工作流）

---

## 3. 竞品定位

| 维度 | enjoyknowledge | ECC (222K ★) | planning-with-files (24K ★) | ai-rules-sync (124 ★) |
|---|---|---|---|---|
| **定位** | 项目知识 SoT + 多工具路由 | Agent harness 性能优化 | 任务级文件规划 | Rule 同步 |
| **工具支持** | ✅ 多工具（v0.2 首发 2：Claude + Cursor）| 部分 | ❌ | 部分（3-4 工具）|
| **frontmatter schema** | ✅（10 类知识 + filter 语法）| ❌ | ❌ | ❌ |
| **YAML 工作流** | ✅（onboard/capture）| ❌ | ❌ | ❌ |
| **项目级 vs 任务级** | 项目（+ knowledge-tasks 短期）| Agent | 任务 | Rule only |
| **作者背景** | Jay 自身痛点驱动 | Affaan（agent 性能）| AI 元老 | 个人项目 |

**差异化 = 3 维组合没人做**：
1. **多工具支持**（竞品平均 3-4；v0.2 首发 2 工具，架构上保留 9 工具 adapter trait）
2. **frontmatter schema**（竞品没有）
3. **YAML 工作流**（竞品没有）

---

## 4. 核心能力

### 4.1 3 层架构

```
Layer 3: 多工具入口（v0.2 首发 2 个 = Claude + Cursor）
  - Claude: .claude/skills/*.md
  - Cursor: .cursor/rules/*.mdc
  - 其他 7 工具（Codex / Copilot / Windsurf / Cline / Trae / Gemini / Generic）: v0.3+ 渐进
  ↓ export (路由表模式，不复制 SoT 内容)

Layer 2: 项目知识 SoT
  - .enjoyknowledge/
    - rules/  (约束, sync)
    - templates/  (范式, sync)
    - knowledge/ (长期)
      - architecture/  gotchas/  patterns/  decisions/  business/  contracts/  conventions/  context/
    - knowledge-tasks/  (短期任务暂存)
    - workflows/  (YAML 工作流定义)
  ↓ core 命令

Layer 1: 文件系统
  - markdown + YAML frontmatter + git
```

### 4.2 核心机制

- **单一 SoT** —— v0.2 首发 2 工具（Claude + Cursor）入口都从 .enjoyknowledge/ export 生成，不各自维护
- **10 类知识资产** —— rule / template / architecture / gotcha / pattern / decision / business / contract / convention / context
- **frontmatter schema** —— 部分必填（rule/template 的 applies_to / gotcha 的 trigger / decision 的 reversible+decided_at）
- **YAML 工作流** —— v0.2 2 个核心工作流（onboard / capture），用户可加文件 = 加工作流
- **4 维 health check** —— doctor 检查 schema / 体积 / 必填字段 / 多工具 export 一致性（**v0.2 砍 5→4**：永久禁用 Rule-Code 一致性）

### 4.3 工具特性保留（v4 哲学 #4）

不做工具统一——保留每个工具的原生语法：
- Cursor `.mdc` 用 `globs` + `alwaysApply`
- Claude skills 用 `description` frontmatter
- Codex 用 `$file:` 引用
- Copilot / Gemini 追加到 `<!-- ek:managed:start -->` 块

---

## 5. MVP 路径（v0.1 当前 → v0.2 收缩）

### 5.1 v0.1 已交付（Core CLI）
`init` / `ls` / `tree` / `grep` / `cat` / `add` / `doctor` / `fix` / `export` / `workflow` —— 10 个 CLI 命令 + profile 系统（v0.2 新增 `export` + `workflow`；`workflow` 下含 `onboard` / `capture` 2 个工作流）

### 5.2 v0.2 砍功能后的 scope（基于 codex + claude 双 AI 验证 + Jay 决策）
**砍**（v4.2 收敛原则）：
- ❌ 9 工具 → **2 工具**（**首发 Claude + Cursor**；2 AI 一致：1 工具 = 杀 thesis；2 工具证明跨工具）
- ❌ 5 工作流 → **2 工作流**（`onboard` + `capture` 是核心）
- ❌ 3 层 scope → **只 project**（team/user 延后）
- ❌ rule_code_sync 检测 → **永久禁用**（NLP 级不可行）
- ❌ **命令名 sync → export**（1 工具时 sync 撒谎；export 暗示单向导出，留名给未来真 sync）

**留**（核心）：
- ✅ 10 类知识资产（完整 frontmatter schema）
- ✅ 路由表模式（AGENTS.md ≤ 50 行）
- ✅ `enjoyknowledge export --tool claude`（首发）+ `--tool cursor`（第二）
- ✅ 2 个核心工作流（onboard + capture）
- ✅ 4 维 doctor（去掉 rule_code_sync）

**延后**（v0.3+）：
- 9 工具完整实现（架构上保留 adapter trait）
- 2 个工作流完整实现
- 语义检索
- 跨项目 / 跨仓库
- Web UI
### 5.3 v0.3+ 路线图（更新）
见 [ROADMAP.md](./ROADMAP.md)（v4.1 同步更新——基于新定位的路线图）

---

## 6. 风险与应对（v4.1 修订）

| 风险 | 应对 |
|---|---|
| **AGENTS.MD 标准化趋势过我们窗口** | 现在发 v0.2 = 抢"项目知识 SoT + 多工具"赛道的标准定义权 |
| **ECC 等巨头切入同空间** | ECC 定位"agent harness"而非"项目知识 SoT"——差异化在项目级；enjoyknowledge 早发 + 早积累 schema 经验 |
| **AI 工具自己统一入口**（OpenAI/Anthropic 推统一 spec）| 可能性低（商业逻辑：让你锁定单工具）；即使发生，enjoyknowledge 作为"项目知识层"独立于工具层 |
| **过度设计**（之前 5 工作流 / 9 工具 / 3 scope）| v0.2 砍到 2 工具 / 2 工作流 / 1 scope；保留 spec 不删，延后实现 |
| **被平台吞噬**（Cursor/Claude 各自加 `sources` 字段就吞掉）| moat = 社区采纳 schema 标准（10 类 frontmatter + filter 语法）+ AGENTS.md 路由表事实参考 |
| **AI 厂商格式迭代维护噩梦**（adapter trait 跟着改）| 没 100+ 用户反馈不会做第 3-9 工具；v0.2 先 2 工具稳定 6 个月再扩 |
| **用户不愿跨工具用 1 套** | r/vibecoding 14 答案已证伪——用户**主动寻找**跨工具方案 |

---

## 7. 一句话推销

> **"一份 markdown，多个 AI 工具。"**（v0.2 首发 Claude + Cursor）
> Claude 写代码、Cursor 审 PR——它们都该知道你们项目用什么架构。
> enjoyknowledge 让 1 份 `.enjoyknowledge/` markdown 自动 export 到多个 AI 工具入口。
> 一次写，无处不在。

---

*文档版本: v4.2 | 最后更新: 2026-06-27 | 调研依据: GitHub API 90+ 竞品 + Reddit r/ClaudeCode/r/cursor/r/vibecoding/r/GithubCopilot 社区证据 + codex + claude 双 AI 砍功能验证*
