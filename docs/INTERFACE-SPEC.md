# EnjoyFlow 接口规范 (Interface Specification)

**版本**: v0.1-draft
**日期**: 2026-06-19
**状态**: 草案
**配套**: [POSITIONING.md](POSITIONING.md) / [DESIGN-PHILOSOPHY.md](DESIGN-PHILOSOPHY.md) / [KNOWLEDGE-ARCHITECTURE.md](KNOWLEDGE-ARCHITECTURE.md)

---

## 1. 目的

本文档定义 EnjoyFlow 的对外接口契约。任何实现（默认实现 / 第三方适配器）必须遵循本文档规定的契约。**契约优先于实现**。

**三条铁律**（来自 DESIGN-PHILOSOPHY §2）:
1. 文档即真实（Documentation as Truth）
2. AI 是不可靠执行者，需要**不可变契约**约束
3. 框架不替代人做架构决策

---

## 2. 范围

本文档覆盖 EnjoyFlow 对外暴露的全部接口：

| 接口 | 简称 | 章节 |
|------|------|------|
| 文档接口 | DocAPI | §6 |
| 记忆接口 | MemoryAPI | §6 |
| 制品接口 | ArtifactAPI | §6 |
| 同步接口 | SyncAPI | §6 |
| 钩子接口 | HookAPI | §7 |
| 插件接口 | PluginAPI | §8 |

---

## 3. 设计目标

| # | 目标 | 衡量 |
|---|------|------|
| G1 | 接口稳定 | 主版本内不破坏性变更 |
| G2 | 默认实现完整 | 每个接口都附参考实现 |
| G3 | 易定制 | 适配器替换无需改主程序 |
| G4 | 渐进披露 | 一行配置可启动，深度配置按需 |
| G5 | 契约严格 | 输入输出有 JSON Schema |
| G6 | 跨工具 | 同一接口在 ≥4 AI 工具下行为一致 |
| G7 | 生命周期覆盖 | 五阶段流水线所有 hook 点 |

---

## 4. 总体架构

```
┌────────────────────────────────────────────────────────┐
│  enjoyflow.yaml  (项目元信息 + API 配置)                │
└────────────────────────────────────────────────────────┘
              ↓
┌────────────────────────────────────────────────────────┐
│  接口层 (Interfaces)                                     │
│  ├── DocAPI      MemoryAPI     ArtifactAPI              │
│  ├── SyncAPI     HookAPI       PluginAPI                │
└────────────────────────────────────────────────────────┘
              ↓
┌────────────────────────────────────────────────────────┐
│  实现层 (Implementations)                                │
│  ├── 默认实现 (Default Implementations)                  │
│  └── 适配器实现 (Adapter Implementations)                │
│      ├── obsidian-adapter                                │
│      ├── memos-adapter                                  │
│      └── ...                                            │
└────────────────────────────────────────────────────────┘
              ↓
┌────────────────────────────────────────────────────────┐
│  落地层 (Runtime)                                        │
│  ├── core/prompts/  core/skills/  core/hooks/           │
│  ├── lifecycle/01-planning/ ... 05-maintenance/         │
│  └── knowledge-base/{project,business,development,...}/ │
└────────────────────────────────────────────────────────┘
```

---

## 5. 项目配置文件 (enjoyflow.yaml)

```yaml
---
# Agent Skills 风格（与 anthropics/skills 152K 兼容）
name: my-project
description: 项目一句话定位
license: Apache-2.0
compatibility: Claude Code / Cursor / Codex / Aider

# Backstage envelope（与 Backstage 33K 兼容）
apiVersion: enjoyflow/v1
kind: Project
metadata:
  name: my-project
  labels:
    domain: water-iot
    scale: small-team
  annotations:
    phase: planning

spec:
  # 6 类 API 配置
  doc:
    adapter: filesystem
    layout: knowledge-architecture-v1
  memory:
    adapter: filesystem-md
    scope: [active-sprint, knowledge-base]
  artifact:
    adapter: filesystem
    path: lifecycle/
  sync:
    adapter: git-hook
    bidirectional: true
  hooks:
    session-start: core/hooks/session-start.md
    pre-implementation: core/hooks/pre-impl.md
    post-verification: core/hooks/post-verify.md
  plugins:
    - obsidian
    - memos
    - backstage-catalog
---
```

---

## 6. 核心 API 契约

> **核心原则**：每个 API 接口都遵循**能力描述 / 输入 / 输出 / 默认实现** 四段结构。

### 6.1 DocAPI（文档接口）

**能力**：CRUD 项目内的结构化文档

**输入**：
```typescript
interface DocInput {
  action: 'read' | 'write' | 'list' | 'delete';
  path: string;            // 相对项目根，例如 knowledge-base/business/domain-a/overview.md
  content?: string;        // write 时必填
  frontmatter?: Record<string, any>;
}
```

**输出**：
```typescript
interface DocOutput {
  status: 'ok' | 'error';
  content?: string;
  frontmatter?: Record<string, any>;
  error?: { code: string; message: string };
}
```

**错误码**：
| code | 含义 |
|------|------|
| DOC-001 | 路径不存在 |
| DOC-002 | 文档已存在（write 时未指定 overwrite） |
| DOC-003 | frontmatter 校验失败 |

**默认实现**：`filesystem`（基于本地文件系统 + Markdown + YAML frontmatter）

---

### 6.2 MemoryAPI（记忆接口）

**能力**：跨会话持久化 AI 短期记忆与项目长期记忆

**输入**：
```typescript
interface MemoryInput {
  action: 'append' | 'read' | 'prune' | 'compress';
  scope: 'session' | 'sprint' | 'project' | 'global';
  key?: string;
  value?: string;
  ttl?: number;            // 秒
}
```

**输出**：
```typescript
interface MemoryOutput {
  status: 'ok' | 'error';
  entries?: Array<{
    key: string;
    value: string;
    scope: string;
    created_at: string;
    expires_at?: string;
  }>;
  error?: { code: string; message: string };
}
```

**错误码**：
| code | 含义 |
|------|------|
| MEM-001 | scope 非法 |
| MEM-002 | 键已存在（append 时未指定 overwrite） |
| MEM-003 | 超过容量上限 |

**默认实现**：`filesystem-md`（每个 scope 一个目录，每个记忆一个 .md 文件）

---

### 6.3 ArtifactAPI（制品接口）

**能力**：管理生命周期五阶段产出的制品（需求、规约、设计、代码、测试）

**输入**：
```typescript
interface ArtifactInput {
  action: 'create' | 'read' | 'transition' | 'archive';
  phase: 'planning' | 'spec' | 'development' | 'verification' | 'maintenance';
  artifact_id: string;     // 例如 REQ-001 / SPEC-001 / TEST-001
  content?: any;
  trace_link?: string;     // 关联上游制品 ID
}
```

**输出**：
```typescript
interface ArtifactOutput {
  status: 'ok' | 'error';
  artifact?: {
    id: string;
    phase: string;
    trace_chain: string[];  // 例如 [REQ-001, SPEC-001, IMPL-001, TEST-001]
    created_at: string;
    status: string;
  };
  error?: { code: string; message: string };
}
```

**错误码**：
| code | 含义 |
|------|------|
| ART-001 | phase 非法 |
| ART-002 | 制品不存在 |
| ART-003 | 追溯链断裂（trace_link 不存在） |
| ART-004 | 阶段跃迁非法 |

**默认实现**：`filesystem`（每个 phase 一个子目录，制品 ID 作为文件名）

---

### 6.4 SyncAPI（同步接口）

**能力**：在多个适配器之间同步文档与记忆

**输入**：
```typescript
interface SyncInput {
  action: 'push' | 'pull' | 'diff' | 'resolve';
  source: string;          // 适配器名，例如 filesystem
  target: string;          // 适配器名，例如 obsidian
  paths?: string[];        // 不指定则全量
  conflict_strategy?: 'source-wins' | 'target-wins' | 'manual';
}
```

**输出**：
```typescript
interface SyncOutput {
  status: 'ok' | 'error' | 'conflict';
  synced?: string[];
  conflicts?: Array<{
    path: string;
    source_version: string;
    target_version: string;
  }>;
  error?: { code: string; message: string };
}
```

**错误码**：
| code | 含义 |
|------|------|
| SYNC-001 | 适配器不存在 |
| SYNC-002 | 同步超时 |
| SYNC-003 | 冲突无法自动解决 |

**默认实现**：`git-hook`（基于 git 提交触发 + 双向 diff）

---

## 7. HookAPI（钩子接口）

### 7.1 能力

在 EnjoyFlow 生命周期的关键事件点注入用户自定义行为。**钩子是适配 EnjoyFlow 与 AI 工具的核心机制**。

### 7.2 钩子点（按生命周期五阶段）

#### 阶段 01-planning

| 钩子名 | 触发时机 | 默认实现 |
|--------|---------|---------|
| `session-start` | AI 会话开始时 | 加载 project metadata + active-sprint 需求 |
| `requirement-created` | REQ-XXX.md 创建 | 校验 AC 完整性，生成追溯链初始节点 |
| `sprint-changed` | Sprint 切换（物理移动文件） | 刷新 backlog 列表，清空过期 AI 记忆 |

#### 阶段 02-spec

| 钩子名 | 触发时机 | 默认实现 |
|--------|---------|---------|
| `pre-spec` | 规约生成前 | 注入 GLOBAL_CONTRACT.md 摘要 |
| `spec-generated` | 规约生成后 | 校验契约完整性（错误码、枚举映射、状态流转） |

#### 阶段 03-development

| 钩子名 | 触发时机 | 默认实现 |
|--------|---------|---------|
| `pre-implementation` | 代码生成前 | 注入规约摘要 + 业务知识 + 设计方案 |
| `post-implementation` | 代码生成后 | 标注 `@ai-generated` 注释 |

#### 阶段 04-verification

| 钩子名 | 触发时机 | 默认实现 |
|--------|---------|---------|
| `pre-verification` | 验证开始前 | 加载独立验证 prompt |
| `verification-failed` | 验证未通过 | 记录失败原因到 artifact 追溯链 |
| `verification-passed` | 验证通过 | 推进 artifact 状态至下一阶段 |

#### 阶段 05-maintenance

| 钩子名 | 触发时机 | 默认实现 |
|--------|---------|---------|
| `commit-created` | git commit 后 | 校验 commit 信息包含 REQ-ID |
| `cross-stack-sync` | 跨特性联调时 | 同步契约变更到所有 domain 目录 |
| `memory-prune` | 项目归档时 | 压缩历史记忆，保留 AC + 关键决策 |

### 7.3 钩子输入输出

```typescript
interface HookInput {
  hook_name: string;          // 例如 session-start
  event: Record<string, any>; // 事件上下文
  project: {
    name: string;
    metadata: Record<string, any>;
  };
}

interface HookOutput {
  status: 'ok' | 'error' | 'skip';
  injected_context?: string;  // 注入到 AI 会话的额外上下文
  artifacts?: Array<{
    type: 'doc' | 'memory' | 'artifact';
    action: 'create' | 'update' | 'delete';
    target: string;
  }>;
  error?: { code: string; message: string };
}
```

### 7.4 钩子实现方式

钩子实现可以是：
- **Markdown prompt 文件**（推荐）：核心/prompts/hook-name.md，由 AI 工具直接读取
- **Python 脚本**（高级）：scripts/hooks/hook-name.py，通过 subprocess 调用
- **MCP 服务器**（生态）：通过 Model Context Protocol 注册

---

## 8. PluginAPI（插件接口）

### 8.1 能力

PluginAPI 是 HookAPI 的**扩展机制**——允许第三方以 npm 包 / pip 包 / git repo 的形式注册钩子与适配器。

### 8.2 插件清单格式 (plugins.yaml)

```yaml
---
plugins:
  - name: obsidian-adapter
    version: 0.1.0
    type: adapter
    provides: [DocAPI, SyncAPI]
    repository: https://github.com/enjoyflow/obsidian-adapter
    entry: obsidian:ObsidianAdapter
    config_schema:
      vault_path: string(required)
      sync_interval: number(default=300)

  - name: memos-adapter
    version: 0.1.0
    type: adapter
    provides: [MemoryAPI]
    repository: https://github.com/enjoyflow/memos-adapter
    entry: memos:MemosAdapter
    config_schema:
      endpoint: string(required)
      api_key: string(required)

  - name: backstage-catalog
    version: 0.1.0
    type: hook
    provides: [HookAPI:sync-to-backstage]
    repository: https://github.com/enjoyflow/backstage-plugin
    entry: backstage:BackstageHook
---
```

### 8.3 插件类型

| type | 说明 | 实现位置 |
|------|------|---------|
| `adapter` | 实现 DocAPI / MemoryAPI / SyncAPI / ArtifactAPI | core/adapters/ |
| `hook` | 实现 HookAPI 的某个钩子点 | core/hooks/plugins/ |
| `prompt` | 实现 AI prompt 模板 | core/prompts/plugins/ |
| `composite` | 同时提供多种类型 | 上述任意 |

### 8.4 插件发现与加载

```typescript
// 启动时
const plugins = discover_plugins([
  './node_modules/@enjoyflow/*',
  './.enjoyflow/plugins/*',
]);

// 校验
for (const plugin of plugins) {
  validate_against_schema(plugin, plugin.config_schema);
  register(plugin);
}
```

### 8.5 插件健康检查

```typescript
interface HealthCheck {
  plugin_name: string;
  status: 'healthy' | 'degraded' | 'failed';
  api_coverage: string[];     // 例如 [DocAPI, MemoryAPI]
  last_health_check: string;
  metrics?: {
    avg_response_ms: number;
    error_rate: number;
  };
}
```

---

## 9. 协作与多模型契约（Collaboration & Multi-Model）

> **核心命题**：享受 Flow 是 **"人与大模型易于使用的知识管理"**——这意味着协作不是可选的，是核心。
>
> 本节定义享受 Flow 在**多人协作 + 多 AI 工具**下的接口契约。

### 9.1 槽位模型（Slot Model）

**核心判断**（DISCUSSION-LOG §11）：**角色不是设计出来的，是被发现的**。

享受 Flow 不预设"4 角色"——**角色本质是任务流反推的独立上下文槽位**。

#### 4 个反推得到的槽位

| 槽位 | 负责任务 | 防 AI 缺陷 | capability 标识 |
|---|---|---|---|
| **architecture_contract** | 需求 + AC + 规约 + 设计 | #2 目标漂移 + #4 过度自信 | `capability: architecture` |
| **execution** | 代码实现 | #1 上下文衰减 | `capability: execution` |
| **independent_verification** | 质量验证 | #3 幻觉一致性 | `capability: verification` |
| **knowledge_routing** | 跨端协作 + 知识沉淀 + 迭代管理 | #5 知识孤岛 | `capability: curation` |

#### 槽位的因果推导

```
推导路径（NOT 预设 → 反推）：
1. 看任务天然有几个独立上下文
2. 反推出槽位数
3. 角色是这些槽位的对外命名

享受 Flow 现有 10 类任务 → 反推合并 → 4 个槽位
```

#### capability schema

```typescript
interface SessionCapability {
  type: "architecture" | "execution" | "verification" | "curation";
  
  // 范围限制
  scope: {
    artifact_ids?: string[];      // 限定制品范围
    memory_scopes?: string[];    // 限定记忆范围
    paths?: string[];            // 限定路径范围
  };
  
  // 继承（用于跨 session 上下文传递）
  inherits_from?: string;        // 父 capability ID
  
  // 跨工具配置
  tools?: string[];              // 允许使用的 AI 工具
}
```

#### 4 条强制不变量

```yaml
invariants:
  - id: inv-001
    name: "单 capability 约束"
    rule: "一个 AI 会话在同一时刻只能有 1 个 type"
    violation: "session cap type=architecture + execution 同时"
  
  - id: inv-002
    name: "执行-验证隔离"
    rule: "execution-capability 会话不能触发 verification-passed hook"
    violation: "execution 会话调用 verify 命令"
  
  - id: inv-003
    name: "验证者只读"
    rule: "verification-capability 会话不能写 artifact / memory"
    violation: "verification 会话修改 PRD"
  
  - id: inv-004
    name: "架构契约锁定"
    rule: "architecture-capability 可以锁 AC，但解锁必须 curation-capability"
    violation: "execution 会话修改已锁定 AC"
```

#### 槽位缩放规则

```yaml
scaling_rules:
  when_team_size_lt_3:
    # 1-2 人团队：合并架构契约 + 执行
    merge: [architecture_contract, execution]
    
  when_team_size_3_to_7:
    # 3-7 人团队：4 槽位齐全
    standard: [architecture_contract, execution, independent_verification, knowledge_routing]
    
  when_team_size_gt_7:
    # 8+ 人团队：执行槽按 domain 拆分
    split:
      execution: [execution_backend, execution_frontend, execution_data]
```

#### capability 的实现示例

```yaml
# 项目级 .enjoyflow/capabilities.yaml
session_capabilities:
  - id: arch-jay
    type: architecture
    scope:
      artifact_ids: [REQ-001, REQ-002]
      memory_scopes: [project, sprint]
    tools: [claude-code, cursor]
    
  - id: exec-bob
    type: execution
    scope:
      artifact_ids: [SPEC-001, IMPL-001]
      paths: [backend/export/]
    inherits_from: arch-jay   # 可以读 arch 锁定的 AC
    tools: [claude-code]
    
  - id: verify-dave
    type: verification
    scope:
      artifact_ids: [IMPL-001, TEST-001]
    # 验证者不继承 execution——强制独立
    tools: [codex]
```

---

### 9.2 生成-验证隔离强制

**核心原则**（DESIGN-PHILOSOPHY §8.4）：**"验证者和生成者必须是陌生人"**。

#### 接口级强制

```typescript
interface VerificationRequest {
  task_id: string;
  artifact_id: string;
  verification_type: "spec" | "standard" | "memory" | "test" | "all";
  
  // 强制要求
  require_independent_session: true;   // 强制新会话
  forbidden_session_id?: string;         // 禁止使用生成者的会话
}

interface VerificationResult {
  status: "passed" | "failed" | "warning";
  checks: {
    spec: CheckResult;        // 规约对标
    standard: CheckResult;    // 规范对标
    memory: CheckResult;      // 记忆对标
    test: CheckResult;        // 测试对标
  };
  evidence: string[];          // 证据路径
  suggestions?: string[];
}

interface CheckResult {
  status: "passed" | "failed" | "skipped";
  issues: Array<{
    severity: "error" | "warning" | "info";
    message: string;
    location?: string;
  }>;
}
```

#### 实现约束

享受 Flow runtime 必须：

1. **拒绝** verification-capability 会话调用 ArtifactAPI.write / MemoryAPI.write
2. **拒绝** execution-capability 会话触发 `verification-passed` 钩子
3. **自动**启动新 AI 会话做验证（即使同工具也强制隔离）
4. **记录** verification 会话 ID 到 artifact metadata（审计追踪）

---

### 9.3 决策者路由契约

**核心判断**（DISCUSSION-LOG §13）：**决策者是路由器 + 轻量化**——不是 AI 模型。

#### 路由 schema

```typescript
interface DecisionRouterInput {
  user_input: string;              // 用户原始输入
  context: {
    project?: string;              // 项目名
    current_task?: string;         // 当前任务 ID
    history?: Array<{              // 最近会话历史
      role: "user" | "assistant";
      content: string;
    }>;
  };
}

interface DecisionRouterOutput {
  selected_scenario?: string;      // 场景预设名（如 "new_feature"）
  matched_rules: Array<{           // 命中的规则
    rule_id: string;
    pattern: string;
    confidence: number;
  }>;
  reasoning: string;               // 路由理由
  fallback_used: boolean;          // 是否用了 LLM fallback
  llm_calls?: number;              // LLM 调用次数（成本监控）
}
```

#### 路由规则（声明式）

```yaml
# core/router/rules.yaml
rules:
  - id: rule-bug-fix-001
    pattern: "(修|fix).*(bug|错|报错|fail)"
    confidence: 0.95
    template: bug_fix
    
  - id: rule-small-task-001
    pattern: "(改|调).*(文案|样式|颜色|文案|文案)"
    confidence: 0.90
    template: small_task
    
  - id: rule-new-feature-001
    pattern: "(加|添加|新增|实现).*(功能|特性|feature)"
    confidence: 0.92
    template: new_feature
    
  - id: rule-hotfix-001
    pattern: "(生产|线上|紧急|hotfix|急)"
    confidence: 0.95
    template: hotfix
    
  - id: rule-architecture-001
    pattern: "(选型|架构|技术选|为什么用)"
    confidence: 0.88
    template: architecture_decision

fallback:
  type: prompt
  prompt_path: core/router/fallback-prompt.md
  max_llm_calls: 1   # 成本控制
```

#### 路由合约

```typescript
interface RouterContract {
  // 输入
  input: DecisionRouterInput;
  
  // 路由规则
  rules: Rule[];
  
  // fallback
  fallback: FallbackStrategy;
  
  // 输出
  output: DecisionRouterOutput;
  
  // 性能约束
  max_latency_ms: number;          // 默认 100ms（轻量化）
  max_llm_calls_per_decision: number;  // 默认 1
}
```

#### 高级用户覆盖

```yaml
# 项目级 .enjoyflow/router-overrides.yaml
rules:
  - id: rule-compliance-001
    pattern: "(合规|审计|合规性)"
    confidence: 0.95
    template: compliance_audit  # 项目自定义模板
    
disable_fallback_llm: true  # 高级用户：纯规则路由
```

---

### 9.4 制品状态机（FSM）

**享受 Flow 制品生命周期**：

```
DRAFT → REVIEW → APPROVED → IN_PROGRESS → VERIFIED → RELEASED → ARCHIVED
   ↓        ↓         ↓             ↓            ↓
 reject  reject  reject         fail        rollback
```

#### 完整 FSM schema

```typescript
type ArtifactState = 
  | "DRAFT"           // 草稿
  | "REVIEW"          // 评审中
  | "APPROVED"        // 已批准
  | "IN_PROGRESS"     // 进行中
  | "VERIFIED"        // 已验证
  | "RELEASED"        // 已发布
  | "ARCHIVED";       // 已归档

interface ArtifactFSM {
  artifact_id: string;
  current_state: ArtifactState;
  
  // 状态转换表
  transitions: Array<{
    from: ArtifactState;
    to: ArtifactState;
    trigger: TransitionTrigger;
    required_capability?: CapabilityType;
    guards?: Guard[];
  }>;
}

type TransitionTrigger =
  | "submit"          // 提交评审
  | "approve"         // 批准
  | "reject"          // 驳回
  | "start_implementation"  // 开始实现
  | "submit_verification"   // 提交验证
  | "verification_passed"  // 验证通过
  | "verification_failed"  // 验证失败
  | "release"         // 发布
  | "rollback"        // 回滚
  | "archive";        // 归档

interface Guard {
  type: "capability" | "field_required" | "check_passed";
  config: Record<string, any>;
}
```

#### 5 个阶段的转换规则

| 阶段 | 转换 | 触发 capability | 守卫条件 |
|---|---|---|---|
| planning | DRAFT → REVIEW | architecture | 5 项检查清单打勾 |
| planning | REVIEW → APPROVED | architecture | 人批准 |
| planning | REVIEW → DRAFT | architecture | 人驳回 |
| spec | APPROVED → SPEC_READY | architecture | SPEC 生成 |
| development | SPEC_READY → IN_PROGRESS | execution | 开始实现 |
| development | IN_PROGRESS → CODE_READY | execution | 实现完成 |
| verification | CODE_READY → VERIFIED | verification（强制独立） | 四重对标全通过 |
| verification | CODE_READY → IN_PROGRESS | verification | 验证失败 |
| maintenance | VERIFIED → RELEASED | architecture | 发布 |
| maintenance | RELEASED → ARCHIVED | curation | 归档 |

#### FSM 持久化

享受 Flow 状态机持久化在 artifact 文件 frontmatter：

```yaml
---
artifact_id: REQ-001
state: IN_PROGRESS
state_history:
  - state: DRAFT
    entered_at: 2026-06-19T10:00:00
    actor: jay
  - state: REVIEW
    entered_at: 2026-06-19T11:00:00
    actor: jay
  - state: APPROVED
    entered_at: 2026-06-19T14:00:00
    actor: alice
---
```

---

### 9.5 多 AI 工具路由

享受 Flow 是 **跨工具中间件**——AI 工具/存储/格式化器都可以接。

#### 接入协议

```typescript
interface AIAdapter {
  name: string;                    // 例如 "claude-code"
  type: "ai_tool";
  
  // 入口格式
  input_format: "mcp" | "skill" | "file" | "slash_command";
  
  // 出口格式
  output_format: "mcp" | "skill" | "file" | "markdown";
  
  // 配置
  config: {
    config_file?: string;          // 工具配置文件路径
    skill_install_path?: string;   // Skill 安装路径
    mcp_servers?: MCPServerConfig[];
  };
  
  // 健康检查
  health_check(): Promise<HealthStatus>;
}
```

#### 跨工具的 capability 传递

```yaml
# AI 工具 A 锁定 AC 后，AI 工具 B 可以读取
session_chaining:
  - producer:
      tool: claude-code
      capability: architecture
      artifact_id: REQ-001
      locked_at: 2026-06-19T14:00:00
      
  - consumer:
      tool: cursor
      capability: execution
      artifact_id: REQ-001
      inherits: locked_at  # 锁定时间传递
      constraints:
        - cannot_modify: [REQ-001.frontmatter.ac]
```

---

### 9.6 上下文窗口预算管理

**核心问题**——AI 上下文窗口有限（80K-200K token），必须主动管理。

#### 三层上下文分级（DESIGN-PHILOSOPHY §2 铁律 1）

| 层 | 加载时机 | 大小限制 | 内容 |
|---|---|---|---|
| **L1 持久层** | 始终加载 | < 10K token | 全局规范 + 架构概览 + 当前 PRD |
| **L2 任务层** | 按需加载 | < 30K token | 当前特性记忆 + 相关规约片段 |
| **L3 会话层** | 临用即弃 | < 50K token | 对话中的澄清、纠正 |

#### 预算分配算法

```typescript
interface ContextBudget {
  total_tokens: number;            // AI 模型上限（80K / 200K）
  
  // 分配
  allocation: {
    L1_persistent: number;        // 默认 10K
    L2_task: number;               // 默认 30K
    L3_session: number;            // 默认 50K
    reserved: number;              // 预留 10%（输出用）
  };
  
  // 当前使用
  used: {
    L1_persistent: number;
    L2_task: number;
    L3_session: number;
  };
}

interface BudgetEnforcement {
  // 超额时
  on_L1_exceeded: "error";        // L1 不允许超额
  on_L2_exceeded: "truncate";     // L2 超额时截断
  on_L3_exceeded: "compress";     // L3 超额时压缩
}
```

#### D6 工具的 max_tokens 参数

```typescript
enjoyflow_context({
  task_id: "REQ-001",
  preset: "new_feature",
  max_tokens: 5000                // AI 主动限制上下文大小
})
```

**效果**——AI 知道自己的上下文窗口剩余，**主动避免溢出**。

---

### 9.7 不可变契约机制

**核心原则**（DESIGN-PHILOSOPHY §2 铁律 2）：**"AI 是不可靠的执行者，需要不可变的契约来约束"**。

#### AC 锁定协议

```typescript
interface ACLockRequest {
  task_id: string;
  ac_id: string;                   // 例如 AC-1
  locked_by: string;               // 人 ID
  locked_at: string;               // ISO 时间
  capability: "architecture";      // 必须是 architecture capability
}

interface ACLockResult {
  status: "locked" | "already_locked" | "unauthorized";
  lock_id?: string;                // 锁 ID（用于解锁）
  locked_version?: number;         // 锁定的 AC 版本号
}
```

#### 锁定规则

| 操作 | 允许 capability | 附加条件 |
|---|---|---|
| **锁定 AC** | architecture | 5 项检查清单全通过 |
| **修改 AC（已锁定）** | ❌ 任何 capability 都不允许 | 必须先解锁 |
| **解锁 AC** | curation | 必须有解锁理由 + 留痕 |
| **查看 AC** | 任何 capability | — |

#### 不可变契约的实现

```yaml
# .enjoyflow/locks/REQ-001.yaml
locks:
  - artifact_id: REQ-001
    type: ac
    target: AC-1, AC-2, AC-3
    locked_by: jay
    locked_at: 2026-06-19T14:00:00
    locked_version: 1
    unlock_history: []   # 解锁记录（空表示从未解锁）

  - artifact_id: REQ-001
    type: prd
    target: REQ-001.md
    locked_by: jay
    locked_at: 2026-06-19T14:00:00
    constraints:
      - no_modify_without_unlock: true
```

#### 强制约束

享受 Flow runtime 必须：

1. **拒绝**任何非 architecture capability 调用 `lock-ac`
2. **拒绝**任何 capability 修改已锁定 AC
3. **强制**解锁必须 curation capability + 留痕
4. **记录**所有锁/解锁操作到 `locks/` 目录

---

## 10. 端到端示例

### 10.1 智慧水务 GIS 采集 APP（参考 EXAMPLE-WALKTHROUGH.md）

```yaml
---
name: water-iot-gis-collector
description: 智慧水务管网 GIS 离线采集 APP
license: Apache-2.0
compatibility: Claude Code / Cursor / Codex / Aider
apiVersion: enjoyflow/v1
kind: Project
metadata:
  name: water-iot-gis-collector
  labels:
    domain: water-iot
    scale: small-team
    phase: planning
spec:
  doc:
    adapter: filesystem
    layout: knowledge-architecture-v1
  memory:
    adapter: filesystem-md
    scope: [active-sprint, knowledge-base]
  artifact:
    adapter: filesystem
    path: lifecycle/
  sync:
    adapter: git-hook
    bidirectional: true
  hooks:
    session-start: core/hooks/session-start.md
    requirement-created: core/hooks/requirement-created.md
    pre-spec: core/hooks/pre-spec.md
    pre-implementation: core/hooks/pre-impl.md
    pre-verification: core/hooks/pre-verify.md
  plugins:
    - obsidian-adapter
    - memos-adapter
---
```

### 10.2 使用流程

1. `enjoyflow init` 生成 enjoyflow.yaml
2. 用户在 active-sprint 创建 REQ-001.md
3. AI 在 pre-spec 钩子触发后产出 SPEC-001.md
4. AI 在 pre-implementation 钩子触发后产出代码
5. 独立 AI 会话在 pre-verification 钩子触发后执行四重对标（规约/规范/记忆/测试）
6. 验证通过后 verification-passed 钩子推进 artifact 状态

---

## 11. 验证流程

### 11.1 契约校验

```bash
# 校验 enjoyflow.yaml
enjoyflow validate enjoyflow.yaml

# 校验插件清单
enjoyflow validate plugins.yaml

# 校验所有制品追溯链
enjoyflow trace-check
```

### 11.2 接口测试

```bash
# 运行 DocAPI 测试套件
enjoyflow test doc-api

# 运行 HookAPI 测试套件
enjoyflow test hook-api --hook session-start
```

### 11.3 CI 集成

```yaml
# .github/workflows/enjoyflow.yml
name: enjoyflow-ci
on: [push, pull_request]
jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npm install -g @enjoyflow/cli
      - run: enjoyflow validate
      - run: enjoyflow test --all
```

---

## 12. 与已有标准的关系

| 借鉴标准 | 借鉴内容 | 享受 Flow 实现 |
|---------|---------|---------------|
| Agent Skills (anthropics/skills 152K) | SKILL.md YAML frontmatter | enjoyflow.yaml 字段名 + 结构 |
| OpenAPI Generator (OpenAPITools 26K) | 模板架构 + generate 工具 | enjoyflow generate 子命令 |
| Backstage (33K + CNCF) | apiVersion/kind/metadata/spec envelope | enjoyflow.yaml envelope |
| LSP (12K) | 能力声明 + 协议协商 | PluginAPI 健康检查 |
| MCP (8.4K + Servers 87K) | 工具注册与发现 | PluginAPI 插件发现 |
| MemOS (10K) | 记忆分类与生命周期 | MemoryAPI scope 设计 |

---

## 13. 设计原则

1. **接口稳定优先于实现简洁**——契约一旦发布不轻易破坏
2. **默认实现完整但不复杂**——每个 API 都有可用的默认实现，但不预设过多策略
3. **适配器可热替换**——切换文档/记忆/制品后端无需重启
4. **契约严格于实现宽松**——输入输出校验严格，但内部实现可灵活
5. **双向翻译优于单向导出**——同一份数据既让人读也让 AI 读
6. **演进友好于一次到位**——通过 apiVersion 标识兼容性

---

## 14. 演进路径

| 版本 | 时间 | 里程碑 |
|------|------|--------|
| v0.1 | 2026 Q3 | MVP：filesystem 默认实现 + 6 类 API 接口冻结 |
| v0.5 | 2026 Q4 | 适配器生态：obsidian/memos/backstage 至少 3 个第三方适配器 |
| v1.0 | 2027 Q1 | 生态成形：≥10 个适配器 + ≥3 个 AI 工具兼容性 |
| v2.0 | 2027 Q3 | 标准化：与 SPEC-Kit/MCP/MemOS 互操作 |

---

## 附录 A：错误码全集

详见各 API §6.x 节的错误码表。汇总在 `errors.md`（待写）。

## 附录 B：默认实现参考

| API | 默认实现 | 路径 |
|-----|---------|------|
| DocAPI | filesystem | core/adapters/doc/filesystem/ |
| MemoryAPI | filesystem-md | core/adapters/memory/filesystem-md/ |
| ArtifactAPI | filesystem | core/adapters/artifact/filesystem/ |
| SyncAPI | git-hook | core/adapters/sync/git-hook/ |
| HookAPI | markdown-prompt | core/hooks/ |
| PluginAPI | npm-discovery | core/plugins/discovery/ |

## 附录 C：变更日志

| 版本 | 日期 | 变更 |
|------|------|------|
| v0.1-draft | 2026-06-19 | 初稿。定义 6 类 API + enjoyflow.yaml + 钩子点 + 插件清单 |
| v0.1-draft+ | 2026-06-20 | 新增 §9 协作与多模型契约（基于 §11/§13 认知刷新） |

---

## 15. 版本与兼容性

> **说明**：本节定义于附录之后——因为 §9 协作契约是这一阶段的核心新增，而版本兼容性是相对独立的"运维级"规范。

### 15.1 语义版本

- **主版本**：破坏性契约变更（例如 DocAPI 输入字段重命名）
- **次版本**：新增 API / 新增钩子点（向后兼容）
- **修订号**：bug 修复、文档修正

### 15.2 兼容矩阵

| EnjoyFlow 版本 | apiVersion | 适配器最低版本 |
|---------------|-----------|---------------|
| 0.1.x | enjoyflow/v1 | 0.1.0 |
| 0.5.x | enjoyflow/v1 | 0.1.0 |
| 1.0.x | enjoyflow/v1 | 0.5.0 |

### 15.3 迁移策略

主版本升级时：
1. 同时维护 v1 和 v2 适配器（最多 6 个月）
2. deprecation 警告至少 3 个次版本
3. 自动迁移工具（migrate_v1_to_v2）
