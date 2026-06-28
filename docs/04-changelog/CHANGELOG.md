# enjoyknowledge 变更记录

## [v0.4.9] — 2026-06-29

### 7 个硬编码全部抽离到 fixture 文件

**抽离方式**：`include_str!`，跟 v0.4.4/v0.4.5 模式 100% 复用。

**7 个 fixture 新建**：
- `tests/fixtures/skills/coding.md` — SKILLS_CODING_MD_CONTENT
- `tests/fixtures/skills/research.md` — SKILLS_RESEARCH_MD_CONTENT
- `tests/fixtures/skills/review.md` — SKILLS_REVIEW_MD_CONTENT
- `tests/fixtures/skills/design.md` — SKILLS_DESIGN_MD_CONTENT
- `tests/fixtures/skills/README.md` — SKILLS_README_MD_CONTENT
- `tests/fixtures/agents/stage.md` — STAGE_AGENTS_MD_CONTENT
- `tests/fixtures/agents/ek.md` — EK_AGENTS_MD_CONTENT

**skeleton.rs 改动**：
- 7 个 MD_CONTENT 硬编码 → `include_str!`（编译期嵌入 = 0 运行时影响）
- `ek_agents_md_content()` 函数 → `const EK_AGENTS_MD_CONTENT`

**哲学**：让用户能改 fixture .md = 重新编译 = 生效。

**测试**：+2 新测试（`v0_4_9_md_content_uses_include_str` + `v0_4_9_user_can_modify_md_content_via_fixture`）

**0 行为改动**：跟 v0.4.8 100% 兼容。

## [v0.4.8] — 2026-06-28

### skills/ 流程类 skill：4 个工作流抽离

**5 个文件创建**：

`.enjoyknowledge/skills/` 新增 4 个工作流 + 1 个索引：
- `coding.md` — 编码工作流（Hermes skill 格式：接任务 → drafts → promote）
- `research.md` — 调研工作流（ek ls/tree/cat/grep）
- `review.md` — 复盘工作流（onboard → drafts → promote → doctor）
- `design.md` — 设计工作流（ek kind add/rm/list）
- `README.md` — 4 个工作流索引 + 怎么用 + 自定义

**init 行为**：
- 新增 `generate_skills_skeleton()` — 创建 skills/ 目录 + 5 个默认文件
- 存在则跳过（v0.4.4 / v0.4.6 / v0.4.7 模式）
- 0 新格式（Hermes skill 格式 = v0.4.6 已有）

**多 AI 工具**：
- AI 工具通过 AGENTS.md briefly 提到（v0.4.7 已实现）→ 引导读 skills/
- 项目内子目录 = 不一定全部 AI 工具自动加载，但 AGENTS.md 提供入口

**测试**：
- 新增 6 个集成测试：4 flow frontmatter + 1 创建 + 1 不覆盖
- 更新 1 个 trycmd 快照（onboard-ok.stdout：6→11）

**不改的部分**：
- AGENTS.md 不改（v0.4.7 已对）
- init 行为不改（存在则跳过）
- stage AGENTS.md 不改（v0.4.7 已对）

## [v0.4.7] — 2026-06-28

### AGENTS.md 默认内容重写：静态目录说明 + briefly 提到流程

**2 个 AGENTS.md 默认内容重写**：

**`.enjoyknowledge/AGENTS.md`（ek AGENTS.md）**：
- 重写为**静态目录说明**：目录树（11 kind + _meta/）、读写原则、常见操作表
- Briefly 提到流程（1-2 句）：`skills/` 目录引用 + `v0.4.8 候选`
- 移除 metadata.hermes 专属字段（多 AI 工具通用）
- 移除 KB_INDEX 标记和英文命令表（中文静态说明更清晰）

**`.enjoyknowledge_stage/AGENTS.md`（stage AGENTS.md）**：
- 重写为**任务执行流程**：4 步任务流（接任务→写 drafts → promote → 沉淀）
- 目录说明（drafts/ + tasks/ + .archive/）+ ek 命令表
- Briefly 提到流程（1-2 句）：`skills/` 目录引用 + `v0.4.8 候选`

**不改的部分**：
- `generate_agents_md`（根目录 AGENTS.md）永远覆盖——AI 工具入口不能"用户拥有"
- init 行为不变——AGENTS.md 存在则跳过（v0.4.4 / v0.4.6 模式）
- `sync_agents_md_summary` 不变——依赖根目录 AGENTS.md 的 LS_START/LS_END 标记

**测试**：
- 新增 3 个集成测试：不覆盖 + 默认模板 + 任务执行
- 更新 4 个 fixture 文件 + 4 个现有测试断言

## [v0.4.6] — 2026-06-28

### ek AGENTS.md 100% 对齐 stage（不覆盖 + Hermes skill 格式）

**2 大模块同步改动**：

**`generate_ek_agents_md` 不覆盖**：
- `.enjoyknowledge/AGENTS.md` 现在跟 `.enjoyknowledge_stage/AGENTS.md` 行为一致：存在则跳过，不存在才写
- 1 行核心改动：`if !path.exists()` guard

**`ek_agents_md_content` = Hermes skill 格式**：
- 重写 ek AGENTS.md 为 Hermes skill frontmatter + body（与 stage AGENTS.md 格式 100% 一致）
- Frontmatter: `name: enjoyknowledge-kb`、`description`、`version: 1.0.0`、`metadata.hermes`
- Body: Overview / KB Index / AI Read Rules / AI Write Rules / Custom Kind Directories / Commands
- KB Index 表格保留（`<!-- enjoyknowledge_KB_INDEX -->` 标记兼容 `sync_agents_md_summary`）
- 末尾加 `*User-owned*` 注释

**不改的部分**：
- `generate_agents_md`（根目录 AGENTS.md）永远覆盖——AI 工具入口不能"用户拥有"
- `sync_agents_md_summary` 不变——依赖 KB_INDEX 标记

**文档更新**：
- INTERFACE-SPEC: §1.1 AGENTS.md 词条 + §3.2 init 加 v0.4.6 说明
- GLOSSARY: `.enjoyknowledge/AGENTS.md` 词条更新（用户拥有 + Hermes skill 格式）
- ROADMAP: v0.4.6 标记已交付

## [v0.4.5] — 2026-06-28

### `ek kind add/rm/list` + kinds.md 运行时读

**3 大模块同步实施**：

**`ek kind` 子命令（新建 `src/cli/kind.rs`）**：
- `ek kind add <name>` — 新增知识种类：更新 kinds.md + 创建目录 + seed 文件
- `ek kind rm <name>` — 删除知识种类：从 kinds.md 移除（`--force` 同时删除目录）
- `ek kind list` — 列出所有知识种类（表格格式：kind / required / summary）
- 名称校验：alphanumeric/underscore/dash（防注入）
- `--yes` 跳过确认，`--force` 强制删除有内容的目录

**kinds.md 运行时读（改 `src/kinds.rs`）**：
- 新增 `all_from_file(path)` — 运行时读取用户版 kinds.md
- `parse_kinds_md` 改为 `pub`（供 `kind.rs` 和 doctor 使用）
- 保留 `KINDS` 编译期嵌入（向后兼容）

**init + doctor 用运行时读**：
- `generate_ek_skeleton` 优先用 `all_from_file()` 创建目录（回退到 `KINDS` 默认）
- `check_kinds_md` 交叉校验用户版与代码 registry 一致性

**新增文件**：
- `src/cli/kind.rs` — 3 子命令 add/rm/list + 单元测试
- `tests/cmd/kind-add.toml` + `.stdout`
- `tests/cmd/kind-list.toml` + `.stdout`
- `tests/cmd/kind-rm.toml` + `.stdout`

**文档更新**：
- INTERFACE-SPEC: 新增 §3.14 `kind` 子命令，doctor 更新为 5 项检查
- GLOSSARY: kinds.md 词条更新（用户可改 + `ek kind` 命令）
- ROADMAP: v0.4.5 标记已交付

## [v0.4.4] — 2026-06-28

### stage 用户可改：stage-defaults.md + user-owned AGENTS.md + Hermes skill 格式

**4 大模块同步重写**：

**stage-defaults.md 默认目录清单**：
- 新建 `_meta/stage-defaults.md` — 用户可编辑的 Markdown 文档，定义默认 stage 目录
- 编译期 `include_str!` 嵌入，init 时复制到项目（不覆盖用户版本）
- `ek init` 读 stage-defaults.md → 解析 Default Directories 表 → 创建目录
- 回退：文件缺失或为空时回到 v0.4.1 硬编码默认（向后兼容）

**init 不覆盖用户 stage AGENTS.md**：
- 如果 `.enjoyknowledge_stage/AGENTS.md` 存在 → init 跳过，不覆盖
- 如果不存在 → 写默认 Hermes skill 格式
- 同理 `_meta/stage-defaults.md` 存在则跳过

**stage AGENTS.md = Hermes skill 格式**：
- 重写 `STAGE_AGENTS_MD_CONTENT` 为 Hermes skill frontmatter + body
- Frontmatter: `name: enjoyknowledge-stage`、`description`、`version: 1.0.0`、`metadata`
- Body 6 段：Overview / Inputs / Workflow / Custom Directories / Promote Workflow / Hard Gate Protocol

**init 创建目录按 stage-defaults.md**：
- 新增 `parse_stage_defaults()` 解析 Markdown 表格
- `generate_stage_skeleton()` 读 `_meta/stage-defaults.md`（用户版本或默认）→ 按解析结果创建目录
- 始终确保 `.archive/tasks` 存在（向后兼容）

**新增文件**：
- `tests/fixtures/stage-defaults.md` — 编译期嵌入的默认值
- 测试：5 新单元测试 + 4 新集成测试

**文档更新**：
- GLOSSARY: 新 stage AGENTS.md 和 stage-defaults.md 词条
- INTERFACE-SPEC: 目录树加 `_meta/`、init 说明加不覆盖行为
- POSITIONING: v0.4.4 版本号 + "用户可改 stage" 第 6 原则

## [v0.4.3] — 2026-06-28

### kind registry 抽离 + 目录回归 + workflow 重构

**3 大模块同步重写**：

**kind registry 抽离**：
- 新建 `.enjoyknowledge/_meta/kinds.md` — Markdown 表格，kind 注册表的单一真相源
- 新建 `src/kinds.rs` — `kinds::all()`、`kinds::dir_for()`、`kinds::required_fields()`、`kinds::is_valid_kind()`
- `ek promote` 改用 `kinds::dir_for()` 替代硬编码 `kind_to_dir()`，消除 5 处派生逻辑

**目录回归（kind = dir，v0.1 哲学）**：
- 5 目录改名：`gotchas/`→`gotcha/`、`decisions/`→`decision/`、`patterns/`→`pattern/`、`rules/`→`rule/`、`commands/`→`command/`
- 11 类目录统一为 kind 名，无 "s" 派生
- 回归 v0.1 DESIGN-V3 L17 "目录名即分类" 哲学

**workflow 命令重构**：
- `ek onboard` 升格顶层命令（不再 `ek workflow onboard`）
- 砍掉 `ek workflow capture`（用 `ek promote` + AI draft 替代）
- 删除 `src/cli/workflow.rs`（1216 行），抽出 `src/cli/onboard.rs`

**doctor 增强**：
- 新增 kinds.md schema 校验（check #5）：文件存在 + 解析合法 + 与代码 registry 一致

### 破坏性变更

- `ek workflow onboard` → `ek onboard`
- `ek workflow capture` 移除（用 `ek promote` 替代）
- 目录结构变化：`.enjoyknowledge/gotchas/` → `.enjoyknowledge/gotcha/`（5 目录改名）
- `.enjoyknowledge_stage/workflow/` 移除

---

## [v0.4.2] — 2026-06-28

### ek fix 保留 frontmatter 字段

`enjoyknowledge fix` 修复缺 `description` 时，保留已有 frontmatter 字段不覆盖。此前 `fix` 写入新 frontmatter 会丢失用户已有的 `tags`/`timestamp` 等字段。

### 文档重写

重写 12 个核心文档：POSITIONING / ROADMAP / directory-design-rationale / DESIGN-PHILOSOPHY / GLOSSARY / DESIGN / INTERFACE-SPEC / rule-system / workflows / knowledge-types / for-coding-design / CHANGELOG。去掉中间设计迭代痕迹，只体现 v0.4.2 当前状态。

---

## [v0.4.1] — 2026-06-28

### 极简上下文层落地

**ek init 增强**：
- 创建 `.enjoyknowledge_stage/{tasks,drafts,.archive}/` + `workflow/`
- 生成 `.enjoyknowledge/AGENTS.md`（KB 写入规则）
- 生成 `.enjoyknowledge_stage/AGENTS.md`（任务写入规范）
- 生成 `tasks/_template/` 8 文件模板

**ek promote 新增**：
- 从 `.enjoyknowledge_stage/drafts/` 复制到 `.enjoyknowledge/<kind>/`
- 自动生成 4 字段 frontmatter（id/kind/created/author）
- 默认 author = `enjoy`
- 原 draft 保留加 `[PROMOTED]` 标记

**ek stage clean 新增**：
- 默认清理 `.archive/` > 180 天文件
- `--dry-run` / `--force` / `--older-than <days>` 3 个 flag

### 砍掉的能力

- C10 trust 体系（confidence/source/last_verified/feedback_count）
- C11 lifecycle 4 状态机（draft/active/deprecated/archived）
- C12 sync 检测（3 类冲突 + 3 级频率）
- frontmatter 6 字段扩展
- `ek capture --from-commit` 提议门
- 独立 `workflow/` 目录（并入 `stage/`）

### 4 极简原则

1. 人类是 authority anchor
2. 物理分离 > 状态字段
3. AGENTS.md > frontmatter
4. 简单 > 完整

### 测试

- 10 个新增集成测试（init × 5 + promote × 3 + stage clean × 4）
- 28 个 trycmd 端到端测试
- enjoyiot-kaiyuan 端到端 dogfooding 通过

---

## [v1.1] — 2026-06-26

### 战略调整

**规则统一管理**

- 新增 .enjoyknowledge/rules/ 目录约定：项目规则的单点真值源（工具无关 Markdown）
- 方案选型：放弃 rules sync 引擎（方案 A），采用 AGENTS.md RULES 推送块（方案 B），复用已有推送通道
- 三层防护设计：① 源规则层 — 每条规则强制带「适用范围」标注，doctor 检查缺标注 → warning；② 推送层 — dd 时间步更新 AGENTS.md RULES 块，按语言/框架分区；③ 兜底层 — doctor 检查源与推送块一致性
- 边界明确：工具特有规则（Cursor globs、Claude system prompt）留在原生文件，不进入统一管理

**蓝图与路线图**

- BLUEPRINT.md (v1.1) — 新增 §1.2 战略优先级（纵向做深 for Coding），§2.4 for Coding 纵深蓝图表（7 个维度的当前 → 做深对比），§2.5 其他领域应用降级为远期方向
- ROADMAP.md (v1.1) — v0.3 从"预设体系"重写为"for Coding 场景深化"，覆盖种子文件增强、捕获体验提升、质量保障深化、搜索能力增强、AI 集成深度、代码编织、团队工作流基础共 7 个维度；其他领域预设移至长期展望

### 文档

- BLUEPRINT.md — 新增 §1.2、§2.4、§2.5
- ROADMAP.md — v0.3 重写，v0.4-v1.0 调整
- GLOSSARY.md — 新增 
ules/ 条目，含三层防护定义
- PRODUCT-DESIGN.md — §6 目录结构增加 
ules/ 及三层防护说明

---

## [v1.0] — 2026-06-22

### 设计

工程知识资产管理层的完整设计：文件资产工程化、三层分离（格式→原语→模板）、推送+拉取双通道、OKF 兼容、目录即类型、Unix 动词复用。

**核心设计文档：**
- DESIGN-PHILOSOPHY.md — 设计哲学：AI 编程时代知识文档的第一性原理
- DESIGN-V3.md — 系统架构：文件系统即知识库的全貌设计
- PRODUCT-DESIGN.md — 产品设计：交付形态、入口、端到端流程、AI 工具集成、团队共享
- INTERFACE-SPEC.md — CLI 合约：ls/grep/cat/add/init/doctor/fix
- POSITIONING.md — 生态定位：通用引擎 + 编码预设
- GLOSSARY.md — 统一术语
- BLUEPRINT.md — 完整愿景与架构蓝图
- ROADMAP.md — 分阶段路线图
## [v1.2] — 2026-06-27

### 文档重排（6 阶段设计流程）

- **新增**：`docs/02-design/DESIGN.md` — 整体设计入口（合并 DESIGN-V3 + PRODUCT-DESIGN）
- **新增**：`docs/02-design/architecture/for-coding-design.md` — for Coding 完整设计（v4 落地）
- **新增**：`docs/02-design/architecture/rule-system.md` — 规则系统（v1+v2 整合）
- **新增**：`docs/02-design/architecture/knowledge-types.md` — 知识类型（v3 §3 落地）
- **新增**：`docs/02-design/architecture/workflows.md` — 工作流（v4 §4 落地 + YAML schema）
- **新增**：`docs/03-discussion/2026-06-27-{rule-core,3mechanisms,for-coding-deep,for-coding-complete}.md` — 4 轮讨论历史
- **新增**：`docs/03-discussion/README.md` — 讨论索引
- **新增**：`AGENTS.md` — 30-50 行 AI 入口（路由表模式）
- **移动**：
  - `docs/POSITIONING.md` → `docs/00-vision/`
  - `docs/ROADMAP.md` → `docs/00-vision/`（去 OMC 污染）
  - `docs/DESIGN-PHILOSOPHY.md` → `docs/01-philosophy/`
  - `docs/GLOSSARY.md` → `docs/01-philosophy/`
  - `docs/INTERFACE-SPEC.md` → `docs/02-design/`
  - `docs/CHANGELOG.md` → `docs/04-changelog/`
  - `docs/research/SCENARIO-TEMPLATES.md` → `docs/02-design/architecture/workflows.md`（去"模板"歧义）
- **合并**：
  - 3 个 `MARKET-RESEARCH-ADDENDUM-*.md` → `docs/99-archive/research/MARKET-RESEARCH-2026Q2.md`
  - `PRODUCT-DESIGN.md` 内容 → `DESIGN.md`
- **归档**（`docs/99-archive/2026-06-26-codex/`）：
  - `BLUEPRINT.md`（OMC 污染 + 与 v4 重复）
  - `ai-tools-rules-comparison.md`
  - `unified-rule-management.md`
  - `CLAUDE-CODE-RULES-BRIEF.md`
  - `common-concepts-divergences.md`
  - `rule-authoring-template.md`
- **保留历史**（`.bak` 后缀）：
  - `docs/02-design/DESIGN-V3.md.bak`
  - `docs/02-design/PRODUCT-DESIGN.md.bak`
- **OMC 接管版备份**：`AGENTS.md` → `docs/04-changelog/OMC-AGENTS-v4.14.4.md`

### 关键设计基线 v4

- for Coding = "AI 编程工具的共享上下文层"
- 3 机制协同：rule（约束）+ template（范式）+ knowledge（上下文）
- SoT 单一（`.enjoyknowledge/`）+ 入口多元（9 工具）
- 元数据驱动工作流（YAML）
- 显式失败，不静默降级
- 工具特性保留，不强制统一

### 文档统计

- 重排前：19 个文档 / 312KB
- 重排后：24 个文档 / 334KB（增加 5 个 v4 设计 + 讨论历史）

---

## [v1.3] — 2026-06-27（v0.2 收尾）

### 砍功能 4 项（双 AI 验证 + 创始团队决策）

基于 codex + claude 双 AI 独立评审 + GitHub API 90+ 竞品 + Reddit r/ClaudeCode/r/cursor/r/vibecoding 痛点社区证据 + 4 轮市场调研（MARKET-RESEARCH-2026Q2），砍 v0.2 scope 到最小可 ship 版本：

1. **9 工具 → 2 工具**（首发 Claude + Cursor）
   - 1 工具会"杀 thesis"（定位塌缩到"Cursor 辅助工具"）
   - 2 工具证明跨工具概念
   - 架构保留 9 工具 adapter trait，v0.3+ 渐进
   - 首发 Claude（r/ClaudeCode 90+ 评论"AGENTS.MD standard"+ 适配更简单 CLAUDE.md 追加 vs .mdc frontmatter + 项目维护者工具栈）

2. **5 工作流 → 2 工作流**（onboard + capture）
   - workflows.md §4.2 preflight.yaml 整段删（46 行）
   - workflows.md §4.5 prd-preprocess.yaml 整段删（38 行）
   - preflight / prd-preprocess 永久禁用（保留历史描述）

3. **3 scope → 1 scope**（只 project）
   - team / user scope 永久不实现
   - rule 文档已删 scope 字段（commit 2dadb14）

4. **禁 rule_code_sync**（NLP 级不可行）
   - rule-system.md §8 改"永久禁用"
   - doctor 不再检查 R-Code 一致性

5. **命令 sync → export**（1 工具时 sync 撒谎）
   - export 暗示单向导出，诚实
   - 未来真 sync 留名

### 定位（v4.2）

- 一句话："**一份 markdown，多个 AI 工具**"（v0.2 首发 Claude + Cursor）
- 推销话术：Claude 写代码 + Cursor 审 PR = 1 份 markdown export 2 工具
- 差异化：3 维组合（多工具 + frontmatter schema + YAML 工作流）vs ECC 222K / planning-with-files 24K / ai-rules-sync 124 ★

### 文档修复 5 commits

| commit | 改 | 价值 |
|---|---|---|
| `ff2af5a` | 3 P0 致命（删 2 .bak 38.9KB + GLOSSARY v4 重写 + workflows filter 语法）| 修 3 致命问题 |
| `2dadb14` | 4 P1（命令名 / 8→10 类 / 6 类 schema / 9 工具 sync 示例）| 修 4 中等问题 |
| `6c4c316` | v4.2 定位 + 砍功能（POSITIONING v4.1 + 3 architecture 改）| 锁 v0.2 定位 |
| `69c12ff` | 清 v4.2 残留（11 处：GLOSSARY + POSITIONING + DESIGN）| 防 v4.2 不一致 |
| `e90ef48` | 砍 5→2 工作流 + 禁 rule_code_sync（5 文档原子一致）| 砍 4 项落地 |
| `d035def` | 跟进 POSITIONING + AGENTS（5 处残留）| 防"定位文档说一套、设计文档说另一套"|

### 调研证据

- **GitHub API 90+ 竞品**：跨 AI 工具 rule sync 赛道 90 个项目（最大 124 ★）/ Provider-agnostic 知识管理 1 个（0 ★）/ 跨 agent memory sync 23 个（< 15 ★ 中位）
- **痛点社区证据**：
  - 痛点 A 跨工具同步（7-8/10）：r/vibecoding 14 答案"switching AI tools kills flow"
  - 痛点 B 知识→AI 上下文（**9/10**）：r/ClaudeCode 90 评论"AGENTS.MD standard" + r/cursor 10 评论 40%→92% 合规率
  - 痛点 C 任务临时文件（5-6/10）：ECC 222K "memory + sessions" 二分模型

### v0.2 终态（文档层面）

- 2 工具：Claude + Cursor
- 2 工作流：onboard + capture
- 1 scope：project
- 1 命令：export（非 sync）
- 4 维 doctor（去 rule_code_sync）

### 下一步（v0.2 真正能 ship）

- 实施 `enjoyknowledge export --tool claude` + `--tool cursor` 命令（Rust + clap）
- AGENTS.md 路由表模式生成
- capture 工作流的 SoT 写入 + 必填字段校验

---

## [v1.4] — 2026-06-28（v0.2.1 收尾 + 行为反转记录）

### v0.2.1 实施（15 commits）

v0.2 收缩后首轮落地，覆盖 6 条命令线：

- **export MVP**（`3470412`）：首发 2 工具导出 `--tool cursor` + `--tool claude`，AGENTS.md 路由表模式生成
- **workflow onboard**（`4a768bf` → `cd9eea2`）：`enjoyknowledge workflow run onboard` 命令实现
- **workflow capture**（`bdcd81f`）：`enjoyknowledge workflow run capture` 命令实现
- **capture 路径修复**（`7454797` → `b5d2d15`）：v0.2.1 capture 路径大小写不一致修复
- **doctor 4 项重写**（`3156695` → `d476e5f`）：4 项健康检查重写，删死代码 + 同步 INTERFACE-SPEC §7
- **doctor 抽象修复**（`b7a84f5`）：check_agents_md / check_pending_archive 走 FilesystemSource
- **UTF-8 修复 + WINDSULF 拼写**（`0a30738`）：修 UTF-8 mojibake + WINDSULF → WINDSURF

**v0.2.1 共 15 commits（2026-06-27 ~ 2026-06-28）**

### AiTool::from_str 行为反转记录

- **v0.1 早期**：`AiTool::from_str` 未知字符串默认返回 `Some(AiTool::Auto)`（宽松默认）
- **v0.1.1**：改为 `_ => None`（严格模式，未知字符串显式失败）—— **此行为反转未记录 CHANGELOG**
- **v0.2.1（当前）**：保持 `_ => None`，10 个合法值（auto / cursor / claude / copilot / windsurf / cline / codex / trae / gemini / generic），大小写不敏感
- **补测**：C2 任务补 17 个单元测试覆盖所有 variant + 大小写 + 未知字符串 + 回归保护

---

## [v1.5] — 待发布（v0.3 一站式收尾）

### 版本合并决定

**原计划**：v0.3（6 大类 25 个功能点）+ v0.4（5 大类 17 个功能点）分 2 个版本，3-4 个迭代/版。

**改后**：v0.3 = 2 大类核心（捕获体验 + 质量保障深化），预计 1-2 个迭代一站式收尾。原 v0.3 剩余 4 大类（搜索/AI 集成/代码编织/团队工作流基础）+ 原 v0.4 全部 5 大类合并到 v0.4。

**理由**：v0.2 已稳定且真实可用，21 commits + 9 命令 + 2 子命令全部端到端跑通。**版本号对外是噪音，不是价值**——v0.3 / v0.4 / v0.5 / v1.0 / v1.x+ 5 个版本号够用。

### v0.3 核心 2 大类

**核心 1：捕获体验**（让 add / capture 零摩擦）
- [ ] `add` 重复检测
- [ ] `add` 自动 tags 建议
- [ ] `add --from-commit`
- [ ] `add --dry-run`
- [ ] 种子文件增强（填写指南 / 常见场景 / 反例警示 / 跨文件关联提示）

**核心 2：质量保障深化**（内容质量维度 + v0.2 善后）
- [ ] 描述一致性检查
- [ ] 跨文件引用有效性
- [ ] 预算与拆分建议
- [ ] **fix.rs 适配 v0.2 4 项 check**（v0.2 善后）

### v0.3 不做（合并到 v0.4）

- 搜索能力增强（grep --related/--semantic）
- AI 集成深度（context 命令 / 智能推送）
- 代码编织（git hook / link 命令）
- 团队工作流基础（doctor --ci JSON）
- 多仓库知识链接
- 共享知识库
- 组织级功能
- 知识废弃
- CI 阻断
