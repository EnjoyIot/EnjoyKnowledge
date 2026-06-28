# v0.3+ 目录重设计：从 5 目录到 8 目录

> 状态：草稿 | 日期：2026-06-28
>
> 本文档解释为什么 v0.2 的 5 目录需要改为 8 目录，以及每项设计决策的推理过程。
> 不包含代码级实施细节——那些属于 commit message 和 `INTERFACE-SPEC.md`。

---

## 1. 背景：v0.2 5 目录的现状

v0.2 的 `for-coding` profile 硬编码 5 个知识目录：`architecture`、`gotchas`、`patterns`、`business`、`decisions`。这个集合定义在 `src/profile/coding.rs:25`，在 `src/init/skeleton.rs:12-32` 中由 `generate_skeleton` 逐目录创建，配套 5 个 seed 模板（`src/profile/coding.rs:29-35`）。这是早期的直觉设计——没有用户研究、没有竞品对标、没有认知框架支撑。它"能用但不够好"。

全项目引用面：`INTERFACE-SPEC.md` §2.1（第 25-36 行）将 5 目录列为"for Coding 默认结构"；`POSITIONING.md` L124 将 `init` 列为 10 个 CLI 命令之一但其输出由这 5 目录定义。任何改动都需同步这些文件。

经 R1 诊断，5 目录存在 3 个真问题：

**问题 1：缺 3 个高频类。** `rules/`（强制规则）、`commands/`（build/test/deploy 步骤）、`context/`（项目背景）在真实项目的 AGENTS.md 中反复出现，但 v0.2 的 init 不给它们建目录。用户只能手建，或在 `architecture/` 里塞不属于架构的内容。

**问题 2：`business/` 不通用。** `business/` 定义域规则、合规、计费逻辑——这对 CRM/电商/水务系统有意义，但对通用 coding 项目永远是空目录。`src/profile/coding.rs:33` 的 seed 模板标题是"Business Rules"，描述是"domain-specific rules that code must enforce"——当项目没有"域"时，这个目录对 AI 是噪音。

**问题 3：kind 与目录不对齐。** `src/cli/workflow.rs:357-368` 定义了 10 个合法 kind（gotcha / decision / pattern / rule / business / architecture / contract / convention / context / template），但 init 只建 5 个目录。其中 5 个 kind（rule / contract / convention / context / template）没有对应的 init 目录——当用户执行 `capture --kind contract` 时，`run_capture`（`src/cli/workflow.rs:566`）会通过 `default_path_for_kind` 自动创建目录（`src/cli/workflow.rs:381-403`），但这种"隐式自动建"不在 init 输出中，用户看不到，属于不规范行为。

---

## 2. 设计原则

目录设计不是审美选择——它直接影响 AI 查找知识的速度和人类的心理负担。本次重设计遵循 5 条原则：

**认知映射。** 目录名必须对应"开发过程中会自然想到的某类信息"。开发者遇到问题时不会想"我该找一个 pattern 还是一个 convention"，他们想的是"这项目的规则是什么 / 怎么构建 / 背景是什么"。目录名应匹配这类自然语言查询。

**频次优先。** 高频类——每次开发都要查的（构建命令、强制规则、项目背景）——应放在显眼位置且命名直观。低频类（如 ADR）可以放在列表后面。当前 `business/` 在频次上不如 `rules/`，但 v0.2 给了它一个中心位置。

**可扩展。** profile 系统已支持多 profile（coding / design / research），每个 profile 可定义不同的目录集。8 目录是 `for-coding` 的默认值，但 `for-design` 可能有完全不同的结构（`moodboards/`、`assets/`、`design-system/`）。目录集随 profile 而变，不绑死。

**kind-目录对齐。** 每个合法 kind 必须有 init 目录。capture 不再做隐式自动建。用户执行 init 后就能看到所有 8 个目录，不需要通过 capture 发现"哦还有这个目录"。这消除了 v0.2 的 10 kind → 5 目录的 gap。

**业务可选。** `business/` 不绑死 `for-coding` profile。对于通用 coding 项目，`business/` 默认为空目录（存在但不生成 seed 文件），对于业务系统，用户可主动填充。这比"不建"好——存在空目录提示"这里有空间可用"，比完全不存在更能引导用户。

---

## 3. 借鉴对标

R1 诊断 + codex 第三轮评审，6 个对标系统各有取舍：

**Code as Craft (Etsy)。** 5 类：How / Why / What / Conventions / Glossary。How ≈ commands/，Why ≈ decisions/，What ≈ context/，Conventions ≈ patterns/ + rules/（但 Etsy 不区分强制规则和推荐模式）。Glossary 在我们体系中等价于术语表文档，不需要独立目录。

**Arc42。** 12 节架构模板：Introduction / Constraints / Context / Solution Strategy / Building Block View / Runtime View / Deployment View / Crosscutting / Architecture Decisions / Quality / Risks / Glossary。12 节对日常 coding 太重——Arc42 是为架构文档设计的，不是为"AI 30 秒建立心智模型"设计的。我们取其中 context/（约束+背景）、decisions/（ADR）、architecture/（模块图+部署）三节，其余合并或省略。

**Diataxis。** 4 类：tutorial / how-to / reference / explanation。这是文档分类法，不是知识分类法。tutorial 在我们的体系里没有对应目录（那是 README 和 docs 的职责）；how-to ≈ commands/；reference ≈ architecture/；explanation ≈ decisions/ + patterns/。Diataxis 对"知识应该怎么组织"有启发，但对"AI 应该查什么目录看什么"帮助不大。

**Divio（codex 评审补充）。** 4 类：tutorial / how-to / reference / explanation。**与 Diataxis 高度相似但更面向开发者文档**。Divio 强调"4 类不应该混"——explanation 不能放 how-to 目录，反之亦然。这与 rules/ 强制分离 patterns/ 的思路一致：分类互斥原则。**我们的 8 目录借鉴了 Divio 的"互斥分类"思路**。

**ADR。** 1 类：decision。已被 v0.2 的 `decisions/` 正确吸收。保持。

**Postmortem。** 3 段：incident / root cause / prevention。这≈我们的 gotchas/ 的 `##` 结构（Instance / Impact / Workaround 三段式——见 `src/profile/coding.rs:93-96`）。已吸收，保持。

**关键启发：** 6 个对标系统的交集是 "context + decisions + patterns + gotchas + architecture"——这 5 个目录有跨系统共识。我们缺的 3 个（rules / commands / context）中，context 在 Arc42 和 Divio 中都被强调为独立类别；rules 是从 patterns 中强制分离出来的——对标系统都不太区分"推荐"和"必须"，但在 AI 编程场景下，这个区分是关键的（AI 应该无条件遵守 rules，可以自行判断是否使用 patterns）。

---

## 4. v0.3+ 8 目录重设计

```
architecture/   系统结构、模块图、tech stack
commands/       build / test / deploy 步骤
context/        项目背景、约束、用户、目标
decisions/      ADR（why）
gotchas/        踩坑、陷阱、workaround
patterns/       最佳实践、推荐
rules/          强制规则（区别于 pattern）
business/       业务规则（条件：仅业务系统）
```

### 4.1 `architecture/` — 系统结构图

**定位：** 系统的静态蓝图——模块划分、技术选型、数据流向、部署拓扑。是 onboard 工作流（`src/cli/workflow.rs:105`）建立心智模型的第 4 步。

**典型内容：** 模块地图、tech stack 表、组件间通信图、部署架构。

**什么算"满"：** 3-5 个文件——overview（模块地图+技术栈）、data-flow（关键数据路径）、deployment（部署拓扑）。

**借鉴：** Arc42 §3-8（Building Block / Runtime / Deployment View 之和）。

**反例：** 不要把"后端是 Rust"这种一行事实单独建文件——那属于 context/ 或 tech-stack.md 的一行。

### 4.2 `commands/` — 操作步骤

**定位：** 开发者每次打开项目都要查的东西——怎么构建、怎么测试、怎么跑起来。**为什么频次最高**（直觉判断，无数据支撑）：每个新 session 第 1 件事就是 build，第 2 件事就是 test，第 3 件事就是 deploy——这 3 个动作 100% 会发生。

**典型内容：** `build.md`（`cargo build` 还是 `just build`）、`test.md`（`cargo test` 还是 `just test`，需要什么环境变量）、`deploy.md`（推哪个分支触发什么）。

**什么算"满"：** 3 个文件——覆盖 build / test / deploy 三阶段。

**借鉴：** Code as Craft 的 How 类 + Diataxis 的 how-to 类。

**反例：** 不要把每个 flag 都写成独立文件——`commands/` 是步骤手册，不是 CLI reference。

### 4.3 `context/` — 项目背景

**定位：** 为什么这个项目存在、为谁服务、有什么约束。新成员 onboard 时读的第一个目录。

**典型内容：** 项目目标（1 句话）、目标用户、技术约束（必须用 PostgreSQL / 必须跑在 k8s）、非技术约束（deadline / budget / compliance）。

**什么算"满"：** 1-2 个文件——`project.md`（背景+用户+目标）、`constraints.md`（技术+非技术约束）。

**借鉴：** Arc42 §2-3（Constraints + Context）。

**反例：** 不要放"为什么要做这个产品"的商业叙事——那是 PRD 的职责。context/ 是给 AI 和开发者的，只放影响技术决策的事实。

### 4.4 `decisions/` — 架构决策记录

**定位：** 回答"为什么这样设计"。v0.2 已有且正确，保持。

**典型内容：** 每个 ADR 一个文件（`001-use-sqlite.md`），含 Context / Decision / Consequences 三段。

**什么算"满"：** 5-15 个有意义的 ADR。空的 `000-template.md`（`src/profile/coding.rs:133`）不算。

**借鉴：** ADR（Michael Nygard）。

**反例：** 不要把"为什么选 Rust"写成 10 页论文——ADR 是轻量级决定的记录，不是语言辩护书。

### 4.5 `gotchas/` — 踩坑记录

**定位：** 开发过程中遇到的意外行为、tricky bugs、workaround。trigger 字段（`GLOSSARY.md` 的 gotcha 词条）决定 AI 什么时候应该读这条。

**典型内容：** 每个 gotcha 一个 `##` 条目，含 Instance / Impact / Workaround / Related。

**什么算"满"：** 3-10 条有意义的 gotcha。单文件 `gotchas.md` 超过 20 条时 doctor 会建议拆分。

**借鉴：** Postmortem 的 incident / root cause / prevention 三段式。

**反例：** 不要把"API 返回 500"这种无根因的日志当 gotcha——gotcha 必须包含可复现的触发条件和已验证的 workaround。

### 4.6 `patterns/` — 推荐模式

**定位：** 已验证有效、但非强制的实践。开发者可以参考，也可以选择不同方案。

**典型内容：** 每个 pattern 一个 `##` 条目，含 When / How / Why / Anti-pattern 四段（`src/profile/coding.rs:101-113` 的 seed 模板）。

**什么算"满"：** 3-8 个有意义的 pattern。

**借鉴：** Code as Craft 的 Conventions 类 + Diataxis 的 reference 类。

**反例：** 不要把"代码格式化用 prettier"这种工具配置当 pattern——那属于 rules/ 或 `.prettierrc`。pattern 是设计层面的推荐，不是工具层面的开关。

### 4.7 `rules/` — 强制规则

**定位：** 不可协商的约束。AI 必须遵守，违反即 bug。这是 8 目录中最关键的区分——v0.2 把"推荐"和"强制"混在 patterns/ 里，AI 无法区分优先级。

**典型内容：** 每个 rule 一个 `##` 条目，含 Rule / Scope / Code location / Test（`src/profile/coding.rs:117-131` 的 seed 模板的规则版本）。

**什么算"满"：** 2-5 条核心规则。rule 是稀缺品——只有真正不可协商的事才放这里，多了就成了 patterns 的重复。

**借鉴：** 从 patterns 中强制分离出来的新类——对标系统中没有直接对应，但在 AI 编程场景下是关键创新。

**反例：** 不要把"建议用 async/await"当 rule——那就是一个 pattern。rule 必须是"只能这样做，不能那样做"。

### 4.8 `business/` — 业务规则

**定位：** 业务域规则——当项目是业务系统时才需要。通用 coding 项目下默认为空目录（无 seed 文件）。

**典型内容：** 计费公式、合规要求、审批流程、角色权限矩阵。

**什么算"满"：** 只有在业务系统中才算满。空目录不算"缺"——它提示"这里可以放业务规则"但不需要被填满。

**反例：** 不要把"产品需求"放 business/——那是 PRD 的职责。business/ 放的是"代码必须执行"的业务约束，不是"产品想要什么"。

---

## 5. kind 与目录对齐

v0.2 有 10 个 legal kind（`src/cli/workflow.rs:357-368`），但只有 5 个 init 目录。v0.3+ 实现一对一映射：

| kind | 目录 | 说明 |
|---|---|---|
| `architecture` | `architecture/` | 系统结构（v0.2 已有） |
| `command` | `commands/` | 操作步骤（v0.3+ 新增） |
| `context` | `context/` | 项目背景（v0.3+ 新增） |
| `decision` | `decisions/` | ADR（v0.2 已有） |
| `gotcha` | `gotchas/` | 踩坑（v0.2 已有） |
| `pattern` | `patterns/` | 推荐模式（v0.2 已有） |
| `rule` | `rules/` | 强制规则（v0.2 只有 kind 没 init 目录） |
| `business` | `business/` | 业务规则（v0.2 已有，改可选） |
| `contract` | `contracts/` | API 契约、接口约定 |
| `convention` | `conventions/` | 代码格式、命名规范、提交格式（与 rule 区分：convention 是可工具检查的规范，rule 是设计层面的强制） |
| `template` | `templates/` | 模板文件 |

> 注：`contract`、`convention`、`template` 三个 kind 在 v0.2 已存在但无 init 目录。v0.3+ 给它们正式目录：`contracts/`、`conventions/`、`templates/`。

---

## 6. 迁移路径（v0.2 → v0.3+）

### 决策树（codex 评审补充）

```
你的项目是 _____________
│
├─ 生产环境（用户已用 v0.2）→ 策略 A（保留不动，最稳）
├─ 活跃开发（你天天用 capture）→ 策略 B（自动平移）
└─ 想重组结构（清理时机）→ 策略 C（手动升级）
```

### 新项目

直接使用 8 目录。init 输出：

```
.enjoyknowledge/
├── architecture/
├── commands/
├── context/
├── decisions/
├── gotchas/
├── patterns/
├── rules/
├── business/        # 空目录，无 seed 文件
├── contracts/
├── conventions/
├── templates/
├── index.md
```

### 旧项目（已有 5 目录）

提供三种兼容策略，用户自选：

**策略 A：保留不动（推荐给生产项目）。** v0.3+ 的 capture 和 doctor 读取时兼容旧 5 目录结构。新 capture 写入时走 8 目录映射，但已有文件不动。doctor 对旧目录结构只 `info` 不 `error`。

**策略 B：自动平移（推荐给活跃开发项目）。** 运行 `enjoyknowledge migrate --from v0.2`。执行：① 创建缺失的 3 个目录（`commands/`、`context/`、`rules/`）；② 创建缺失的 3 个 kind 目录（`contracts/`、`conventions/`、`templates/`）；③ `business/` 的 seed 文件**保留不动**（避免数据丢失），只新增 `business/000-README.md` 提示"v0.3+ 起 business/ 为可选，业务系统才填"；④ 输出平移报告。**绝不删除任何已有文件**——所有变更可逆。

**策略 C：手动升级。** 用户按迁移指南手动重组目录（适合自己想做结构调整的项目）。

### 迁移示例（策略 B）

```bash
# v0.2 状态：5 目录
$ enjoyknowledge ls
architecture/  business/  decisions/  gotchas/  patterns/

# 执行迁移
$ enjoyknowledge migrate --from v0.2
== migrate: v0.2 → v0.3+ ==
created:   commands/
created:   context/
created:   rules/
created:   contracts/
created:   conventions/
created:   templates/
removed seed: business/business.md (empty dir kept)
migration complete. 5→11 directories (8 knowledge + 3 extended)
run 'enjoyknowledge doctor' to verify.

# v0.3+ 状态：8 目录
$ enjoyknowledge ls
architecture/  business/  commands/  context/  decisions/
gotchas/  patterns/  rules/
```

### v0.2.1 的 capture 行为不变

v0.2.1 用户用 `capture --kind contract` 写入的文件路径保持不变（`contracts/contracts.md`），但 v0.3+ 中该目录在 init 时就存在——不再是隐式自动建。向后兼容。

---

## 7. 风险评估

**风险 1：capture 命令 kind→dir 映射改动 + 单复数 bug。** `src/cli/workflow.rs:381-403` 的 `default_path_for_kind` 需要更新——但**当前代码 + 文档有 R6 漏洞**：kind `"contract"` 在代码中映射到 `contract/`（singular），但 §5 设计文档写 `contracts/`（plural）。`convention` → `convention/` vs `conventions/` 同理。`default_path_for_kind` 现 L387 `_ => kind` 用 kind 名作为目录名（contract / convention / context / template），与 §5 表的复数目录名（contracts / conventions / contexts / templates）不一致。

**统一规则（v0.3+ 决定）**：目录名 = kind 名 + s，**除非不可数**（architecture / business / context / template 用原形）。这与 v0.2 已有的"patterns/gotchas/decisions"惯例一致。完整映射：

| kind | 目录 |
|---|---|
| architecture | architecture/ |
| business | business/ |
| command | commands/ |
| context | context/ |
| convention | conventions/ |
| contract | contracts/ |
| decision | decisions/ |
| gotcha | gotchas/ |
| pattern | patterns/ |
| rule | rules/ |
| template | templates/ |

影响面：使用 `capture --kind contract` 的 v0.2.1 用户脚本（写入 `contract/contracts.md`）→ v0.3+ 改为 `contracts/contracts.md`。**v0.3+ 必须用新路径**——这是破坏性变更，需要在 CHANGELOG v1.5 标注 + 在 `init` 升级时给旧路径文件 `mv` 操作（迁移工具负责）。

**风险 2：`business/` 可选化。** 最终决策：始终创建空 `business/` 目录（与 §4.8 一致）。无 seed 文件——通过 `business/000-README.md` 一行提示"业务系统才填"代替。`contracts/`、`conventions/`、`templates/` 同策略。**统一原则：init 输出 11 个目录，8 个有 seed，3 个空骨架**。

**风险 B 完整工具集**（codex 评审补充）：`migrate --from v0.2`（平移）+ `migrate --rollback`（回退）+ `migrate --dry-run`（预览）+ `migrate --status`（查当前版本）。rollback 必须保留：所有 v0.3 新建目录的清单 + v0.2 单复数改名清单（contract/ → contracts/ 等），保证一键回退到 v0.2 路径。

**风险 3：AGENTS.md 路由表膨胀（codex 评审关键发现）。** v0.2.1 A1 实现的 AGENTS.md 路由表为 5 目录设计（`src/cli/workflow.rs:122`）。8 目录 → 11 目录（含 contracts/conventions/templates）= 路由表从 5 行变 11 行。**这破坏 v0.2 核心承诺"30 秒建立心智模型"**。缓解方案：路由表用分组折叠（3 组：Core（5）+ Context（3）+ Conditional（3）），AGENTS.md 顶部展示 Core 5 行，Context 在 `<details>` 折叠块，Conditional 标注"v0.3+ 才用"。token 成本：当前 5 行 ≈ 80 tokens，新方案 Core 5 行 ≈ 80 tokens（不变）+ Context 折叠 ≈ 30 tokens 隐藏 = 实际增量很小。

**风险 4：doctor 检查项膨胀（codex 评审关键发现）。** v0.2 4 项 check → v0.3+ 预计 ~15 项（11 目录空/非空 + seed 完整性 + index.md 覆盖 + 新 4 项 check 关联）。文档未列出 v0.3 doctor 完整清单是信息缺失。**v0.3+ 强制要求**：必须在 INTERFACE-SPEC §7 列出 doctor 完整 check 清单 + 每项 severity。

**风险 5：`POSITIONING.md` L124 要同步改。** 该行写"10 个 CLI 命令"，需要确认 v0.3+ CLI 命令数是否变化（不变——init 输出变但 init 命令本身在）。

**风险 6：`INTERFACE-SPEC.md` §2.1 要同步改。** 第 25-36 行的目录结构图需从 5 目录改为 8 目录。§4.2 的 `ls` 示例输出（第 113-122 行）需更新目录列表。

**风险 7：`GLOSSARY.md` 的 for Coding 词条。** 描述从"默认知识结构（10 类资产）"改为"默认知识结构（11 类资产 + 8 个目录）"。

---

## 8.5 全 v0.3 估时（codex 评审补充）

v0.3 完整版 = C1-C7（7 commit / 6-8 天） + C8（目录重设计 8 子步骤）。**总估时**：

| 阶段 | commit | why 估 | claude 估 | codex 估 | 最终 |
|---|---|---|---|---|---|
| C1 | add --dry-run + --field | 0.5 天 | agree | agree | 0.5 天 |
| C2 | add 重复检测 | 0.5 天 | agree | agree | 0.5 天 |
| C3 | add 自动 tags | 0.5 天 | agree | agree | 0.5 天 |
| C4 | add --from-commit | 0.5 天 | agree | agree | 0.5 天 |
| C5 | 种子文件增强 | 1 天 | agree | agree | 1 天 |
| C6 | doctor 跨文件引用 + 描述一致性 | 1.5 天 | 2 天 | agree | 2 天 |
| C7 | fix.rs 适配 + 预算拆分 | 1.5 天 | 2 天 | agree | 2 天 |
| C8 | 目录重设计 8 子步骤 | - | 0.5 天 | **2.5-3 天** | **2.5-3 天** |
| **v0.3 全量** | | **6-8 天** | | | **9-11 天** |

**关键上调**：
- C6 +0.5 天（claude 谨慎评估）
- C7 +0.5 天（4 新 case + 8 snapshot 测试）
- C8 +2-2.5 天（codex 估 migrate 2h + dogfooding 1.5h + 决策树/rollback/路由表/doctor 列表/8 文档同步）

---

## 8. 实施步骤（v0.3+ 内部拆 commit）

| 步骤 | 内容 | 时间 | 依赖 |
|---|---|---|---|
| C8.1 | `src/profile/coding.rs:25` 目录列表 5→8 | 0.5h | 无 |
| C8.2 | `src/profile/coding.rs:29` seed 文件 5→8（新增 commands/context/rules 的 seed） | 0.5h | C8.1 |
| C8.3 | `src/cli/workflow.rs:381-403` kind→dir 映射更新（新增 command kind + 确认 context mapping） | 0.25h | C8.1 |
| C8.4 | `src/init/skeleton.rs` 兼容旧结构（init 时检测已有 .enjoyknowledge/ 为 5 目录 → 输出升级提示） | 0.5h | C8.1 |
| C8.5 | 迁移工具：`enjoyknowledge migrate --from v0.2` 命令 | 1h | C8.1-4 |
| C8.6 | 文档同步：POSITIONING / GLOSSARY / INTERFACE-SPEC / ROADMAP | 0.5h | C8.1-5 |
| C8.7 | 废弃警告：`capture --kind contract/convention/template` 输出 `[INFO]` 提示 init 已有目录，建议先 `enjoyknowledge init --reinit` | 0.25h | C8.3 |
| C8.8 | dogfooding 验证：在 `enjoyiot-kaiyuan` 仓库上跑 `init` + `capture` 全流程 | 0.5h | C8.1-7 |
| **合计** | | **1-1.5 天** (含缓冲；claude 直觉估 0.5 天偏乐观，参考 v0.2.1 D 批次) | |

> 1 天是上限，实际可能半天内完成。commit 顺序：C8.1-3 一个 commit，C8.4-5 一个 commit，C8.6-7 一个 commit，C8.8 一个 commit——共 4 个 commit 分 2 批（代码 + 文档 + 验证）。
