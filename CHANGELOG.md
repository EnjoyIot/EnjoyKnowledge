# Changelog

本项目遵循 [Semantic Versioning](https://semver.org/) 和 [Keep a Changelog](https://keepachangelog.com/) 格式。

## [Unreleased]

### Added
- `enjoyknowledge init` — 项目初始化（含 --scan, --describe, --link, --ai）
- `enjoyknowledge search` — 知识检索（含 --class, --tag, --archive）
- `enjoyknowledge record` — 知识写入（gotcha / pattern / decision）
- **v0.4 极简上下文层** — 给 AI 工具提供人类已审核的好上下文
  - `enjoyknowledge init` 增强：创建 `.enjoyknowledge_stage/{tasks,drafts,.archive}/` 目录 + 生成 2 个 AGENTS.md + 8 文件模板
  - `enjoyknowledge promote <draft> --to <kind>` — 从 stage/drafts/ 复制到 .enjoyknowledge/<kind>/（最小 4 字段 frontmatter）
  - `enjoyknowledge stage clean [--dry-run] [--force] [--older-than <days>]` — TTL 清理（默认 180 天）
- v0.4 哲学：人类是 authority anchor / 简单 > 完整 / 物理分离 > 状态字段 / AGENTS.md > frontmatter
- v0.4 砍掉：C10 trust 体系 / C11 lifecycle / C12 sync（v0.5+ 按需）
- `enjoyknowledge doctor` / `enjoyknowledge fix` — 诊断与修复
- 文件系统源适配器
- YAML frontmatter 解析
- 惰性索引 (.index.json)
- AGENTS.md + 7 种 AI 工具配置文件生成

### Fixed (v0.4.1)
- **stage/AGENTS.md 加 Draft frontmatter 必填规范** — AI 写 `stage/drafts/<id>.md` 时必填 kind-specific 字段（gotcha.trigger / rule.applies_to / decision.reversible+decided_at），promote 后 `ek doctor` 直接通过
  - 之前：AI 写 gotcha draft 缺 trigger → `ek promote`（4 字段基础）→ `ek doctor` 报 "gotcha missing required field 'trigger'"
  - 之后：stage/AGENTS.md 明确列出 3 类必填 + 例子 + "AI must fill these when writing drafts" 强提示
  - 原因：R6 教训链 v9 "看起来错≠真错" — promote 强加 `trigger: "manual"` 默认值会污染灵魂字段，**规范放在文档层比工具层更对**

### Fixed (v0.4.2)
- **`ek fix` Fix 1 步骤保留 frontmatter 字段** — 之前 `ek fix` 调 `generate_frontmatter(description)` 只输出 2 字段（description + timestamp），**完全覆盖 promote 写的 4 字段**（id/kind/created/author）。改为**字段合并**（追加缺失 description，保留所有已有字段）。
  - 之前：v0.4.1 enjoyiot-kaiyuan 真实工作流触发 — `ek promote` → `ek fix` → 4 字段全丢 → 必须手动回退
  - 之后：v0.4.2 真实 dogfooding 验证 — promote 4 字段保留 + description 用 `## heading` 自动推
  - 原因：R6 教训链 v12.1 "R3 评估问题要看代码" + v12.2 "字段合并 vs 字段重写"
  - 测试：3 新单元测试（4 字段保留 / 已有 description 跳过 / trigger+applies_to 多字段保留）

[Unreleased]: https://github.com/enjoyknowledge/enjoyknowledge/compare/v0.1.0...HEAD
