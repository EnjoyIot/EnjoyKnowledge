# EnjoyFlow 文档索引

## 快速导航

| 你想知道 | 读这篇 |
|---|---|
| EnjoyFlow 是什么、为什么存在 | [DESIGN-PHILOSOPHY.md](DESIGN-PHILOSOPHY.md) |
| EnjoyFlow 在生态中的位置、跟竞品的区别 | [POSITIONING.md](POSITIONING.md) |
| EnjoyFlow 的用户体验、命令、端到端流程 | [PRODUCT-DESIGN.md](PRODUCT-DESIGN.md) |
| EnjoyFlow 内部怎么运作 | [SYSTEM-DESIGN.md](SYSTEM-DESIGN.md) ⏳ |
| ContextFlow 引擎的精确规格 | [CONTEXTFLOW-SPEC.md](CONTEXTFLOW-SPEC.md) ⚠️ |
| 知识怎么分类、存储、检索 | [KNOWLEDGE-ARCHITECTURE.md](KNOWLEDGE-ARCHITECTURE.md) |
| 8 个场景模板的完整定义 | [SCENARIO-TEMPLATES.md](SCENARIO-TEMPLATES.md) |
| 第三方怎么对接 EnjoyFlow | [INTERFACE-SPEC.md](INTERFACE-SPEC.md) ⚠️ |
| 统一术语定义 | [GLOSSARY.md](GLOSSARY.md) ⏳ |
| 市场数据和研究证据 | [MARKET-RESEARCH-2026Q2.md](MARKET-RESEARCH-2026Q2.md) |

---

## 阅读顺序

```
 0. GLOSSARY.md               ← 术语参考，随时查阅，不按顺序读

 1. DESIGN-PHILOSOPHY.md      ← 起点：为什么存在、设计原则
          │
 2. POSITIONING.md            ← 市场定位和竞争策略
          │
 3. PRODUCT-DESIGN.md         ← 用户看到什么、怎么用
          │
 4. SYSTEM-DESIGN.md          ← 内部架构全景
          │
    ┌─────┼─────┐
    ▼     ▼     ▼
 5a. KNOWLEDGE-  5b. SCENARIO-  5c. CONTEXTFLOW-
   ARCHITECTURE   TEMPLATES      SPEC.md ⚠️
          │
          ▼
 6. INTERFACE-SPEC.md         ← 外部合约 ⚠️
```

**独立阅读：** MARKET-RESEARCH-*.md 是市场证据快照，随时可读。GLOSSARY.md 是术语参考，所有文档共享。

---

## 文档职责边界

### 根基层（为什么）

| 文档 | 回答的问题 | 不负责 |
|---|---|---|
| **DESIGN-PHILOSOPHY.md** | EnjoyFlow 为什么必须存在？所有设计决策遵循什么原则？ | 不定义具体结构、不写产品功能 |
| **POSITIONING.md** | EnjoyFlow 在生态中占什么位置？跟竞品什么关系？ | 不推导设计原则、不写实现细节 |

### 设计层（怎么做）

| 文档 | 回答的问题 | 不负责 |
|---|---|---|
| **PRODUCT-DESIGN.md** | 用户怎么安装和使用？命令有哪些？端到端体验？AI 工具怎么集成？ | 不写内部实现、不写接口合约 |
| **SYSTEM-DESIGN.md** | 四通道怎么运作？子系统间怎么协作？架构全貌？ | 不定义外部 API、不写产品功能 |

### 规格层（精确定义）

| 文档 | 回答的问题 | 不负责 |
|---|---|---|
| **KNOWLEDGE-ARCHITECTURE.md** | 知识怎么分类？32 子类是什么？目录怎么组织？ | 不定义外部 API、不写场景流程 |
| **SCENARIO-TEMPLATES.md** | 每个场景的状态机是什么？拉取哪些知识维度？ | 不定义知识分类法、不写产品命令 |
| **CONTEXTFLOW-SPEC.md** ⚠️ | `enjoyflow_context` 的参数/返回值/匹配算法/扩展点？ | 不写总体架构、不写场景流程 |

### 接口层（合约）

| 文档 | 回答的问题 | 不负责 |
|---|---|---|
| **INTERFACE-SPEC.md** ⚠️ | 第三方实现 EnjoyFlow 兼容工具的最小接口合约是什么？ | 不写内部实现、不写产品体验 |

### 参考层

| 文档 | 回答的问题 | 不负责 |
|---|---|---|
| **GLOSSARY.md** ⏳ | ContextFlow、四通道、三层合约、L1/L2/L3、ABCD 等术语的精确定义？ | 不做推论、不写流程 |
| **MARKET-RESEARCH-*.md** | 竞品有哪些？市场格局？用户痛点？ | 不做产品决策 |

---

## 文档层级关系

```
DESIGN-PHILOSOPHY  ← 根基。所有文档应能从此推导
    │
POSITIONING        ← 从哲学推导的市场策略
    │
PRODUCT-DESIGN     ← 从哲学推导的产品体验
    │
SYSTEM-DESIGN      ← 从产品设计推导的系统架构
    │
    ├── KNOWLEDGE-ARCHITECTURE  ← 从系统设计展开的知识层
    ├── SCENARIO-TEMPLATES      ← 从系统设计展开的流程层
    └── CONTEXTFLOW-SPEC        ← 从系统设计展开的引擎规格
    │
INTERFACE-SPEC     ← 从系统设计提取的外部合约

GLOSSARY           ← 横向贯穿，所有文档的术语锚点
MARKET-RESEARCH    ← 证据层，独立于推导链
```

---

## 当前状态

| 文档 | 状态 | 备注 |
|---|---|---|
| GLOSSARY.md | ✅ 完成 | 统一术语定义 |
| DESIGN-PHILOSOPHY.md | ✅ 完成 | 五原则经过审视和修订 |
| POSITIONING.md | ✅ 更新 | 命名已对齐新哲学 |
| PRODUCT-DESIGN.md | ✅ 完成 | 6 个产品设计问题已讨论定稿 |
| SYSTEM-DESIGN.md | ✅ 完成 | CLI 架构、search/record 管线、源抽象、doctor 机制 |
| CONTEXTFLOW-SPEC.md | ✅ 重写 | 对齐 search+record 模型 |
| KNOWLEDGE-ARCHITECTURE.md | ✅ 更新 | A7 冲突已解决，目录结构已对齐 |
| SCENARIO-TEMPLATES.md | ✅ 更新 | 命名已对齐 |
| INTERFACE-SPEC.md | ✅ 重写 | 缩减为第三方开发者可编码的规格 |
| MARKET-RESEARCH-*.md | ✅ 历史快照 | 不做修改 |

---

## 已识别的冲突

| 冲突 | 涉及文档 | 状态 |
|---|---|---|
| A7 定义不一致 | DESIGN-PHILOSOPHY + KNOWLEDGE-ARCHITECTURE | ✅ A7 = UI/UX Design，已统一 |
| INTERFACE-SPEC 混了内部设计和外部合约 | INTERFACE-SPEC | ✅ 已重写，纯外部合约 |
| CONTEXTFLOW-SPEC 内容过时 | CONTEXTFLOW-SPEC | ✅ 已重写，对齐 search+record |
| KNOWLEDGE-ARCHITECTURE 目录结构过时 | KNOWLEDGE-ARCHITECTURE | ✅ 已改为 .enjoyflow/knowledge-base/ |
