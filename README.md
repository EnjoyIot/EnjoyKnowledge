# EnjoyFlow

<p align="center">
  <strong>工程团队的人机共享任务上下文层</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/enjoyflow"><img src="https://img.shields.io/crates/v/enjoyflow" alt="Crates.io"></a>
  <a href="https://docs.rs/enjoyflow"><img src="https://img.shields.io/docsrs/enjoyflow" alt="Docs"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
</p>

EnjoyFlow 是一套 **CLI 工具 + 知识库结构 + 接口规范**，让工程团队的开发者与 AI 编码工具（Cursor / Copilot / Claude Code）共享同一份开发上下文知识。当 AI 写代码时，它知道你的架构、规范、踩坑、业务规则和当前任务进度。

## 一句话定位

> EnjoyFlow 位于 AI 编码工具和已有知识库之间——**人和 AI 用同一份知识干活。**

## 快速开始

```bash
# 安装
cargo install enjoyflow

# 在当前项目初始化
enjoyflow init

# 搜索知识
enjoyflow search "导出" --class gotchas --tag excel

# 记录踩坑
enjoyflow record gotcha --tag excel --content "t_export_record 表无 status 字段"

# 诊断知识库健康度
enjoyflow doctor
```

## 核心理念

| 问题 | EnjoyFlow 的答案 |
|---|---|
| AI 不知道项目架构 | `enjoyflow search` 提供架构上下文 |
| AI 重复踩已知坑 | `enjoyflow record gotcha` 沉淀踩坑，`search` 检索 |
| 换 AI 工具知识丢失 | 知识在文件系统，工具无关 |
| 团队知识孤岛 | `knowledge-base/` 进 git，团队共享 |

## 架构

```
AI 编码工具 (Cursor/Copilot/Claude Code)
         │
         ▼
    AGENTS.md  ← "想搜架构 → enjoyflow search ..."
         │
         ▼
┌─ enjoyflow CLI ────────────────────┐
│  init │ search │ record │ doctor   │
└────────────────────────────────────┘
         │
         ▼
    .enjoyflow/
    ├── knowledge-base/   ← 静态基础层
    ├── knowledge-tasks/  ← 动态任务层
    ├── config.yaml
    └── .index.json
```

## 文档

| 文档 | 内容 |
|---|---|
| [SYSTEM-DESIGN.md](docs/SYSTEM-DESIGN.md) | 系统架构、模块划分、数据流 |
| [INTERFACE-SPEC.md](docs/INTERFACE-SPEC.md) | CLI 合约、输出格式、兼容性级别 |
| [KNOWLEDGE-ARCHITECTURE.md](docs/KNOWLEDGE-ARCHITECTURE.md) | 32 类知识分类、目录结构 |
| [POSITIONING.md](docs/POSITIONING.md) | 定位宣言、竞品对比 |
| [PRODUCT-DESIGN.md](docs/PRODUCT-DESIGN.md) | 用户视角的产品设计 |

## 贡献

欢迎贡献！详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 许可

MIT © EnjoyFlow Team
