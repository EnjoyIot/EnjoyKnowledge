//! CLI argument definitions — separated from main.rs for cleanliness.

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(name = "enjoyknowledge", version, about = "AI coding context layer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
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

    /// Generate AI tool entry files from .enjoyknowledge/ (v0.2 首发 2 工具：cursor / claude)
    Export {
        /// AI tool to generate entry file for (cursor / claude / auto; repeatable, or comma-separated)
        #[arg(long, default_value = "auto", num_args = 1.., value_delimiter = ',')]
        tool: Vec<String>,

        /// Print what would be generated without writing files
        #[arg(long)]
        dry_run: bool,
    },

    /// Run a named workflow (v0.2: onboard, capture)
    Workflow {
        /// Workflow name (v0.2: onboard, capture)
        workflow: String,

        /// Knowledge kind (capture only): gotcha/decision/pattern/rule/business/architecture/contract/convention/context/template
        #[arg(long, value_name = "KIND")]
        kind: Option<String>,

        /// Frontmatter field as KEY=VALUE (capture only, repeatable)
        #[arg(long = "field", value_name = "KEY=VALUE", value_parser = parse_field)]
        field: Vec<(String, String)>,

        /// Markdown body content (capture only: ## sections + text)
        #[arg(long, value_name = "BODY")]
        body: Option<String>,

        /// Target file path under .enjoyknowledge/ (capture only; auto-derived from kind when omitted)
        #[arg(long, value_name = "PATH")]
        path: Option<String>,
    },

    /// Promote a draft from `.enjoyknowledge_stage/drafts/` to the knowledge base (v0.4)
    Promote {
        /// Draft file path relative to `.enjoyknowledge_stage/drafts/`
        draft_file: String,

        /// Target knowledge kind
        #[arg(long, value_name = "KIND")]
        to: String,

        /// Knowledge entry ID (defaults to draft filename without .md)
        #[arg(long, value_name = "ID")]
        id: Option<String>,

        /// Author name (defaults to 'unknown')
        #[arg(long, value_name = "NAME")]
        author: Option<String>,
    },

    /// Stage management commands (v0.4)
    Stage {
        #[command(subcommand)]
        action: StageAction,
    },
}

/// Parse a `KEY=VALUE` string for `--field`.
fn parse_field(s: &str) -> Result<(String, String), String> {
    let (k, v) = s.split_once('=').ok_or_else(|| format!("expected KEY=VALUE, got '{s}'"))?;
    Ok((k.trim().to_string(), v.trim().to_string()))
}

/// Subcommand actions for `ek stage`.
#[derive(Subcommand)]
pub enum StageAction {
    /// Clean archived tasks older than TTL threshold
    Clean {
        /// Print what would be deleted without deleting
        #[arg(long)]
        dry_run: bool,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,

        /// Delete tasks older than N days (default: 180)
        #[arg(long, value_name = "DAYS")]
        older_than: Option<u64>,
    },
}
