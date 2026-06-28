# 知识类型设计

> v0.4.2 | 2026-06-28
>
> 定义 10 类知识资产 + 判断标准 + 必填字段。

---

## 1. 10 类知识资产

| # | kind | 目录 | 本质 | 必填 frontmatter |
|---|---|---|---|---|
| 1 | gotcha | gotchas/ | 踩坑记录 | `trigger` |
| 2 | decision | decisions/ | 架构决策 | `reversible` + `decided_at` |
| 3 | pattern | patterns/ | 最佳实践 | — |
| 4 | rule | rules/ | 强制规则 | `applies_to` |
| 5 | business | business/ | 业务规则 | — |
| 6 | architecture | architecture/ | 系统结构 | — |
| 7 | contract | contracts/ | 接口契约 | `applies_to` |
| 8 | convention | conventions/ | 命名/格式约定 | `applies_to` |
| 9 | context | context/ | 项目背景/运行时 | — |
| 10 | template | templates/ | 范式模板 | `applies_to` |

---

## 2. 判断标准

**关键问题**："这条知识 AI 在什么时候消费？"

| 消费时机 | kind |
|---|---|
| AI 启动时自动加载 | rule + template |
| AI 写代码时按 glob 触发 | rule（带 `applies_to`） |
| AI 写代码时按"如果...那么..."触发 | gotcha（带 `trigger`） |
| AI 写代码时需要"本项目怎么写" | pattern |
| AI 写代码前需要"为什么这么设计" | decision + architecture |
| AI 写代码前需要"环境是什么" | context |
| AI 改代码时需要"哪些字段要同步" | contract |
| AI 改代码时需要"命名/格式规范" | convention |

**判断流程**：
1. 含"禁止/必须/不能/应当"？→ rule
2. 是"如果 X 触发，则 Y 解决"？→ gotcha（必含 trigger）
3. 是"我们做 X 因为 Y"？→ decision（必含 reversible）
4. 是项目模块结构？→ architecture
5. 是 API 跨模块约束？→ contract
6. 是命名/目录/commit 格式？→ convention
7. 是 env 变量/端口/服务依赖？→ context
8. 是可复用的"本项目特有"模式？→ pattern
9. 是业务领域规则？→ business
10. 是可复用代码/文档骨架？→ template

---

## 3. 必填字段

```yaml
# gotcha（必含 trigger）
id: useeffect-deps-must-be-stable
trigger: "use useEffect"
tags: [react, hooks]

# decision（必含 reversible + decided_at）
id: postgresql-over-mysql
reversible: false
decided_at: 2026-04-15

# rule / contract / convention / template（必含 applies_to）
id: rust-no-unwrap
applies_to: ["*.rs"]
tags: [rust, error-handling]

# architecture / pattern / business / context（无必填字段）
id: api-layer-axum-tower
tags: [api, backend]
```

---

## 4. 体积约束

| 约束 | 阈值 | 处理 |
|---|---|---|
| 单条长度 | > 100 词 | warning |
| 总 SoT 长度 | > 4000 词 | warning + 建议归档 |
| 单文件条目数 | > 20 条 | doctor 建议拆分 |

---

## 5. 命名约定

**文件名 = kebab-case slug**：
- `rust-no-unwrap.md`
- `api-design.md`

**目录即分类**（无嵌套子目录）：
- `knowledge/architecture/overview.md`

---

## 6. 强制 / 禁止

**强制**：
- gotcha 必含 `trigger` 字段（缺它 doctor 报错）
- decision 必含 `reversible` + `decided_at`
- rule / contract / convention / template 必含 `applies_to`
- 体积上限 100 词单条 / 4000 词总

**禁止**：
- 在 gotchas/ 放"注意性能"类空洞条目
- 在 rules/ 写代码风格（留给 formatter/linter）

---

*关联文档：[rule-system.md](./rule-system.md) · [workflows.md](./workflows.md) · [for-coding-design.md](./for-coding-design.md) · [GLOSSARY.md](../../01-philosophy/GLOSSARY.md)*
