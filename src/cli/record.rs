//! enjoyflow record — 知识写入

use crate::knowledge::source::KnowledgeSource;
use crate::knowledge::types::RecordType;
use crate::record::router;

pub fn run_gotcha(
    task: Option<&str>,
    tag: &[String],
    content: &str,
    root: &std::path::Path,
) -> anyhow::Result<()> {
    let rel_path = router::route(RecordType::Gotcha, task, tag, content, root)?;
    let source = crate::knowledge::filesystem::FilesystemSource::new(
        root.join(".enjoyflow"),
    );

    let entry = format!(
        "- {}: {} ({}, {})",
        tag.join(", "),
        content,
        task.unwrap_or("-"),
        chrono::Local::now().format("%Y-%m-%d")
    );
    source.append_to_file(&rel_path, &entry)?;
    println!("✓ 已记录到 {}", rel_path);
    Ok(())
}

pub fn run_pattern(
    tag: &[String],
    content: &str,
    root: &std::path::Path,
) -> anyhow::Result<()> {
    let rel_path = router::route(RecordType::Pattern, None, tag, content, root)?;
    let source = crate::knowledge::filesystem::FilesystemSource::new(
        root.join(".enjoyflow"),
    );

    let entry = format!(
        "- {}: {} ({})",
        tag.join(", "),
        content,
        chrono::Local::now().format("%Y-%m-%d")
    );
    source.append_to_file(&rel_path, &entry)?;
    println!("✓ 已记录到 {}", rel_path);
    Ok(())
}

pub fn run_decision(
    task: &str,
    content: &str,
    root: &std::path::Path,
) -> anyhow::Result<()> {
    let rel_path = router::route(RecordType::Decision, Some(task), &[], content, root)?;
    let source = crate::knowledge::filesystem::FilesystemSource::new(
        root.join(".enjoyflow"),
    );

    let entry = format!(
        "- {}: {} ({})",
        task,
        content,
        chrono::Local::now().format("%Y-%m-%d")
    );
    source.append_to_file(&rel_path, &entry)?;
    println!("✓ 已记录到 {}", rel_path);
    Ok(())
}
