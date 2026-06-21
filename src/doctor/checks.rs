/// doctor 诊断的 5 项检查
use std::path::Path;

/// 单个检查结果
#[derive(Debug)]
pub struct CheckResult {
    pub file: String,
    pub issue: String,
    pub severity: Severity,
}

#[derive(Debug)]
pub enum Severity {
    Error,
    Warning,
}

/// 检查 1: 缺 class 字段
pub fn check_missing_class(root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    // TODO: 遍历所有 .md 文件，检查 frontmatter 是否有 class 字段
    let _ = root;
    results
}

/// 检查 2: 缺 tags 字段
pub fn check_missing_tags(root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let _ = root;
    results
}

/// 检查 3: 文件超出预算
pub fn check_budget(root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let _ = root;
    results
}

/// 检查 4: 疑似重复条目
pub fn check_duplicates(root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let _ = root;
    results
}

/// 检查 5: 索引与文件不一致
pub fn check_index_consistency(root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let _ = root;
    results
}
