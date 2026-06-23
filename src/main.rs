// enjoyknowledge — CLI entry point for the AI coding context layer

mod cli;
mod core;
mod doctor;
mod format;
mod init;
mod knowledge;
mod profile;
mod template;

use clap::Parser;
use clap::Subcommand;
use std::path::Path;

#[derive(Parser)]
#[command(name = "enjoyknowledge", version, about = "AI coding context layer")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize .enjoyknowledge/ knowledge base skeleton
    Init {
        /// Project path (default: current directory)
        path: Option<String>,

        /// AI coding tool to generate tool-specific files for
        #[arg(long)]
        ai: Option<String>,

        /// Apply a named template (from ~/.enjoyknowledge/templates/ or .enjoyknowledge/templates/)
        #[arg(long)]
        template: Option<String>,

        /// Link to an external knowledge base directory
        #[arg(long)]
        link: Option<String>,

        /// Knowledge profile: "for-coding" (default), "for-design", etc.
        #[arg(long, default_value = "for-coding")]
        profile: String,
    },

    /// List knowledge files with descriptions
    Ls {
        /// Subdirectory path to list (e.g. "architecture")
        path: Option<String>,

        /// Bare mode — filename only, no descriptions or counts
        #[arg(long)]
        bare: bool,
    },

    /// Recursive directory tree with descriptions
    Tree {
        /// Bare mode — filenames only
        #[arg(long)]
        bare: bool,
    },

    /// Read a knowledge file
    Cat {
        /// Relative path under .enjoyknowledge/
        path: String,
    },

    /// Structure-aware search inside ## sections
    Grep {
        /// Case-insensitive search pattern
        pattern: String,

        /// Filter by directory name (repeatable)
        #[arg(long = "type", value_name = "DIR")]
        type_filter: Vec<String>,

        /// Filter by frontmatter tags (repeatable)
        #[arg(long, value_name = "TAG")]
        tags: Vec<String>,

        /// Limit search to a subdirectory
        #[arg(long, value_name = "PATH")]
        path: Option<String>,

        /// Include knowledge-tasks/ in search scope
        #[arg(long)]
        archive: bool,

        /// Search within a specific task directory: knowledge-tasks/<REQ-ID>
        #[arg(long, value_name = "REQ-ID", conflicts_with = "archive")]
        req: Option<String>,
    },

    /// Append or create a knowledge entry
    Add {
        /// Relative path under .enjoyknowledge/ (e.g. "gotchas/export.md")
        path: String,

        /// Markdown content to append (## sections + body)
        content: String,
    },

    /// Run health checks against the knowledge base
    Doctor {
        /// Exit with non-zero code if any warnings exist (for CI pipelines)
        #[arg(long)]
        ci: bool,
    },

    /// Auto-fix common issues
    Fix {
        /// Fix/archive a specific task: knowledge-tasks/<REQ-ID>
        #[arg(long, value_name = "REQ-ID")]
        req: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { path, ai, template, link, profile: profile_name } => {
            let profile = init::resolve_profile(&profile_name).unwrap_or_else(|| {
                eprintln!("enjoyknowledge: unknown profile '{profile_name}', using for-coding");
                Box::new(profile::coding::CodingProfile)
            });

            cli::init::run(
                path.as_deref(),
                ai.as_deref(),
                template.as_deref(),
                link.as_deref(),
                profile.as_ref(),
            )?;
        }
        Command::Ls { path, bare } => {
            let source = knowledge::filesystem::FilesystemSource::new(".enjoyknowledge", ".");
            cli::ls::run(&source, path.as_deref(), bare)?;
        }
        Command::Tree { bare } => {
            let source = knowledge::filesystem::FilesystemSource::new(".enjoyknowledge", ".");
            cli::tree::run(&source, bare)?;
        }
        Command::Cat { path } => {
            let source = knowledge::filesystem::FilesystemSource::new(".enjoyknowledge", ".");
            cli::cat::run(&source, &path);
        }
        Command::Grep { pattern, type_filter, tags, path, archive, req } => {
            let source = knowledge::filesystem::FilesystemSource::new(".enjoyknowledge", ".");
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
            let source = knowledge::filesystem::FilesystemSource::new(".enjoyknowledge", ".");
            cli::add::run(&source, &path, &content)?;
        }
        Command::Doctor { ci } => {
            cli::doctor::run_doctor(Path::new("."), ci)?;
        }
        Command::Fix { req } => {
            cli::doctor::run_fix(Path::new("."), req.as_deref())?;
        }
    }

    Ok(())
}
