# EnjoyFlow 知识架构设计 v2 (Knowledge Architecture)

> 解决"AI 编码时上下文混乱"的问题。设计原则：**按知识类型 ABCD 分离存储，按任务场景按需路由，按 tag 检索。**
>
> **版本演进**:
> v1（基于 6 类 + 4 人团队 + 5 阶段，已废弃）
> **v2（基于 4 大类 ABCD + 32 子类 + tag 驱动 + ContextFlow 上下文供给引擎）**

---

## 1. 核心命题

EnjoyFlow 知识架构的根本问题：

> **"如何在 AI 不可靠的前提下，让 AI 在每个任务都有'刚好够'的上下文知识"**

v1 用"5 阶段流水线"硬路由——**过时**。
v2 用"4 大类 ABCD + 32 子类 + tag 检索 + ContextFlow 上下文供给引擎"——**适应 AI 编程的真实场景**。

### 1.1 v1 的 3 个根本缺陷

```
缺陷 1: 知识类数不够
  v1: 6 类（缺上下文知识）
  → AI 编程产生的会话记录、决策历史、任务进度无处可存

缺陷 2: 阶段路由硬编码
  v1: 5 阶段流水线对应固定文件
  → 实际项目场景多样，"修 bug" 不等于"开发"

缺陷 3: 无自动检索机制
  v1: 人工告诉 AI 读哪些文件 / 或 script 组装
  → 维护成本高，无法处理动态场景
```

### 1.2 v2 的 4 大类

| 大类 | 含义 | 文档数 | 例子 |
|---|---|---|---|
| **A 项目知识** | 项目本身的事实（架构/规约/AC/数据） | 12 类 | ARCHITECTURE.md / GLOBAL_CONTRACT.md |
| **B 业务知识** | 业务领域的事实（规则/术语/流程） | 5 类 | domain-a.md / glossary.md |
| **C 流程知识** | 团队的实践（踩坑/模式/决策） | 8 类 | GOTCHAS.md / PATTERNS.md / ADR |
| **D 上下文知识** | AI 编程产生的痕迹（任务/会话/决策历史） | 7 类 | tasks/ / sessions/ / snapshots/ |

---

## 2. 4 大类 × 32 子类——完整知识图谱

### 2.1 A 项目知识（12 子类）

| # | 子类 | 含义 | 当前 demo 路径 | 优先级 |
|---|---|---|---|---|
| A1 | architecture | 系统架构、模块拓扑 | `project/ARCHITECTURE.md` | P0 |
| A2 | code_standards | **项目代码规范**（技术栈强相关，如 Vue3 组件规范） | `project/CODE-STANDARDS.md` | P0 || A3 | api_contract | 全局 API 契约 | `contract/GLOBAL_CONTRACT.md` | P0 |
| A4 | data_model | 数据模型、表结构 | **新增** `project/DATA-MODEL.md` | P0 |
| A5 | api_spec | 单个 API 详细规约 | `contract/modules/*.md` | P0 |
| **A6** | **requirement** | **PRD 需求内容**（业务目标/AC/数据结构，归 knowledge-base） | **`knowledge-base/project/requirements/`** | **P0** |
| **A7** | **ui_ux_design** | **UI/UX 设计**（原型、交互规范、设计系统、可访问性） | `lifecycle/02-design/*.md` | P0 |
| A8 | adr | **当前生效**的架构决策记录（活文档） | `lifecycle/02-design/ADRs/*.md` | P1 |
| A9 | test_report | 测试覆盖率、报告 | `lifecycle/04-verification/reports/*.md` | P1 |
| A10 | dependencies | 依赖关系图 | **新增** `project/DEPENDENCIES.md` | P1 |
| A11 | environment | 环境配置（dev/staging/prod 的连接串/密钥/基础设施） | `deployment/ENVIRONMENTS.md` | P1 |
| A12 | release | **发布流程与版本管理**（RELEASE.md 唯一归属） | `deployment/RELEASE.md` | P1 |

#### 部署类三维度边界（A11 / A12 / C8）

> 部署相关知识容易散落，三者按**内容类型**严格分离：

| 维度 | A11 environment | A12 release | C8 deployment |
|---|---|---|---|
| **内容类型** | 环境事实（连接串/密钥/基础设施拓扑） | 流程规范（发版步骤/版本号规则/灰度策略） | 操作清单（checklist/回滚步骤/健康检查项） |
| **变更频率** | 低（环境稳定） | 中（每次发版更新版本记录） | 低（清单稳定，偶尔补充） |
| **典型内容** | "prod 库地址=db.prod:3306，redis=redis.prod:6379" | "v1.2.0 发版流程：1.打 tag 2.触发 CI 3.灰度 10%→50%→100%" | "部署前检查：□ 备份 □ 通知 □ 灰度；回滚步骤：1.revert tag 2.重新部署" |
| **物理位置** | `deployment/ENVIRONMENTS.md` | `deployment/RELEASE.md` | `deployment/DEPLOYMENT-CHECKLIST.md` |

**关键约束**——C8 不再"部分"放在 RELEASE.md，独立文件避免内容混杂。

### 2.2 B 业务知识（5 子类）

| # | 子类 | 含义 | 当前 demo 路径 | 优先级 |
|---|---|---|---|---|
| B1 | glossary | 业务术语表 | **新增** `business/GLOSSARY.md` | P0 |
| B2 | business_rules | 业务规则、计算公式 | `business/water-iot.md` 等 | P0 |
| B3 | business_flow | 业务流程、状态流转 | **新增** `business/flows/*.md` | P1 |
| B4 | constraints | 业务约束（合规/性能/法律） | **新增** `business/CONSTRAINTS.md` | P1 |
| B5 | cases | 业务案例 | 散落各文档 | P2 |

### 2.3 C 流程知识（8 子类）

| # | 子类 | 含义 | 当前 demo 路径 | 优先级 |
|---|---|---|---|---|
| C1 | gotchas | 踩坑清单 | `development/GOTCHAS.md` | P0 |
| C2 | patterns | 最佳实践 | `development/PATTERNS.md` | P0 |
| C3 | decisions | **历史决策归档**（已被替代/废弃的 ADR 沉淀） | `development/decisions/*.md` | P0 |
| C4 | test_strategy | 测试策略 | `testing/STRATEGY.md` | P0 |
| C5 | known_issues | 已知问题 | `testing/KNOWN-ISSUES.md` | P1 |
| C6 | team_convention | **团队约定**（跨项目通用习惯，如 commit/PR/分支策略） | **新增** `shared/TEAM-CONVENTION.md` | P1 |
| C7 | review_checklist | 代码审查清单 | **新增** `shared/REVIEW-CHECKLIST.md` | P1 |
| C8 | deployment | **部署操作清单与回滚步骤**（checklist 风格，不重复 A12） | **新增** `deployment/DEPLOYMENT-CHECKLIST.md` | P1 |

#### 规范类双维度边界（A2 / C6）

> 原 v2 在 `project/` 和 `shared/` 都放 CODE-STANDARDS.md，内容重叠。现按**作用范围**分离：

| 维度 | A2 code_standards | C6 team_convention |
|---|---|---|
| **作用范围** | 单项目内（技术栈强相关） | 跨项目通用（团队习惯） |
| **变更触发** | 技术栈升级/重构 | 团队流程调整 |
| **典型内容** | "Vue3 组件用 `<script setup>`，props 用 defineProps" | "commit message 用 conventional commits，分支用 feature/REQ-XXX" |
| **物理位置** | `knowledge-base/project/CODE-STANDARDS.md` | `knowledge-base/shared/TEAM-CONVENTION.md` |

**关键约束**——`shared/` 不再放 CODE-STANDARDS.md，避免与 `project/` 重复。代码规范归 A2，团队流程归 C6。

### 2.4 D 上下文知识（7 子类）——**v2 新增**

| # | 子类 | 含义 | 当前 demo 路径 | 优先级 |
|---|---|---|---|---|
| D1 | task_progress | **任务执行进度**（状态/时间线/里程碑，归 lifecycle） | `lifecycle/01-planning/active-sprint/` | P0 |
| D2 | session_log | AI 会话记录 | **新增** `context/sessions/*.md` | P1 |
| D3 | decision_history | **任务级**临时决策记录（单任务内的取舍） | **新增** `context/decisions/` | P1 |
| D4 | contract_sync | 跨特性契约对齐 | **新增** `context/contract-sync/` | P1 |
| D5 | ai_call_chain | AI 调用链路 | **新增** `context/ai-calls/*.md` | P2 |
| **D6** | **contextflow** | **ContextFlow 上下文供给引擎** | **新增** `.enjoyflow/snapshots/*.md` | **P0** |
| D7 | failure_modes | 失败模式记录 | **新增** `context/failures/*.md` | P2 |

#### 决策类三维度边界（A8 / C3 / D3）

> 一个"决策"放哪里？三者职责严格分离，按**生效状态**和**作用范围**区分：

| 维度 | A8 adr | C3 decisions | D3 decision_history |
|---|---|---|---|
| **生效状态** | 当前生效 | 已废弃/被替代 | 任务内临时 |
| **作用范围** | 项目级架构 | 项目级架构 | 单任务内 |
| **典型内容** | "我们选 PostgreSQL 不选 MySQL，理由 X" | "ADR-003 已被 ADR-007 替代，保留归档" | "REQ-001 中临时用同步导出，等 REQ-005 改异步" |
| **生命周期** | 活文档（状态机：Proposed→Accepted→Deprecated） | 只读归档（不修改） | 任务完成后归档到 C3 或升级为 A8 |
| **物理位置** | `lifecycle/02-design/ADRs/` | `development/decisions/archive/` | `context/decisions/` |

**决策流转规则**：

```
任务内临时决策（D3） ──任务完成──→ 影响项目架构？──是──→ 升级为 A8（新 ADR）
                                  └──否──→ 归档到 C3
A8 生效后 ──被新 ADR 替代──→ 状态改 Deprecated，内容迁入 C3 归档
```

### 2.5 32 子类汇总

| 大类 | 子类数 | P0 | P1 | P2 |
|---|---|---|---|---|
| A 项目知识 | 12 | 9 | 3 | 0 |
| B 业务知识 | 5 | 2 | 2 | 1 |
| C 流程知识 | 8 | 4 | 3 | 0 |
| D 上下文知识 | 7 | 2 | 3 | 2 |
| **总计** | **32** | **17** | **11** | **3** |

---

## 3. 目录结构

### 3.0 PRD 归属（A6 vs D1 边界——重点说明）

> **v2.1 修正**：原 v2 把 PRD（A6）和任务进度（D1）都放在 `lifecycle/01-planning/active-sprint/`，导致同一目录既是需求内容又是执行进度，归属混乱。现按**内容类型**分离。

#### A6 requirement（需求内容）vs D1 task_progress（执行进度）

| 维度 | A6 requirement | D1 task_progress |
|---|---|---|
| **内容类型** | 需求规约（做什么、为什么、验收标准） | 执行状态（做到哪了、谁在做、何时完成） |
| **变更阶段** | 规划期定稿，开发期冻结 | 开发期持续更新 |
| **典型内容** | 业务目标 / AC / 数据结构 / 交互流程 | 状态（TODO/DOING/DONE）/ 时间线 / 里程碑 / 阻塞项 |
| **物理位置** | `knowledge-base/project/requirements/REQ-XXX.md` | `lifecycle/01-planning/active-sprint/REQ-XXX.md` |
| **ABCD 类别** | A6（项目知识，稳定） | D1（上下文知识，强时效） |

**关键区分**——A6 回答"做什么"，D1 回答"做到哪了"。一份需求的两面，分属不同大类，物理隔离。

#### 目录结构

```
knowledge-base/project/requirements/    # A6 的"家"（需求内容）
├── REQ-001-export-excel.md            # PRD 正文：业务目标 + AC + 数据结构
├── REQ-002-login-bug-fix.md
└── archive/                           # 已完成需求归档
    └── 2026-05/
        └── REQ-050.md

lifecycle/01-planning/                  # D1 的"家"（执行进度）
├── active-sprint/                      # 当前 Sprint 进行中
│   ├── REQ-001.md                     # 进度：状态/时间线/里程碑（不含 PRD 内容）
│   └── REQ-002.md
├── backlog/                           # 待排期
├── archive/                           # 已归档
├── PRD-template.md                    # 模板
└── requirement-lifecycle.md           # 状态机文档
```

#### D1 进度文件示例（只放执行信息，不含 PRD 内容）

```markdown
---
class: D1_task_progress
tags: [export, excel, backend]
last_modified: 2026-06-20
---

# REQ-001 导出 Excel 功能 · 执行进度

status: DOING
owner: 张三
started: 2026-06-15
target: 2026-06-25

## 时间线
- 2026-06-15 PRD 定稿（A6 冻结）
- 2026-06-16 SPEC 生成完成
- 2026-06-18 后端实现 60%
- 2026-06-20 [当前] 联调中

## 里程碑
- [x] PRD Approved
- [x] SPEC Generated
- [ ] Implementation Done
- [ ] Verification Passed

## 阻塞项
- 前端等后端 API 联调

## 关联
- 需求内容：knowledge-base/project/requirements/REQ-001-export-excel.md
- 设计规约：lifecycle/02-design/specs/SPEC-001.md
```

#### 为什么 A6 移到 knowledge-base/

| 理由 | 说明 |
|---|---|
| **A6 属于项目知识** | PRD 是项目事实，跟架构/契约同类，应在 knowledge-base/project/ |
| **D1 属于上下文知识** | 进度是强时效上下文，跟会话/快照同类，在 lifecycle/ |
| **避免同目录混类** | 原 v2 让 active-sprint/ 同时承载 A6+D1，写文件时不知道该写内容还是进度 |
| **ContextFlow 引用清晰** | 快照分别引用 A6（需求内容）和 D1（进度），路径不冲突 |

#### PRD 在 ContextFlow 快照里的角色

ContextFlow 快照**引用** PRD：

```yaml
# .enjoyflow/snapshots/REQ-001.md
task: REQ-001
references:
  - path: lifecycle/01-planning/active-sprint/REQ-001.md  # 引用 PRD
    type: prd
  - path: knowledge-base/project/ARCHITECTURE.md
    type: architecture
  ...
```

**ContextFlow 快照不复制 PRD 内容**——只引用——避免重复。

#### PRD 数量预估

| 项目阶段 | PRD 数量 |
|---|---|
| MVP | 10-30 |
| 成熟产品 | 100-300 |
| 长期项目 | 1,000+ |

**PRD 数量增长快**——**独立顶级目录**比混在 knowledge-base/ 更合适。

---

### 3.1 v2 完整目录

```
.enjoyflow/
├── knowledge-base/
│   │
│   ├── project/                   # A 项目知识（稳定、低频变更）
│   │   ├── ARCHITECTURE.md        # A1
│   │   ├── CODE-STANDARDS.md      # A2
│   │   ├── DATA-MODEL.md          # A4（新增）
│   │   ├── DEPENDENCIES.md        # A10（新增）
│   │   ├── requirements/          # A6（PRD 需求内容）
│   │   │   ├── REQ-001.md
│   │   │   └── archive/
│   │   ├── TOOLCHAIN.md
│   │   └── AI-FEATURE-SPEC.md
│   │
│   ├── contract/                  # A3/A5 API 契约
│   │   ├── GLOBAL_CONTRACT.md     # A3
│   │   ├── CHANGELOG.md
│   │   ├── INDEX.md
│   │   └── modules/               # A5
│   │
│   ├── business/                  # B 业务知识
│   │   ├── GLOSSARY.md            # B1（新增）
│   │   ├── INDEX.md
│   │   ├── CONSTRAINTS.md         # B4（新增）
│   │   ├── water-app.md           # B2
│   │   ├── water-billing.md
│   │   ├── water-iot.md
│   │   ├── water-monitor.md
│   │   └── flows/                 # B3（新增）
│   │
│   ├── development/               # C 流程知识
│   │   ├── GOTCHAS.md             # C1
│   │   ├── PATTERNS.md            # C2
│   │   ├── INDEX.md
│   │   ├── decisions/             # C3
│   │   └── archive/
│   │
│   ├── testing/                   # C4/C5
│   │   ├── STRATEGY.md            # C4
│   │   └── KNOWN-ISSUES.md        # C5
│   │
│   ├── deployment/                # A11/A12/C8
│   │   ├── ENVIRONMENTS.md        # A11
│   │   ├── RELEASE.md             # A12（发布流程唯一归属）
│   │   └── DEPLOYMENT-CHECKLIST.md # C8（操作清单 + 回滚步骤）
│   │
│   ├── shared/                    # C6/C7（跨项目通用）
│   │   ├── TEAM-CONVENTION.md     # C6（团队约定，不含代码规范）
│   │   ├── GOTCHAS.md             # 跨项目通用踩坑
│   │   ├── PATTERNS.md            # 跨项目通用模式
│   │   ├── REVIEW-CHECKLIST.md    # C7（新增）
│   │   └── INDEX.md
│   │
│   └── context/                   # D 上下文知识（v2 新增）
│       ├── tasks/                 # D1 任务进度
│       │   └── ${REQ-ID}.md
│       ├── sessions/              # D2 AI 会话记录
│       │   └── ${SESSION-ID}.md
│       ├── decisions/             # D3 决策历史
│       ├── contract-sync/         # D4 跨特性契约对齐
│       ├── ai-calls/              # D5 AI 调用链路
│       └── failures/              # D7 失败模式
│
├── lifecycle/                     # D1/A8/A9
│   ├── 01-planning/
│   │   ├── active-sprint/         # A6 当前需求
│   │   ├── backlog/
│   │   ├── archive/
│   │   ├── PRD-template.md
│   │   └── requirement-lifecycle.md
│   ├── 02-design/
│   │   ├── ADRs/                  # A8
│   │   ├── design-plan-template.md
│   ├── 03-development/
│   │   └── TASK-BUNDLE-template.md
│   ├── 04-verification/
│   │   ├── quality-gate-prompt.md
│   │   └── reports/               # A9
│   ├── 05-maintenance/
│   │   ├── cross-stack-memory-prompt.md
│   │   └── memory-update-prompt.md
│   └── 06-iteration-management/
│       └── iteration-guide.md
│
└── .enjoyflow/                    # ContextFlow 快照
    └── snapshots/
        └── ${REQ-ID}.md
```

### 3.2 与 v1 目录对比

| v1 目录 | v2 处理 | 变化 |
|---|---|---|
| `project/` | `project/` | 保留，加 A4/A10 |
| `business/` | `business/` | 保留，加 B1/B3/B4 |
| `development/` | `development/` | 保留 |
| `testing/` | `testing/` | 保留 |
| `deployment/` | `deployment/` | 保留 |
| `contract/` | `contract/` | 保留 |
| (无) | **`context/`（新增）** | D1-D7 |
| (无) | **`shared/`（保留）** | C6/C7 |
| (无) | **`.enjoyflow/`（新增）** | ContextFlow 快照 |

---

## 4. tag 驱动检索（v2 新机制）

### 4.1 核心设计

EnjoyFlow **不维护专门索引**——**用 grep 实现 tag 搜索**（详见 [CONTEXTFLOW-SPEC.md](CONTEXTFLOW-SPEC.md) §6.1）。

### 4.2 每篇文档的 frontmatter

```markdown
---
title: Excel 导出踩坑
class: C1_gotchas              # ← 32 子类之一
tags:                          # ← 主题关键词
  - excel
  - export
  - backend
last_modified: 2026-06-15
---

# Excel 导出踩坑清单
...
```

**约束**：
- ✅ tag **只写在 YAML frontmatter**（不在正文）
- ✅ 每篇文档必须有 `class` 字段（必填）
- ✅ 每篇文档必须有 ≥1 个 `tags` 字段
- ❌ 正文不写 tag（避免 grep 误判）

### 4.3 6 个 tag 命令

```bash
enjoyflow tag-search <tag>      # 找含某 tag 的文档
enjoyflow tag-show <file>       # 显示某文档的所有 tag
enjoyflow tag-stats             # 统计所有 tag 频率
enjoyflow tag-check             # 检查所有文档都有 tag
enjoyflow tag-find-orphans      # 找无 tag 的文档
enjoyflow tag-find-cold         # 找冷僻 tag（只出现 1 次）
```

### 4.4 升级路径

| 文档数 | 实现 | 升级触发 |
|---|---|---|
| < 2000 | 纯 grep | 不升级 |
| 2000-5000 | grep + 简单缓存 | 用户主动 |
| 5000+ | 专门索引 + LRU 缓存 | 自动检测延迟 |

**MVP 不实现专门索引**——用 grep 够用。

---

## 5. ContextFlow 上下文供给引擎（v2 核心差异化）

### 5.1 什么是 ContextFlow

**ContextFlow = AI 可调用的上下文工具**——AI 处理任务时主动调用 `enjoyflow_context` 获取相关知识索引，结果可选缓存为快照文件。工具是主形态，缓存是辅形态。

详见 [CONTEXTFLOW-SPEC.md](CONTEXTFLOW-SPEC.md) §4。

### 5.2 ContextFlow 缓存文件的存储位置（可选）

> 缓存是 `enjoyflow_context` 工具调用的可选产物，不是 ContextFlow 的必需形态。无缓存时 ContextFlow 仍可正常工作（每次实时聚合）。缓存的价值在于加速、审计和跨会话传递。

```
.enjoyflow/snapshots/${REQ-ID}.md
```

**举例**：

```
.enjoyflow/snapshots/
├── REQ-001-export-excel.md
├── REQ-002-login-bug-fix.md
└── REQ-003-water-meter-feature.md
```

### 5.3 ContextFlow 与知识库的关系

```
.enjoyflow/knowledge-base/         # 静态知识（ABCD 32 类）
       ↓
  search 动态查询                 # 按当前任务查询相关子集
       ↓
AI 工具注入使用
```

**ContextFlow 不是替代 knowledge-base**——**是 knowledge-base 的任务级视图**。

---

## 6. 任务场景路由（替代 v1 阶段路由）

### 6.1 v1 阶段路由（已废弃）

v1 用 5 阶段流水线对应固定文件——**问题**：实际场景多样，"修 bug" 不走流水线。

### 6.2 v2 场景路由（10 套场景模板）

详细定义见 [CONTEXTFLOW-SPEC.md](CONTEXTFLOW-SPEC.md) §4。**MVP 5 个核心预设**：

| 场景 | 维度集 | 用途 |
|---|---|---|
| `new_feature` | A1+A2+A6+A7+B1+B2+C1+C3+D1+D6 | 新功能开发 |
| `bug_fix` | A1+A2+A3+A4+C1+C5+D1+D7 | Bug 修复 |
| `refactor` | A1+A2+A3+A4+C1+C2+C3+D1 | 技术重构 |
| `hotfix` | A1+A3+C1+C5+D1+D7 | 紧急 hotfix |
| `architecture_decision` | A1+A8+C3+D3 | 架构决策 |

**路由实现**——**ContextFlow 工具的 `preset` 参数**：

```typescript
enjoyflow_context({
  task_id: "REQ-001",
  preset: "new_feature"  // 自动选择维度集
})
```

### 6.3 高级用户覆盖

```typescript
enjoyflow_context({
  task_id: "REQ-001",
  dimensions: ["A3", "A4", "C1"]  // 自定义维度
})
```

---

## 7. 文件大小预算与归档

### 7.1 预算表

| 文件 | 最大行数 | 超出后操作 |
|---|---|---|
| `project/ARCHITECTURE.md` | 150 | 移到 `development/decisions/` |
| `business/{feature}.md` | 80 | 按日期追加；超后归档 |
| `development/GOTCHAS.md` | 100 | 按日期追加；超后归档 |
| `development/decisions/INDEX.md` | 50 | 只是索引 |
| `testing/STRATEGY.md` | 80 | 覆盖更新 |
| `deployment/ENVIRONMENTS.md` | 80 | 覆盖更新 |
| `contract/GLOBAL_CONTRACT.md` | 150 | 按模块拆分 |
| `context/sessions/*.md` | 30 天 TTL | 过期归档 |
| `.enjoyflow/snapshots/*.md` | 50 | 超过重新生成 |

### 7.2 自动归档命令

```bash
enjoyflow archive check      # 检查所有文件
enjoyflow archive run        # 执行归档
enjoyflow archive status     # 显示归档状态
```

---

## 8. 更新触发机制

### 8.1 更新触发矩阵

| 触发事件 | 更新哪些文件 | 谁来更新 |
|---|---|---|
| **新架构决策** | A8 新 ADR（Accepted 状态）+ A1 ARCHITECTURE 更新 | 决策提出者 |
| **ADR 被替代** | 旧 A8 状态改 Deprecated + 内容迁入 C3 归档 | 决策提出者 |
| **任务完成** | D3 临时决策按规则升级 A8 或归档 C3 | 开发 Owner |
| **完成特性开发** | B2 业务规则 + C2 PATTERNS + C1 GOTCHAS | 开发 Owner |
| **发现通用坑** | C1 GOTCHAS | 发现者 |
| **API 契约变更** | A3 GLOBAL_CONTRACT + CHANGELOG | 变更者 |
| **新增枚举** | A3 + B2 | 后端开发者 |
| **测试策略调整** | C4 STRATEGY | QA |
| **环境变更** | A11 ENVIRONMENTS | 运维 |
| **AI 会话结束** | D2 SESSION_LOG | 自动（enjoyflow record） |
| **任务完成** | ContextFlow 快照更新 | 自动 |
| **Sprint 结束** | 按归档策略清理 | 知识管理员 |
| **新人入职** | A1 ARCHITECTURE + TOOLCHAIN | 导师 |

### 8.2 自动化更新命令

```bash
enjoyflow record decision --task REQ-001  # 记录临时决策到 D3（任务级）
enjoyflow record adr --title "ADR-XXX"    # 升级为 A8（项目级架构决策）
enjoyflow archive decision ADR-003        # 将已废弃 ADR 归档到 C3
enjoyflow record gotcha --tag excel       # 记录踩坑到 C1
enjoyflow record pattern --tag api       # 记录模式到 C2
enjoyflow snapshot REQ-001               # 生成/更新 ContextFlow 快照
```

---

## 9. 与旧 v1 的迁移对照

| v1 路径 | v2 路径 | 变化 |
|---|---|---|
| `project/` | `project/` | 保留，加 A4/A10 |
| `business/{feature}.md` | `business/{feature}.md` | 保留，加 B1/B3/B4 |
| `development/GOTCHAS.md` | `development/GOTCHAS.md` | 保留 |
| `development/PATTERNS.md` | `development/PATTERNS.md` | 保留 |
| `development/decisions/ADR-*.md` | `lifecycle/02-design/ADRs/ADR-*.md` | 移到 lifecycle |
| `testing/` | `testing/` | 保留 |
| `deployment/` | `deployment/` | 保留 |
| `contract/` | `contract/` | 保留 |
| (无) | `context/` | **新增（D1-D7）** |
| (无) | `.enjoyflow/snapshots/` | **新增（D6）** |
| `shared/` | `shared/` | 保留 |

---

## 10. 简化方案（如果 32 类太多）

EnjoyFlow 提供 **MVP 11 项** 简化版：

```
.enjoyflow/knowledge-base/（MVP 11 项）
├── project/
│   ├── ARCHITECTURE.md       # A1
│   ├── CODE-STANDARDS.md     # A2
│   └── requirements/         # A6（PRD 需求内容）
├── contract/
│   └── GLOBAL_CONTRACT.md    # A3
├── business/
│   ├── GLOSSARY.md           # B1（新增）
│   └── water-iot.md          # B2
├── development/
│   ├── GOTCHAS.md            # C1
│   └── decisions/            # C3
├── testing/
│   └── STRATEGY.md           # C4
├── lifecycle/01-planning/
│   └── active-sprint/        # D1（执行进度，非 PRD 内容）
└── .enjoyflow/snapshots/     # ContextFlow（核心）
```

**推荐**：MVP 先 11 项，**ContextFlow 必须有**，其他按需扩展。

---

## 11. 验证清单

- [ ] 每篇文档有 `class` 字段（ABCD 32 子类之一）
- [ ] 每篇文档有 ≥1 个 `tag` 字段
- [ ] 每篇文档 frontmatter 格式正确
- [ ] tag 只在 frontmatter（不在正文）
- [ ] ContextFlow 快照能正确生成
- [ ] 32 子类映射清晰（每个子类的存储路径明确）
- [ ] 7 项 P0 缺口（A4/B1/C6/C7/D2/D4/D6/D7）明确标记为 PENDING

---

## 12. 缺口清单（v2 vs 现状）

EnjoyFlow 当前 demo **缺失的 8 项**：

| # | 缺口 | 路径 | 优先级 |
|---|---|---|---|
| 1 | A4 data_model | `project/DATA-MODEL.md` | P0 |
| 2 | B1 glossary | `business/GLOSSARY.md` | P0 |
| 3 | C6 team_convention | `shared/TEAM-CONVENTION.md` | P1 |
| 4 | C7 review_checklist | `shared/REVIEW-CHECKLIST.md` | P1 |
| 5 | D2 session_log | `context/sessions/` | P1 |
| 6 | D4 contract_sync | `context/contract-sync/` | P1 |
| **7** | **D6 task_snapshot** | **`.enjoyflow/snapshots/`** | **P0** |
| 8 | D7 failure_modes | `context/failures/` | P2 |

**最关键**——**ContextFlow 上下文供给引擎**是 v2 核心差异化能力。

---

*文档版本: v2.0 | 最后更新: 2026-06-20*
