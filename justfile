# EnjoyFlow 开发任务快捷命令
# 需要 just: cargo install just

default:
    @just --list

# 编译
build:
    cargo build

# 编译 (release)
build-release:
    cargo build --release

# 运行检查
check:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-features

# 格式化
fmt:
    cargo fmt --all

# 测试
test:
    cargo test

# 测试 (含所有 features)
test-all:
    cargo test --all-features

# 运行 clippy
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# 快速修复
fix:
    cargo clippy --fix --allow-dirty
    cargo fmt --all

# 清理构建产物
clean:
    cargo clean

# 初始化 demo 项目
demo-init:
    cargo run -- init ./demo-project

# 搜索 demo
demo-search:
    cargo run -- search "test" --class gotchas

# 完整 CI 检查
ci: check build-release
