//! Centralised path constants and default values for enjoyknowledge.
//!
//! Every directory name, file name, and magic number lives here so that
//! users who customise their `.enjoyknowledge/` structure can change
//! behaviour without touching code spread across a dozen modules.
//!
//! # Naming convention
//!
//! - `*_DIR`  — directory name (single path component)
//! - `*_FILE` — file name (basename, no leading path)
//! - `*_PATH` — relative path (may include `/` separators)
//! - `DEFAULT_*` — tunable numeric / string default

// ── Canonical top-level directories ──────────────────────────────────────

/// Knowledge-base root (project-relative).
pub const EK_DIR: &str = ".enjoyknowledge";

/// AI task-staging root (project-relative).
pub const STAGE_DIR: &str = ".enjoyknowledge_stage";

// ── Sub-directories (single component, relative to EK_DIR or STAGE_DIR) ──

/// Metadata directory inside `.enjoyknowledge/`.
pub const META_DIR: &str = "_meta";

/// Workflow-skills directory inside `.enjoyknowledge/`.
pub const SKILLS_DIR: &str = "skills";

/// Drafts directory inside `.enjoyknowledge_stage/`.
pub const DRAFTS_DIR: &str = "drafts";

/// Active-tasks directory inside `.enjoyknowledge_stage/`.
pub const TASKS_DIR: &str = "tasks";

/// Archive directory inside `.enjoyknowledge_stage/`.
pub const ARCHIVE_DIR: &str = ".archive";

/// Project-root-level legacy tasks directory (pre-stage, kept for compat).
pub const KNOWLEDGE_TASKS_DIR: &str = "knowledge-tasks";

// ── Well-known file names ────────────────────────────────────────────────

/// Kind-registry Markdown table (lives in `_meta/`).
pub const KINDS_FILE: &str = "kinds.md";

/// Stage-directory specification (lives in `_meta/`).
pub const STAGE_DEFAULTS_FILE: &str = "stage-defaults.md";

/// Project-root routing table for AI tools.
pub const AGENTS_FILE: &str = "AGENTS.md";

/// KB-level index written alongside kind directories.
pub const INDEX_FILE: &str = "index.md";

/// KB-level AGENTS.md inside `.enjoyknowledge/`.
pub const EK_AGENTS_FILE: &str = "AGENTS.md";

/// Stage-level AGENTS.md inside `.enjoyknowledge_stage/`.
pub const STAGE_AGENTS_FILE: &str = "AGENTS.md";

// ── Relative paths (built from the components above) ─────────────────────

/// Path to the kind-registry file relative to the project root.
pub const KINDS_MD_REL: &str = ".enjoyknowledge/_meta/kinds.md";

// ── Default values ───────────────────────────────────────────────────────

/// Fallback author name used by `ek promote`.
pub const DEFAULT_AUTHOR: &str = "enjoy";

/// Maximum number of `##` sections per file before `ek doctor` warns.
pub const DEFAULT_BUDGET_LIMIT: usize = 20;

/// Number of days before a knowledge entry is considered stale.
pub const DEFAULT_TTL_DAYS: u64 = 180;

/// Default walk-depth for listing knowledge entries.
pub const DEFAULT_WALK_DEPTH: usize = 2;

/// Walk-depth for search / grep operations.
pub const SEARCH_WALK_DEPTH: usize = 3;

/// Lines of context shown before a grep match.
pub const GREP_CONTEXT_BEFORE: usize = 3;

/// Lines of context shown after a grep match.
pub const GREP_CONTEXT_AFTER: usize = 4;

/// Max depth when scanning for mtime (stage-clean).
pub const MTIME_SCAN_DEPTH: usize = 10;
