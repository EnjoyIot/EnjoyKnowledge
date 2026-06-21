# ContextFlow 规范

> 版本 1.0 | 2026-06-21
>
> ContextFlow 是 EnjoyFlow 的核心知识流——AI 在执行任务时通过 `search` 获取上下文，通过 `record` 写回发现。
> 这不是一个独立引擎，而是 search → act → record 的闭环。

---

## 1. 模型

```
                    ┌──────────────┐
                    │  AI 编码工具   │
                    └──────┬───────┘
                           │
              ┌────────────┼────────────┐
              ▼            │            ▼
         search(query)     │     record(gotcha)
              │            │            │
              ▼            │            ▼
    ┌─────────────┐        │   ┌─────────────┐
    │ 知识库 + 外部源│       │   │ GOTCHAS.md   │
    │ (多源检索)    │       │   │ PATTERNS.md  │
    └─────────────┘        │   │ decisions/   │
                           │   └─────────────┘
                           │
                    AI 拿到上下文
                    写代码 / 修 bug
```

**一个完整循环**：

1. AI 识别任务 → `enjoyflow search "关键词" --class gotchas` 查踩坑
2. AI 读上下文 → 理解架构、规范、已知问题
3. AI 执行 → 写代码
4. AI 发现新坑 → `enjoyflow record gotcha --tag excel --content "..."` 写回
5. 下次别人搜 `excel` 就能看到这条新踩坑

---

## 2. search

```
enjoyflow search <query> [--class <label>] [--tag <tag>] [--archive]
```

返回段级摘要（文件路径 + 标题锚点 + 匹配行上下文），AI 自己判断读哪些完整文件。

精确合约见 [INTERFACE-SPEC.md §4](INTERFACE-SPEC.md)。

搜索源在 `config.yaml` 中配置——内置知识库、Obsidian vault、远程 git 仓库、MCP 知识库统一通过 Source Adapter 接口接入。

---

## 3. record

```
enjoyflow record gotcha --tag <tag> --content "<文本>"
enjoyflow record pattern --tag <tag> --content "<文本>"
enjoyflow record decision --task <REQ-ID> --content "<文本>"
```

只追加不修改。零审批——噪音成本低，漏记成本高。

精确合约见 [INTERFACE-SPEC.md §5](INTERFACE-SPEC.md)。

---

## 4. 与 AI 工具的协作

AI 工具通过 AGENTS.md 知道 EnjoyFlow 的存在：

```markdown
# AGENTS.md（EnjoyFlow 初始化的块）

当任务开始时：
1. 用 `enjoyflow search` 查相关知识
2. 读相关结果，理解上下文
3. 执行任务
4. 用 `enjoyflow record` 写回发现
```

AI 自主决定搜什么、读多少——EnjoyFlow 不替它做判断。

---

## 5. 与旧设计的区别

| 旧（D6-SNAPSHOT-SPEC v0.1） | 新（ContextFlow v1.0） |
|---|---|
| `enjoyflow_context` 工具（预设维度 + 自动聚合） | `enjoyflow search`（AI 主动查询） |
| 32 维度 + 8 预设场景 | 语义标签 + AI 自主组合 |
| 快照文件为核心产物 | search 直接输出为核心 |
| TypeScript schema 定义输入输出 | 文本流转，AI 原生可读 |

---

*文件原名: D6-SNAPSHOT-SPEC.md | 重命名: 2026-06-21*
