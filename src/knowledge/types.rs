//! Search query, result, and entry types for the knowledge layer.

/// Structured search filter.
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// Case-insensitive pattern to match inside `##` sections (body only).
    pub pattern: String,
    /// Filter by directory name (AND logic; empty = no filter).
    pub type_filter: Vec<String>,
    /// Filter by frontmatter tags (AND logic; empty = no filter).
    pub tags: Vec<String>,
    /// Limit search to a specific subdirectory under `.enjoyknowledge/`.
    pub path: Option<String>,
    /// Include archived task materials (`knowledge-tasks/`).
    pub include_archive: bool,
    /// If set, limit archive search to specific task: knowledge-tasks/<req>
    pub req: Option<String>,
}

/// A single search hit keyed to the enclosing `##` section.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Relative path under `.enjoyknowledge/` (forward slashes).
    pub file: String,
    /// Closest `##` section title above the match.
    pub section: String,
    /// Matching line ± 3 lines of context.
    pub snippet: String,
}

/// Directory or file entry for `ls` / `tree` output.
#[derive(Debug, Clone)]
pub struct KnowledgeEntry {
    /// Relative path (directory name or file path).
    pub path: String,
    /// Frontmatter `description` — the primary indexing signal.
    pub description: Option<String>,
    /// `true` for directories, `false` for files.
    pub is_dir: bool,
    /// Number of `##` sections in the file (files only).
    pub entry_count: Option<usize>,
    /// Sub-entries for recursive tree display.
    pub children: Vec<Self>,
}
