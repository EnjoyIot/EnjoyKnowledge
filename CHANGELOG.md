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

[Unreleased]: https://github.com/enjoyknowledge/enjoyknowledge/compare/v0.1.0...HEAD
