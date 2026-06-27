# v3 整合：for Coding 深度设计（rule / knowledge / workflow 落地）

**整合日期**：2026-06-27（同一会话第三轮）
**本轮主题**：在 Core + 3 机制协同已清楚的基础上，深度讨论 for Coding 领域应用层设计
**Codex v3 状态**：用 `--sandbox workspace-write` 修复了 v2 的 sandbox helper bug，能读项目，回答完整
**Claude v3 状态**：完整读了项目，引用具体行号
**输入**：6 个深度问题（Q1 目的 / Q2 场景 / Q3 知识类型 / Q4 工作流 / Q5 最佳实践 / Q6 MVP 边界）

---

## 🟢 高度共识（两个 AI 完全一致 = 关键决策）

| # | 共识 | 关键证据 |
|---|---|---|
| **V3-C1** | **for Coding = "AI 编程工具的共享上下文层"**（不是 Obsidian / Linear / Copilot Workspace） | codex: ".gitmodules + .editorconfig" · claude: "AI 时代 .env 的位置" |
| **V3-C2** | **Core 管"格式是否正确"，for Coding 管"内容是否有用"** | 两个完全一致 |
| **V3-C3** | **Q1 验证标准**：新 AI 工具 30 秒读完 AGENTS.md 能正确完成任务 | codex: 30 秒 · claude: 3 分钟（采纳 codex 30 秒） |
| **V3-C4** | **现有 5-6 类分类基本够用**，但都有结构性缺口 | 两个都提到"patterns/ 语义模糊"和"缺 conventions/ 或 context/" |
| **V3-C5** | **AGENTS.md 的理想长度 = 30-50 行**（做路由，不做存储） | claude: "30-50 行" · codex: "≤ 4000 词"（采纳 claude，更严） |
| **V3-C6** | **knowledge 不能替代 AI 工具读源码**——是"加速理解"不是"替代理解" | 两个完全一致 |
| **V3-C7** | **Rule 数量 ≤ 8 条**（多了 AI 记不住） | codex 明确 · claude 暗示 |
| **V3-C8** | **Gotcha 必须 if-then 格式**（带 trigger 字段） | 两个完全一致 |
| **V3-C9** | **Decision 必须有 reversible 字段 + 写"为什么"** | 两个完全一致 |
| **V3-C10** | **MVP 必含 = add + search + 路由 + AGENTS.md 同步 + doctor 3 项基础检查** | 两个 80% 重合 |
| **V3-C11** | **永不做：AI 自动生成 gotcha / LLM 扩写 knowledge / 知识质量评分** | 两个完全一致 |
| **V3-C12** | **GOTCHA 单条 ≤ 100 词 + doctor 强制** | 两个完全一致 |

---

## 🟡 设计细节分歧（需要你定）

### D3-1：for Coding 定位的精确措辞

| codex | claude |
|---|---|
| "AI 时代的 .gitmodules + .editorconfig"（技术工具类比） | "AI 时代的 .env 的位置"（配置文件类比） |

**两者都准确**——codex 强调"配置工具链"，claude 强调"上下文配置"。

### D3-2：核心场景数量

| codex | claude |
|---|---|
| 7 个 | 6 个 |

**重叠 4 个**（Onboard / Capture Gotcha / Enforce Rule / Sync）+ codex 多 3 个（Make Decision / Plan Dev / Doctor）、claude 多 2 个（PR 前置 / 任务拆解）。

**推荐合并为 6 个**（采纳两者并集去重）：
1. Onboard Agent
2. Capture Gotcha
3. Enforce Rule
4. PR Preflight
5. Sync to AI Tools
6. Doctor Check

### D3-3：知识类型新增

| codex 建议 | claude 建议 |
|---|---|
| `conventions/`（命名规范、目录约定、commit 格式） | `conventions/` 或 `context/`（运行时上下文：环境变量、端口、服务依赖） |

**两个互补**——codex 强调"代码层约定"，claude 强调"运行时上下文"。**两个都加**。

### D3-4：工作流 5 个的命名

| codex | claude |
|---|---|
| Onboard / Capture / Pre-Gen Check / Arch Review / Rot Check | onboard / prd-preprocess / preflight / capture / sync |

**两者本质相同**，叫法不同。建议**统一**为：onboard / capture / preflight / review / sync。

### D3-5：MVP "必须做"列表

| codex 5 必须 | claude 4 必须 |
|---|---|
| AGENTS.md 聚合 + sync 校验 | add + search + 路由 |
| Gotcha 捕获 + if-then 模板 | AGENTS.md 生成 + 同步 |
| Rule managed section + ACID sync | doctor（3 项基础） |
| 4000 词上限 + doctor | frontmatter 校验 |
| ls/grep/cat 基础查询 | — |

**采纳并集** = 6 项（`add` + `search` + 路由 + `AGENTS.md` 同步 + doctor 3 项 + 4000 词上限）。

---

## 🔴 重要发现（必须告诉你）

### F6：knowledge 的作用被高估，AI 读源码不能被替代 ⚠️

**Claude 直接指出**：
> "knowledge 的目的是**加速 AI 工具的理解**，不是**替代 AI 工具的理解**。AI 工具最终还是要读源码——knowledge 只是告诉它'先看哪、注意什么、别看哪'。"

**这意味着 for Coding 的成功标准不是"AI 工具读 knowledge 就够"，而是"AI 工具读了 knowledge 后读源码更快、更准"**。

### F7：100 词上限不只是约束，是"信号纯度"机制 ⚠️

**Claude 提出**（最反直觉判断）：
> "**不完美的知识 > 没有知识**。一条 20 词的 gotcha 价值远超一条 500 词被放弃的 gotcha。强制 100 词上限是'逼迫写清楚'的手段，不是限制。"

**更激进**：
> "如果用户连 100 词都写不出，说明 ta 没想清楚，不该记。这条应**永远不做** LLM 辅助生成 knowledge。"

### F8：5 个工作流间的依赖关系 ⚠️

**Codex 指出**（最容易被忽略）：
> "**W1（onboard）和 W3（preflight）不是平行的**——如果 W1 没做好（AGENTS.md 过期/超限），W3 的检查结果不可信。"

**含义**：W1 是其他所有工作流的**前置依赖**。Onboard 不做好，其他全部失效。

### F9：Rule 和 Code 不同步 = 最致命反模式 ⚠️

**Codex 评为"最致命"**：
> "Rule 说'API 返回格式必须是 {data, error}'，但代码里混用 {result, msg} 和 {data, error}。AI 读了 rule 生成 {data, error} 格式的代码，但项目现有代码是 {result, msg} 格式→AI 生成的不一致。"

**这意味着**：**for Coding 必须有"Rule-Code 同步检测"**（不是 doctor 通用检查，是 for Coding 专属）。

### F10：现有 6 个目录的"模板"歧义 ⚠️

**Cl 发现新问题**：`knowledge-tasks/` 定位不清——
- 名字暗示"知识的任务"
- 实际用途 = "需要补充的知识清单" 还是 "知识的 TODO"？
- **应该是 doctor 的产物之一**，不是独立分类

**建议**：v0.2 时**重命名**为 `tasks/pending-knowledge.md`（明确"待补充知识清单"的语义）。

---

## 📋 for Coding 整合设计（v3.1）

### 1. 定位与 JTBD

> enjoyknowledge for Coding = **AI 编程工具的共享上下文层**。
>
> **JTBD**：让任意 AI 工具在 30 秒内读完 AGENTS.md，正确完成 `add a new API endpoint` 这类任务——不触犯已知陷阱、不打破架构约定、不选错技术栈。

### 2. 6 个核心场景

| # | 场景 | 触发 | 价值 | 失败模式 |
|---|---|---|---|---|
| 1 | **Onboard Agent** | AI 工具首次读 AGENTS.md | 消除"AI 瞎猜架构" | AGENTS.md 超限或过期 |
| 2 | **Capture Gotcha** | 开发者发现隐性坑 | 永久消除同类 bug | 描述模糊/无 trigger 字段 |
| 3 | **Enforce Rule** | AI 工具生成代码前 | "生成时就不违反" | 规则 > 8 条被 AI 忽略 |
| 4 | **PR Preflight** | 提交/PR 前 | CI 之前发现冲突 | 只匹配文件名不匹配语义 |
| 5 | **Sync to AI Tools** | 改 rule/加 knowledge | 8 个工具一次同步 | 同步不完整导致工具间不一致 |
| 6 | **Doctor Check** | pre-commit / CI | 防止知识腐烂 | 假阳/假阴/被忽略 |

### 3. 知识类型（修订 6+2）

```
.enjoyknowledge/
├── architecture/       # 系统结构 + 约束（不是教程）
├── gotchas/            # IF-THEN 触发器（必须 trigger 字段）
├── patterns/           # 仅"本项目特有"的模式（不存通用模式）
├── decisions/          # ADR（必须 reversible + "为什么"）
├── business/           # 业务规则（最稳定）
├── contracts/          # 跨模块接口契约（新增 ⭐）
├── conventions/        # 命名/目录/commit 格式（新增 ⭐ codex 建议）
├── context/            # 运行时：env 变量/端口/服务依赖（新增 ⭐ claude 建议）
└── knowledge-tasks/    # 待补充知识清单（v0.2 重命名 → tasks/pending.md）
```

**知识类型依赖图**：

```
business/           ← 顶层约束（最稳定）
    ↓ 约束
architecture/       ← 静态结构
    ↓ 产生
contracts/ + patterns/ + gotchas/    ← 具体化
    ↓ 影响
decisions/          ← 上下文
    ↓ 派生
context/ + conventions/    ← 底层（被所有层引用）
```

### 4. 5 个工作流（依赖图）

```
onboard (W1) ──── 唯一前置依赖
    ↓
    ├──→ prd-preprocess (W2)  ← 每次新需求
    │       ↓
    │       └──→ preflight (W3) ← 每次变更前
    │               ↓
    │               └──→ capture (W4) ← 每次有价值的产出
    │
    └──→ sync (W5) ← AI 工具切换时（低频）
```

**主动 vs 被动分层**：
- **被动**（AGENTS.md 触发）：W1 onboard
- **主动**（用户/AI 显式调用）：W2/W3/W4/W5
- **设计原则**："读"走被动；"写"走主动

### 5. 强制 / 禁止 / 可选（for Coding 默认策略）

| 强制 | 禁止 | 可选 |
|---|---|---|
| Rule ≤ 8 条 | AGENTS.md > 50 行 | 团队自定义模板 |
| Gotcha ≤ 100 词 + trigger 字段 | "注意性能"类空洞条目 | capture 触发方式（手动/自动建议） |
| Decision 必含 reversible 字段 | 直接删 gotcha（必须先走 doctor 确认） | tasks/ 启用与否 |
| 体积 ≤ 4000 词（doctor 检测） | Rule 写代码风格（留 formatter/linter） | 跨项目知识共享 |
| `doctor` 必接 CI 且 fail=block | AGENTS.md 写"项目 README 第二份" | AI 工具主动 capture |
| Doctor 检测"Rule-Code 同步" | gotcha 50+ 文件不收敛 | 知识图谱可视化 |

### 6. MVP 边界（v0.1）

**🔴 必须包含**（6 项）：
1. `add` + 路由（自动判断 gotcha/pattern/decision）
2. `search` + frontmatter filter
3. AGENTS.md 生成 + 同步（managed section）
4. `doctor` 3 项基础（frontmatter 有效 / 体积上限 / 链接完整）
5. 4000 词硬上限 + 100 词单条
6. frontmatter 必填校验

**🟡 应包含但可简单做**（5 项）：
1. `capture` 基础版（手动 `/remember`）
2. `preflight` 基础版（只做路径匹配，不做语义匹配）
3. 多 AI 工具同步（先支持 Claude + Copilot 2 种）
4. `pattern` 全文搜索 + tags filter
5. 简单 `KNOWLEDGE-INDEX.md` 目录生成

**🟢 延后到 v0.2/v0.3**（5 项）：
1. 语义级 preflight（embedding）
2. 跨项目知识共享
3. AI 工具主动建议 capture
4. 知识过期自动检测（对比 package.json 版本）
5. 知识图谱可视化

**⚫ 永不做 / 靠生态**（5 项）：
1. 在线知识库托管（不是 SaaS）
2. 协作编辑 / 实时同步（Git 解决）
3. **AI 自动生成 gotcha**（违反 F7 信号纯度）
4. 知识"质量评分"（错误激励）
5. **LLM 扩写 knowledge**（违反 100 词约束）

---

## ⚠️ 关键风险（v3 轮）

### R4：100 词上限可能让规则不可用

有些 rule 不可能 ≤ 100 词（例如"API 错误码体系"有 20+ 错误码定义）。**建议**：把"100 词"理解为"**单条决策/单条 gotcha**"，不是"单条 rule"——复杂 rule 拆成多条。

### R5：AGENTS.md 30-50 行 vs 4000 词约束冲突

4000 词 = ~3000 行（按 1.3 词/字算），但 30-50 行 = 1000-2000 词。**采纳 30-50 行约束更严**——超过 30 行必须用 managed section 折叠到子文件。

### R6：5 个工作流都需要实现 MVP 支持

**采纳 W1 + W3 + W4 + W5**（4 个）进 MVP。**W2 (prd-preprocess) 延后到 v0.2**——它是产品化最深的工作流，先把基础做好。

---

## 📂 完整记录

- **整合文档**：`C:\Users\jay\Documents\why-workspace\daily\2026-06-27-enjoyknowledge-rule-design-integration.md`
- **Codex v3 log**：`C:\Users\jay\AppData\Local\Temp\ek-codex-fc.log`（761 行）
- **Claude v3 log**：`C:\Users\jay\AppData\Local\Temp\ek-claude-fc.log`（321 行）
- **B 站原文**：https://mp.weixin.qq.com/s/YH5iiaW7OQ9AvXtZFUyK-A

---

## 🔗 下一步

1. **执行 for Coding v0.1 MVP**（按 §6 表的 6 项必须 + 5 项简单）
2. **补 `docs/architecture/for-coding-design.md`**（覆盖 v3.1 全部 6 节）
3. **重命名 `knowledge-tasks/` → `tasks/pending.md`**（F10）
4. **加 `conventions/` + `context/` 目录**（D3-3）
5. **加 `Rule-Code 同步检测`**（F9 的"最致命"反模式）
6. **更新 memory**（for Coding 定位 / 30-50 行约束 / Rule ≤ 8 条）


---

