# EnjoyFlow 定位宣言 (Positioning Manifesto) v5.1

> 回答：EnjoyFlow 在 2026 年 AI 编程生态里占什么位、解决什么机制问题、为什么用户应该选它。
>
> **版本演进**:
> v5（基于"人与大模型易于使用的知识管理" + 4 类知识 ABCD + ContextFlow 上下文供给引擎）
> **v5.1（v5 基础上收敛定位窄缝 + 对接 Breunig 共识语言 + 双护城河 + 3 类起步）**

---

## 1. 一句话定位

> **EnjoyFlow 是工程团队的人机共享任务上下文层——人和 AI 用同一份知识干活。**

三个关键词钉死差异化：

| 关键词 | 排除了什么 | 为什么是窄缝 |
|---|---|---|
| **工程团队** | 排除个人开发者的临时记忆需求 | 个人用 mem0/Cursor 自带记忆够了；团队共享才是真痛点 |
| **人机共享** | 排除单边工具（只给人 or 只给 AI） | Notion 只给人、Cursor 记忆只给 AI；两者必须共用一份 |
| **任务上下文** | 排除通用知识管理（不分场景的记忆） | mem0 是通用 memory，spec-kit 是 spec；EnjoyFlow 只管"当前任务相关的上下文" |

三者交集 = **工程团队 ∩ 人机共享 ∩ 任务上下文** = EnjoyFlow 唯一独占的窄缝。

具体说：EnjoyFlow 是一套 **CLI 工具 + 知识库结构 + 接口规范**，让工程团队的开发者与 AI 编码工具（Cursor / Copilot / Claude Code / Codex）共享 32 类开发文档与上下文。当 AI 写代码时，它知道你的架构、规范、踩坑、业务规则和当前任务进度——**而且这份知识是全团队共用的同一份，不是每个 AI 工具各自维护**。

它不是：
- ❌ AI 编码工具（Cursor / Copilot / Claude Code 是）
- ❌ 单一知识库（Notion / Confluence / Obsidian 是）
- ❌ 通用 AI memory（mem0 / MemOS / MemPalace 是——它们不分场景）
- ❌ Spec 框架（spec-kit / OpenSpec 是——它们管 spec 制品不管共享记忆）
- ❌ Skills 框架（Superpowers 是——它管能力组合不管知识沉淀）
- ❌ 项目管理工具（Jira / Linear 是）

它是：
- ✅ **工程团队的人机共享任务上下文层**——位于 AI 编码工具和已有知识库之间
- ✅ **32 类开发文档的组织框架**——按 ABCD 4 类知识结构化
- ✅ **ContextFlow 上下文供给引擎**——AI 会话中主动获取当前任务的所有相关知识
- ✅ **接口规范 + 默认实现 + 开放生态**——定义知识层标准，提供开箱即用版本

---

## 2. 根本命题：人与大模型易于使用的知识管理

### 2.1 核心张力

```
人脑容量有限       vs      AI 上下文窗口有限
       ↓                            ↓
   人需要外部记忆                AI 需要外部记忆
       ↓                            ↓
           共享同一个外部记忆系统
                     ↓
       EnjoyFlow = 工程团队的人机共享任务上下文层
```

### 2.2 "易于使用"的含义

EnjoyFlow 不只是"管知识"——是"**让人和 AI 都易于使用**知识"。具体表现：

| 维度 | 含义 |
|---|---|
| **决策者路由** | 普通用户说一句"修 bug"，自动选场景模板 |
| **场景模板** | 8 套模板覆盖研发全流程（bug_fix / new_feature / hotfix / refactor / architecture_decision / release_deployment / code_review / monitoring_response） |
| **轻量化** | 规则优先 + 兜底 LLM，EnjoyFlow 不内置大模型 |
| **接口规范** | 高级用户可精细控制，绕过路由器 |

### 2.3 对接 2026 共识语言（传播前置）

EnjoyFlow 的机制不改，但对外用圈内有共识的词重新包装——传播成本立刻降一半。

> 背景：Phil Schmid（2025-06）命名了 **Context Engineering**，Drew Breunig（2025-06）命名了 **Context Rot** 的 4 种失败模式 + 5+1 种修复机制。这些已是 HN/Anthropic 圈共识。GSD Core 已在用这套语言抢位。

| EnjoyFlow 现有机制 | Breunig 共识语言 | 价值 |
|---|---|---|
| 六类知识分离 + 分片加载 | **Context Quarantine**（隔离） | 不同上下文互不污染 |
| `sync_memory` 四段巡检 | **Context Pruning**（修剪） | 自动删除不相关信息 |
| `progress.md` 进度账本 | **Context Offloading**（外存） | 信息外存到 LLM context 之外 |
| 场景模板按任务选维度 | **Tool Loadout**（装弹） | 像游戏选装备一样按任务选知识 |
| 独立验证（强制新会话） | 防 **Context Poisoning**（毒化） | 幻觉不进入 context 被反复引用 |
| active-sprint 物理隔离 | **Context Quarantine**（隔离） | 当前任务上下文与归档隔离 |
| `enjoyflow_context` 工具 | **RAG**（选择性注入） | 按相关性注入而非全量加载 |

**关键**——EnjoyFlow 的 6 大机制**完整覆盖** Breunig 命名的全部 5+1 种修复机制。这不是巧合——是真实痛点的自然对位。**对外文案直接用 Breunig 语言**，让圈内人一眼看懂"哦，这就是 context engineering 的工程化落地"。

---

## 3. 知识分类：ABCD 四类全覆盖

EnjoyFlow 把"开发文档资料和上下文"分成 4 类：

| 类 | 例子 | 显隐性 | 时效 | 稳定性 |
|---|---|---|---|---|
| **A 项目知识** | 架构/规约/AC/数据模型 | 显性 | 长 | 高 |
| **B 业务知识** | 领域规则/业务术语 | 显性+隐性 | 长 | 中 |
| **C 流程知识** | 团队习惯/踩坑/最佳实践 | 隐性>显性 | 中 | 低 |
| **D 上下文知识** | 任务进度/会话记录/决策历史 | 显性+强时效 | 短 | 低 |

### 3.1 32 类文档清单

| ABCD | 类数 | P0 必填 | P1 重要 | P2 可选 |
|---|---|---|---|---|
| A 项目知识 | 12 | 9 | 3 | 0 |
| B 业务知识 | 5 | 2 | 2 | 1 |
| C 流程知识 | 8 | 4 | 3 | 0 |
| D 上下文知识 | 7 | 2 | 3 | 2 |
| **总计** | **32** | **17** | **11** | **3** |

### 3.2 P0 必填清单（17 项）

A1 架构总览 / A2 代码规范 / A3 API 契约 / A4 数据模型 / A5 接口规约 / A6 产品需求 / A7 UI/UX 设计 / B1 业务术语表 / B2 业务规则 / C1 GOTCHAS / C2 PATTERNS / C3 DECISIONS / C4 测试策略 / D1 任务进度 / D6 ContextFlow

---

## 4. 核心能力：ContextFlow 上下文供给引擎

### 4.1 ContextFlow 是什么

**ContextFlow = AI 可调用的上下文工具（主形态）+ 可选缓存文件（辅形态）**

ContextFlow 的本质是 `enjoyflow_context` 工具——AI 在会话中主动调用，获取与当前任务相关的所有知识索引。工具调用的结果**可选**缓存到 `.enjoyflow/snapshots/${REQ-ID}.md`，承担三个角色：

- **加速缓存**——重复调用时直接读缓存，避免重新聚合
- **可检查产物**——人可以查看 AI 当时用了什么上下文（审计追溯）
- **跨会话传递**——上一个会话的快照可被下一个会话参考（连续性）

**关键区分**：缓存是衍生物，不是源。源是 `knowledge-base/` 里的 32 类知识；任务上下文快照 工具按任务聚合它们；缓存文件只是聚合结果的物理落盘。详见 [CONTEXTFLOW-SPEC.md.md](CONTEXTFLOW-SPEC.md.md) §2。

```yaml
# .enjoyflow/snapshots/REQ-001-export-excel.yaml（缓存文件示例）
task: REQ-001-export-excel
generated_at: 2026-06-20T14:30:00
generated_by: enjoyflow_context  # 标注来源是工具调用
snapshot:
  architecture:
    - 模块: backend/export
    - 技术栈: Java + Spring Boot + MyBatis
    - 影响表: t_export_record
  business:
    - 术语: 导出 = ExportRecord 实体
    - 规则: 单次最多 10 万行，超过分批
  process:
    - 已知坑: t_export_record 没有 status 字段
    - 最佳实践: 异步导出用 @Async + 进度回调
  context:
    - 当前状态: IN_PROGRESS
    - 关联: SPEC-001, IMPL-001
    - 失败记录: 之前试过同步导出，超时
```

### 4.2 任务上下文快照 解决的 AI 缺陷

| AI 缺陷 | 任务上下文快照 怎么解决 |
|---|---|
| **上下文衰减** | 长任务拆快照，每段新会话都有完整上下文 |
| **目标漂移** | 快照固定当前任务 + AC，AI 不跑题 |
| **幻觉一致性** | 快照提供真实业务规则，AI 不瞎编 |
| **过度自信** | 快照标注已知坑，AI 主动避让 |
| **知识孤岛** | 快照聚合团队知识，跨人一致 |

### 4.3 任务上下文快照 的 5 个工程子问题

| 子问题 | EnjoyFlow 答案 |
|---|---|
| 什么是"相关"？ | MVP 用显式链接 + 关键词；后期升级语义检索 |
| 怎么"聚合"？ | 场景模板（8 套）= 天然聚合规则 |
| 什么时候"快照"？ | 任务创建时 + 阶段转换 + AI 会话开始时按需刷新 |
| 快照的"保质期"？ | 缓存 + 失效机制 |
| 怎么"喂给 AI"？ | 支持多种（@file / Skill / MCP / prompt 注入） |

### 4.4 任务上下文快照 是EnjoyFlow 的护城河（之一）


| 竞品 | 定位 | 是否做"工程团队任务级共享" |
|---|---|---|
| Notion / Confluence | 团队知识库 | ❌ 不做任务级聚合，不分人机 |
| Cursor / Copilot | AI 编码工具 | ❌ 自带 IDE 记忆但不跨工具，团队不共享 |
| spec-kit / OpenSpec | Spec 框架 | ❌ 管 spec 制品不管共享记忆 |
| Superpowers | Skills 框架 | ❌ 管能力组合不管知识沉淀 |
| **mem0 / MemOS** | 通用 AI memory | ⚠️ 做 memory 但不分场景，不聚合项目业务规则 |
| **MemPalace / basic-memory** | Markdown + AI | ⚠️ Markdown 知识 + AI，但不主动快照不分任务 |
| **smriti / keepr / engineering-os** | 工程团队 memory | ⚠️ 方向接近但全部 < 50 stars，无品牌无成熟方案 |
| **EnjoyFlow** | **工程团队人机共享任务上下文层** | ✅ ** 任务上下文工具** |

**关键判断**——任务上下文快照 的护城河是**先发优势型**而非**技术壁垒型**：32 类结构是公开设计，大厂一夜能抄。因此 必须配合第二护城河（开放生态，见 §4.5）才能成立。

### 4.5 双护城河策略：ContextFlow（先发型）+ 开放生态（壁垒型）

任务上下文快照 单靠结构挡不住大厂。真正能挡的是**生态**——让第三方写适配器、写场景模板、写维度扩展，形成 OpenAPI 式的"规范 + 生成器 + 生态"三重模式（见 ADDENDUM-3 §1.2）。

| 护城河 | 类型 | 防御对象 | 时效 |
|---|---|---|---|
| ** 任务上下文工具** | 先发型 | 大厂自建（Cursor/Notion） | 6-12 个月窗口期 |
| **开放生态（适配器 + 场景 + 维度）** | 壁垒型 | 同类竞品（GSD Core 等） | 长期，越早建越牢 |

**生态战略的 3 个开放点**（详见 CONTEXTFLOW-SPEC.md §12 可扩展性）：

1. **场景模板开放**——用户/团队/社区可写 `.enjoyflow/scenarios/*.yaml`，EnjoyFlow 内置 8 个，社区扩展无限
2. **维度开放**——用户可加 `R1_rust_embedded` 这类自定义维度，类别字符 A-Z 可扩展
3. **适配器开放**——第三方可写 Cursor/Copilot/Continue/Cody 适配器，EnjoyFlow 不绑定任何 AI 工具

**借鉴 OpenAPI 成功模式**（ADDENDUM-3 §1）：

```
接口规范层（EnjoyFlow 标准）   ← 类比 OpenAPI 规范
       ↓ 自动生成
默认实现层（CLI + 8 场景）     ← 类比 OpenAPI Generator（26K stars）
       ↓ 生态扩展
适配器生态（N 个第三方）        ← 类比 OpenAPI 158 个 plugins
```

**关键**——只做规范层会被边缘化（LSP 模式的风险），做规范+生成器+生态才能成为事实标准（OpenAPI 模式）。EnjoyFlow 必须三件全做。

---

## 5. 三层架构：用户—决策者—执行

```
┌─────────────────────────────────────┐
│  用户（开发者）                        │
│  - 高级用户：直接配 enjoyflow.yaml    │
│  - 普通用户：跟"决策者"聊            │
└─────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────┐
│  决策者层（路由器）                      │
│  - 规则匹配（关键词/任务类型）             │
│  - 兜底 LLM 分类（可选）                 │
│  - 选场景模板                          │
└─────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────┐
│  场景模板（8 套 MVP）                     │
│  - bug_fix / new_feature / refactor   │
│  - hotfix / architecture_decision     │
│  - release_deployment / code_review   │
│  - monitoring_response                │
└─────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────┐
│  执行层（slots = 独立上下文）             │
│  - requirement / design               │
│  - implementation / verification      │
│  - knowledge                          │
└─────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────┐
│  框架核心（API + 钩子 + 插件）            │
│  - 32 类知识库                          │
│  - ContextFlow 快照生成器                        │
│  - 钩子（session-start / verify 等）   │
└─────────────────────────────────────┘
```

### 5.1 角色本质：独立上下文槽位

EnjoyFlow 不预设"4 角色 / 5 阶段"——**角色本质是任务流反推的独立上下文槽位**：

```
推导路径：任务天然有几个独立上下文 → 反推出槽位数 → 角色是这些槽位的对外命名
```

实际反推得到 **4 个槽位**：架构契约 / 执行实现 / 独立验证 / 知识路由。

---

## 6. 跨工具：中间件定位

```
┌──────────────────────────────────────────┐
│  Cursor / Copilot / Codex / Claude Code  │  ← AI 工具
└──────────────────────────────────────────┘
                  ↓ 通过适配器
┌──────────────────────────────────────────┐
│           EnjoyFlow 知识层                  │  ← EnjoyFlow（中间件）
└──────────────────────────────────────────┘
                  ↓ 通过适配器
┌──────────────────────────────────────────┐
│  Notion / Obsidian / mem0 / Jira         │  ← 已有工具
└──────────────────────────────────────────┘
```

**EnjoyFlow 不是 AI 工具的替代——是 AI 工具和已有工具之间的知识中间件**。

### 6.1 跨工具的核心价值

| 用户价值 | 没有EnjoyFlow | 有EnjoyFlow |
|---|---|---|
| 换 AI 工具 | 知识全部丢失 | 知识跨工具保留 |
| 多人协作 | 各自 AI 知识不同 | 共享同一份知识 |
| 项目交接 | 新人/AI 啥都不知道 | 一次性加载所有上下文 |

### 6.2 各 AI 工具的接入方式

| AI 工具 | 注入方式 | 捕获方式 |
|---|---|---|
| Cursor | @file 引用 / .cursorrules | .cursor/logs/ 导入 |
| Copilot | Chat 引用 / Workspace 索引 | 日志导入 |
| Claude Code | Skills 加载 / CLAUDE.md | .claude/sessions/ 导入 |
| Codex CLI | 上下文注入 / MCP server | CLI 日志 |

---

## 7. MVP 路径

### 7.1 最小可用集：3 类起步（降低心理门槛）

> 32 类知识对小团队是负担。真实起步只需要 **3 类核心**——就能让 AI 不犯大错：

| 类 | 为什么是它 | 缺了会怎样 |
|---|---|---|
| **A1 架构总览** | AI 不知道项目长什么样，会瞎猜模块边界 | AI 把代码放错位置，跨模块破坏 |
| **A2 代码规范** | AI 不知道团队怎么写，按训练数据随机写 | 代码风格不一致，PR 被打回 |
| **C1 踩坑清单** | AI 不知道历史坑，会重新踩一遍 | 重复踩已修过的 bug |

**3 类起步 → 11 项 P0 → 32 类全量** 是渐进路径：

```
个人/小项目：3 类（A1+A2+C1）           ← 5 分钟起步
小团队：11 项 P0（见 §7.2）              ← MVP 推荐
成熟团队：32 类全量                       ← v1.0 目标
```

### 7.2 MVP 三件套

```
MVP = CLI 工具 + 11 项 P0 知识 + 1 个核心命令（snapshot）
```

> 个人开发者可从 3 类起步（§7.1），团队推荐直接上 11 项 P0。

### 7.3 11 项 P0（精简版）

A1 架构总览 / A2 代码规范 / A3 API 契约 / A6 产品需求 / A7 UI/UX 设计 / B1 业务术语表 / B2 业务规则 / C1 GOTCHAS / C3 DECISIONS / D1 任务进度 / **D6 ContextFlow**

### 7.4 MVP 命令清单

```bash
enjoyflow init                  # 初始化项目
enjoyflow new REQ-001           # 创建需求
enjoyflow snapshot REQ-001      # 生成任务快照（MVP 核心）
enjoyflow show REQ-001          # 显示任务状态
enjoyflow verify REQ-001        # 独立验证（轻量级）
```

### 7.5 配置生成工具集（让用户少写配置）

**核心原则**——**默认能用，少改就行；交互优先，配置文件兜底**。

EnjoyFlow 提供 **8 个生成工具**，覆盖 4 个配置层次：

| 工具 | 用途 | 适用层次 |
|---|---|---|
| `enjoyflow init` | 项目初始化（0 配置起步） | L1 项目级 |
| `enjoyflow new` | 交互式创建 PRD | L4 任务级 |
| `enjoyflow wizard` | 向导式配置所有高级选项 | L1 项目级 |
| `enjoyflow add-tag` | AI 自动建议 tag + 用户确认 | L3 文档级 |
| `enjoyflow add-dimension` | 交互式新增自定义维度 | L2 模板级 |
| `enjoyflow add-adapter` | 引导式配置 AI 工具适配器 | L2 模板级 |
| `enjoyflow doctor` | 自动诊断配置问题 | 所有层 |
| `enjoyflow fix` | 自动修复配置问题 | 所有层 |

#### 默认配置 + 0 配置起步

```bash
# 0 配置启动（推荐新项目）
$ enjoyflow init --default

# 自动生成
✓ 生成默认 enjoyflow.yaml
✓ 启用内置 5 场景模板
✓ 启用内置 32 维度
✓ 生成示例文档
✓ 立即可用
```

#### 交互式 PRD 生成

```bash
$ enjoyflow new REQ-001 "导出 Excel 功能"

# 1. 询问 PRD 内容（交互式）
? 业务目标: 允许用户导出账单为 Excel
? 验收标准 (AC): ...
? 涉及模块: backend/export
? 数据库变更: 否

# 2. 自动生成
✓ 生成 PRD 文件
✓ 自动加 tags + class
✓ 添加追溯链

# 3. 询问后续
? 现在生成 SPEC 吗？(y/n)
```

#### AI 自动建议 tag

```bash
$ enjoyflow add-tag knowledge-base/development/GOTCHAS.md

# AI 读取文档并建议
建议的 tag:
  - excel      (出现 5 次)
  - export     (出现 8 次)
  - backend    (出现 3 次)
建议的 class: C1_gotchas

? 接受这些 tag？(y/n): y
```

#### 引导式新增维度

```bash
$ enjoyflow add-dimension

? 维度 ID: R1_rust_embedded
? 类别: R
? 标题: Rust 嵌入式规范
? 描述: Rust 嵌入式开发规范
? 默认路径: knowledge-base/embedded/rust-standards.md
? 默认 tag: rust, embedded

✓ 已生成 .enjoyflow/dimensions/R1_rust_embedded.yaml
✓ 已在 enjoyflow.yaml 注册
```

#### 配置诊断 + 自动修复

```bash
$ enjoyflow doctor
✓ 配置总体健康
⚠ 发现 5 个问题

$ enjoyflow fix
? 应用所有修复？(y/n): y
✓ 已修复 5 个问题（自动补 class / 自动归一化 tag / 清理孤立文件等）
```

#### 设计原则

| 原则 | 体现 |
|---|---|
| **默认能用** | `enjoyflow init --default` 一行命令启动 |
| **交互优先** | 能交互就不写配置文件 |
| **AI 辅助** | 配置 = 用户意图 + AI 生成 |
| **可逆** | 所有修改可回滚（`enjoyflow undo`） |
| **可扩展** | 用户自定义生成工具 |

### 7.6 MVP 不做的事

- 语义检索（MVP 用模板/手动够用）
- Web UI（CLI 优先）
- 全 32 类知识（MVP 11 项 P0，个人可 3 类起步）
- 全 8 场景模板（MVP 8 个：new_feature / bug_fix / refactor / hotfix / architecture_decision / release_deployment / code_review / monitoring_response）

### 7.7 演进路径

| 版本 | 时间 | 里程碑 |
|---|---|---|
| v0.1 | 2026 Q3 | MVP：CLI + 3 类起步/11 P0 + ContextFlow 快照 + 8 场景模板 |
| v0.5 | 2026 Q4 | 模板驱动快照 + 扩展场景 + MCP 接入 |
| v1.0 | 2027 Q1 | 全 32 类知识 + 多 AI 工具适配 + 语义检索 |
| v2.0 | 2027 Q3 | 智能快照 + 生态 + 标准化 |

---

## 8. 风险与应对

| 风险 | 概率 | 应对 |
|---|---|---|
| Cursor 自建知识管理 | 高 | 抢先发布，占领生态 |
| Notion 加 AI memory | 高 | 做 Notion 适配器，借力 |
| OpenAI/Anthropic 自建 | 中 | 跟进，被收购 |
| 团队不愿用 | 中 | 降低门槛（CLI 极简） |
| AI 变得太智能 | 低 | 知识管理仍有价值 |
| D 类知识膨胀 | 中 | 定期归档 + TTL + 重要性筛选 |

---

## 9. 验证问题

EnjoyFlow v5 成功的衡量标准：

1. 一个真实项目用EnjoyFlow 完成 10 个 REQ，每个 REQ 都用 ContextFlow 快照
2. 用户反馈"AI 像团队成员了"
3. ≥3 个 AI 工具支持（Cursor / Copilot / Claude Code）
4. ≥10 个第三方适配器（Notion / Obsidian / mem0 等）
5. ContextFlow 快照被其他框架借鉴（开放标准生效）

---

## 附录 A：哲学回溯

| 命题 | 替换关系 |
|---|---|
| "AI 不可靠前提下稳定产出" | 被 §14 根本命题替换 |
| "接口规范 + 默认实现 + 易定制" | v4 三重哲学降级为手段 |
| "5/6 阶段生命周期" | 阶段可配置，10 场景模板替代 |
| "需求 = 流程" | 需求 = 资产不是流程 |
| "决策者 = AI 模型" | 决策者 = 路由器 + 轻量化 |

## 附录 B：v5 与 v4 的核心差异

| 维度 | v4 | v5 |
|---|---|---|
| 根本命题 | AI 时代项目文档与记忆管理 | 人与大模型易于使用的知识管理 |
| 核心能力 | 接口规范 + 默认实现 + 易定制 | **ContextFlow 上下文供给引擎**（核心） |
| 知识范围 | 文档与记忆 | **32 类 ABCD 全覆盖** |
| 用户分层 | 单层（开发者） | **双层（高级用户 / 普通用户）** |
| 决策机制 | 用户自选模板 | **决策者路由（轻量化）** |
| 场景模板 | 单一流程 | **8 场景模板（v5.1）** |
| 跨工具定位 | 未明确 | **明确为中间件** |
| MVP 路径 | 未明确 | **CLI + 3 类起步/11 P0 + ContextFlow 快照** |

## 附录 B2：v5.1 与 v5 的核心差异

| 维度 | v5 | v5.1 |
|---|---|---|
| 一句话定位 | "让 AI 像团队成员一样懂你的项目" | **"工程团队的人机共享任务上下文层"**（三关键词钉死窄缝） |
| 共识语言 | 自有术语 | **对接 Breunig/Schmid**（context rot/quarantine/pruning/offloading/loadout） |
| 护城河 | ContextFlow 单一 | **双护城河**：ContextFlow（先发型）+ 开放生态（壁垒型） |
| 起步门槛 | 11 项 P0 | **3 类起步**（A1+A2+C1）→ 11 P0 → 32 类 |
| 市场判断 | "AI Memory 空白带" | **修正**：通用层有玩家，工程团队任务级共享层无强势品牌 |

## 附录 C：待写文档清单

- [ ] INTERFACE-SPEC §9 协作与多模型契约（基于 §11/§13 重写）
- [x] KNOWLEDGE-ARCHITECTURE v2（基于 §15 32 类清单，已含决策/部署/PRD/规范四组边界修复）
- [x] SCENARIO-TEMPLATES（8 套 MVP 场景模板定义，已含 release/code_review/monitoring）
- [ ] DECISION-ROUTER 规范（决策者层实现）
- [x] CONTEXTFLOW-SPEC.md（原名 D6-SNAPSHOT-SPEC.md，ContextFlow 快照格式 + 生成器规范，已含自指修复）
- [ ] CROSS-TOOL-ADAPTERS（跨工具适配器清单）
- [ ] README 钩子段（参考 Superpowers "step back" 哲学钩子）
- [ ] 5 分钟 demo（参考 Superpowers 早期病毒传播路径）
