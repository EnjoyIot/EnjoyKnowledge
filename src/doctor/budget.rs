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
    let base = root.join(".enjoyknowledge");

    for (rel_path, max_lines) in BUDGETS {
        let path = base.join(rel_path);
        if let Ok(content) = std::fs::read_to_string(&path) {
            let lines = content.lines().count();
            if lines > *max_lines {
                exceeded.push(ExceededFile { path: rel_path.to_string(), lines, max: *max_lines });
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
    let base = root.join(".enjoyknowledge");
    let path = base.join(&file.path);

    let content = std::fs::read_to_string(&path)?;
    let lines: Vec<&str> = content.lines().collect();

    // 保留 frontmatter 和标题行，归档最老的一半条目行
    let _frontmatter_end = lines.iter().position(|l| l.trim() == "---").map_or(0, |i| i + 1);
    // 更准确：找第一个 --- 和第二个 ---
    let fm_start = lines.iter().position(|l| l.trim() == "---");
    let fm_end = fm_start.and_then(|start| {
        lines.iter().skip(start + 1).position(|l| l.trim() == "---").map(|p| start + 1 + p + 1)
    });

    let body_start = fm_end.unwrap_or(0);
    let header_end =
        body_start + lines.iter().skip(body_start).take_while(|l| !l.starts_with("- ")).count();

    let entry_start = body_start + header_end;
    let entries: Vec<&str> = lines.iter().skip(entry_start).copied().collect();

    if entries.len() < 2 {
        return Ok(());
    }

    let split = entries.len() / 2;
    let to_archive = &entries[..split];
    let to_keep = &entries[split..];

    // 写回保留的条目
    let new_content = format!("{}\n{}", lines[..entry_start].join("\n"), to_keep.join("\n"));
    std::fs::write(&path, new_content)?;

    // 归档到 archive/
    let archive_dir = base.join("knowledge-base/archive");
    std::fs::create_dir_all(&archive_dir)?;
    let archive_path =
        archive_dir.join(std::path::Path::new(&file.path).file_name().unwrap_or_default());
    let archive_content =
        format!("{}\n{}\n", lines[..entry_start].join("\n"), to_archive.join("\n"));
    std::fs::write(&archive_path, archive_content)?;

    println!("✓ 归档 {} 条到 {}", to_archive.len(), archive_path.to_string_lossy());
    Ok(())
}
