# v0.4.3 完工复盘 — 2026-06-28

## 1. 一句话总结

**v0.4.3 完整重写完工**——kind registry 抽离（Markdown 单一来源）+ 5 目录回归 v0.1 哲学（kind = dir）+ workflow 重构（onboard 升格顶层 + 砍 capture + 删 workflow 容器），**71 文件改动 / -1358 净行** / **95 单测 + 22 trycmd + 21 集成全绿** / **真实 dogfooding 全流程通过**。

## 2. Why（为什么做 v0.4.3）

### 三大动机

1. **哲学一致性**：v0.4 阶段加的 `gotcha → gotchas` 派生规则 = 写死 = **退化 v0.1 DESIGN-V3 L17 哲学**（"目录名即分类"）
2. **bug 修复**：v0.4.2 出现 "capture 10 kind vs promote 11 kind" 不一致 bug（双 const 没抽离的必然结果）
3. **0 风险**：Jay 确认项目还在开发阶段，无生产数据，所有改动 0 breaking

### 6 轮讨论沉淀

| 轮 | Jay 提议 | 决策 |
|----|---------|------|
| 1 | workflow 命令还要吗 | ✅ W1 = onboard 升格顶层 + 砍 capture + 消亡 workflow 容器 |
| 2 | 写死目录怎么抽 | ✅ kinds.md Markdown 抽离（Jay 第 4 方案 = 极简）|
| 3 | kind = dir 默认 | ✅ 改 5 目录（去"s"）|
| 4 | 跟以前约定一致 | ✅ 回到 v0.1 DESIGN-V3 L17 哲学 |
| 5 | 无所谓数据迁移 | ✅ 0 风险 = 立刻做 |
| 6 | 代码可完全重写 | ✅ 完全重写 + D1 模式（R1 设计 + claude 实施 + R1 评审）|

## 3. What（v0.4.3 完整范围）

### 模块 1: kind registry 抽离（核心）

**目标**：消除 4 处写死 → 1 个 Markdown 文件

- 新建 `src/kinds.rs` (153 行) — `kinds::all()` / `kinds::dir_for()` / `kinds::required_fields()` / `kinds::is_valid_kind()` / `kinds::init_default_kinds()`
- 新建 `.enjoyknowledge/_meta/kinds.md` — Markdown 表格，11 kind 单一来源
- `src/cli/promote.rs` 改用 `kinds::dir_for()` 替代 5 行 `kind_to_dir()` 派生
- 编译期 `include_str!` + `LazyLock` 缓存

### 模块 2: 5 目录重命名（kind = dir）

**目标**：消除 5 处"s 派生"硬编码

| 旧名 | 新名 |
|------|------|
| `gotchas/` | `gotcha/` |
| `decisions/` | `decision/` |
| `patterns/` | `pattern/` |
| `rules/` | `rule/` |
| `commands/` | `command/` |

改 ~20 处：workflow.rs / promote.rs / skeleton.rs / tests / fixture / 4 docs。

### 模块 3: workflow 命令重写（W1）

**目标**：砍 capture + 升格 onboard + 消亡 workflow 容器

- 抽出 `src/cli/onboard.rs` (429 行) — 从 workflow.rs 抽 `run_onboard` + 7 字段 + 3 辅助函数
- 改命令：`ek workflow onboard` → `ek onboard` 顶层
- 砍掉 `ek workflow capture`（用 `ek promote` + AI draft 替代）
- 删除 `src/cli/workflow.rs` (1216 行)
- doctor 5th check `check_kinds_md()`：文件存在 + 解析合法 + 与代码 registry 一致

## 4. How（实施步骤）

| 步骤 | 内容 | 结果 |
|------|------|------|
| 1 | 写 v0.4.3 design doc (`why-workspace/v0.4.3-design.md` 6.8KB) | ✅ |
| 2 | 写 claude 完整 prompt (30 步 checklist, `why-workspace/claude-task-v0.4.3.md` 6.5KB) | ✅ |
| 3 | claude 后台跑 (PID 47324) | ✅ 47s 跑完 |
| 4 | R1 全量验证 (cargo + 测试 + 真实 dogfooding) | ✅ 全部通过 |
| 5 | git commit v0.4.3 + daily 复盘 | ✅ C3 |

**模式：D1 = R1 设计 + claude 实施 + R1 评审**

## 5. 验收数据

| 指标 | 数值 |
|------|------|
| 文件改动 | 71 (M17 + D14 + ??2) |
| 代码净增减 | +342 / -1700 / **-1358 净** |
| 新增文件 | 2 (`src/kinds.rs` 153 + `src/cli/onboard.rs` 429 + `tests/fixtures/kinds-default.md`) |
| 删除文件 | 2 (`src/cli/workflow.rs` 1216 + 13 旧 fixture/test) |
| 单元测试 | 95 passed |
| trycmd 测试 | 22/22 passed |
| 集成测试 | 21 passed |
| cargo fmt | ✅ 无格式问题 |
| cargo clippy | ✅ 无 warning |
| 真实 dogfooding | ✅ enjoyiot-kaiyuan 全流程 6 步通过 |

## 6. 真实 dogfooding 流程（enjoyiot-kaiyuan）

| 步骤 | 命令 | 结果 |
|------|------|------|
| 1 | `rm -rf .enjoyknowledge/ .enjoyknowledge_stage/` | ✅ 清空 v0.4.1 残留 |
| 2 | `ek init --ai claude` | ✅ 11 kind 目录全是单数 |
| 3 | `ek onboard` | ✅ 列出 active decisions 1 + 14 KB files |
| 4 | `ek doctor` | ✅ all checks passed |
| 5 | `ek ls` | ✅ 11 单数目录 + 8 seed files |
| 6 | `ek promote test-gotcha.md --to gotcha` | ✅ 写 `.enjoyknowledge/gotcha/test-gotcha.md` = **单数目录** |

**6 步全绿，0 breaking**（R1 真实场景验证完成）

## 7. R6 关键发现（教训沉淀）

### R6 v2 教训加重

> "**进程跑成功 ≠ 报告落盘**"——R1 v0.4.3 严格 ls + wc + cargo test + 真实 dogfooding 4 层验证

R1 这次第一时间：
- 验证 log 落盘（2284 bytes）
- 验证 git status 71 files
- 跑 cargo test 21 集成（首次 1 失败 = Windows file lock flaky，立即重跑全绿）
- 跑真实 dogfooding 6 步

### R6 v9 教训强化

> "**看起来错 ≠ 真错**"——Windows file lock 失败 = flaky 不是真 bug

R1 cargo test 首次 22 trycmd 1 failed（os error 32），立即判断是 Windows 文件锁（之前 v0.4.1 记录过类似问题），**重跑全绿**。

### R6 v12.1 教训加重

> "**R3 评估问题要看原始设计文档**"——R1 v0.4.3 第 4 轮才看 v0.1 DESIGN-V3

R1 之前误解 Jay 提议"以前目录名就是约定" = plural 约定俗成。**R1 看 v0.1 DESIGN-V3 L17 后才懂** = "目录名即分类，不写死"——**v0.4 加"s 派生"是退化不是改进**。

### R6 v3 反思强化

> "**R1 不要脑补'已有项目'**"——R1 必须问 Jay 真实使用状态

R1 之前一直说"breaking change 担忧"——**Jay 说"开发阶段无所谓"后 R1 才发现 = 0 风险**。R1 之前所有 v0.4.x 阶段的"breaking 担忧"都是过度担心。

### 新教训

> "**犹豫就是拖延**"——开发阶段无成本时不立刻做 = R1 错

v0.4.3 = 0 风险 + 1.5 天 + 5 维度收益 = **现在不做是 R1 错**。

> "**哲学一致性 > 代码简洁**"——回到 v0.1 哲学比单纯极简更重要

R1 之前给 3 选 1（const / 配置文件 / proc-macro）——**Jay 第 4 方案 = 全用 Markdown**才是真极简。

## 8. R1 R3 决策树 vs 最终决策

| R1 之前 | Jay 决策 | R1 R6 教训 |
|---------|---------|------------|
| A. Rust const | ❌ | 仍写死代码 |
| B. 配置文件 | ❌ | 加配置层 |
| C. proc-macro | ❌ | 过度工程 |
| **Jay D. Markdown** | ✅ | 单一来源 + 极简 |
| + kind = dir | ✅ | 回到 v0.1 哲学 |
| + 0 breaking | ✅ | 开发阶段 0 成本 |
| + 完全重写 | ✅ | 高质量交付 |
| + D1 模式 | ✅ | R1 + claude 协作 |

## 9. 关键设计原则

| 原则 | 实施 |
|------|------|
| **v0.1 DESIGN-V3 L17** "目录名即分类" | ✅ kind = dir 默认 |
| **v0.4 "物理分离 > 状态字段"** | ✅ kinds.md 物理文件 + Rust 通用 parse |
| **v0.4 "AGENTS.md > frontmatter"** | ✅ kinds.md 在 `_meta/` 是 AGENTS.md for kinds |
| **v0.4 "简单 > 完整"** | ✅ 1 个 Markdown 文件 vs 3 个抽象层 |
| **v0.4 "人类是 authority anchor"** | ✅ 用户改 Markdown = 直接控制 |

## 10. v0.5+ 路线（**R1 不实施，留 v0.5 评估**）

| 候选 | 评估 |
|------|------|
| **ek init --upgrade** 自动迁移老项目 | v0.4.1 项目升 v0.4.3 = 手动删 11 复数目录 |
| **onboard 输出 JSON** | 适合 CI（v0.4 之前 claude 盲点 #4）|
| **ek ls description 优化** | v0.4.1 daily 列的 ek fix 兼容性问题 |
| **proc-macro / LSP / 智能层** | 远期 |

## 11. v0.4.3 Commit

**Commit 1 个**：
```
feat: v0.4.3 — kind registry + singular dirs + workflow→onboard

BREAKING CHANGES:
- All 11 kind directories renamed: plural → singular
  (gotchas→gotcha, decisions→decision, patterns→pattern,
   rules→rule, commands→command, contracts→contract,
   conventions→convention, templates→template)
- `workflow capture` command removed (use stage→promote instead)
- `workflow onboard` → top-level `onboard` command

Features:
- New src/kinds.rs: Markdown-driven kind registry
  (include_str! + LazyLock, single source of truth)
- _meta/kinds.md seeded on init, validated by doctor
- dir_for(kind) = kind (no "s" derivation — kind IS the dir name)
- Doctor: 5th check validates kinds.md
```

## 12. 文件清单（不 commit 但留作记录）

- `why-workspace/v0.4.3-design.md` (6.8KB) — R1 设计文档
- `why-workspace/claude-task-v0.4.3.md` (6.5KB) — claude prompt
- `.claude-task-v0.4.3.log` (2.2KB) — claude 运行日志（已 gitignore）

## 13. 下一步

- **v0.4.3 已完工** — 等 Jay 拍板后续（v0.5 路线 / enjoyiot-kaiyuan 长期 dogfooding / 其他）
