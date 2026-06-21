/// 文件预算与归档
use std::path::Path;

/// 预算表（文件 → 最大行数）
const BUDGETS: &[(&str, usize)] = &[
    ("knowledge-base/development/GOTCHAS.md", 100),
    ("knowledge-base/development/PATTERNS.md", 100),
    ("knowledge-base/project/ARCHITECTURE.md", 150),
];

/// 检查文件是否超出预算
pub fn check_budget(root: &Path) -> Vec<ExceededFile> {
    let mut exceeded = Vec::new();
    let base = root.join(".enjoyflow");

    for (rel_path, max_lines) in BUDGETS {
        let path = base.join(rel_path);
        if let Ok(content) = std::fs::read_to_string(&path) {
            let lines = content.lines().count();
            if lines > *max_lines {
                exceeded.push(ExceededFile {
                    path: rel_path.to_string(),
                    lines,
                    max: *max_lines,
                });
            }
        }
    }
    exceeded
}

#[derive(Debug)]
pub struct ExceededFile {
    pub path: String,
    pub lines: usize,
    pub max: usize,
}

/// 归档文件（超标时移动最老的一半条目到 archive/）
pub fn archive_file(root: &Path, file: &ExceededFile) -> anyhow::Result<()> {
    let _ = (root, file);
    // TODO: 切割文件，移动最老条目到 archive/
    Ok(())
}
