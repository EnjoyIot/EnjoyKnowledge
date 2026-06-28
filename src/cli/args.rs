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

    /// Establish project mental model (AGENTS.md + positioning + gotchas + decisions)
    Onboard,

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

        /// Author name (defaults to 'enjoy')
        #[arg(long, value_name = "NAME")]
        author: Option<String>,
    },

    /// Stage management commands (v0.4)
    Stage {
        #[command(subcommand)]
        action: StageAction,
    },

    /// Manage knowledge kinds (v0.4.5)
    Kind {
        #[command(subcommand)]
        kind_cmd: KindCmd,
    },
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

/// Subcommand actions for `ek kind` (v0.4.5).
#[derive(Subcommand)]
pub enum KindCmd {
    /// Add a new knowledge kind
    Add {
        /// Kind name (alphanumeric/underscore/dash)
        name: String,

        /// Comma-separated required frontmatter fields
        #[arg(long, num_args = 1.., value_delimiter = ',')]
        required: Vec<String>,

        /// Human-readable summary of the kind
        #[arg(long, default_value = "")]
        summary: String,

        /// Skip confirmation prompt
        #[arg(long, short)]
        yes: bool,
    },

    /// Remove a knowledge kind
    Rm {
        /// Kind name to remove
        name: String,

        /// Force removal even if directory has entries
        #[arg(long, short)]
        force: bool,

        /// Skip confirmation prompt
        #[arg(long, short)]
        yes: bool,
    },

    /// List all knowledge kinds
    List,
}
