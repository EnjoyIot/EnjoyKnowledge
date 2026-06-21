/// L1 + L2 去重算法
///
/// L1: 按 tag 交集过滤（tag 无交集的条目不可能重复）
/// L2: LLM 语义比对（需要 llm feature）
use std::path::Path;

/// 去重结果
#[derive(Debug)]
pub struct DedupResult {
    /// 保留的条目
    pub keep: Vec<String>,
    /// 建议删除的条目（重复）
    pub remove: Vec<String>,
}

/// 执行去重
pub fn deduplicate(_root: &Path, _files: &[String]) -> anyhow::Result<Vec<DedupResult>> {
    // TODO: L1 tag 交集过滤 + L2 语义比对
    Ok(Vec::new())
}
