# v2 整合：3 机制协同设计（rule / 知识库命名 / 工作流）

**整合日期**：2026-06-27（同一会话第二轮）
**输入扩展**：用户加问"微信公众号 B 站文章里还有知识库文档名称 / rule / 工作流"——所以本轮不只 rule
**本轮新增 5 个问题**：
- Q1：知识库文档命名（naming convention）机制
- Q2：rule 与 B 站对比
- Q3：工作流机制
- Q4：**3 机制协同设计**（本轮重点）
- Q5：前轮产物的反思

**Codex v2 限制**：因 sandbox 错误（`codex-windows-sandbox-setup.exe` 缺失）**没读到任何项目文件**——Q5 反思是基于 prompt 推理的，不是基于实际项目状态。Q1-Q4 设计回答仍然有价值。
**Claude v2 正常**：完整读了项目，引用了具体行号（`DESIGN-V3.md:71` / `INTERFACE-SPEC.md:44` 等）。

---

## 🟢 v2 共识（3 机制协同 + 各自设计）

| # | 共识 | 关键证据 |
|---|---|---|
| **V2-C1** | **enjoyknowledge 当前没有工作流是设计选择（功能边界）**，非漏洞 | claude: "AGENTS.md 推送模式就是无 workflow" · codex: "先管好数据，再建消费层" |
| **V2-C2** | **rule 应当被 sync 到 8 个工具；knowledge 不应被 sync（AI 按需读）** | claude: 区分消费路径 · codex: 工作流只解析路径，不直接渲染 knowledge |
| **V2-C3** | **rule 和 knowledge 的本质区别 = 角色而非格式**（都是 markdown） | claude: "约束性 vs 描述性" · codex: "可操作 vs 可参考" |
| **V2-C4** | **"模板"（template）是 B 站第三层，享受独立的同步路径** | 两个都明确说"模板缺失是设计漏洞" |
| **V2-C5** | **协同优先级 = rule 先 → knowledge 后** | claude: "先知道边界，再在边界内发挥" · codex: "先约束，再上下文" |
| **V2-C6** | **路径即 ID**（enjoyknowledge 现状，B 站不该照搬） | claude: "天然带命名空间" · codex: "namespace by path" |

---

## 🟡 v2 设计细节分歧

### D2-1：rule vs knowledge 的判断标准

| 立场 | 主张 | 评价 |
|---|---|---|
| **claude** | "禁止/必须/不能/应当" → rules；"项目用 Rust + Axum" → knowledge | **实操可执行** ✅ |
| **codex** | "能否写自动化检查器验证" → rule；不能 → knowledge | **理想化**（很多软约束 rule 无法自动验证） |

**采纳 claude**——claude 的标准更实用，codex 的标准会产生"半 rule 半 knowledge"的中间地带，难分类。

### D2-2：模板机制的实现位置

| 立场 | 主张 |
|---|---|
| **claude** | "模板"概念已存在（`DESIGN-V3.md §9.1 init --template`），但那是**目录骨架**而非**代码模板**——和 B 站同名不同义。**如果加代码模板，应独立 `templates/` 目录 + 子命令** |
| **codex** | "`rules/` 旁边加 `templates/`，用类似的 SoT + managed section 机制" |

**两者一致**——都要加独立的 `templates/` 目录。codex 更激进（建议用同样的 SoT+managed section 机制），claude 更谨慎（先评估和现有 §9.1 的关系）。

### D2-3：路径即 ID 的"反模式"风险

| 立场 | 主张 |
|---|---|
| **claude** | "路径即 ID" 是 enjoyknowledge 的核心优势——天然 namespace 隔离 |
| **codex** | 加 `namespace` frontmatter 字段以支持跨项目引用 |

**两者兼容**——路径即 ID 是默认行为，`namespace` 字段是跨项目引用的扩展机制，不是替代。

---

## 🔴 v2 重大发现（必须告诉你）

### F1：项目里已存在"模板"概念但和 B 站同名不同义 ⚠️

**Claude 发现**：
- `docs/research/SCENARIO-TEMPLATES.md`（1498 行）= **场景工作流模板**（状态机+prompt）
- `docs/DESIGN-V3.md §9.1` "模板系统" = **目录骨架模板**（init --template 展开）
- B 站 `template/` = **代码模板**（"按规则写代码"的范式）

**3 个"模板"在不同文档里指 3 个不同东西**——读者困惑。建议重命名 `SCENARIO-TEMPLATES.md` → `SCENARIO-WORKFLOWS.md`。

### F2：项目文档的"协同设计"完全缺失 ⚠️

**Claude 直接指出**：
> "没有一份文档回答'rule / knowledge / workflow 三者怎么协同'。`unified-rule-management.md` 只讲 rule→sync→工具，`DESIGN-V3.md` 只讲 knowledge→索引→AI 消费，`SCENARIO-TEMPLATES.md` 只讲工作流状态机。三份文档各自完整，但没有人画那张'三个齿轮咬在一起'的图。"

### F3：3 处文档都讲命名但没统一 ⚠️

**Claude 发现**：
- `DESIGN-V3.md:71`（路径即 ID）
- `INTERFACE-SPEC.md:44`（文件名自解释）
- `rule-authoring-template.md:40-48`（rules 命名规则）

**3 处都在说命名，没有统一成一个"命名体系规范"**。

### F4：rule 设计有内部冲突 ⚠️

**Claude 发现**：
- `rule-authoring-template.md §2` 字段表里有 `priority` 字段
- `unified-rule-management.md` 说"不发明新抽象"

**`priority` 字段是否引入 = 内部矛盾**——需要统一。

### F5：项目代码现状实际没有 sync 命令 ⚠️

**Claude 提到 `docs/specs/rules-sync.md` 完整**——但**实际代码里只有 `init --ai <tool>` 没有 `rule sync --tool <tool>`**。也就是 codex 写的是"未来 spec"，不是"现在代码"。

---

## 📋 v2 整合后的设计（最终方案 v3）

```
<project_root>/
├── .enjoyknowledge/
│   ├── rules/                       # 约束性（同步到 8 个 AI 工具）
│   │   ├── coding-style.md          # 纯 markdown，frontmatter 可选
│   │   ├── rust-unwrap.md           # 带 frontmatter（appliesTo=rust）
│   │   └── archive/                 # 不被 sync 包含
│   ├── templates/                   # 范式性（同步到 AI 工具的 templates）
│   │   ├── rust-builder-pattern.md
│   │   └── archive/
│   └── knowledge/                   # 描述性（AI 按需读，不 sync）
│       ├── architecture/
│       │   └── overview.md
│       ├── api/
│       │   └── api-design.md
│       └── business/
│           └── modules.md
├── CLAUDE.md                        # 渲染产物：rules + templates（managed section）
├── AGENTS.md
├── .cursorrules
└── ...
```

### 3 机制的"消费路径"

| 机制 | 渲染/sync 目标 | AI 消费方式 |
|---|---|---|
| **Rule** | 8 个 AI 工具的配置文件 | AI 启动时**自动加载**（必读） |
| **Template** | 8 个 AI 工具的 templates/ | AI 按场景**按需拉取** |
| **Knowledge** | 不渲染，留在 `.enjoyknowledge/knowledge/` | AI 通过 `enjoyknowledge ls/grep/cat` **按需拉取** |

### 3 机制判断标准

| 内容 | 归类 | 判断方法 |
|---|---|---|
| "禁止用 unwrap" | Rule | 含禁止/必须/不能/应当 |
| "API 用 RESTful 分页" | Knowledge | 描述系统是什么样 |
| "Builder 模式骨架" | Template | 可复用的代码/文档骨架 |
| "项目用 Rust + Axum" | Knowledge | 描述性，无检查器 |
| "commit msg 用 Conventional Commits" | Rule | 含约束 |

### 协同三角

```
        Rule（约束层）
            ↓ 规定边界
        Template（范式层）
            ↓ 演示怎么用
        Knowledge（上下文层）
            ↓ 解释为什么用
```

工作流执行顺序：**先 rule（边界）→ 再 template（范式）→ 最后 knowledge（细节）**

---

## 🎯 v2 给你的 3 条具体行动建议

### 建议 1：补 `docs/architecture/workflow-and-naming.md`（覆盖 V2 共识 + 3 机制协同）

**理由**：Claude 直接指出"没有一份文档回答 3 机制协同"——这是当前最大缺口。

**内容**：
- 3 机制的本质区别（约束性 / 范式性 / 描述性）
- 判断标准（"禁止/必须/应当" → rule）
- 协同三角图
- 路径即 ID 规范（统一 3 处命名分散）

**工作量**：1-2 小时

### 建议 2：解决"模板"概念同名不同义问题

**动作**：
- `docs/research/SCENARIO-TEMPLATES.md` → 重命名为 `docs/research/SCENARIO-WORKFLOWS.md`
- `docs/DESIGN-V3.md §9.1` 标题加前缀"目录骨架模板"或"骨架模板系统"
- 新增 `docs/templates/SPEC.md`（如果决定加 B 站风格的代码模板）

**理由**：3 个"模板"在不同文档里指 3 个不同东西——读者困惑。

**工作量**：30 分钟

### 建议 3：决定 `priority` 字段去留（解决 F4 内部冲突）

**选项**：
- A：采纳 `priority`（claude 主张），更新 `unified-rule-management.md` 把 "不发明新抽象" 改成 "不发明 priority 以外的抽象"
- B：删除 `priority`（codex 主张），更新 `rule-authoring-template.md` 删除该字段

**理由**：规则系统不应有内部矛盾。

**工作量**：15 分钟

---

## ⚠️ v2 关键风险（必须告诉你的）

### R1：codex v2 没读到任何项目文件

Codex 开头坦白："All filesystem access is blocked by a broken sandbox... I'll answer from the AGENTS.md context you provided."

**影响**：Q5 反思部分是基于 prompt 推理的，不是基于实际项目状态。Q1-Q4 设计回答仍有价值，但 Q5 不可信。

**修复建议**：下次跑 codex 用 `--sandbox workspace-write` 替代 `--sandbox read-only`（如果 read-only 触发 sandbox bug）。

### R2：claude 引用了"未来 spec"作为现状

Claude 说"`docs/specs/rules-sync.md` 完整"——但**实际代码里没有 `rule sync` 命令**，只有 `init --ai`。

**影响**：设计文档（spec）和代码实现（current code）之间有 gap。这是 enjoyknowledge 项目的状态，不要当成"已实现"。

### R3：F4 `priority` 字段冲突可能在 commit 时才暴露

`rule-authoring-template.md` 和 `unified-rule-management.md` 在同一仓库——可能让 v0.1.0 release 时出现 spec 不一致。

---

## 📂 v2 完整记录

- **整合文档**：`C:\Users\jay\Documents\why-workspace\daily\2026-06-27-enjoyknowledge-rule-design-integration.md`（v3 方案 = v1 rule 整合 + v2 3 机制协同）
- **Codex v2 log**：`C:\Users\jay\AppData\Local\Temp\ek-codex-v2-output.log`（489 行，含 sandbox 错误记录）
- **Claude v2 log**：`C:\Users\jay\AppData\Local\Temp\ek-claude-v2-output.log`（108 行，干净）

---

## 🔗 下一步（按 C3 不替你决策）

1. **执行 3 条行动建议**（补 workflow-and-naming.md / 重命名 SCENARIO-TEMPLATES / 解决 priority 冲突）——告诉我执行哪个
2. **验证 R1 风险**——重跑 codex（用 `--sandbox workspace-write` 替代 read-only）确认它能读文件
3. **推动 v3 方案实现**——写最小 PoC（`.enjoyknowledge/rules/coding-style.md` + `enjoyknowledge rule sync --tool claude` mock）
4. **更新 memory**——把今天的关键设计判断沉淀到长期记忆（"3 机制协同三角" / "路径即 ID" / "priority 字段冲突"）
5. **别的方向**


---

