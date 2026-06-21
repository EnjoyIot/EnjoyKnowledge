# EnjoyFlow 场景模板规范 (Scenario Templates)

**版本**: v0.1-draft
**日期**: 2026-06-20
**状态**: 草案
**配套**: [D6-SNAPSHOT-SPEC.md](D6-SNAPSHOT-SPEC.md) / [POSITIONING.md](POSITIONING.md) v5

---

## 1. 目的

本文档定义 EnjoyFlow **场景模板**的完整规范——5 个 MVP 预设 + 5 个扩展预设的可变流程、维度集和默认 prompt。

**场景模板**——把"开发者做某类任务时的最佳实践"形式化为可复用的模板，让 AI 工具自动选择适合当前任务的预设。

**核心约束**——场景模板的状态机必须配合 [INTERFACE-SPEC.md](INTERFACE-SPEC.md) §9 的 capability 模型：

| 状态 | 对应 capability | 防 AI 缺陷 |
|---|---|---|
| requirement_*, spec_* | architecture | #2 目标漂移 + #4 过度自信 |
| implementation_* | execution | #1 上下文衰减 |
| verification_* | verification（独立会话） | #3 幻觉一致性 |
| knowledge_*, archive_* | curation | #5 知识孤岛 |

**执行-验证隔离强制**——verification 状态必须**强制新会话**（§9.2 不变量 inv-002）。

---

## 2. 场景模板的核心结构

每个场景模板包含 **5 个部分**：

```yaml
---
name: <场景名>
description: <场景描述>
version: 0.1.0

# 1. 维度集（必填）—— 该场景需要哪些知识类别
dimensions:
  - A1_architecture
  - A2_code_standards
  - ...

# 2. 状态机（必填）—— 该场景的任务状态流转
states:
  - name: <状态名>
    entry_actions: [<动作列表>]
    exit_actions: [<动作列表>]
    auto_transitions:
      - to: <下一状态>
        when: <触发条件>
    manual_transitions:
      - to: <下一状态>
        when: <人工触发条件>

# 3. 分支条件（可选）—— 按任务特征走不同子流程
branches:
  - name: <分支名>
    when: <判定条件>
    states_override: [<替代状态序列>]

# 4. 默认 prompt（必填）—— AI 在该场景下应该问/做什么
prompts:
  opening: |
    <开场 prompt>
  per_state:
    - state: <状态名>
      prompt: |
        <该状态下的 prompt>
  output_format: |
    <输出格式说明>

# 5. 钩子集成（可选）—— 在状态转换时触发享受 Flow 钩子
hooks:
  on_state_enter:
    - state: <状态名>
      hook: <享受 Flow 钩子名>
---
```

---

## 3. MVP 5 个核心场景模板

### 3.1 new_feature（新功能开发）

#### 完整定义

```yaml
# core/scenarios/new_feature.yaml
---
name: new_feature
description: 新功能开发（中-大型，覆盖完整生命周期）
version: 0.1.0

# 维度集
dimensions:
  - A1_architecture       # 项目架构
  - A2_code_standards     # 代码规范
  - A6_requirement        # 当前需求
  - A7_design_spec        # 设计规约
  - B1_glossary           # 业务术语
  - B2_business_rules     # 业务规则
  - C1_gotchas            # 已知踩坑
  - C3_decisions          # 历史决策
  - D1_task_progress      # 任务进度
  - D6_task_snapshot      # 任务快照

# 状态机
states:
  - name: requirement_drafting
    description: 编写 PRD
    entry_actions:
      - 加载 B1 业务术语
      - 加载 B2 相关业务规则
    exit_actions:
      - 生成 PRD v1
      - 自动设置 status=Draft
    auto_transitions:
      - to: requirement_review
        when: PRD 完成检查清单 5 项打勾

  - name: requirement_review
    description: PRD 评审
    entry_actions:
      - 通知需求 Owner
    exit_actions:
      - 更新 PRD v2
      - 设置 status=Approved（如果批准）
    manual_transitions:
      - to: requirement_drafting
        when: 评审不通过，需要修改

  - name: spec_generation
    description: 规约生成
    entry_actions:
      - 加载 A1 架构
      - 加载 A3 API 契约
      - 加载 D6 任务快照
    exit_actions:
      - 生成 SPEC-XXX.md
      - 添加追溯链 [REQ-XXX → SPEC-XXX]

  - name: implementation
    description: 代码实现
    entry_actions:
      - 加载 A2 代码规范
      - 加载 C1 已知踩坑
    exit_actions:
      - 自动标注 @ai-generated
      - Git commit 含 REQ-ID

  - name: verification
    description: 独立验证（不同会话）
    entry_actions:
      - 启动独立 AI 会话（强制隔离）
      - 加载 A5 接口规约
    exit_actions:
      - 验证报告
      - 设置 status=VERIFIED 或 VERIFICATION_FAILED

  - name: knowledge_archiving
    description: 知识归档
    entry_actions:
      - 加载 C2 PATTERNS
    exit_actions:
      - 提取新模式到 C2（如有）
      - 提取新踩坑到 C1（如有）

# 分支条件
branches:
  - name: pure_frontend_feature
    description: 纯前端功能（无后端改动）
    when: PRD 标注 frontend_only=true
    states_override: [requirement_drafting, requirement_review, spec_generation, implementation, verification, knowledge_archiving]
    skip_steps:
      - 跳过 A4 数据模型加载
      - 跳过 A3 API 契约加载

  - name: pure_backend_feature
    description: 纯后端功能（无前端改动）
    when: PRD 标注 backend_only=true
    states_override: [requirement_drafting, requirement_review, spec_generation, implementation, verification, knowledge_archiving]
    skip_steps:
      - 跳过 B3 业务流程加载

  - name: data_model_change
    description: 涉及数据模型变更
    when: PRD 涉及新表或表结构变更
    add_steps:
      - 添加 A4 data_model 维度
      - 添加 D4 contract_sync 钩子

# 默认 prompt
prompts:
  opening: |
    你正在处理一个新功能需求。
    任务 ID: {task_id}
    任务标题: {task_title}

    第一步：阅读 PRD（lifecycle/01-planning/active-sprint/{task_id}.md）
    第二步：调用 enjoyflow_context 加载相关知识
    第三步：按状态机逐步推进

  per_state:
    - state: requirement_drafting
      prompt: |
        你在"编写 PRD"阶段。
        请基于用户输入，按 PRD 模板填空：
        - 基础信息（REQ-ID / Owner / Status）
        - 业务目标 + AC（验收标准）
        - 技术约束（涉及模块 / 数据库 / MQTT 等）
        - 数据结构
        - 交互流程

        注意：完成检查清单 5 项必须全部打勾才能进入下一阶段。

    - state: spec_generation
      prompt: |
        你在"规约生成"阶段。
        PRD 已批准（status=Approved）。
        请基于 PRD 生成技术规约：
        - API 契约（请求/响应/错误码）
        - 数据模型（表结构/关系/约束）
        - 状态流转
        - 枚举映射

        规约文件：lifecycle/02-design/specs/SPEC-{task_id}.md

    - state: implementation
      prompt: |
        你在"代码实现"阶段。
        SPEC 已生成。
        请按 SPEC 实现代码，注意：
        - 严格遵循 A2 代码规范
        - 检查 C1 已知踩坑，避免重复
        - 每个 AI 生成的函数头部标注 @ai-generated

    - state: verification
      prompt: |
        你是独立验证 AI（与实现 AI 不同会话）。
        请对实现进行四重对标：
        1. 规约对标：代码是否符合 SPEC
        2. 规范对标：代码是否遵循 A2
        3. 记忆对标：是否考虑了 B 业务规则 + C 踩坑
        4. 测试对标：是否有对应测试用例

  output_format: |
    每个状态结束后输出：
    - 状态：<状态名>
    - 产出：<文件路径>
    - 下一步：<下一状态>
---

#### 维度集详解（10 个维度）

| # | 维度 | 用途 |
|---|---|---|
| A1 | architecture | 项目整体架构 |
| A2 | code_standards | 后端/前端规范 |
| A6 | requirement | PRD 本身 |
| A7 | design_spec | 设计规约 |
| B1 | glossary | 业务术语 |
| B2 | business_rules | 业务规则 |
| C1 | gotchas | 已知踩坑 |
| C3 | decisions | 历史决策 |
| D1 | task_progress | 任务状态 |
| D6 | task_snapshot | 任务快照 |

#### 状态机详解（6 个状态）

```
requirement_drafting → requirement_review → spec_generation →
implementation → verification → knowledge_archiving
```

#### 适用场景

- 中-大型新功能
- 涉及前后端 + 数据库
- 需要完整 AC 锁定 + 规约 + 验证 + 归档

---

### 3.2 bug_fix（Bug 修复）

#### 完整定义

```yaml
# core/scenarios/bug_fix.yaml
---
name: bug_fix
description: 修复线上 Bug
version: 0.1.0

# 维度集
dimensions:
  - A1_architecture       # 项目架构（定位模块）
  - A2_code_standards     # 代码规范
  - A3_api_contract       # API 契约（如果是 API bug）
  - A4_data_model         # 数据模型（如果是 DB bug）
  - C1_gotchas            # 已知踩坑（避免旧坑）
  - C5_known_issues       # 已知问题（看是否相关）
  - D1_task_progress      # 任务状态
  - D7_failure_modes      # 失败模式

# 状态机（更短——5 个状态）
states:
  - name: bug_investigation
    description: Bug 调查
    entry_actions:
      - 加载 D7 失败模式
      - 加载 C5 已知问题
    exit_actions:
      - 记录调查结论
      - 标记 bug 类型（前端/后端/数据库）

  - name: root_cause_analysis
    description: 根因分析
    entry_actions:
      - 根据 bug 类型加载相关知识
    exit_actions:
      - 输出根因报告

  - name: fix_implementation
    description: 修复实现
    entry_actions:
      - 加载 C1 踩坑
    exit_actions:
      - 提交修复代码
      - Git commit 含 "fix(BUG-XXX):"

  - name: fix_verification
    description: 修复验证（独立会话）
    entry_actions:
      - 启动独立 AI 会话
      - 重现 bug 场景
    exit_actions:
      - 验证 bug 已修复
      - 验证无新 bug 引入

  - name: knowledge_recording
    description: 知识记录
    entry_actions:
      - 加载 C2 PATTERNS
    exit_actions:
      - 如果是新型 bug → 写入 C1 GOTCHAS
      - 如果是新模式 → 写入 C2 PATTERNS

# 分支条件
branches:
  - name: frontend_only_bug
    when: bug_type=frontend
    skip_steps:
      - 跳过 A4 data_model
      - 跳过 A3 api_contract
    add_dimensions:
      - A2_code_standards（前端部分）

  - name: data_related_bug
    when: bug_type=database
    add_dimensions:
      - A4_data_model
    add_steps:
      - 添加 SQL 验证步骤

  - name: performance_bug
    when: bug_type=performance
    add_dimensions:
      - C1_gotchas（性能相关）
    add_steps:
      - 添加 profiling 步骤

  - name: recurring_bug
    when: bug 已修复过类似问题
    add_steps:
      - 添加 D7 failure_modes 必查
      - 检查 C1 GOTCHAS 是否有相同 bug 类型

# 默认 prompt
prompts:
  opening: |
    你正在修复一个 bug。
    Bug ID: {bug_id}
    Bug 描述: {bug_description}

    第一步：调用 enjoyflow_context 检查 D7 失败模式
    第二步：检查 C5 已知问题
    第三步：进入 bug_investigation 状态

  per_state:
    - state: bug_investigation
      prompt: |
        你在"Bug 调查"阶段。
        请按以下步骤调查：
        1. 重现 bug（必须能稳定重现）
        2. 定位相关代码模块
        3. 检查 C1 GOTCHAS（是否踩过类似坑）
        4. 输出：bug 类型（前端/后端/DB/性能/其他）

    - state: root_cause_analysis
      prompt: |
        你在"根因分析"阶段。
        请基于调查输出根因报告：
        - 直接原因（哪段代码出错）
        - 深层原因（为什么会这样写）
        - 影响范围（哪些用户/场景受影响）

    - state: fix_implementation
      prompt: |
        你在"修复实现"阶段。
        请实现修复：
        - 最小改动原则（只修必要的）
        - 添加回归测试（防止再次出现）
        - Git commit: "fix(BUG-XXX): 简短描述"

    - state: fix_verification
      prompt: |
        你是独立验证 AI（与修复 AI 不同会话）。
        请验证：
        1. 重现 bug 场景——确认已修复
        2. 跑回归测试——确认无新 bug
        3. 检查边界条件

  output_format: |
    每个状态输出：
    - 状态：<状态名>
    - 产出：<文件路径>
    - 根因：<一句话>
---
```

#### 维度集详解（8 个维度）

| # | 维度 | 用途 |
|---|---|---|
| A1 | architecture | 定位模块 |
| A2 | code_standards | 代码规范 |
| A3 | api_contract | API bug 时必看 |
| A4 | data_model | DB bug 时必看 |
| C1 | gotchas | 避免旧坑 |
| C5 | known_issues | 看是否相关 |
| D1 | task_progress | 任务状态 |
| D7 | failure_modes | 失败模式 |

#### 状态机详解（5 个状态）

```
bug_investigation → root_cause_analysis → fix_implementation →
fix_verification → knowledge_recording
```

---

### 3.3 refactor（技术重构）

#### 完整定义

```yaml
# core/scenarios/refactor.yaml
---
name: refactor
description: 技术重构（影响多个模块）
version: 0.1.0

# 维度集
dimensions:
  - A1_architecture
  - A2_code_standards
  - A3_api_contract       # 重构可能涉及 API
  - A4_data_model         # 重构可能涉及 DB
  - C1_gotchas
  - C2_patterns           # 看现有模式
  - C3_decisions          # 历史决策不能违背
  - D1_task_progress

# 状态机
states:
  - name: impact_analysis
    description: 影响范围分析
    entry_actions:
      - 加载 A1 架构
      - 加载 A3 API 契约
    exit_actions:
      - 影响模块清单
      - 影响接口清单
      - 风险评估

  - name: refactor_design
    description: 重构设计
    entry_actions:
      - 加载 C2 现有模式
    exit_actions:
      - 重构方案文档
      - 渐进式迁移计划

  - name: refactor_implementation
    description: 重构实施（按模块）
    entry_actions:
      - 加载 A2 规范
      - 加载 C1 踩坑
    exit_actions:
      - 按模块分批提交
      - 保持向后兼容

  - name: regression_testing
    description: 回归测试
    entry_actions:
      - 跑全量回归
    exit_actions:
      - 回归测试报告

  - name: documentation_update
    description: 文档更新
    entry_actions:
      - 加载 A8 ADR 模板
    exit_actions:
      - ADR 文档（如涉及架构）
      - A1 ARCHITECTURE 更新
      - C2 PATTERNS 更新（如有新模式）

# 默认 prompt
prompts:
  opening: |
    你在进行技术重构。
    重构 ID: {refactor_id}
    重构范围: {scope}

    第一步：调用 enjoyflow_context 加载架构 + 决策
    第二步：进入 impact_analysis 状态

  per_state:
    - state: impact_analysis
      prompt: |
        你在"影响范围分析"阶段。
        请分析：
        1. 涉及哪些模块？（列出所有）
        2. 涉及哪些 API？（列出所有接口）
        3. 涉及哪些数据库表？（如有）
        4. 风险点（可能破坏什么）

    - state: refactor_design
      prompt: |
        你在"重构设计"阶段。
        请设计：
        1. 重构目标（解决什么问题）
        2. 重构方案（具体怎么改）
        3. 渐进式迁移计划（按模块分批）
        4. 回滚方案（如果失败怎么办）

    - state: refactor_implementation
      prompt: |
        你在"重构实施"阶段。
        注意：
        - 一次只改一个模块
        - 每个模块提交后立即验证
        - 保持向后兼容（旧接口仍可用）
        - 不破坏现有功能
---
```

#### 维度集详解（8 个维度）

| # | 维度 | 用途 |
|---|---|---|
| A1 | architecture | 整体架构 |
| A2 | code_standards | 重构规范 |
| A3 | api_contract | API 影响 |
| A4 | data_model | DB 影响 |
| C1 | gotchas | 避免旧坑 |
| C2 | patterns | 现有模式 |
| C3 | decisions | 历史决策 |
| D1 | task_progress | 任务状态 |

#### 状态机详解（5 个状态）

```
impact_analysis → refactor_design → refactor_implementation →
regression_testing → documentation_update
```

---

### 3.4 hotfix（紧急 hotfix）

#### 完整定义

```yaml
# core/scenarios/hotfix.yaml
---
name: hotfix
description: 紧急 hotfix（线上故障，生产环境）
version: 0.1.0

# 维度集（精简——只关注紧急）
dimensions:
  - A1_architecture       # 快速定位模块
  - A3_api_contract       # API 优先
  - C1_gotchas            # 已知坑
  - C5_known_issues       # 已知问题
  - D1_task_progress
  - D7_failure_modes      # 失败模式

# 状态机（极短——4 个状态，强调速度）
states:
  - name: emergency_diagnosis
    description: 紧急诊断（5 分钟内）
    entry_actions:
      - 加载 D7 失败模式
      - 加载 C5 已知问题
    exit_actions:
      - 紧急修复点定位

  - name: minimal_fix
    description: 最小修复
    entry_actions:
      - 加载 A1 架构
    exit_actions:
      - 最小改动修复
      - 立即部署（绕过常规流程）

  - name: production_verification
    description: 生产环境验证
    entry_actions:
      - 直接在生产验证
    exit_actions:
      - 验证报告

  - name: post_incident_review
    description: 事后复盘
    entry_actions:
      - 通知相关方
    exit_actions:
      - 复盘报告
      - 更新 C1 GOTCHAS
      - 更新 D7 failure_modes

# 分支条件
branches:
  - name: data_loss_critical
    when: 涉及数据丢失或损坏
    add_steps:
      - 立即停止部署
      - 启动数据回滚流程
      - 通知 DBA

  - name: security_related
    when: 安全漏洞
    add_steps:
      - 立即评估影响范围
      - 通知安全团队

  - name: traffic_spike
    when: 流量激增导致
    add_steps:
      - 启动扩容
      - 临时限流

# 默认 prompt
prompts:
  opening: |
    ⚠️ 紧急 hotfix ⚠️
    Bug ID: {bug_id}
    严重级别: {severity}

    跳过常规流程，最小修复 + 立即部署。

  per_state:
    - state: emergency_diagnosis
      prompt: |
        你在"紧急诊断"阶段。
        时间限制：5 分钟内定位。
        1. 看 D7 failure_modes（类似故障？）
        2. 看 C5 known_issues
        3. 直接看生产日志/监控
        输出：1 句话定位修复点

    - state: minimal_fix
      prompt: |
        你在"最小修复"阶段。
        原则：
        - 最小改动（不要顺手重构）
        - 优先回滚而不是修复（如适用）
        - 立即部署（绕过常规审批）
        - 记录所有改动

    - state: production_verification
      prompt: |
        你在"生产环境验证"阶段。
        1. 验证修复生效
        2. 监控指标恢复
        3. 准备回滚（如验证失败）

    - state: post_incident_review
      prompt: |
        你在"事后复盘"阶段。
        1. 根因分析
        2. 修复过程文档化
        3. 更新 C1 GOTCHAS（如果新型故障）
        4. 更新 D7 failure_modes
        5. 提出预防措施
---
```

#### 维度集详解（6 个维度——精简）

| # | 维度 | 用途 |
|---|---|---|
| A1 | architecture | 快速定位 |
| A3 | api_contract | API 优先 |
| C1 | gotchas | 已知坑 |
| C5 | known_issues | 已知问题 |
| D1 | task_progress | 任务状态 |
| D7 | failure_modes | 失败模式 |

#### 状态机详解（4 个状态——极短）

```
emergency_diagnosis → minimal_fix → production_verification → post_incident_review
```

#### 适用场景

- 生产环境故障
- 必须快速响应
- 跳过常规流程

---

### 3.5 architecture_decision（架构决策）

#### 完整定义

```yaml
# core/scenarios/architecture_decision.yaml
---
name: architecture_decision
description: 架构/技术选型决策
version: 0.1.0

# 维度集（强调决策历史）
dimensions:
  - A1_architecture       # 当前架构
  - A8_adr                # 已有 ADR
  - C3_decisions          # 历史决策
  - D3_decision_history   # 决策历史

# 状态机
states:
  - name: context_gathering
    description: 收集上下文
    entry_actions:
      - 加载 A1 当前架构
      - 加载 C3 历史决策
    exit_actions:
      - 决策背景文档

  - name: options_analysis
    description: 方案分析
    entry_actions:
      - 列出 3-5 个候选方案
    exit_actions:
      - 各方案优劣对比表
      - trade-off 分析

  - name: decision_making
    description: 决策
    entry_actions:
      - 人在决策点介入（强约束）
      - 列出决策理由
    exit_actions:
      - 决策结果
      - 影响范围评估

  - name: adr_documentation
    description: ADR 文档化
    entry_actions:
      - 加载 ADR 模板
    exit_actions:
      - ADR-XXX.md 文档
      - 包含：背景 / 决策 / 影响 / 替代方案

  - name: propagation
    description: 传播影响
    entry_actions:
      - 通知相关模块 Owner
    exit_actions:
      - 更新 A1 架构文档（如必要）
      - 更新 B 业务知识（如影响业务规则）

# 默认 prompt
prompts:
  opening: |
    你在进行架构决策。
    决策主题: {topic}

    重要：架构决策必须在规约阶段画框（DESIGN-PHILOSOPHY §1 #4），
    AI 不应单独做架构决策，必须有人介入。

  per_state:
    - state: context_gathering
      prompt: |
        你在"收集上下文"阶段。
        1. 阅读当前 A1 架构
        2. 阅读所有相关 C3 历史决策
        3. 输出决策背景

    - state: options_analysis
      prompt: |
        你在"方案分析"阶段。
        请列出 3-5 个候选方案，每个方案：
        - 优点
        - 缺点
        - 适用场景
        - 实施成本
        - 长期影响
        不要给"推荐"——决策权在人。

    - state: decision_making
      prompt: |
        ⚠️ 决策必须在人在场时做出 ⚠️
        AI 可以列举选项，但不可以替人决策。
        请：
        1. 列出所有选项（已在 options_analysis）
        2. 等待人决策
        3. 记录人的决策 + 理由

    - state: adr_documentation
      prompt: |
        你在"ADR 文档化"阶段。
        按 MADR 模板写 ADR：
        - 标题：ADR-XXX: <决策主题>
        - 状态：Proposed / Accepted / Deprecated
        - 上下文
        - 决策
        - 影响
        - 替代方案
---
```

#### 维度集详解（4 个维度——精简）

| # | 维度 | 用途 |
|---|---|---|
| A1 | architecture | 当前架构 |
| A8 | adr | 已有 ADR |
| C3 | decisions | 历史决策 |
| D3 | decision_history | 决策历史 |

#### 状态机详解（5 个状态）

```
context_gathering → options_analysis → decision_making → adr_documentation → propagation
```

#### 关键约束

**架构决策必须有人在场**（DESIGN-PHILOSOPHY §1 #4 "过度自信"防御）——AI 不可以单独决策。

---

## 4. 扩展场景（v1.0 后）

### 4.1 cross_stack（跨特性联调）

**维度集**：A3+A5+B2+D4
**状态机**：feature_a_setup → feature_b_setup → contract_negotiation → integration_testing → sync_to_knowledge_base
**关键**：跨特性契约对齐（D4）

### 4.2 project_handover（项目交接/新人入职）

**维度集**：全部 A/B/C + 关键 D
**状态机**：knowledge_transfer → shadow_mode → solo_practice → handover_complete
**关键**：1-2 周密集知识传递

### 4.3 perf_optimize（性能优化）

**维度集**：A1+A4+C1+C2+D7
**状态机**：profiling → bottleneck_analysis → optimization_plan → implementation → verification

### 4.4 regression_test（回归测试）

**维度集**：C4+C5+D1
**状态机**：scope_decide → execute → triage

### 4.5 small_task（小型调整）

**维度集**：A2+D1
**状态机**：execute → done
**特点**：1 个会话完成，无验证

### 4.6 requirement_change（需求变更）

**维度集**：A6+B2+D3
**状态机**：old_review → new_lock → propagation

---

## 5. 场景模板的可扩展性

### 5.1 三层来源

```
core/scenarios/            # 内置（最低优先级）
~/.enjoyflow/scenarios/    # 用户级（中等）
.enjoyflow/scenarios/      # 项目级（最高优先级）
```

### 5.2 用户自定义场景

```yaml
# .enjoyflow/scenarios/compliance_audit.yaml
---
name: compliance_audit
description: 合规审计场景
extends: new_feature           # 可继承其他预设
dimensions:
  - A1_architecture
  - B4_constraints
  - C6_team_convention
  - C7_review_checklist
states:
  - name: compliance_check
  - name: audit_report
---
```

---

## 6. 场景模板与 D6 工具的集成

### 6.1 AI 调用方式

```typescript
// 普通用户：直接用预设名
enjoyflow_context({
  task_id: "REQ-001",
  preset: "new_feature"
})

// 高级用户：覆盖维度
enjoyflow_context({
  task_id: "REQ-001",
  dimensions: ["A3", "A4", "C1"]  // 忽略 new_feature 的默认维度
})

// 高级用户：覆盖 prompt
enjoyflow_context({
  task_id: "REQ-001",
  preset: "new_feature",
  prompt_override: |
    你在第 2 阶段（规约生成），请直接输出 SPEC。
})
```

### 6.2 钩子集成

场景模板的每个状态可以挂载享受 Flow 钩子：

```yaml
hooks:
  on_state_enter:
    - state: implementation
      hook: pre-implementation   # 触发享受 Flow 钩子
    - state: verification
      hook: pre-verification     # 触发独立验证钩子
```

---

## 7. 5 个 MVP 场景对比

| 维度 | new_feature | bug_fix | refactor | hotfix | architecture_decision |
|---|---|---|---|---|---|
| 状态数 | 6 | 5 | 5 | 4 | 5 |
| 维度数 | 10 | 8 | 8 | 6 | 4 |
| 分支数 | 3 | 4 | 0 | 3 | 0 |
| 持续时间 | 长 | 中 | 长 | 极短 | 中 |
| 适用频率 | 每周 | 每天 | 每月 | 每月 | 每月 |
| 验证要求 | 独立会话 | 独立会话 | 回归测试 | 生产验证 | 人决策 |

---

## 8. 关键设计原则

| 原则 | 体现 |
|---|---|
| **状态机而非阶段** | 场景模板用状态机定义，比 v1 阶段更灵活 |
| **可变分支** | 同场景下按特征走不同子流程 |
| **人在决策点** | architecture_decision 必须有人介入 |
| **独立验证** | bug_fix / new_feature 强制独立会话验证 |
| **知识沉淀** | 每个场景最后都有 knowledge 阶段 |
| **可继承** | 用户场景可 extends 内置场景 |

---

## 9. 验证清单

- [ ] 5 个 MVP 场景模板已定义
- [ ] 每个模板有完整维度集 + 状态机 + prompt
- [ ] 模板支持可继承
- [ ] 模板支持钩子集成
- [ ] 模板支持项目级覆盖
- [ ] 模板有分支条件（按任务特征走不同流程）

---

*文档版本: v0.1-draft | 最后更新: 2026-06-20*
