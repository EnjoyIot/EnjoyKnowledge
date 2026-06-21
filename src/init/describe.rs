/// --describe 功能：自然语言描述 → LLM 生成初始 proposal
///
/// 需要 `llm` feature flag 启用。
#[cfg(feature = "llm")]
pub async fn describe_to_proposal(
    _description: &str,
    _root: &std::path::Path,
) -> anyhow::Result<String> {
    // TODO: 调用 LLM API 生成 proposal
    anyhow::bail!("LLM describe 功能尚未实现")
}

#[cfg(not(feature = "llm"))]
pub fn describe_to_proposal(
    _description: &str,
    _root: &std::path::Path,
) -> anyhow::Result<String> {
    anyhow::bail!("需要启用 'llm' feature 才能使用 --describe 功能。请用 `cargo build --features llm` 编译。")
}
