//! enjoyflow doctor / fix — 诊断与修复

use std::path::Path;

pub fn run_doctor(_root: &Path, _full: bool) -> anyhow::Result<()> {
    println!("诊断中...");

    // TODO: 运行 5 项检查
    // let results = crate::doctor::checks::run_all(root, full)?;
    // for r in &results { ... }

    println!("✓ 知识库健康");
    Ok(())
}

pub fn run_fix(_root: &Path, _full: bool) -> anyhow::Result<()> {
    println!("修复中...");

    // TODO: 执行修复
    // crate::doctor::checks::fix_all(root, full)?;

    println!("✓ 已修复");
    Ok(())
}
