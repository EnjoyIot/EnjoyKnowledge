# v0.4.9 完工复盘 — enjoyknowledge 7 个硬编码全部抽离到 fixture 文件

**日期**：2026-06-29
**版本**：v0.4.9

## 做了什么

把 v0.4.7/v0.4.8 硬编码在 `src/init/skeleton.rs` 的 7 个 MD_CONTENT 全部抽离到 `tests/fixtures/` 目录，用 `include_str!` 编译期嵌入。0 行为改动，100% 兼容 v0.4.8。

## 动机

R1 v0.4.7/v0.4.8 把 5+1+1=7 个 MD_CONTENT 硬编码在 .rs 文件 = **用户改不动** = **违反 v0.4 哲学（让用户能改）**。v0.4.4 stage-defaults.md 已经用了 `include_str!` 模式，v0.4.7/v0.4.8 退步了。v0.4.9 修正。

## 改动清单

| 文件 | 改动 |
|------|------|
| `src/init/skeleton.rs` | 7 个 MD_CONTENT 硬编码 → `include_str!`；`ek_agents_md_content()` 函数 → const |
| `tests/fixtures/skills/coding.md` | 新建（SKILLS_CODING_MD_CONTENT） |
| `tests/fixtures/skills/research.md` | 新建（SKILLS_RESEARCH_MD_CONTENT） |
| `tests/fixtures/skills/review.md` | 新建（SKILLS_REVIEW_MD_CONTENT） |
| `tests/fixtures/skills/design.md` | 新建（SKILLS_DESIGN_MD_CONTENT） |
| `tests/fixtures/skills/README.md` | 新建（SKILLS_README_MD_CONTENT） |
| `tests/fixtures/agents/stage.md` | 新建（STAGE_AGENTS_MD_CONTENT） |
| `tests/fixtures/agents/ek.md` | 新建（EK_AGENTS_MD_CONTENT） |

## 抽离方式

```rust
// 抽离前（v0.4.8）
const SKILLS_CODING_MD_CONTENT: &str = r#"---
name: enjoyknowledge-flow-coding
...
"#;

// 抽离后（v0.4.9）
const SKILLS_CODING_MD_CONTENT: &str = include_str!("../../tests/fixtures/skills/coding.md");
```

跟 v0.4.4/v0.4.5 模式 100% 复用：编译期嵌入 = 0 运行时影响。

## 测试

- **2 个新测试**：`v0_4_9_md_content_uses_include_str` + `v0_4_9_user_can_modify_md_content_via_fixture`
- 全量：108 单 + 25 trycmd + 34 集成 = 167 全部通过

## 验收标准

✅ 7 个 MD_CONTENT 全部用 `include_str!` 替换硬编码
✅ 7 个 fixture .md 文件创建
✅ 编译期嵌入 = 0 运行时影响
✅ 行为 100% 不变（0 行为改动）
✅ cargo test 全绿（167 通过）
✅ cargo fmt + clippy 通过
✅ 真实 dogfooding：ek init → ek doctor → [OK] all checks passed
✅ 用户改 fixture 后重新编译 → 常量内容跟着改

## 教训

- **v0.4.4 模式对了** = 用 `include_str!` 抽离到 fixture
- **v0.4.7/v0.4.8 退步了** = 硬编码在 .rs 忘记用 v0.4.4 模式
- **v0.4 哲学** = 让用户能改 = 0 硬编码 = 全部 `include_str!`

## v0.5 展望

- v0.4.X 系列：7 个硬编码全部抽离完毕，v0.4 哲学闭环
- v0.5：考虑用户自定义 fixture 路径、动态加载等
