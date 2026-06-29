# v0.4.4 完工复盘 — 2026-06-28

## 1. 一句话总结

**v0.4.4 完整重写完工**——stage 用户可改（stage-defaults.md + user-owned AGENTS.md + Hermes skill 格式），**10 文件改动 / +411/-170 净 +241 行** / **99 单测 + 22 trycmd + 25 集成全绿** / **真实 dogfooding 核心不覆盖测试通过**。

## 2. Why（为什么做 v0.4.4）

### Enjoy 5 个问题 + 5 个答案

| 问题 | Enjoy 答案 |
|------|---------|
| Q1: 默认 stage 目录从哪里来？ | "**有个 md 文档里列默认目录，系统根据这个 md 来生成默认目录**" |
| Q2: 用户改 stage 目录的入口？ | **A** = 直接编辑 stage AGENTS.md |
| Q3: 改了 stage AGENTS.md AI 怎么知道？ | **B** = AI 启动重读 + 触发 |
| Q4: init 覆盖怎么解决？ | **B** = stage AGENTS.md 用户拥有 = init 永远不覆盖 |
| Q5: skill 里面说明怎么写？ | "**按 skill 的组织形式写**" = Hermes skill frontmatter + body |

### 9 轮讨论沉淀

| 轮 | R1 错在哪 |
|----|----------|
| 1-7 | R1 自己定方案 = 全部偏离 Enjoy |
| 8 | R1 问 5 问 = 学到 |
| **9 (v0.4.4 拍板)** | R1 听清 Enjoy 5 答 = 完整方案 |
| **10 (实施)** | R1 设计 + claude 实施 + R1 评审（D1 模式）|

### R1 9 轮反思教训

| 教训 | 沉淀 |
|------|------|
| **R6 v3 强化** | "R1 不要脑补"——9 轮自己定方案 = 全部错 |
| **R6 v9 强化** | "看似对 ≠ 真对"——YAML 配置被 Enjoy 拒 = **v0.4.4 用 Markdown 不用 YAML** |
| **R6 v12.1 强化** | "看代码再说"——v0.4.4 跟 v0.4.3 kinds.rs 嵌入方式一样（编译期 include_str!）|

## 3. What（v0.4.4 完整范围）

### 模块 1: 新增 `_meta/stage-defaults.md`

**目标**：默认 stage 目录清单 = **用户可编辑**的 Markdown 文档

- 新建 `tests/fixtures/stage-defaults.md` (24 行)
- 编译期 `include_str!` 嵌入 `src/init/skeleton.rs:12` `STAGE_DEFAULTS_MD_DEFAULT`
- init 时**复制**到 `.enjoyknowledge_stage/_meta/stage-defaults.md`（**用户拥有 = 可改**）
- 下次 init = **不覆盖**用户的版本

### 模块 2: 改 init 行为

**目标**：init **不覆盖**用户已改的 stage AGENTS.md + **读** stage-defaults.md

- `init/skeleton.rs` 加逻辑：
  - 如果 `.enjoyknowledge_stage/AGENTS.md` **存在** → **不覆盖**（**Q4 答案**）
  - 如果 `.enjoyknowledge_stage/_meta/stage-defaults.md` **存在** → **不覆盖**
  - 如果不存在 → 写默认 Hermes skill 格式
- 新增 `generate_stage_defaults_md()` 函数 (L268-274)
- `generate_stage_agents_md()` 加存在检查 (L293-299)

### 模块 3: 重写 stage AGENTS.md = Hermes skill 格式

**目标**：stage AGENTS.md 按 Hermes skill frontmatter + body 组织

- 改 `STAGE_AGENTS_MD_CONTENT` 常量 (L387)
- 加 frontmatter：`name: enjoyknowledge-stage` + `description` + `version: 1.0.0` + `metadata.hermes`
- body 6 段：Overview / Inputs / Workflow / Custom Directories / Promote Workflow / Hard Gate Protocol
- **用户可改** = init 不覆盖

### 模块 4: 改 init 创建目录用 stage-defaults.md

**目标**：init **读** stage-defaults.md 创建默认目录

- `init/skeleton.rs` 加函数 `parse_stage_defaults(md_content) -> Vec<StageDir>`
- `generate_stage_skeleton()` 改：
  - 读 `.enjoyknowledge_stage/_meta/stage-defaults.md`（用户版本或默认）
  - 解析出目录列表 + 任务文件列表
  - 按解析结果创建目录
  - **fallback** = 用户没改 → 用 v0.4.1 默认（向后兼容）

## 4. How（实施步骤）

| 步骤 | 内容 | 结果 |
|------|------|------|
| 1 | 写 v0.4.4 design doc (`why-workspace/v0.4.4-design.md` 7.3KB) | ✅ |
| 2 | 写 claude 完整 prompt (5.4KB) | ✅ |
| 3 | claude 后台跑 (PID 33456) | ✅ |
| 4 | R1 4 层验证（log + git + cargo + dogfooding）| ✅ 全部通过 |
| 5 | git commit v0.4.4 + daily 复盘 | ✅ C3 |

**模式：D1 = R1 设计 + claude 实施 + R1 评审**

## 5. 验收数据

| 指标 | 数值 |
|------|------|
| 文件改动 | 10 (M8 + ??2) |
| 代码净增减 | +411 / -170 / **+241 净** |
| 新增文件 | 2 (`tests/fixtures/stage-defaults.md` + `_meta/`) |
| skeleton.rs 行数 | 808 → **951**（+143）|
| 单元测试 | **99 passed** (v0.4.3 是 95，+4) |
| trycmd 测试 | 22/22 passed |
| 集成测试 | **25 passed** (v0.4.3 是 21，+4) |
| cargo fmt | ✅ 无格式问题 |
| cargo clippy | ✅ 无 warning |
| 真实 dogfooding | ✅ enjoyiot-kaiyuan 5 步全流程 + **不覆盖**用户改的 AGENTS.md |

## 6. 真实 dogfooding 流程（enjoyiot-kaiyuan 全部成功）

| 步骤 | 命令 | 结果 |
|------|------|------|
| 1 | `ek init --ai claude` | ✅ 创建 11 kind + _meta/stage-defaults.md + AGENTS.md |
| 2 | 用户加 "USER-MARKER-12345" 到 AGENTS.md 末尾 | ✅ |
| 3 | **重跑 `ek init`** | ✅ **不覆盖用户标记**（**Q4 答案 B 完美**）|
| 4 | `ek doctor` | ✅ all checks passed |
| 5 | `ek onboard` | ✅ 1 active decision |
| 6 | `ek promote v0.4.4-test.md --to gotcha` | ✅ 写 `.enjoyknowledge/gotcha/v0.4.4-test.md` |

**核心不覆盖测试通过：重跑 init 用户加的 "USER-MARKER-12345" 还在 = 极简实现**。

## 7. R6 关键发现（教训沉淀）

### R6 v2 教训加重

> "**进程跑成功 ≠ 报告落盘**"——R1 v0.4.4 严格 4 层验证

R1 这次：
- **log 落盘 1 byte** ⚠️（claude 没输出总结但 git status 12 files）——R1 R3 不慌
- 验证 log 落盘（1 byte 异常但 git status 12 files = 真的改了）
- 跑 cargo test 25 集成全绿
- 跑真实 dogfooding 5 步全绿
- 跑核心不覆盖测试 = **Q4 答案 B 完美**

### R6 v9 教训强化

> "**看似对 ≠ 真对**"——YAML 配置被 Enjoy 拒

R1 之前 7 轮建议 "YAML stage-extend 配置"——**Enjoy 明确拒**："格式不好配"。R1 v0.4.4 用 **Markdown 嵌入 + 用户编辑 stage-defaults.md** = **0 新格式 = 极简**。

### R6 v12.1 教训加重

> "**R3 评估问题要看代码 + 借鉴前例**"——v0.4.4 跟 v0.4.3 嵌入方式一样

R1 之前 v0.4.3 抽 `kinds.md` 用 `include_str!` 编译期嵌入 + `LazyLock` 缓存。**v0.4.4 stage-defaults.md 100% 复用这个模式**——**0 新技术** = 极简。

### R6 v3 教训加重

> "**R1 不要脑补**"——9 轮自己定方案 = 全部错

R1 之前 7 轮建议"抽 skill" / "YAML 配置"——**全部偏**。**R1 第 8 轮问 5 问 + 第 9 轮听 Enjoy 5 答 = 完整方案**。

### 新教训

> "**先问再说 = Enjoy 拍板 = v0.4.4 完整实施**"——9 轮反思最大收获

R1 9 轮最大的教训 = **R1 不应该自己定方案**。R1 应该问 Enjoy → Enjoy 答 → R1 实施 = **D1 模式**。

## 8. v0.4.4 vs v0.4.3 对比

| 维度 | v0.4.3 | v0.4.4 |
|------|--------|--------|
| **stage 模式** | 默认 8 文件 3 目录 + Rust 写死 | stage-defaults.md 用户可改 + 解析 |
| **init 覆盖行为** | ⚠️ 覆盖 stage AGENTS.md | ✅ **永远不覆盖**（Q4 答案）|
| **stage AGENTS.md 格式** | 纯 Markdown | **Hermes skill frontmatter + body**（Q5 答案）|
| **用户改 stage** | ❌ 改完 init 重跑 = 覆盖 | ✅ **改完 init 不动**（Q4 答案）|
| **加新 stage 目录** | ❌ 改 Rust | ✅ 改 stage-defaults.md |
| **新单元测试** | 95 | 99 (+4) |
| **新集成测试** | 21 | 25 (+4) |
| **代码行数** | -1358 净（v0.4.2→v0.4.3）| +241 净（v0.4.3→v0.4.4）|

## 9. 关键设计原则对齐

| 原则 | v0.4.4 实施 |
|------|------------|
| **v0.4 "物理分离 > 状态字段"** | ✅ stage-defaults.md 物理文件 + 编译期嵌入 |
| **v0.4 "AGENTS.md > frontmatter"** | ✅ stage AGENTS.md = Hermes skill frontmatter |
| **v0.4 "简单 > 完整"** | ✅ 0 新格式（用 Markdown 不用 YAML）|
| **v0.4 "人类是 authority anchor"** | ✅ 用户改 stage-defaults.md = 直接控制 |

## 10. v0.5+ 路线（**R1 不实施，留 v0.5 评估**）

| 候选 | 评估 |
|------|------|
| `ek kind add/rm/list` 命令 | v0.4.4 stage 用户可改 = 11 kind 目录用户可改（同样思路）|
| `ek stage extend` 命令 | 加 stage 子目录命令（**v0.4.4 = 用户改 stage-defaults.md = 0 命令**）|
| ek init --update flag | v0.4.4 init 永远不覆盖 stage AGENTS.md = 不用 --update |
| onboard JSON 输出 | v0.4.3 daily 列的盲点 |
| proc-macro / LSP / 智能层 | 远期 |

## 11. v0.4.4 Commit

**Commit 1 个**：
```
feat: v0.4.4 — stage 用户可改（stage-defaults.md + user-owned AGENTS.md + Hermes skill 格式）

4 大模块同步重写：

stage-defaults.md 默认目录清单：
- 新建 _meta/stage-defaults.md — 用户可编辑的 Markdown 文档
- 编译期 include_str! 嵌入，init 时复制到项目（不覆盖用户版本）
- ek init 读 stage-defaults.md → 解析 Default Directories 表 → 创建目录
- 回退：文件缺失或为空时回到 v0.4.1 硬编码默认

init 不覆盖用户 stage AGENTS.md：
- 如果 .enjoyknowledge_stage/AGENTS.md 存在 → init 跳过
- 如果 .enjoyknowledge_stage/_meta/stage-defaults.md 存在 → init 跳过
- 如果不存在 → 写默认 Hermes skill 格式

stage AGENTS.md = Hermes skill 格式：
- 重写 STAGE_AGENTS_MD_CONTENT 为 frontmatter + body
- Frontmatter: name/description/version/metadata.hermes
- Body 6 段：Overview / Inputs / Workflow / Custom Directories / Promote Workflow / Hard Gate Protocol

init 创建目录按 stage-defaults.md：
- 新增 parse_stage_defaults() 解析 Markdown 表格
- generate_stage_skeleton() 读用户版本或默认 → 解析结果创建目录

Tests:
- 99 unit (v0.4.3 95 + 4 new) + 22 trycmd + 25 integration (v0.4.3 21 + 4 new)
- 10 files changed: +411 / -170 (net +241)
- Real dogfooding on enjoyiot-kaiyuan: USER-MARKER-12345 not overwritten on re-init
```

## 12. 文件清单（不 commit 但留作记录）

- `why-workspace/v0.4.4-design.md` (7.3KB) — R1 设计文档
- `why-workspace/claude-task-v0.4.4.md` (5.4KB) — claude prompt
- `.claude-task-v0.4.4.log` (1 byte) — claude 运行日志（**注意：v0.4.4 log 只 1 byte = claude 没输出总结**，但 git status 12 files = 真的改了）

## 13. 下一步

- **v0.4.4 已完工** — 等 Enjoy 拍板后续（v0.5 路线 / enjoyiot-kaiyuan 长期 dogfooding / 其他）
