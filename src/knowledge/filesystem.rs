/// Filesystem-backed knowledge source implementing `KnowledgeSource`.
use super::source::KnowledgeSource;
use super::types::{KnowledgeEntry, SearchQuery, SearchResult};
use regex::Regex;
use std::path::{Path, PathBuf};

/// Knowledge stored as OKF-compatible Markdown under a local directory.
pub struct FilesystemSource {
    /// The `.enjoyknowledge/` directory.
    pub(crate) root: PathBuf,
    /// Project root (for knowledge-tasks/ access).
    pub(crate) project_root: PathBuf,
}

impl FilesystemSource {
    /// Create a new filesystem source.
    /// `root` is the `.enjoyknowledge/` directory.
    /// `project_root` is the project directory (for `knowledge-tasks/` access).
    pub fn new(root: impl Into<PathBuf>, project_root: impl Into<PathBuf>) -> Self {
        Self { root: root.into(), project_root: project_root.into() }
    }

    // ── helpers ──────────────────────────────────────────────

    /// Build a case-insensitive regex from a literal query string.
    fn build_pattern(query: &str) -> Regex {
        Regex::new(&format!("(?i){}", regex::escape(query)))
            .unwrap_or_else(|_| Regex::new(".*").unwrap())
    }

    /// Count `##` headings in file content.
    fn count_entries(content: &str) -> usize {
        content.lines().filter(|l| l.starts_with("## ") && !l.starts_with("### ")).count()
    }

    /// Walk `.enjoyknowledge/` for `.md` files, optionally filtered by subdirectory.
    pub fn walk_md_files(&self, dir: Option<&str>) -> Vec<(PathBuf, String)> {
        let scan_root = dir.map_or_else(|| self.root.clone(), |d| self.root.join(d));

        walkdir::WalkDir::new(&scan_root)
            .max_depth(2)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
            .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
            .map(|e| {
                let rel = e
                    .path()
                    .strip_prefix(&self.root)
                    .unwrap_or_else(|_| e.path())
                    .to_string_lossy()
                    .replace('\\', "/");
                (e.path().to_path_buf(), rel)
            })
            .collect()
    }

    /// Read a file's content from `.enjoyknowledge/`.
    #[allow(dead_code)]
    fn read_enjoyknowledge_file(&self, rel_path: &str) -> anyhow::Result<String> {
        let full = self.root.join(rel_path);
        Ok(std::fs::read_to_string(full)?)
    }

    /// Ensure parent directories exist for a path under `.enjoyknowledge/`.
    fn ensure_parent(&self, rel_path: &str) -> anyhow::Result<()> {
        if let Some(parent) = Path::new(rel_path).parent() {
            let full = self.root.join(parent);
            std::fs::create_dir_all(&full)?;
        }
        Ok(())
    }
}

impl KnowledgeSource for FilesystemSource {
    fn list_entries(&self, dir: Option<&str>, bare: bool) -> anyhow::Result<Vec<KnowledgeEntry>> {
        let scan_root = dir.map_or_else(|| self.root.clone(), |d| self.root.join(d));

        if !scan_root.exists() {
            return Ok(Vec::new());
        }

        let mut entries: Vec<KnowledgeEntry> = Vec::new();

        // Collect subdirectories
        let mut subdirs: Vec<String> = Vec::new();
        if let Ok(iter) = std::fs::read_dir(&scan_root) {
            for entry in iter.flatten() {
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                if is_dir {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !name.starts_with('.') && name != "templates" {
                        subdirs.push(name);
                    }
                }
            }
        }
        subdirs.sort();

        for dir_name in &subdirs {
            let desc = if bare { None } else { Some(format!("{dir_name}/")) };
            entries.push(KnowledgeEntry {
                path: dir_name.clone(),
                description: desc,
                is_dir: true,
                entry_count: None,
                children: Vec::new(),
            });
        }

        // Collect files
        let files = self.walk_md_files(dir);
        for (abs_path, rel_path) in &files {
            // Skip files inside sub-subdirectories for flat listing
            let depth = rel_path.chars().filter(|&c| c == '/').count();
            if depth > 1 {
                continue;
            }

            let (description, entry_count) = if bare {
                (None, None)
            } else {
                std::fs::read_to_string(abs_path).map_or((None, None), |content| {
                    let fm = crate::format::frontmatter::parse_frontmatter(&content);
                    let desc = fm.and_then(|f| f.description);
                    let count = Some(Self::count_entries(&content));
                    (desc, count)
                })
            };

            let name =
                Path::new(rel_path).file_name().unwrap_or_default().to_string_lossy().to_string();

            entries.push(KnowledgeEntry {
                path: name,
                description,
                is_dir: false,
                entry_count,
                children: Vec::new(),
            });
        }

        Ok(entries)
    }

    // ── tree_entries ─────────────────────────────────────────
    fn tree_entries(&self, bare: bool) -> anyhow::Result<Vec<KnowledgeEntry>> {
        let all_files = self.walk_md_files(None);
        #[allow(clippy::type_complexity)]
        let mut dirs: std::collections::BTreeMap<
            String,
            Vec<(String, Option<String>, Option<usize>)>,
        > = std::collections::BTreeMap::new();

        for (abs_path, rel_path) in &all_files {
            let parent =
                Path::new(rel_path).parent().and_then(|p| p.to_str()).unwrap_or("").to_string();

            let (description, entry_count) = if bare {
                (None, None)
            } else {
                std::fs::read_to_string(abs_path).map_or((None, None), |content| {
                    let fm = crate::format::frontmatter::parse_frontmatter(&content);
                    let desc = fm.and_then(|f| f.description);
                    let count = Some(Self::count_entries(&content));
                    (desc, count)
                })
            };

            let name =
                Path::new(rel_path).file_name().unwrap_or_default().to_string_lossy().to_string();

            dirs.entry(parent).or_default().push((name, description, entry_count));
        }

        let mut entries: Vec<KnowledgeEntry> = Vec::new();
        for (dir_name, files) in &dirs {
            let description = if bare { None } else { Some(format!("{dir_name}/")) };
            let children: Vec<KnowledgeEntry> = files
                .iter()
                .map(|(name, desc, count)| KnowledgeEntry {
                    path: name.clone(),
                    description: desc.clone(),
                    is_dir: false,
                    entry_count: *count,
                    children: Vec::new(),
                })
                .collect();

            entries.push(KnowledgeEntry {
                path: dir_name.clone(),
                description,
                is_dir: true,
                entry_count: None,
                children,
            });
        }

        Ok(entries)
    }

    // ── read_file ────────────────────────────────────────────
    fn read_file(&self, path: &str) -> anyhow::Result<String> {
        let full = self.root.join(path);
        Ok(std::fs::read_to_string(full)?)
    }

    // ── search ───────────────────────────────────────────────
    fn search(&self, query: &SearchQuery) -> anyhow::Result<Vec<SearchResult>> {
        let pattern = Self::build_pattern(&query.pattern);
        let mut results: Vec<SearchResult> = Vec::new();

        // Determine search scope
        let search_roots: Vec<PathBuf> = query.req.as_ref().map_or_else(
            || {
                if query.include_archive {
                    vec![self.root.clone(), self.project_root.join("knowledge-tasks")]
                } else {
                    vec![self.root.clone()]
                }
            },
            |req_id| {
                vec![self.root.clone(), self.project_root.join("knowledge-tasks").join(req_id)]
            },
        );

        for search_root in &search_roots {
            if !search_root.exists() {
                continue;
            }

            let scan_dir =
                query.path.as_ref().map_or_else(|| search_root.clone(), |p| search_root.join(p));

            for entry in walkdir::WalkDir::new(&scan_dir)
                .max_depth(3)
                .into_iter()
                .filter_map(std::result::Result::ok)
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
            {
                let Ok(content) = std::fs::read_to_string(entry.path()) else { continue };

                // Frontmatter tag/type filter
                if !query.type_filter.is_empty() || !query.tags.is_empty() {
                    if let Some(fm) = crate::format::frontmatter::parse_frontmatter(&content) {
                        if !query.type_filter.is_empty() {
                            // type filter = directory name check
                            let dir_name = entry
                                .path()
                                .parent()
                                .and_then(|p| p.file_name())
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default();
                            if !query.type_filter.iter().any(|t| t.eq_ignore_ascii_case(&dir_name))
                            {
                                continue;
                            }
                        }
                        if !query.tags.is_empty()
                            && !query.tags.iter().all(|t| fm.tags.iter().any(|ft| ft == t))
                        {
                            continue;
                        }
                    } else if !query.type_filter.is_empty() || !query.tags.is_empty() {
                        // No frontmatter but filters active → skip
                        continue;
                    }
                }

                // Search body only (skip frontmatter)
                let body_start = crate::format::document::find_body_start(&content);
                let body = &content[body_start..];
                let body_line_offset = content[..body_start].lines().count();

                if let Some((body_line, _)) =
                    body.lines().enumerate().find(|(_, line)| pattern.is_match(line))
                {
                    let actual_line = body_line + body_line_offset;
                    let rel_path = entry
                        .path()
                        .strip_prefix(search_root)
                        .unwrap_or_else(|_| entry.path())
                        .to_string_lossy()
                        .replace('\\', "/");

                    let section =
                        crate::format::document::find_section_at_line(&content, actual_line + 1)
                            .unwrap_or_default();

                    let all_lines: Vec<&str> = content.lines().collect();
                    let start = actual_line.saturating_sub(3);
                    let end = (actual_line + 4).min(all_lines.len());
                    let snippet = all_lines[start..end].join("\n");

                    results.push(SearchResult { file: rel_path, section, snippet });
                }
            }
        }

        // Sort by content density: longer snippet first
        results.sort_by(|a, b| b.snippet.len().cmp(&a.snippet.len()));
        Ok(results)
    }

    // ── add_entry ────────────────────────────────────────────
    fn add_entry(&self, path: &str, content: &str) -> anyhow::Result<()> {
        let full = self.root.join(path);

        if full.exists() {
            // Append to existing file, update timestamp
            let existing = std::fs::read_to_string(&full)?;
            let updated = crate::format::frontmatter::update_timestamp(&existing);
            let new_content = format!("{updated}\n{content}\n");
            std::fs::write(&full, new_content)?;
        } else {
            // Create new file with frontmatter
            self.ensure_parent(path)?;
            let desc = content.lines().find(|l| l.starts_with("## ")).map_or_else(
                || "New entry".to_string(),
                |l| l.trim_start_matches("## ").to_string(),
            );
            let fm = crate::format::frontmatter::generate_frontmatter(&desc);
            std::fs::write(&full, format!("{fm}{content}\n"))?;
        }

        Ok(())
    }
}
