# enjoyknowledge 定位宣言

> 回答：enjoyknowledge 在知识资产生态里占什么位，Core 和 for Coding 怎么分工，为什么用户应该选它。

---

## 1. 一句话定位

> **enjoyknowledge Core 是知识资产的管理层；enjoyknowledge for Coding 是它面向 AI 编程的第一个领域应用。**

三个关键词：

| 关键词 | 排除了什么 |
|---|---|
| **知识资产** | 只管理"该写什么"的文件——架构、决策、踩坑、业务规则。不管理代码 |
| **工程化** | 手写 Markdown 没有 schema。enjoyknowledge 给文件加上格式校验、过期检测、结构搜索 |
| **Core / 应用分层** | Core 不写死目录名。for Coding 只是第一个领域应用——换应用不伤引擎 |

对用户的说法：

| 层 | 用户该怎么理解 | 当前形态 |
|---|---|---|
| **enjoyknowledge Core** | 通用知识资产引擎 | OKF 格式、目录索引、frontmatter 校验、`ls`/`grep`/`cat`/`add`/`doctor` |
| **enjoyknowledge for Coding** | AI 编程知识应用 | `architecture/`、`gotchas/`、`patterns/`、`business/`、`decisions/`、`knowledge-tasks/`、AGENTS.md |
| **未来 for X** | 同一个 Core 上的其他应用 | `for support`、`for research`、`for sales`、`for legal` 等 |

核心机制：**推送 + 拉取双通道**。AGENTS.md 内嵌知识摘要（推送），AI 被唤醒时就看到了。需要深入时通过 `ls`/`grep`/`cat` 按需拉取。

它不是：
- ❌ AI 编码工具（Cursor / Copilot / Claude Code）
- ❌ 通用知识库（Notion / Confluence）
- ❌ 通用 AI memory（mem0 / MemOS）
- ❌ Spec 框架（spec-kit / OpenSpec）
- ❌ 项目管理工具（Jira / Linear）

它是：
- ✅ **知识资产的管理层**——让知识文件的错误出声，过时文档不再安静腐烂
- ✅ **OKF 兼容的知识目录**——标准 Markdown + YAML frontmatter，任何工具都能消费
- ✅ **Core + for Coding**——Core 管知识资产，for Coding 提供 AI 编程的默认目录、任务暂存区和 AI 入口
- ✅ **Unix 动词复用**——AI 不需要学新 DSL，`ls`/`cat`/`grep`/`add` 就是入口

---

## 2. 根本命题

```
代码有编译器、测试、依赖分析、过期检测
文件资产（架构、决策、业务规则、踩坑）只有 git
  → 格式错误发现不了
    → 描述缺失无人知
      → 与代码不一致时静默腐烂
        → AI 读到过期知识 → 做出错误决策
```

AI 让编码加速 10 倍，编码不再是瓶颈。真正的瓶颈是"知道写什么"和"知道旧决策是否还有效"——而这些知识文件的管理工具链，还停留在手工作坊时代。

**解法：让知识文件的错误出声。** `enjoyknowledge doctor` 是知识文件的 linter——缺 frontmatter、缺 description、条目超标、AGENTS.md 过期，不通过就是不合格。`enjoyknowledge add` 带 schema 创建——不写 frontmatter 就不能提交。`enjoyknowledge grep` 定位到 `##` 段——不是乱序行号。

---

## 3. 核心能力

### 3.1 三层分离

```
Layer 3: 领域应用         ← 领域意见
  内置: architecture/ gotchas/ patterns/ business/ decisions/
  当前: enjoyknowledge for Coding
  未来: for Support / for Research / for Sales / for Legal / ...

Layer 2: 知识管理原语      ← 通用引擎
  enjoyknowledge add     → 带 schema 校验的创建
  enjoyknowledge doctor  → 知识文件的结构检查
  enjoyknowledge grep    → 结构感知的搜索
  enjoyknowledge ls/cat  → 带摘要的浏览

Layer 1: OKF 文件格式      ← 通用格式
  YAML frontmatter (description, tags, timestamp)
  Markdown 正文 (## 段界)
```

引擎不写死目录名——`gotchas/`、`architecture/`、`knowledge-tasks/` 这些名字属于 for Coding，不属于 Core。换领域应用不伤引擎。

### 3.2 推送 + 拉取双通道

**推送通道：** AGENTS.md 内嵌 `ls` 输出，AI 启动时自动加载，零成本感知知识库全貌。

**拉取通道：** `enjoyknowledge ls`/`grep`/`cat`/`add`，AI 需要深入时按需探查。

---

## 4. 跨工具定位

```
┌──────────────────────────────────┐
│  Cursor / Copilot / Claude Code  │  ← AI 工具
└──────────────────────────────────┘
              ↓ AGENTS.md 加载
┌──────────────────────────────────┐
│        enjoyknowledge Core             │  ← Layer 2: 知识管理原语
│   add / doctor / grep / ls       │     (通用，不关心领域)
│   ┌──────────────────────────┐   │
│   │  for Coding (Layer 3)     │   │  ← 当前默认领域应用
│   │  architecture/ gotchas/  │   │     (未来可换 for X)
│   └──────────────────────────┘   │
└──────────────────────────────────┘
              ↓ OKF 兼容格式
┌──────────────────────────────────┐
│   .enjoyknowledge/ 目录（Git 管理）    │  ← Layer 1: 文件资产
│   标准 Markdown + YAML frontmatter│
└──────────────────────────────────┘
```

**enjoyknowledge 不是 AI 工具的替代——是给知识文件加上代码资产同等的工程纪律。**

---

## 5. MVP 路径

```
个人/小项目：enjoyknowledge init → 默认启用 for Coding，3 个目录起步
   architecture/overview.md
   gotchas/
   AGENTS.md（AI 入口）

团队：enjoyknowledge init → for Coding 完整结构，5 个长期知识目录 + 任务暂存区
   architecture/  gotchas/  patterns/
   business/  decisions/
   knowledge-tasks/（短期任务工作区，审核后沉淀）

成熟团队：扩展自定义领域应用 + OKF 合规
```

MVP 不做的事：
- Web UI（CLI 优先）
- 语义检索（`grep` + `ls` 够用）
- MCP 适配器（文件系统是唯一源）

---

## 6. 风险与应对

| 风险 | 应对 |
|---|---|
| AI 工具自建知识管理 | 抢先做 OKF 兼容 + 文件即标准格式——不绑定特定 AI 工具 |
| 团队不愿维护 | CLI 极简，`add` 一行命令；`doctor` 自动发现问题 |
| OKF 自身演化 | 遵循 OKF 规范，不过度扩展 |
| AI 能力进化使推送通道不再独特 | 核心价值在"文件资产的工程化"，不在"AI 上下文盲区"。AI 越强，知识资产越重要 |
