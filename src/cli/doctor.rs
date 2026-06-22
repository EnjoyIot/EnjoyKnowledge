//! enjoyknowledge doctor / fix — 诊断与修复

use crate::doctor::checks::{self, Severity};
use std::path::Path;

pub fn run_doctor(root: &Path, _full: bool) -> anyhow::Result<()> {
    let results = checks::run_all(root)?;

    if results.is_empty() {
        println!("✓ 知识库健康");
        return Ok(());
    }

    let errors: Vec<_> = results.iter().filter(|r| r.severity == Severity::Error).collect();
    let warnings: Vec<_> = results.iter().filter(|r| r.severity == Severity::Warning).collect();

    if !errors.is_empty() {
        println!("错误 ({} 项):", errors.len());
        for r in &errors {
            println!("  [E] {} — {}", r.file, r.issue);
        }
    }

    if !warnings.is_empty() {
        if !errors.is_empty() {
            println!();
        }
        println!("警告 ({} 项):", warnings.len());
        for r in &warnings {
            println!("  [W] {} — {}", r.file, r.issue);
        }
    }

    let total = results.len();
    println!(
        "\n共发现 {} 个问题（{} 错误, {} 警告）。运行 `enjoyknowledge fix` 自动修复。",
        total,
        errors.len(),
        warnings.len()
    );

    if !errors.is_empty() {
        std::process::exit(2);
    }

    Ok(())
}

pub fn run_fix(root: &Path, _full: bool) -> anyhow::Result<()> {
    let results = checks::run_all(root)?;

    let mut fixed = 0u32;

    for r in &results {
        match r.issue.as_str() {
            s if s.starts_with("超出预算：") => {
                let budget_results = crate::doctor::budget::check_budget(root);
                for exceeded in &budget_results {
                    crate::doctor::budget::archive_file(root, exceeded)?;
                    println!("✓ 已归档 {}", exceeded.path);
                    fixed += 1;
                }
            }
            s if s.starts_with("索引文件不存在") || s.starts_with("文件存在但索引中无记录") =>
            {
                // 重建索引
                rebuild_index(root)?;
                println!("✓ 已重建索引");
                fixed += 1;
                break; // 索引重建一次就够了
            }
            _ => {}
        }
    }

    let remaining = u32::try_from(results.len()).unwrap_or(0) - fixed;
    if remaining > 0 {
        println!("✓ 已自动修复 {fixed} 项，{remaining} 项需手动处理（如缺 class/tag 字段）");
    } else {
        println!("✓ 所有问题已自动修复");
    }

    Ok(())
}

fn rebuild_index(root: &Path) -> anyhow::Result<()> {
    use crate::knowledge::index::Index;

    let mut index = Index::default();
    let base = root.join(".enjoyknowledge");

    for entry in walkdir::WalkDir::new(&base)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        if let Ok(content) = std::fs::read_to_string(entry.path()) {
            if let Some(fm) = crate::format::frontmatter::parse_frontmatter(&content) {
                if let Ok(rel) = entry.path().strip_prefix(&base) {
                    let rel = rel.to_string_lossy().to_string();
                    if let Some(ref class) = fm.class {
                        index.add_class_file(class, &rel);
                    }
                    for tag in &fm.tags {
                        index.add_tag_file(tag, &rel);
                    }
                }
            }
        }
    }

    index.last_rebuilt = Some(chrono::Local::now().format("%Y-%m-%d").to_string());
    index.save(root)?;
    Ok(())
}
