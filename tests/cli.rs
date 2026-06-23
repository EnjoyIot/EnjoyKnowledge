//! CLI snapshot tests using trycmd.
//!
//! Tests run from the fixture directory `tests/fixtures/minimal-project/`
//! so that commands like `grep` have knowledge files to search.

#[test]
fn cli_tests() {
    let t = trycmd::TestCases::new();
    t.register_bin("enjoyknowledge", std::path::Path::new(env!("CARGO_BIN_EXE_enjoyknowledge")));
    // Run all trycmd tests from the fixture workspace.
    t.case("tests/cmd/*.trycmd");
}
