//! `enjoyknowledge doctor` / `fix` — diagnose and auto-repair the knowledge base.
use crate::doctor;
use crate::knowledge::FilesystemSource;
use crate::EK_DIR;
use std::path::Path;

pub fn run_doctor(root: &Path, ci: bool) -> anyhow::Result<()> {
    let ek = root.join(EK_DIR);
    let source = FilesystemSource::new(&ek, root);

    let mut exit_code = 0i32;

    // Run all 4 v0.2 checks (GLOSSARY L51)
    for result in doctor::checks::run_all(&source, root) {
        let prefix = match result.severity {
            doctor::checks::Severity::Error => {
                exit_code = 3;
                "\u{2717}"
            }
            doctor::checks::Severity::Warning => {
                if ci {
                    exit_code = 3;
                }
                "\u{26a0}"
            }
        };
        println!("{prefix} {} — {}", result.file, result.issue);
    }

    if exit_code == 0 {
        println!(
            "{} all checks passed",
            if colored::control::SHOULD_COLORIZE.should_colorize() { "\u{2705}" } else { "[OK]" }
        );
    }

    std::process::exit(exit_code);
}

pub fn run_fix(root: &Path, req: Option<&str>) -> anyhow::Result<()> {
    let ek = root.join(EK_DIR);
    let source = FilesystemSource::new(&ek, root);

    doctor::fix::run_fix(&source, root, req)?;
    Ok(())
}
