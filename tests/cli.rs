//! CLI 测试
//!
//! 两层测试：
//! 1. tests/integration.rs — \`assert_cmd` 集成测试（验证端到端行为）
//! 2. tests/cmd/*.trycmd — trycmd 快照测试（参考文档，CWD 配置待完善）

#[test]
fn cli_tests() {
    let t = trycmd::TestCases::new();
    t.register_bin("enjoyknowledge", std::path::Path::new(env!("CARGO_BIN_EXE_enjoyknowledge")));
    t.case("tests/cmd/*.trycmd");
}
