# enjoyknowledge 路线图

> v0.4.3 | 2026-06-28

## 版本节奏

| 版本 | 状态 | 主题 |
|---|---|---|
| v0.1 | 已交付 (2026-06-22) | Core CLI：init / ls / grep / cat / add / doctor / fix |
| v0.2 | 已交付 (2026-06-27) | 多工具 export + workflow（onboard / capture）+ 4 项 doctor |
| v0.3 | 待定 | 捕获体验 + 质量保障深化 |
| v0.4 | 已交付 (2026-06-28) | 极简上下文层（stage + promote + stage clean） |
| v0.4.2 | 已交付 (2026-06-28) | ek fix 保留 frontmatter 字段 + 文档重写 |
| v0.4.3 | **当前** (2026-06-28) | kind registry 抽离 + 目录回归 kind=dir + workflow 重构 |
| v0.5+ | 方向 | 智能化（语义检索、知识新鲜度），不预设完整 scope |

---

## v0.4.2 当前状态

### CLI 命令（13 个）

| 命令 | 用途 |
|---|---|
| `init` | 初始化 `.enjoyknowledge/` + `.enjoyknowledge_stage/` |
| `ls` | 列表（带 description） |
| `tree` | 递归目录树 |
| `cat` | 读取文件 |
| `grep` | 结构化搜索（定位到 `##` 段） |
| `add` | 追加/创建知识条目 |
| `doctor` | 5 项健康检查（`--ci` 模式 warning 也非零退出） |
| `fix` | 自动修复（`--req <REQ-ID>`） |
| `export` | 生成 AI 工具入口文件（`--tool cursor/claude/auto`） |
| `onboard` | 建立项目心智模型 |
| `promote` | draft → KB（`--to <kind>` `--id <id>` `--author <name>`） |
| `stage clean` | TTL 清理（`--dry-run` `--force` `--older-than <days>`） |

### 11 类知识资产

`gotcha` / `decision` / `pattern` / `rule` / `business` / `architecture` / `contract` / `convention` / `context` / `template` / `command`

### 必填字段

| kind | 必填 frontmatter |
|---|---|
| gotcha | `trigger` |
| decision | `reversible` + `decided_at` |
| rule / contract / convention / template / command | `applies_to` |
| 其他 | 无 |

### `.enjoyknowledge/` 结构

```
.enjoyknowledge/
├── _meta/kinds.md      (kind 注册表)
├── AGENTS.md           (KB 写入规则)
├── index.md
└── <kind>/             (11 类目录，目录名 = kind 名)
```

### `.enjoyknowledge_stage/` 结构

```
.enjoyknowledge_stage/
├── AGENTS.md           (任务写入规范)
├── tasks/<task-id>/    (8 文件)
├── drafts/             (待 promote 草稿)
└── .archive/           (TTL 过期)
```

---

## v0.5+ 方向

- 语义检索（`grep --semantic`，可选向量后端）
- 知识新鲜度评分
- 导入迁移工具（从 Confluence / Notion / Markdown 批量导入）
- 扩展 adapter trait 支持更多 AI 工具

---

*关联文档：[POSITIONING.md](./POSITIONING.md) · [CHANGELOG.md](../04-changelog/CHANGELOG.md)*
