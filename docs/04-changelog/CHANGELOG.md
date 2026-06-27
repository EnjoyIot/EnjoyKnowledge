# enjoyknowledge 变更记录

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
- GLOSSARY.md — 新增 ules/ 条目，含三层防护定义
- PRODUCT-DESIGN.md — §6 目录结构增加 ules/ 及三层防护说明

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
