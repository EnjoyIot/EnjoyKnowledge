# EnjoyFlow 知识架构设计 v2 (Knowledge Architecture)

> 解决"AI 编码时上下文混乱"的问题。设计原则：**按知识类型 ABCD 分离存储，按任务场景按需路由，按 tag 检索。**
>
> **版本演进**:
> v1（基于 6 类 + 4 人团队 + 5 阶段，已废弃）
> **v2（基于 4 大类 ABCD + 32 子类 + tag 驱动 + D6 任务快照）**

---

## 1. 核心命题

享受 Flow 知识架构的根本问题：

> **"如何在 AI 不可靠的前提下，让 AI 在每个任务都有'刚好够'的上下文知识"**

v1 用"5 阶段流水线"硬路由——**过时**。
v2 用"4 大类 ABCD + 32 子类 + tag 检索 + D6 任务快照"——**适应 AI 编程的真实场景**。

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
| A2 | code_standards | 编码规范速查 | `project/CODE-STANDARDS.md` | P0 |
| A3 | api_contract | 全局 API 契约 | `contract/GLOBAL_CONTRACT.md` | P0 |
| A4 | data_model | 数据模型、表结构 | **新增** `project/DATA-MODEL.md` | P0 |
| A5 | api_spec | 单个 API 详细规约 | `contract/modules/*.md` | P0 |
| **A6** | **requirement** | **PRD（产品需求文档）** | **`lifecycle/01-planning/`** | **P0** |
| A7 | design_spec | 设计方案（SPEC） | `lifecycle/02-design/*.md` | P0 |
| A8 | adr | 架构决策记录 | `lifecycle/02-design/ADRs/*.md` | P1 |
| A9 | test_report | 测试覆盖率、报告 | `lifecycle/04-verification/reports/*.md` | P1 |
| A10 | dependencies | 依赖关系图 | **新增** `project/DEPENDENCIES.md` | P1 |
| A11 | environment | 环境配置（dev/staging/prod） | `deployment/ENVIRONMENTS.md` | P1 |
| A12 | release | 发布流程 | `deployment/RELEASE.md` | P1 |

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
| C3 | decisions | 历史决策 | `development/decisions/*.md` | P0 |
| C4 | test_strategy | 测试策略 | `testing/STRATEGY.md` | P0 |
| C5 | known_issues | 已知问题 | `testing/KNOWN-ISSUES.md` | P1 |
| C6 | team_convention | 团队约定（commit/PR） | **新增** `shared/TEAM-CONVENTION.md` | P1 |
| C7 | review_checklist | 代码审查清单 | **新增** `shared/REVIEW-CHECKLIST.md` | P1 |
| C8 | deployment | 部署清单 | `deployment/RELEASE.md`（部分） | P1 |

### 2.4 D 上下文知识（7 子类）——**v2 新增**

| # | 子类 | 含义 | 当前 demo 路径 | 优先级 |
|---|---|---|---|---|
| D1 | task_progress | 当前任务进度 | `lifecycle/01-planning/active-sprint/` | P0 |
| D2 | session_log | AI 会话记录 | **新增** `context/sessions/*.md` | P1 |
| D3 | decision_history | 决策历史 | `development/decisions/`（部分） | P1 |
| D4 | contract_sync | 跨特性契约对齐 | **新增** `context/contract-sync/` | P1 |
| D5 | ai_call_chain | AI 调用链路 | **新增** `context/ai-calls/*.md` | P2 |
| **D6** | **task_snapshot** | **任务上下文快照** | **新增** `.enjoyflow/snapshots/*.md` | **P0** |
| D7 | failure_modes | 失败模式记录 | **新增** `context/failures/*.md` | P2 |

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

### 3.0 PRD 位置（重点说明）

**PRD（产品需求文档）放在 `lifecycle/01-planning/`**——**不是 knowledge-base/**：

```
lifecycle/
└── 01-planning/                  # PRD 的"家"
    ├── active-sprint/            # 当前 Sprint 的 PRD（正在做）
    │   ├── REQ-001.md           # PRD 实例
    │   ├── REQ-002.md
    │   └── ...
    ├── backlog/                  # 待讨论的 PRD（暂不做）
    │   ├── REQ-101.md
    │   └── ...
    ├── archive/                  # 已完成 PRD（按年月组织）
    │   └── 2026-05/
    │       └── REQ-050.md
    ├── PRD-template.md           # PRD 模板（不是具体需求）
    └── requirement-lifecycle.md  # 需求状态机文档
```

#### 为什么 PRD 在 lifecycle/01-planning/ 而不在 knowledge-base/

**3 个理由**：

| 理由 | 说明 |
|---|---|
| **PRD 是阶段 1 的产物** | 阶段 1（规划）产生 PRD，位置应反映生命周期 |
| **物理隔离已建立** | active-sprint/backlog/archive 三层目录结构清晰 |
| **PRD 引用 knowledge-base** | PRD 模板已引用 `knowledge-base/business/`、`project/ARCHITECTURE.md`——PRD 是消费者，知识库是被消费者 |

**PRD 跟 ABCD 4 大类的关系**：

| PRD 内容 | 对应 ABCD |
|---|---|
| 业务目标 + AC | B 业务知识 |
| 技术约束 | A 项目知识 |
| 数据结构 | A4 数据模型 |
| 交互流程 | B3 业务流程 |
| 状态机 | D1 任务进度 |

**PRD 跨 ABCD 4 大类**——**它不是单一知识类别，而是"知识聚合"**——所以独立目录合理。

#### PRD 在 D6 快照里的角色

D6 任务快照**引用** PRD：

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

**D6 快照不复制 PRD 内容**——只引用——避免重复。

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
enjoyflow/
├── knowledge-base/
│   │
│   ├── project/                   # A 项目知识（稳定、低频变更）
│   │   ├── ARCHITECTURE.md        # A1
│   │   ├── CODE-STANDARDS.md      # A2
│   │   ├── DATA-MODEL.md          # A4（新增）
│   │   ├── DEPENDENCIES.md        # A10（新增）
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
│   ├── deployment/                # A11/A12
│   │   ├── ENVIRONMENTS.md        # A11
│   │   └── RELEASE.md             # A12
│   │
│   ├── shared/                    # C6/C7
│   │   ├── TEAM-CONVENTION.md     # C6（新增）
│   │   ├── CODE-STANDARDS.md
│   │   ├── GOTCHAS.md
│   │   ├── PATTERNS.md
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
├── lifecycle/                     # A6/A7/A9
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
└── .enjoyflow/                    # D6 任务快照
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
| (无) | **`.enjoyflow/`（新增）** | D6 快照 |

---

## 4. tag 驱动检索（v2 新机制）

### 4.1 核心设计

享受 Flow **不维护专门索引**——**用 grep 实现 tag 搜索**（详见 [D6-SNAPSHOT-SPEC.md](D6-SNAPSHOT-SPEC.md) §6.1）。

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

## 5. D6 任务上下文快照（v2 核心差异化）

### 5.1 什么是 D6

**D6 = AI 任务上下文快照**——当 AI 处理某个任务时，自动加载的"当前任务相关的所有知识"。

详见 [D6-SNAPSHOT-SPEC.md](D6-SNAPSHOT-SPEC.md) §4。

### 5.2 D6 快照的存储位置

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

### 5.3 D6 快照与 knowledge-base 的关系

```
knowledge-base/                  # 静态知识（ABCD 32 类）
       ↓
  D6 快照工具动态聚合            # 按当前任务聚合相关子集
       ↓
.enjoyflow/snapshots/${REQ}.md   # 任务特定的上下文快照
       ↓
  AI 工具注入使用
```

**D6 不是替代 knowledge-base**——**是 knowledge-base 的任务级视图**。

---

## 6. 任务场景路由（替代 v1 阶段路由）

### 6.1 v1 阶段路由（已废弃）

v1 用 5 阶段流水线对应固定文件——**问题**：实际场景多样，"修 bug" 不走流水线。

### 6.2 v2 场景路由（10 套场景模板）

详细定义见 [D6-SNAPSHOT-SPEC.md](D6-SNAPSHOT-SPEC.md) §4。**MVP 5 个核心预设**：

| 场景 | 维度集 | 用途 |
|---|---|---|
| `new_feature` | A1+A2+A6+A7+B1+B2+C1+C3+D1+D6 | 新功能开发 |
| `bug_fix` | A1+A2+A3+A4+C1+C5+D1+D7 | Bug 修复 |
| `refactor` | A1+A2+A3+A4+C1+C2+C3+D1 | 技术重构 |
| `hotfix` | A1+A3+C1+C5+D1+D7 | 紧急 hotfix |
| `architecture_decision` | A1+A8+C3+D3 | 架构决策 |

**路由实现**——**D6 工具的 `preset` 参数**：

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
| **新架构决策** | A8 ADR + A1 ARCHITECTURE | 决策提出者 |
| **完成特性开发** | B2 业务规则 + C2 PATTERNS + C1 GOTCHAS | 开发 Owner |
| **发现通用坑** | C1 GOTCHAS | 发现者 |
| **API 契约变更** | A3 GLOBAL_CONTRACT + CHANGELOG | 变更者 |
| **新增枚举** | A3 + B2 | 后端开发者 |
| **测试策略调整** | C4 STRATEGY | QA |
| **环境变更** | A11 ENVIRONMENTS | 运维 |
| **AI 会话结束** | D2 SESSION_LOG | 自动（enjoyflow record） |
| **任务完成** | D6 SNAPSHOT 更新 | 自动 |
| **Sprint 结束** | 按归档策略清理 | 知识管理员 |
| **新人入职** | A1 ARCHITECTURE + TOOLCHAIN | 导师 |

### 8.2 自动化更新命令

```bash
enjoyflow record decision --task REQ-001  # 记录决策到 D3 + A8
enjoyflow record gotcha --tag excel       # 记录踩坑到 C1
enjoyflow record pattern --tag api       # 记录模式到 C2
enjoyflow snapshot REQ-001               # 生成/更新 D6 快照
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

享受 Flow 提供 **MVP 11 项** 简化版：

```
knowledge-base/（MVP 11 项）
├── project/
│   ├── ARCHITECTURE.md       # A1
│   └── CODE-STANDARDS.md     # A2
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
│   └── active-sprint/        # D1
└── .enjoyflow/snapshots/     # D6（核心）
```

**推荐**：MVP 先 11 项，**D6 必须有**，其他按需扩展。

---

## 11. 验证清单

- [ ] 每篇文档有 `class` 字段（ABCD 32 子类之一）
- [ ] 每篇文档有 ≥1 个 `tag` 字段
- [ ] 每篇文档 frontmatter 格式正确
- [ ] tag 只在 frontmatter（不在正文）
- [ ] D6 快照能正确生成
- [ ] 32 子类映射清晰（每个子类的存储路径明确）
- [ ] 7 项 P0 缺口（A4/B1/C6/C7/D2/D4/D6/D7）明确标记为 PENDING

---

## 12. 缺口清单（v2 vs 现状）

享受 Flow 当前 demo **缺失的 8 项**：

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

**最关键**——**D6 任务快照**是 v2 核心差异化能力。

---

*文档版本: v2.0 | 最后更新: 2026-06-20*
