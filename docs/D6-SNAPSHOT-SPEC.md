# D6 任务上下文快照规范 (D6 Snapshot Specification)

**版本**: v0.1-draft
**日期**: 2026-06-20
**状态**: 草案
**配套**: [POSITIONING.md](POSITIONING.md) v5 / [DISCUSSION-LOG-2026-06-19.md](DISCUSSION-LOG-2026-06-19.md) §15

---

## 1. 目的

本文档定义 EnjoyFlow **D6 任务上下文快照**的完整规范。

D6 是享受 Flow 的**核心差异化能力**——它不是预先生成的快照文件，而是 **AI 可调用的工具**，让 AI 在会话中**主动获取与当前任务相关的所有上下文知识**。

---

## 2. 设计哲学

### 2.1 跟传统快照的本质差别

| 维度 | 传统快照 | 享受 Flow D6 |
|---|---|---|
| 形态 | 预先生成文件 | 工具调用动态注入 |
| 时机 | 创建时一次性 | AI 主动按需 |
| 新鲜度 | 过期风险 | 实时查询 |
| AI 自主性 | 被动接收 | 主动调用 |
| 跨工具 | 需要转换 | 统一接口 |
| 跟 AI 搜索关系 | 独立 | **协同**（前置过滤） |

### 2.2 跟 AI 工具搜索（grep/Read）的协同

**享受 Flow 不替代 grep——是 AI 搜索的"前置过滤器"**：

```
Step 1: AI 调用 enjoyflow_context → 获得相关知识索引
Step 2: AI 用 grep/Read 深入某条知识 → 读到具体细节
Step 3: AI 综合信息写代码
Step 4: AI 调用 enjoyflow_record → 记录决策/踩坑
```

**D6 价值**——**把散落的 32 类知识变成"AI 可推理的结构"**，让 AI 不需要知道文档结构/tag 命名/格式转换。

---

## 3. 工具集

### 3.1 核心工具：`enjoyflow_context`

D6 由 **1 个核心工具** + 2 个辅助工具 构成：

| 工具 | 作用 |
|---|---|
| `enjoyflow_context` | **核心**——获取上下文知识索引 |
| `enjoyflow_record` | 记录决策/踩坑 |
| `enjoyflow_search` | 全文/语义搜索（补充） |

### 3.2 `enjoyflow_context` 完整 schema

```typescript
interface EnjoyFlowContextInput {
  // === 任务标识 ===
  task_id?: string;              // 例如 "REQ-001"
  module?: string;               // 例如 "backend/export"

  // === 上下文范围 ===
  scope?: ContextScope;          // all | task | module | tag | class
  tags?: string[];               // 例如 ["export", "excel"]

  // === 维度选择（核心创新）===
  preset?: ScenarioPreset;       // 场景预设名
  dimensions?: Dimension[];      // 自定义维度（32 选 N）

  // === 输出控制 ===
  format?: ContextFormat;        // compact | detailed | yaml | markdown | json
  max_tokens?: number;           // AI 主动控制返回量（关键！）
  top_k?: number;                // 最多返回条目数（默认 10）

  // === 时间过滤 ===
  since?: string;                // ISO 时间，只返回此时间之后的变更

  // === 高级选项 ===
  include_summaries?: boolean;   // 是否包含摘要（默认 true）
  include_paths?: boolean;       // 是否包含路径（默认 true）
  include_tags?: boolean;        // 是否包含 tag（默认 true）
}

type ContextScope =
  | "all"        // 全部知识
  | "task"       // 当前任务相关
  | "module"     // 指定模块相关
  | "tag"        // 按 tag 匹配
  | "class";     // 按 32 类知识分类

type ContextFormat =
  | "compact"    // 极简（一行一个）
  | "detailed"   // 详细（多行 + 注释）
  | "yaml"       // YAML 格式
  | "markdown"   // Markdown 格式（人可读）
  | "json";      // JSON 格式（机器可读）
```

### 3.3 输出 schema

```typescript
interface EnjoyFlowContextResult {
  task_id: string;

  // 相关知识条目（核心）
  matches: KnowledgeMatch[];

  // 元数据
  total_candidates: number;    // 候选总数
  filtered_count: number;      // 实际返回数
  tokens_used: number;
  generated_at: string;

  // AI 提示
  next_actions?: string[];     // 建议的下一步操作
}

interface KnowledgeMatch {
  path: string;           // 文件路径
  class: string;          // 知识分类 (A1/A2/.../D7)
  title: string;          // 标题
  relevance: number;      // 0-1 相关度评分
  summary?: string;       // 100 字内摘要
  tags: string[];
  last_modified?: string;
}
```

### 3.4 维度全集（32 个）

#### A 项目知识（12）

```typescript
type A_Dimensions =
  | "A1_architecture"   // 架构总览
  | "A2_code_standards" // 代码规范
  | "A3_api_contract"   // API 契约
  | "A4_data_model"     // 数据模型
  | "A5_api_spec"       // 接口规约
  | "A6_requirement"    // 当前需求
  | "A7_design_spec"    // 规约文档
  | "A8_adr"            // 设计决策
  | "A9_test_report"    // 测试报告
  | "A10_dependencies"  // 依赖关系
  | "A11_environment"   // 环境配置
  | "A12_release";      // 发布流程
```

#### B 业务知识（5）

```typescript
type B_Dimensions =
  | "B1_glossary"        // 业务术语
  | "B2_business_rules"  // 业务规则
  | "B3_business_flow"   // 业务流程
  | "B4_constraints"     // 业务约束
  | "B5_cases";          // 业务案例
```

#### C 流程知识（8）

```typescript
type C_Dimensions =
  | "C1_gotchas"          // 踩坑清单
  | "C2_patterns"         // 最佳实践
  | "C3_decisions"        // 历史决策
  | "C4_test_strategy"    // 测试策略
  | "C5_known_issues"     // 已知问题
  | "C6_team_convention"  // 团队约定
  | "C7_review_checklist" // 审查清单
  | "C8_deployment";      // 部署清单
```

#### D 上下文知识（7）

```typescript
type D_Dimensions =
  | "D1_task_progress"     // 任务进度
  | "D2_session_log"       // 会话记录
  | "D3_decision_history"  // 决策历史
  | "D4_contract_sync"     // 跨特性契约对齐
  | "D5_ai_call_chain"     // AI 调用链路
  | "D6_task_snapshot"     // 任务快照（自指）
  | "D7_failure_modes";    // 失败模式
```

---

## 4. 场景预设（MVP 5 个）

### 4.1 MVP 5 个核心预设

| # | 预设名 | 维度集 | 用途 |
|---|---|---|---|
| 1 | `new_feature` | A1+A2+A6+A7+B1+B2+C1+C3+D1+D6 | 新功能开发 |
| 2 | `bug_fix` | A1+A2+A3+A4+C1+C5+D1+D7 | Bug 修复 |
| 3 | `refactor` | A1+A2+A3+A4+C1+C2+C3+D1 | 技术重构 |
| 4 | `hotfix` | A1+A3+C1+C5+D1+D7 | 紧急 hotfix |
| 5 | `architecture_decision` | A1+A8+C3+D3 | 架构决策 |

### 4.2 后期扩展预设（v1.0）

| # | 预设名 | 维度集 |
|---|---|---|
| 6 | `cross_stack` | A3+A5+B2+D4 |
| 7 | `project_handover` | 全部 A/B/C + 关键 D |
| 8 | `perf_optimize` | A1+A4+C1+C2+D7 |
| 9 | `regression_test` | C4+C5+D1 |
| 10 | `small_task` | A2+D1 |

### 4.3 预设的完整定义

```yaml
# core/scenarios/new_feature.yaml
name: new_feature
description: 新功能开发
dimensions:
  - A1_architecture       # 项目架构（模块划分）
  - A2_code_standards     # 代码规范
  - A6_requirement        # 当前需求
  - A7_design_spec        # 设计规约
  - B1_glossary           # 业务术语
  - B2_business_rules     # 业务规则
  - C1_gotchas            # 已知踩坑
  - C3_decisions          # 历史决策
  - D1_task_progress      # 任务进度
  - D6_task_snapshot      # 任务快照
default_format: detailed
default_top_k: 15
```

---

## 5. 工作流示例

### 5.1 普通用户（用预设）

```
用户：「我要新增登录功能」
AI：调用 enjoyflow_context(
  task_id="REQ-XXX",
  preset="new_feature"
)
返回：
{
  "matches": [
    {
      "path": "knowledge-base/project/ARCHITECTURE.md",
      "class": "A1_architecture",
      "title": "项目架构总览",
      "relevance": 0.92,
      "summary": "本项目采用前后端分离架构...",
      "tags": ["architecture", "overview"]
    },
    {
      "path": "knowledge-base/contract/GLOBAL_CONTRACT.md",
      "class": "A3_api_contract",
      "title": "API 全局契约",
      "relevance": 0.88,
      "summary": "REST 风格，错误码 0=成功...",
      "tags": ["api", "contract"]
    },
    ...
  ],
  "total_candidates": 47,
  "filtered_count": 12,
  "tokens_used": 1850
}
AI：用 Read 深入某条 → 写代码
```

### 5.2 高级用户（声明维度）

```
用户：「改后端导出接口」
AI：调用 enjoyflow_context(
  task_id="REQ-XXX",
  scope="module",
  module="backend/export",
  dimensions=[
    "A3_api_contract",
    "A4_data_model",
    "A2_code_standards",
    "C1_gotchas"
  ]
)
```

### 5.3 性能优化（多维度 + 时间过滤）

```
用户：「优化导出性能」
AI：调用 enjoyflow_context(
  task_id="REQ-XXX",
  dimensions=[
    "A1_architecture",
    "A4_data_model",
    "C1_gotchas",
    "C2_patterns",
    "D7_failure_modes"
  ],
  since="2026-01-01"
)
```

---

## 6. 内部实现机制

### 6.1 tag 驱动（用 grep 实现）

**关键设计决策**——享受 Flow **不维护专门的 tag 索引文件**——**用 grep 实现 tag 搜索**。

#### 为什么用 grep 而不维护专门索引

**享受 Flow 真实规模预估**：

| 阶段 | 文档数 | grep 时间 | 是否需要专门索引 |
|---|---|---|---|
| demo 阶段 | 38 | < 10ms | ❌ 不需要 |
| 1-2 年项目 | 100-500 | 10-100ms | ❌ 不需要 |
| 3-5 年项目 | 500-2,000 | 100-500ms | ⚠️ 可选 |
| 5+ 年项目 | 2,000+ | 1s+ | ✅ 建议升级 |

**结论**——**享受 Flow 早期（< 2000 文档）用 grep 完全够用**——**不维护专门索引**。

#### grep 实现的核心机制

**markdown 的 YAML frontmatter 是 grep 友好的**：

```markdown
---
tags:
  - export
  - excel
class: C1_gotchas
---

# Excel 导出踩坑清单
...
```

**grep 搜 "export" tag**：

```bash
# 匹配 tags 字段含 export 的文档
grep -rl "^  - export$" knowledge-base/ 2>/dev/null
```

或更宽松（整行包含）：

```bash
# 任一行包含 export
grep -rl "export" knowledge-base/ 2>/dev/null

# 但要排除偶然出现（比如"避免与 export 混淆"）
# 严格模式：只匹配 tags 字段
grep -rl --include="*.md" -A 5 "^tags:" knowledge-base/ | xargs grep -l "^  - export"
```

#### 4 个核心 grep 命令

```bash
# 1. 找某个 tag 的所有文档
enjoyflow tag-search export
# 内部：grep -rl --include="*.md" -A 5 "^tags:" knowledge-base/ | xargs grep -l "^  - export$"

# 2. 找某个文档的所有 tag
enjoyflow tag-show knowledge-base/development/GOTCHAS.md
# 内部：sed -n '/^---$/,/^---$/p' file.md | grep "^  - "

# 3. 找带某个 tag 的文档数
enjoyflow tag-stats
# 内部：grep -rh "^  - " knowledge-base/ | sort | uniq -c | sort -rn

# 4. 反向搜索（找包含多个 tag 的文档）
enjoyflow tag-search export excel
# 内部：先找 export 的，再找 excel 的，求交集
```

#### 完整 grep 工作流

```
1. AI 调用 enjoyflow tag-search export
   ↓
2. 内部 grep:
   grep -rl --include="*.md" -A 5 "^tags:" knowledge-base/ \
     | xargs grep -l "^  - export$"
   ↓
3. 返回文档路径列表
   ↓
4. AI 用 Read 工具深入某条
```

#### 跟专门索引的对比

| 维度 | grep 实现 | 专门索引 |
|---|---|---|
| 实现复杂度 | ⭐（1 行 grep） | ⭐⭐⭐⭐⭐（5 个组件） |
| 维护成本 | 0 | 高（要更新/失效/分片） |
| 准确性 | ⚠️ 中（依赖 tag 写得规范） | ✅ 高（归一化） |
| 速度（< 2000 文档） | ✅ < 500ms | ✅ < 1ms |
| 速度（> 5000 文档） | ❌ 1s+ | ✅ < 1ms |
| 新鲜度 | ✅ 永远新鲜 | ⚠️ 缓存可能过期 |

**关键差异**——**grep 实现牺牲一点准确性换极致简单**。

#### grep 实现的准确性保证

**用 grep 不准的场景**：

```markdown
本文讨论 pricing 模块，**避免与 export 模块混淆**。
```

**解决**——**tag 写规范**：

```yaml
# 错的写法（grep 会误判）
本文讨论 pricing，避免与 export 混淆。

# 对的写法（grep 不会误判）
---
tags:
  - pricing             # ← tag 写在 frontmatter，不在正文
  - billing
---
```

**享受 Flow 的约束**——tag **只写在 YAML frontmatter**——**正文不写 tag**。

#### 升级路径——从 grep 到专门索引

享受 Flow 提供**渐进升级路径**：

```
阶段 1（MVP）：纯 grep
   ↓ 用户项目超过 2000 文档
阶段 2（v0.5）：grep + 简单缓存（grep-cache.yaml）
   ↓ 用户需要语义搜索
阶段 3（v1.0）：专门索引（tags-index.yaml）+ LRU 缓存
   ↓ 项目规模继续增长
阶段 4（v2.0）：向量索引（vector-search.yaml）
```

**升级触发条件**——**文档数 / 查询延迟**——**不是设计初期就决定的**。

#### 4 个真实工程约束

享受 Flow 用 grep 实现 tag 搜索的 4 个约束：

1. **tag 只在 YAML frontmatter**（不在正文）
2. **每篇文档必须有 tag**（不强制 1 个粗粒度，但 ≥1 个细粒度）
3. **tag 大小写不敏感**（grep -i 标志）
4. **不支持同义词**（要写标准 tag，aliases 留给 v1.0）

#### tag 健康检查命令

```bash
enjoyflow tag-stats              # 统计所有 tag 频率
enjoyflow tag-check              # 检查所有文档都有 tag
enjoyflow tag-find-orphans       # 找无 tag 的文档
enjoyflow tag-find-cold          # 找只出现 1 次的 tag（冷僻 tag）
```

#### 完整命令清单

| 命令 | 作用 |
|---|---|
| `enjoyflow tag-search <tag>` | 找含某 tag 的文档 |
| `enjoyflow tag-show <file>` | 显示某文档的所有 tag |
| `enjoyflow tag-stats` | 统计 tag 频率 |
| `enjoyflow tag-check` | 检查所有文档 |
| `enjoyflow tag-find-orphans` | 找无 tag 文档 |
| `enjoyflow tag-find-cold` | 找冷僻 tag |

#### 一句话总结

**享受 Flow 用 grep 实现 tag 搜索——不维护专门索引，享受 0 维护成本 + 永远新鲜的索引**。

### 6.2 tag 索引（未来扩展）

**当文档数 > 2000 或查询延迟 > 500ms 时**，享受 Flow 可升级到专门索引：

```yaml
# .enjoyflow/tags-index.yaml
# 这是升级后的索引格式，grep 实现不够用时启用

tags:
  export:
    count: 234
    documents:
      - path: knowledge-base/project/ARCHITECTURE.md
        anchor: "#export-module"
        class: A1_architecture
```

**但 MVP 不实现这个**——**用 grep 够用**。

**真实工程问题**——tag 数量会随文档增长而**爆炸**：

| 项目规模 | 文档数 | 平均 tag/文档 | 不优化 tag 总数 |
|---|---|---|---|
| 小型 | 50 | 3 | 150 |
| 中型 | 500 | 5 | 2,500 |
| 大型 | 5,000 | 8 | 40,000 |
| 巨型 | 50,000 | 10 | 500,000 |

#### 4 个真实风险

1. **命名不一致**——同一概念用不同 tag（Excel / EXCEL / excel-export / Excel导出）
2. **tag 太宽泛**——几乎所有文档都有，索引等于没建
3. **tag 太冷僻**——只用一次，浪费索引条目
4. **tag 永不过期**——5 年前的 tag 还活着，搜到错误文档

#### 4 层防御体系

#### 第 1 层：tag 规范化（写入时控制）

```yaml
# core/tag-vocabulary.yaml
canonical_tags:
  export:
    aliases: [Export, EXPORT, 导出, exports, exporting]
  excel:
    aliases: [Excel, EXCEL, xls, xlsx]

coarse_tags:                    # 必须从这 20 个粗粒度 tag 里选
  - architecture
  - backend
  - frontend
  - data
  - business
  - deployment
  - testing
  - security
  - performance
  - integration
  - api
  - ui
  - database
  - infrastructure
  - documentation
  - monitoring
  - compliance
  - migration
  - refactoring
  - feature

fine_tags: unlimited            # 细粒度 tag 任意添加
```

**约束**——每篇文档**至少 1 个粗粒度 tag + 任意细粒度 tag**。

#### 第 2 层：自动归一化（索引时执行）

```python
def index_documents():
    for doc in scan_all_documents():
        normalized_tags = []
        for tag in doc.tags:
            tag = normalize_tag(tag)             # 大小写、同义词
            if is_cold_tag(tag):
                tag = suggest_merge(tag)         # 冷僻 tag 合并
            normalized_tags.append(tag)
        doc.tags = normalized_tags
```

#### 第 3 层：分片 + 增量（存储优化）

```
.enjoyflow/
└── tags-index/
    ├── _manifest.yaml         # 索引清单
    ├── export.yaml            # 模块级索引
    ├── billing.yaml
    └── auth.yaml
```

**AI 查 export**——只加载 export.yaml——**不加载整个索引**。

**增量更新**：

```bash
enjoyflow index --incremental  # 只扫描变更的文档
```

#### 第 4 层：LRU 缓存 + 失效机制（查询优化）

```python
class TagIndexCache:
    def __init__(self, maxsize=100):
        self.cache = LRUCache(maxsize)
    
    def get(self, tag):
        if tag in self.cache:
            return self.cache[tag]
        if self.is_stale():
            self.rebuild()
        result = self.index[tag]
        self.cache[tag] = result
        return result
```

**效果**——80% 的查询命中缓存，索引查询 < 1ms。

#### 应用 4 层防御后的效果

| 项目规模 | 不优化 tag 总数 | 4 层防御后 | 减少 |
|---|---|---|---|
| 小型 | 150 | 50 | 3x |
| 中型 | 2,500 | 300 | 8x |
| 大型 | 40,000 | 1,500 | 27x |
| 巨型 | 500,000 | 8,000 | **62x** |

**索引文件大小**：

| 项目规模 | 不优化 | 4 层防御后 |
|---|---|---|
| 小型 | 50KB | 20KB |
| 中型 | 2MB | 200KB |
| 大型 | 50MB | 1.5MB |
| 巨型 | 500MB | 8MB |

#### 关键约束

- ✅ **20 个粗粒度 tag 固定**（控制广度）
- ✅ **细粒度 tag 自动归一化**（控制碎度）
- ✅ **冷僻 tag 自动合并**（控制僵尸）
- ✅ **模块化分片索引**（控制加载）
- ✅ **LRU 缓存**（控制查询）

#### tag 健康检查命令

```bash
enjoyflow tag-stats             # 显示 tag 频率
enjoyflow tag-cleanup --dry-run # 建议清理的冷僻 tag
enjoyflow tag-normalize         # 自动归一化所有文档的 tag
```

### 6.4 匹配算法

```python
def enjoyflow_context(req: EnjoyFlowContextInput) -> EnjoyFlowContextResult:
    # 1. 维度解析
    if req.preset:
        dimensions = SCENARIO_PRESETS[req.preset].dimensions
    elif req.dimensions:
        dimensions = req.dimensions
    else:
        dimensions = DEFAULT_DIMENSIONS  # [A1, A6, D1]

    # 2. 范围过滤
    if req.scope == "task":
        req_tags = load_req(req.task_id).tags
        candidates = filter_by_tags(all_docs, req_tags)
    elif req.scope == "module":
        candidates = filter_by_module(all_docs, req.module)
    elif req.scope == "class":
        candidates = filter_by_class(all_docs, dimensions)
    else:
        candidates = all_docs

    # 3. 维度过滤
    docs = [d for d in candidates if d.class in dimensions]

    # 4. 相关性排序
    if req.task_id:
        scored = score_by_relevance(docs, req.task_id)
    else:
        scored = docs

    # 5. 时间过滤
    if req.since:
        scored = [d for d in scored if d.last_modified >= req.since]

    # 6. 截取 top_k
    top = scored[:req.top_k or 10]

    # 7. 格式化输出
    output = format_docs(top, req.format, req.include_summaries)

    # 8. token 限制（按需）
    if req.max_tokens:
        output = truncate_to_tokens(output, req.max_tokens)

    return output
```

### 6.4 相关性评分

```python
def score_by_relevance(docs, task_id):
    req = load_req(task_id)
    req_tags = set(req.tags)

    scored = []
    for doc in docs:
        doc_tags = set(doc.tags)

        # tag 重合度
        overlap = len(req_tags & doc_tags)
        total = len(req_tags | doc_tags)
        jaccard = overlap / total if total > 0 else 0

        # 路径相关性（任务关联文件优先）
        path_bonus = 0.2 if any(f in doc.path for f in req.related_files) else 0

        # 时效性（近期变更优先）
        recency = days_since_modification(doc) < 30 and 0.1 or 0

        score = jaccard + path_bonus + recency
        scored.append((doc, score))

    return sorted(scored, key=lambda x: -x[1])
```

---

## 7. 跨工具接入

### 7.1 三通道接入

| 通道 | 适用工具 | 配置方式 |
|---|---|---|
| **MCP 协议** | Cursor / Claude Code / Codex CLI | `.mcp.json` |
| **Skill 加载** | Claude Code / Codex CLI | `/skill load enjoyflow` |
| **文件协议** | 任何 AI 工具 | `@enjoyflow/context` 命令 |

### 7.2 MCP 配置示例

```json
{
  "mcpServers": {
    "enjoyflow": {
      "command": "enjoyflow",
      "args": ["mcp", "serve"],
      "env": {
        "ENJOYFLOW_PROJECT": "/path/to/project"
      }
    }
  }
}
```

### 7.3 Claude Code Skill

```markdown
# skills/enjoyflow-context/SKILL.md
---
name: enjoyflow-context
description: 获取当前任务的上下文知识
---

# EnjoyFlow Context Skill

调用 enjoyflow CLI 获取与当前任务相关的所有知识：

\`\`\`bash
enjoyflow context --task $TASK_ID --preset new_feature
\`\`\`

参数：
- \`--task\`: 任务 ID
- \`--preset\`: 场景预设
- \`--dimensions\`: 自定义维度
- \`--max-tokens\`: 输出限制
```

---

## 8. 辅助工具

### 8.1 `enjoyflow_record`

```typescript
interface EnjoyFlowRecordInput {
  type: "decision" | "gotcha" | "pattern" | "issue";
  title: string;
  content: string;             // Markdown 内容
  task_id?: string;            // 关联任务
  tags?: string[];
  class?: Dimension;           // 自动推断
}
```

**行为**：
- 自动推断 class（A1-D7）
- 自动添加 tag
- 自动写入对应知识文件
- 自动更新 tags-index.yaml
- 自动记录 last_modified

### 8.2 `enjoyflow_search`

```typescript
interface EnjoyFlowSearchInput {
  query: string;                // 自然语言查询
  scope?: "title" | "content" | "all";
  class?: Dimension[];
  top_k?: number;
}
```

**行为**：
- 全文搜索（grep 风格）
- 或语义搜索（向量检索，v1.0+）
- 返回匹配项 + 摘要

---

## 9. 错误码

| 错误码 | 含义 |
|---|---|
| D6-001 | task_id 不存在 |
| D6-002 | 无效的 dimension |
| D6-003 | 无效的 preset |
| D6-004 | tag 索引损坏 |
| D6-005 | 知识库目录不可访问 |
| D6-006 | max_tokens 超出限制 |

---

## 10. 与其他模块关系

| 模块 | 关系 |
|---|---|
| [POSITIONING.md](POSITIONING.md) | v5 §4 定义 D6 为核心差异化 |
| [INTERFACE-SPEC.md](INTERFACE-SPEC.md) | §6.2 MemoryAPI 提供底层存储 |
| [DISCUSSION-LOG-2026-06-19.md](DISCUSSION-LOG-2026-06-19.md) | §15 详细讨论 D6 的 5 个子问题 |

---

## 11. 演进路径

| 版本 | 时间 | 里程碑 |
|---|---|---|
| v0.1 | 2026 Q3 | MVP：tag 驱动 + 5 预设 + CLI |
| v0.5 | 2026 Q4 | MCP 接入 + 语义搜索 + 10 预设 |
| v1.0 | 2027 Q1 | 智能推荐 + 自动维度推断 + 多 AI 工具 |

---

## 12. 可扩展性设计（4 个可扩展点）

D6 必须支持可扩展——用户在不动享受 Flow 内核的情况下，能扩展预设/维度/适配器/算法。

### 12.1 4 个可扩展点总览

| # | 可扩展点 | 用途 | 配置文件 |
|---|---|---|---|
| 1 | **scenarios** | 用户新增场景预设 | `scenarios/*.yaml` |
| 2 | **dimensions** | 用户新增知识维度 | `dimensions/*.yaml` |
| 3 | **adapters** | 用户接新 AI 工具/存储 | `adapters/*.yaml` |
| 4 | **matchers** | 用户换匹配算法 | `matchers/*.yaml` 或 `matchers/*.py` |

### 12.2 三层优先级

```
内置（core/） < 用户级（~/.enjoyflow/） < 项目级（.enjoyflow/）
```

- **内置**——享受 Flow 框架自带（最低优先级）
- **用户级**——开发者跨项目通用（中等优先级）
- **项目级**——本项目专属（最高优先级）

**冲突解决**——高层覆盖低层。

### 12.3 可扩展点 1：预设（scenarios）

#### 用户场景

某团队常用 `compliance_audit`（合规审计）场景——享受 Flow 默认没这个预设。

#### 项目级定义

```yaml
# .enjoyflow/scenarios/compliance_audit.yaml
name: compliance_audit
description: 合规审计场景
extends: new_feature           # 可继承其他预设
dimensions:
  - A1_architecture
  - A3_api_contract
  - B4_constraints             # 业务约束
  - C6_team_convention
  - C7_review_checklist
  - D3_decision_history
default_format: detailed
default_top_k: 20
```

#### 关键能力

- ✅ 预设可在项目级 / 用户级定义
- ✅ 预设可继承其他预设（`extends`）
- ✅ 预设可加自定义维度
- ❌ 不能删除内置预设（兼容性）

### 12.4 可扩展点 2：维度（dimensions）

#### 用户场景

某团队用 Rust 做嵌入式，需要 `R1_rust_embedded` 这种专属维度。

#### 项目级定义

```yaml
# .enjoyflow/dimensions/R1_rust_embedded.yaml
name: R1_rust_embedded
class: R                      # 自定义类别（扩展 ABCD 之外）
title: Rust 嵌入式规范
description: Rust 嵌入式开发规范
default_path: knowledge-base/embedded/rust-standards.md
tags: [rust, embedded]
```

#### 类别可扩展

不限于 ABCD——可以加 E/F/G... R 等任意字母。

#### 类型系统扩展

```typescript
// 旧：封闭枚举
type Dimension = "A1_architecture" | ... | "D7_failure_modes";

// 新：开放（基于运行时注册）
type Dimension = string & { __brand: "Dimension" };
```

#### 关键能力

- ✅ 用户项目可加任意维度
- ✅ 维度可项目级 / 用户级 / 全局级定义
- ✅ 类别字符可扩展（A-Z）
- ❌ 内置 32 个不可删（兼容性）

### 12.5 可扩展点 3：适配器（adapters）

#### 用户场景

某团队用 Continue IDE / Cody / Tabnine 这些 AI 工具——享受 Flow 默认不直接支持。

#### 项目级定义

```yaml
# .enjoyflow/adapters/continue.yaml
name: continue
type: ai_tool                 # 类型：ai_tool | storage | formatter | indexer
description: Continue IDE 适配器
input_format: skill           # 入口格式：mcp | skill | file | slash_command
output_format: skill
config:
  config_file: ~/.continue/config.json
  skill_install_path: ~/.continue/skills/
  mcp_servers:
    - name: enjoyflow
      command: enjoyflow
      args: [mcp, serve]
```

#### 适配器类型

| type | 含义 | 例子 |
|---|---|---|
| `ai_tool` | AI 编码工具 | Cursor/Copilot/Claude Code/Continue/Cody |
| `storage` | 知识存储后端 | Notion/Obsidian/mem0 |
| `formatter` | 输出格式化 | yaml/markdown/json/html |
| `indexer` | 知识索引 | tag/语义/全文 |

#### 关键能力

- ✅ 任意 AI 工具可接入（不绑定）
- ✅ 适配器可继承（Continue 基于 VSCode）
- ✅ 适配器可重写（覆盖内置行为）
- ✅ 适配器可声明依赖

### 12.6 可扩展点 4：匹配器（matchers）

#### 用户场景

某团队用向量检索，不想要 tag 匹配——他们想接 Pinecone。

#### 项目级定义（声明式）

```yaml
# .enjoyflow/matchers/vector_search.yaml
name: vector_search
type: matcher
description: 基于向量检索的匹配器
config:
  backend: pinecone
  index: enjoyflow-knowledge
  embedding_model: text-embedding-3-small
  top_k: 20
  threshold: 0.7
```

#### 项目级定义（编程式）

```yaml
# .enjoyflow/matchers/custom_matcher.yaml
name: custom_matcher
type: matcher
implementation: scripts/matchers/custom.py
```

```python
# scripts/matchers/custom.py
from enjoyflow.matcher import Matcher, MatchRequest, MatchResult

class CustomMatcher(Matcher):
    name = "custom_matcher"
    description = "团队自定义匹配器"
    
    async def match(self, req: MatchRequest) -> list[MatchResult]:
        # 自定义匹配逻辑
        ...
```

#### Matcher 接口

```typescript
interface Matcher {
  name: string;
  
  match(req: MatchRequest): Promise<MatchResult[]>;
  
  description: string;
  version: string;
  health_check(): Promise<HealthStatus>;
}

interface MatchRequest {
  task_id?: string;
  tags?: string[];
  dimensions?: string[];
  scope: ContextScope;
  top_k: number;
  max_tokens?: number;
}

interface MatchResult {
  path: string;
  class: string;
  title: string;
  relevance: number;       // 0-1
  summary?: string;
  tags: string[];
  matched_by: string;      // 哪个匹配器产生的
}
```

#### 内置匹配器

| 匹配器 | 机制 | 适用 |
|---|---|---|
| `tag` | tag 精确匹配 | MVP |
| `jaccard` | tag Jaccard 相似度 | MVP |
| `keyword` | BM25 全文检索 | v0.5 |
| `vector` | 向量语义检索 | v1.0 |
| `hybrid` | 混合（tag + keyword + vector） | v1.0 |

#### 关键能力

- ✅ 匹配器接口稳定（Matcher interface）
- ✅ 用户可写 Python/JS 匹配器
- ✅ 匹配器可链式（fallback 链：tag → jaccard → vector → hybrid）

### 12.7 Registry 注册中心

```python
class Registry:
    """享受 Flow 可扩展点注册中心"""
    
    def __init__(self):
        self._scenarios: dict[str, Scenario] = {}
        self._dimensions: dict[str, Dimension] = {}
        self._adapters: dict[str, Adapter] = {}
        self._matchers: list[Matcher] = []
    
    def register_scenario(self, scenario, source):
        """注册预设：项目级 > 用户级 > 内置"""
    
    def register_dimension(self, dimension, source):
        """注册维度：项目级 > 用户级 > 内置"""
    
    def register_adapter(self, adapter, source):
        """注册适配器"""
    
    def register_matcher(self, matcher, source):
        """注册匹配器（追加到链尾）"""
    
    def discover(self):
        """从 3 个来源发现所有可扩展点"""
        # 1. 内置（core/）
        # 2. 用户级（~/.enjoyflow/）
        # 3. 项目级（.enjoyflow/）
```

### 12.8 跟享受 Flow 哲学的关系

4 个可扩展点都是 POSITIONING v5 三重哲学的落地：

| 哲学 | 对应可扩展点 |
|---|---|
| **接口规范** | Matcher / Adapter 接口稳定 |
| **默认实现** | 内置 5 预设 / 32 维度 / 5 匹配器 / 4 适配器类型 |
| **易定制** | 项目级可覆盖 + 用户级可扩展 |

### 12.9 关键约束

- ❌ **不能修改内置**（只覆盖/扩展）
- ❌ **不能删除内置预设/维度**（兼容性）
- ✅ **可新增任意可扩展点**
- ✅ **可重写内置行为**（覆盖模式）
- ✅ **可链式 fallback**

---

## 13. 待写文档

- [ ] `core/scenarios/*.yaml` —— 5 个场景预设完整定义
- [ ] `tags-index-schema.md` —— tag 索引 schema
- [ ] `enjoyflow-record-schema.md` —— 记录工具 schema
- [ ] `cross-tool-adapters.md` —— 跨工具适配器详细配置
