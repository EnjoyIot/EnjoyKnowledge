
# enjoyknowledge 路线图

> 版本 1.1 | 2026-06-26
>
> 本文档描述 enjoyknowledge 的分阶段发展计划。
> 每个阶段有明确的目标、交付物和完成标准。
>
> **关联文档**（已更新到新结构）：
> - [DESIGN.md](../02-design/DESIGN.md) — 整体设计入口
> - [for-coding-design.md](../02-design/architecture/for-coding-design.md) — for Coding 完整设计
> - [DESIGN-PHILOSOPHY.md](../01-philosophy/DESIGN-PHILOSOPHY.md) — 设计哲学
> - [CHANGELOG.md](../04-changelog/CHANGELOG.md) — 版本变更记录
> - 历史蓝图见 [99-archive/2026-06-26-codex/BLUEPRINT.md](../99-archive/2026-06-26-codex/BLUEPRINT.md)
> 路线图是活的——随着用户反馈和生态变化而调整。

---

## 总览

```
v0.1 (当前)     v0.2          v0.3                v0.4            v0.5            v1.0
CLI 核心       打磨稳定      for Coding 场景深化   团队规模化      智能化          生态化
    │             │             │                   │               │               │
    ├─ init       ├─ 测试覆盖   ├─ 种子文件增强       ├─ 多仓库链接    ├─ 语义检索     ├─ MCP Server
    ├─ add        ├─ 错误处理   ├─ 捕获体验提升       ├─ 共享知识库    ├─ 新鲜度评分   ├─ 插件系统
    ├─ ls         ├─ 文档完善   ├─ 质量保障深化       ├─ 冲突解决      ├─ 知识图谱     ├─ Web UI (可选)
    ├─ tree       ├─ CI 集成    ├─ 搜索能力增强       ├─ 组织级预设    ├─ 智能建议     ├─ 社区市场
    ├─ grep       └─ 性能优化   ├─ AI 集成深度        ├─ 废弃检测      ├─ 导入迁移     └─ 生态集成
    ├─ cat                      ├─ 代码编织           └─ CI 阻断
    ├─ doctor                   └─ 团队工作流基础
    └─ fix
```

> **当前战略重点：v0.3 for Coding 场景深化。**
> 在编码场景下把产品做深、做透，比急于横向扩展到其他领域更重要。
> v0.4 及之后的横向扩展（其他领域预设、组织级功能）在 for Coding 达到深度后再启动。

---

## v0.1 — Core CLI (当前)

**目标**: 交付可用的知识管理原语和 for Coding 预设。

### 已交付

| 命令 | 功能 | 状态 |
|---|---|---|
| `init` | 初始化知识库骨架 + AI 入口文件 | ✅ |
| `init --template` | 指定领域模板 | ✅ |
| `init --ai` | 指定 AI 工具生成对应入口 | ✅ |
| `init --link` | 引用外部知识库 | ✅ |
| `add` | 追加知识条目，自动补全 frontmatter | ✅ |
| `ls` | 浏览目录（默认带 description） | ✅ |
| `tree` | 递归浏览（默认带 description） | ✅ |
| `grep` | 结构感知搜索 | ✅ |
| `cat` | 查看文件内容 | ✅ |
| `doctor` | 4 项健康检查 | ✅ |
| `fix` | 自动修复可处理问题 | ✅ |

### 架构

- 三层分离（OKF 格式 → Core 引擎 → for Coding 预设）
- 推送 + 拉取双通道
- 文件系统即索引
- 目录名即分类

### 文档

- [DESIGN-PHILOSOPHY.md](../01-philosophy/DESIGN-PHILOSOPHY.md) — 设计哲学
- [DESIGN.md](../02-design/DESIGN.md) — 整体设计入口（含历史 DESIGN-V3 内容）
- [INTERFACE-SPEC.md](../02-design/INTERFACE-SPEC.md) — CLI 行为合约
- [POSITIONING.md](POSITIONING.md) — 生态定位
- [GLOSSARY.md](../01-philosophy/GLOSSARY.md) — 统一术语
- 历史蓝图见 [99-archive/2026-06-26-codex/BLUEPRINT.md](../99-archive/2026-06-26-codex/BLUEPRINT.md)

---

## v0.2 — 打磨与稳定

**目标**: 提升代码质量、测试覆盖率和用户体验，使 v0.1 达到生产级稳定。

**预计**: 1-2 个迭代

### 测试

- [ ] trycmd 测试覆盖所有 CLI 命令的 happy path
- [ ] trycmd 测试覆盖所有错误码（1=arg error, 2=not found, 3=format, 4=unreadable）
- [ ] 集成测试：完整的 `init → add → doctor → fix` 工作流
- [ ] 边界测试：空目录、损坏的 frontmatter、超大文件、Unicode 路径
- [ ] Windows / macOS / Linux CI 矩阵

### 错误处理

- [ ] 所有错误路径返回明确错误码（不依赖 anyhow! 的默认行为）
- [ ] 友好错误信息：告诉用户哪里出错了、怎么修
- [ ] Frontmatter 解析失败时给出具体行号和预期格式
- [ ] 文件权限问题给出可操作建议

### 文档

- [ ] `enjoyknowledge --help` 每个子命令有完整示例
- [ ] README.md 增加快速开始和常见场景
- [ ] CONTRIBUTING.md 增加本地开发环境搭建指南
- [ ] 错误码参考文档

### CI/CD

- [ ] `just check` 在 GitHub Actions 中零警告通过
- [ ] 跨平台构建验证（Windows / macOS / Linux）
- [ ] 发布自动化：tag 推送 → 构建二进制 → GitHub Release

### 性能

- [ ] 知识库文件数 > 500 时的 `ls` / `grep` 性能基准
- [ ] 增量索引：只重新读取变更文件
- [ ] 启动时间 < 50ms（当前 CLI 的每次调用）

---

## v0.3 — 编码场景深化（一站式收尾）

**目标**: 在编码场景下把产品体验做深做透。不是加目录，而是在知识生命周期的每个环节提升密度和可用性。

**预计**: 1-2 个迭代（一站式收尾，不分散 v0.3.1/v0.3.2）

### 核心 1：捕获体验

让 `add` / `workflow capture` 真正"零摩擦"——开发者不思考怎么写，只想记什么：

- [ ] `add` 追加前自动检测**重复/相似条目**（基于标题和 description 的相似度），提示合并
- [ ] `add` 自动建议 **tags**（基于已有 tags 和当前内容的匹配）
- [ ] `add --from-commit`：从最近的 git commit message 中提取可能的知识入口，提示是否记录
- [ ] `add --dry-run`：预览即将添加的内容和自动生成的 frontmatter
- [ ] **种子文件增强**：让 `init` 生成的骨架文件不再是简单占位符（填写指南 / 常见场景 / 反例警示 / 跨文件关联提示）

### 核心 2：质量保障深化

在现有 4 项结构检查之外，增加内容质量维度：

- [ ] **描述一致性检查**：`doctor` 检查 `description` 是否与正文存在明显偏差（基于关键词覆盖率）
- [ ] **跨文件引用有效性**：`doctor` 检测正文中引用的其他 `.enjoyknowledge/` 文件路径是否真实存在
- [ ] **预算与拆分建议**：单文件条目数超过阈值（>20 条）时，`doctor` 给出具体的拆分建议
- [ ] **fix.rs 适配 v0.2 4 项 check**（v0.2 收尾遗留）：让 `fix` 命令能修 `check_frontmatter` / `check_required_fields` / `check_sot_staleness` / `check_export_consistency` 报的问题

> **v0.3 不做的事**（合并到 v0.4）：搜索增强（grep --related/--semantic）、AI 集成深度（context 命令）、代码编织（git hook/link）、团队工作流基础（doctor --ci JSON）

---

## v0.4 — 知识 + 开发流程交织 + 团队规模化（合并发布）

**目标**: 让知识不再只是静态文档，而是与开发流程交织；支持多仓库、多团队场景。

**预计**: 2-3 个迭代（**原 v0.3 剩余 4 大类 + 原 v0.4 5 大类合并**）

### 搜索能力增强

- [ ] `grep --related <file>`：基于 tags 和正文关键词，查找与指定文件主题相关的其他知识条目
- [ ] `grep --semantic <query>`：可选语义搜索后端（本地嵌入模型，不依赖外部 API），作为结构搜索的补充
- [ ] 搜索结果排序优化：匹配在 description 中的权重高于正文弱匹配
- [ ] `grep --snippet-lines N`：控制匹配结果中上下文片段的行数

### AI 集成深度

- [ ] **智能推送范围**：当知识库增大时，AGENTS.md 中的推送块根据当前任务上下文智能缩减
- [ ] 对不同 AI 工具的**接入深度优化**：Cursor rules / Claude Code Skill / Codex prompt
- [ ] `enjoyknowledge context <task-description>`：根据任务描述输出相关知识的摘要

### 代码编织

- [ ] **规则统一管理（三层防护）**：源规则层 + 推送层 + 兜底层
- [ ] **git commit hook**：commit 时自动检测变更文件，提示相关知识条目
- [ ] **PR 模板自动引用**：创建 PR 时自动检测变更涉及的目录
- [ ] `enjoyknowledge link <file> --to <code-path>`：手动建立代码文件与知识条目的关联

### 团队工作流基础

- [ ] 知识 PR 审核指南：review checklist
- [ ] `doctor --ci` 输出 JSON 格式
- [ ] 可配置严重级别（error / warning / info）
- [ ] GitHub Actions / GitLab CI 集成模板

### 多仓库知识链接

- [ ] `--link` 支持 Git URL（自动 clone + 缓存）
- [ ] 知识溯源：OKF `resource` 字段标注来源仓库
- [ ] `ls` / `grep` 标注知识归属（本地 / 上游 / 组织）

### 共享知识库 + 组织级 + 知识废弃

- [ ] 独立知识库仓库的推荐结构 + 语义化版本管理
- [ ] `enjoyknowledge knowledge pull`：拉取上游知识库更新
- [ ] 组织级预设：`--org` 标志
- [ ] `enjoyknowledge aggregate` 汇总多个知识库
- [ ] `deprecated` frontmatter 字段 + `doctor` 检测废弃引用

---

## v0.5 — 智能化

**目标**: 引入智能辅助功能，让知识管理从被动记录走向主动洞察。

**预计**: 3-5 个迭代

### 语义检索（可选）

- [ ] 向量索引作为 `grep` 的可选后端（`grep --semantic`）
- [ ] 本地嵌入模型（不依赖外部 API）
- [ ] 结构搜索 + 语义搜索混合排序
- [ ] 语义检索仅在明确需求且不牺牲结构搜索可靠性时启用

### 知识新鲜度

- [ ] 新鲜度评分：基于 timestamp、更新频率、引用次数
- [ ] `doctor` 报告过期知识（超过 N 天未更新）
- [ ] 自动建议：哪些文件可能需要更新（代码变更与知识描述不匹配）
- [ ] 新鲜度趋势图（`enjoyknowledge stats`）

### 知识图谱

- [ ] 基于 tags 的跨文件关联图
- [ ] 反向引用：自动发现哪些知识引用了当前条目
- [ ] `grep --related`：查找与当前主题相关的其他知识

### 智能建议

- [ ] `add` 时自动建议 tags（基于已有 tags 和历史）
- [ ] `add` 时检测重复/相似条目，提示合并或追加
- [ ] git commit hook：检测代码变更，提示可能需要更新的知识文件
### 导入迁移

- [ ] 从 Confluence / Notion 导出迁移工具
- [ ] 从已有 Markdown 文档批量导入（自动生成 frontmatter）
- [ ] `enjoyknowledge import` 命令

---

## v1.0 — 生态化

**目标**: 从独立工具走向生态平台。

**预计**: 5+ 个迭代

### MCP Server

- [ ] 独立 MCP Server 进程（可选的拉取通道增强）
- [ ] MCP tools 映射：`knowledge_ls`、`knowledge_grep`、`knowledge_cat`、`knowledge_add`
- [ ] MCP resources：将 `.enjoyknowledge/` 目录暴露为 resource
- [ ] 远程知识库：MCP Server 支持访问非本地文件系统的知识库

### 插件系统

- [ ] 插件接口：格式转换、自定义检查、自定义输出
- [ ] 插件分发：GitHub Release + 注册表
- [ ] 内置插件市场：`enjoyknowledge plugin search/install`

### Web UI（可选）

- [ ] 知识库浏览界面
- [ ] 可视化知识图谱
- [ ] 新鲜度仪表盘
- [ ] Web UI 仅作为 CLI 的补充，CLI 永远是主要入口

### 社区市场

- [ ] 预设市场：浏览、安装、评价领域预设
- [ ] 插件市场：浏览、安装插件
- [ ] 集成模板：CI/CD、AI 工具、文档生成器

### 生态集成

- [ ] 官方 GitHub Action：`enjoyknowledge/doctor-action`
- [ ] VS Code 扩展：编辑器内浏览和编辑知识
- [ ] 文档站点生成：从 `.enjoyknowledge/` 生成静态文档站
- [ ] API 文档同步：从代码注释提取并注入知识库

---

## 长期展望 (v1.x+)

这些是方向性想法，尚未进入具体的版本规划：

- **其他领域应用** — 当 for Coding 达到足够深度后，复用核心能力扩展到 for Support、for Research、for Sales、for Legal、for GameDev、for DataEng 等领域预设
- **跨组织知识共享** — 公开知识库、社区贡献的通用知识（如特定框架的踩坑集）
- **知识联邦** — 多个组织间的受控知识共享协议
- **AI 驱动的知识生成** — 从代码仓库和 PR 讨论中自动提取知识条目
- **多语言支持** — `description` 和 `title` 的多语言字段
- **合规审计** — 知识变更的完整审计轨迹

---

## 版本策略

- **主版本号** (v1.0, v2.0): 架构性变化或不兼容的 CLI 变更
- **次版本号** (v0.1, v0.2): 新功能、新预设、新命令
- **修订号** (v0.1.1): Bug 修复、文档更新、性能优化

CLI 行为合约 (INTERFACE-SPEC.md) 的破坏性变更只能在主版本号升级时引入。

---

*文档版本: 1.1 | 最后更新: 2026-06-26*
