/// File budget management: split files over 20 `##` entries by moving
/// the oldest entries to an archive file.
use crate::config::DEFAULT_BUDGET_LIMIT;
use crate::knowledge::FilesystemSource;
use crate::knowledge::KnowledgeSource;
use std::path::Path;

/// Archive the oldest `##` entries from a file when it exceeds the budget limit.
pub fn archive_old_entries(source: &FilesystemSource, rel: &str) -> anyhow::Result<()> {
    let content = source.read_file(rel)?;
    let lines: Vec<&str> = content.lines().collect();

    // Find the second `---` (end of frontmatter)
    let mut fm_end = 0usize;
    let mut dashes = 0usize;
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "---" {
            dashes += 1;
            if dashes == 2 {
                fm_end = i + 1;
                break;
            }
        }
    }

    if fm_end == 0 {
        // No frontmatter or can't find boundary — skip
        return Ok(());
    }

    let body_lines = &lines[fm_end..];

    // Find `##` section boundaries
    let mut section_starts: Vec<usize> = Vec::new();
    for (i, line) in body_lines.iter().enumerate() {
        if line.starts_with("## ") && !line.starts_with("### ") {
            section_starts.push(i);
        }
    }

    if section_starts.len() <= DEFAULT_BUDGET_LIMIT {
        return Ok(());
    }

    // Move oldest half to archive
    let split = section_starts.len() / 2;
    let archive_start = section_starts[0];
    let keep_start = section_starts[split];

    let to_archive = &body_lines[archive_start..keep_start];
    let to_keep = &body_lines[keep_start..];

    // Write back kept entries
    let new_content = format!("{}\n{}", lines[..fm_end].join("\n"), to_keep.join("\n"));
    std::fs::write(source.root.join(rel), new_content)?;

    // Write archived entries
    let archive_rel = rel.replace(".md", ".archive.md");
    std::fs::create_dir_all(
        source.root.join(Path::new(&archive_rel).parent().unwrap_or_else(|| Path::new(""))),
    )?;
    std::fs::write(
        source.root.join(&archive_rel),
        format!("{}\n{}", lines[..fm_end].join("\n"), to_archive.join("\n")),
    )?;

    eprintln!(
        "enjoyknowledge: archived {} entries from {} to {}",
        to_archive.len(),
        rel,
        archive_rel
    );

    Ok(())
}
