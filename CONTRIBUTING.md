# 贡献指南

感谢你对 enjoyknowledge 的关注！

## 快速开始

```bash
git clone https://github.com/enjoyknowledge/enjoyknowledge
cd enjoyknowledge
cargo build
cargo test
```

## 开发工作流

1. **Fork** 本仓库
2. 创建特性分支: `git checkout -b feature/my-feature`
3. 提交变更: `git commit -m "feat: add my feature"`
4. 推到你的 fork: `git push origin feature/my-feature`
5. 创建 Pull Request

## Commit 规范

遵循 [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` 新功能
- `fix:` 修复 bug
- `docs:` 文档变更
- `refactor:` 重构
- `test:` 测试
- `chore:` 工程配置

## 代码风格

- `cargo fmt` 格式化代码
- `cargo clippy -- -D warnings` 检查 lint
- `cargo test` 确保测试通过

## 项目结构

```
src/
├── main.rs          # CLI 入口
├── config.rs        # 配置解析
├── cli/             # 命令处理 (init/search/record/doctor)
├── knowledge/       # 知识源适配器 + 索引
├── format/          # Markdown/frontmatter 解析
├── init/            # init 命令实现
├── doctor/          # doctor/fix 命令实现
└── record/          # record 路由
```

## 设计文档

设计决策见 `docs/` 目录。
