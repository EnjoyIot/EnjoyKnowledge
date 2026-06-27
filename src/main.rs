// enjoyknowledge — CLI entry point for the AI coding context layer

mod cli;
mod core;
mod doctor;
mod format;
mod init;
mod knowledge;
mod profile;
mod template;

use crate::cli::args::{Cli, Command};
use clap::Parser;
use std::path::Path;

/// Canonical directory name for enjoyknowledge data.
pub(crate) const EK_DIR: &str = ".enjoyknowledge";

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { path, ai, template, link, profile: profile_name } => {
            let Some(profile) = crate::init::resolve_profile(&profile_name) else {
                eprintln!("enjoyknowledge: unknown profile '{profile_name}'");
                eprintln!("Available: for-coding");
                std::process::exit(1);
            };

            cli::init::run(
                path.as_deref(),
                ai.as_deref(),
                template.as_deref(),
                link.as_deref(),
                profile.as_ref(),
            )?;
        }
        Command::Ls { path, bare } => {
            let source = knowledge::filesystem::FilesystemSource::new(EK_DIR, ".");
            cli::ls::run(&source, path.as_deref(), bare)?;
        }
        Command::Tree { bare } => {
            let source = knowledge::filesystem::FilesystemSource::new(EK_DIR, ".");
            cli::tree::run(&source, bare)?;
        }
        Command::Cat { path } => {
            let source = knowledge::filesystem::FilesystemSource::new(EK_DIR, ".");
            cli::cat::run(&source, &path);
        }
        Command::Grep { pattern, type_filter, tags, path, archive, req } => {
            let source = knowledge::filesystem::FilesystemSource::new(EK_DIR, ".");
            cli::grep::run(
                &source,
                &pattern,
                &type_filter,
                &tags,
                path.as_deref(),
                archive,
                req.as_deref(),
            )?;
        }
        Command::Add { path, content } => {
            let source = knowledge::filesystem::FilesystemSource::new(EK_DIR, ".");
            cli::add::run(&source, &path, &content)?;
        }
        Command::Doctor { ci } => {
            let _ = cli::doctor::run_doctor(Path::new("."), ci);
        }
        Command::Fix { req } => {
            cli::doctor::run_fix(Path::new("."), req.as_deref())?;
        }
        Command::Export { tool, dry_run } => {
            cli::export::run(Path::new("."), &tool, dry_run)?;
        }
    }

    Ok(())
}
