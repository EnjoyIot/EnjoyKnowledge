/// Unified knowledge-source interface — every backend implements this.
use super::types::{KnowledgeEntry, SearchQuery, SearchResult};

/// The single abstraction that all CLI commands operate through.
pub trait KnowledgeSource {
    /// Flat listing of directories and files in `dir` (None = root).
    fn list_entries(&self, dir: Option<&str>, bare: bool) -> anyhow::Result<Vec<KnowledgeEntry>>;

    /// Recursive tree of all directories and files.
    fn tree_entries(&self, bare: bool) -> anyhow::Result<Vec<KnowledgeEntry>>;

    /// Read the full content of a single knowledge file.
    fn read_file(&self, path: &str) -> anyhow::Result<String>;

    /// Structure-aware search — results are keyed to `##` sections.
    fn search(&self, query: &SearchQuery) -> anyhow::Result<Vec<SearchResult>>;

    /// Append content to a knowledge file, creating it (with frontmatter) if needed.
    /// Automatically updates the `timestamp` field in existing frontmatter.
    fn add_entry(&self, path: &str, content: &str) -> anyhow::Result<()>;

    /// Return relative paths of all Markdown knowledge files.
    /// Used by doctor checks for comprehensive diagnostics.
    fn all_entry_paths(&self) -> Vec<String>;

    /// Read the project's AGENTS.md content, if it exists.
    fn read_agents_md(&self) -> Option<String>;

    /// List knowledge-tasks .md files with their content.
    /// Returns (`relative_path`, content) pairs.
    fn list_knowledge_tasks(&self) -> Vec<(String, String)>;
}
