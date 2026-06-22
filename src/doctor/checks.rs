/// doctor 诊断的 5 项检查
use crate::doctor::budget;
use crate::knowledge::index::Index;
use std::collections::HashSet;
use std::path::Path;

/// 单个检查结果
#[derive(Debug)]
pub struct CheckResult {
    pub file: String,
    pub issue: String,
    pub severity: Severity,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

/// 运行全部 5 项检查
#[allow(clippy::unnecessary_wraps)]
pub fn run_all(root: &Path) -> anyhow::Result<Vec<CheckResult>> {
    let mut results = Vec::new();
    results.extend(check_missing_class(root));
    results.extend(check_missing_tags(root));
    results.extend(check_budget(root));
    results.extend(check_duplicates(root));
    results.extend(check_index_consistency(root));
    Ok(results)
}

/// 收集 .enjoyknowledge/ 下所有 .md 文件路径（相对路径）
fn collect_md_files(root: &Path) -> Vec<String> {
    let base = root.join(".enjoyknowledge");
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(&base)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        if let Ok(rel) = entry.path().strip_prefix(&base) {
            files.push(rel.to_string_lossy().to_string());
        }
    }
    files
}

/// 检查 1: 缺 class 字段
pub fn check_missing_class(root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let base = root.join(".enjoyknowledge");
    for rel in collect_md_files(root) {
        let path = base.join(&rel);
        if let Ok(content) = std::fs::read_to_string(&path) {
            match crate::format::frontmatter::parse_frontmatter(&content) {
                None => results.push(CheckResult {
                    file: rel,
                    issue: "缺少 YAML frontmatter".into(),
                    severity: Severity::Error,
                }),
                Some(fm) if fm.class.is_none() => results.push(CheckResult {
                    file: rel,
                    issue: "frontmatter 缺少 class 字段".into(),
                    severity: Severity::Error,
                }),
                _ => {}
            }
        }
    }
    results
}

/// 检查 2: 缺 tags 或 tags 为空
pub fn check_missing_tags(root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let base = root.join(".enjoyknowledge");
    for rel in collect_md_files(root) {
        let path = base.join(&rel);
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Some(fm) = crate::format::frontmatter::parse_frontmatter(&content) {
                if fm.tags.is_empty() {
                    results.push(CheckResult {
                        file: rel,
                        issue: "frontmatter 中 tags 为空（需要至少 1 个 tag）".into(),
                        severity: Severity::Error,
                    });
                }
            }
        }
    }
    results
}

/// 检查 3: 文件超出预算
pub fn check_budget(root: &Path) -> Vec<CheckResult> {
    budget::check_budget(root)
        .into_iter()
        .map(|e| CheckResult {
            file: e.path,
            issue: format!("超出预算：{} 行 > {} 行上限", e.lines, e.max),
            severity: Severity::Warning,
        })
        .collect()
}

/// 检查 4: 疑似重复条目（L1 tag 交集过滤）
pub fn check_duplicates(root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let base = root.join(".enjoyknowledge");
    let gotchas_path = base.join("knowledge-base/development/GOTCHAS.md");

    if let Ok(content) = std::fs::read_to_string(&gotchas_path) {
        let entries: Vec<&str> = content.lines().filter(|l| l.starts_with("- ")).collect();

        // 按首 tag 分组（粗粒度 L1 过滤）
        let mut by_tag: std::collections::HashMap<String, Vec<(usize, &str)>> =
            std::collections::HashMap::new();
        for (i, entry) in entries.iter().enumerate() {
            let tag = entry
                .split(':')
                .next()
                .map(|t| t.trim_start_matches("- ").trim().to_lowercase())
                .unwrap_or_default();
            by_tag.entry(tag).or_default().push((i, entry));
        }

        for group in by_tag.values() {
            if group.len() < 2 {
                continue;
            }
            // 简单的词重叠相似度检测
            for i in 0..group.len() {
                for j in (i + 1)..group.len() {
                    let words_a: HashSet<&str> = group[i].1.split_whitespace().collect();
                    let words_b: HashSet<&str> = group[j].1.split_whitespace().collect();
                    let intersection = words_a.intersection(&words_b).count();
                    let union = words_a.union(&words_b).count();
                    if union > 0 {
                        #[allow(clippy::cast_precision_loss)]
                        let similarity = intersection as f64 / union as f64;
                        if similarity > 0.7 {
                            results.push(CheckResult {
                                file: "knowledge-base/development/GOTCHAS.md".into(),
                                issue: format!(
                                    "疑似重复条目（相似度 {:.0}%）：\n  行 {}: {}\n  行 {}: {}",
                                    similarity * 100.0,
                                    group[i].0 + 1,
                                    group[i].1,
                                    group[j].0 + 1,
                                    group[j].1,
                                ),
                                severity: Severity::Warning,
                            });
                        }
                    }
                }
            }
        }
    }
    results
}

/// 检查 5: 索引与文件系统不一致
pub fn check_index_consistency(root: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();
    let fs_files: HashSet<String> = collect_md_files(root).into_iter().collect();

    if let Ok(Some(index)) = Index::load(root) {
        let mut indexed_files = HashSet::new();
        for files in index.by_class.values() {
            for f in files {
                indexed_files.insert(f.clone());
            }
        }
        for files in index.by_tag.values() {
            for f in files {
                indexed_files.insert(f.clone());
            }
        }

        // 索引中有但文件系统没有的
        for f in indexed_files.difference(&fs_files) {
            results.push(CheckResult {
                file: f.clone(),
                issue: "索引中引用但文件不存在（孤立条目）".into(),
                severity: Severity::Warning,
            });
        }

        // 文件系统有但索引中没有的
        for f in fs_files.difference(&indexed_files) {
            results.push(CheckResult {
                file: f.clone(),
                issue: "文件存在但索引中无记录（需重建索引）".into(),
                severity: Severity::Warning,
            });
        }
    } else {
        results.push(CheckResult {
            file: ".index.json".into(),
            issue: "索引文件不存在或无法解析，运行 enjoyknowledge doctor 重建".into(),
            severity: Severity::Warning,
        });
    }

    results
}
