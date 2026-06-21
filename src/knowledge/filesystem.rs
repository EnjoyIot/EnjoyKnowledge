/// 文件系统源适配器
use super::source::KnowledgeSource;
use super::types::{SearchQuery, SearchResult};
use regex::Regex;
use std::path::PathBuf;

/// 本地文件系统知识源
pub struct FilesystemSource {
    root: PathBuf,
}

impl FilesystemSource {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// 构建 grep 正则（不区分大小写）
    fn build_pattern(query: &str) -> Regex {
        Regex::new(&regex::escape(query)).unwrap_or_else(|_| Regex::new(".*").unwrap())
    }
}

impl KnowledgeSource for FilesystemSource {
    fn search(&self, query: &SearchQuery) -> anyhow::Result<Vec<SearchResult>> {
        let pattern = Self::build_pattern(&query.text);
        let mut results = Vec::new();

        for entry in walkdir::WalkDir::new(&self.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            // 跳过归档（除非明确要求）
            if !query.include_archive
                && entry
                    .path()
                    .to_string_lossy()
                    .contains("archive")
            {
                continue;
            }

            let content = std::fs::read_to_string(entry.path())?;

            // class / tag 过滤（在 frontmatter 中匹配）
            if !query.class.is_empty() || !query.tags.is_empty() {
                if let Some(fm) = crate::format::frontmatter::parse_frontmatter(&content) {
                    if !query.class.is_empty()
                        && !query
                            .class
                            .iter()
                            .any(|c| fm.class.as_deref() == Some(c))
                    {
                        continue;
                    }
                    if !query.tags.is_empty()
                        && !query
                            .tags
                            .iter()
                            .all(|t| fm.tags.contains(&t.to_string()))
                    {
                        continue;
                    }
                }
            }

            // 内容匹配（标题 + 正文前 200 字符）
            if pattern.is_match(&content) {
                let rel_path = entry
                    .path()
                    .strip_prefix(&self.root)
                    .unwrap_or(entry.path())
                    .to_string_lossy()
                    .to_string();

                results.push(SearchResult {
                    file: rel_path,
                    section: String::new(), // TODO: 提取 ## 标题
                    snippet: String::new(),  // TODO: 提取匹配行上下文
                });
            }
        }

        results.sort_by(|a, b| a.file.cmp(&b.file));
        Ok(results)
    }

    fn read_file(&self, path: &str) -> anyhow::Result<String> {
        let full = self.root.join(path);
        Ok(std::fs::read_to_string(full)?)
    }

    fn append_to_file(&self, path: &str, content: &str) -> anyhow::Result<()> {
        let full = self.root.join(path);
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(full)?;
        writeln!(file, "{}", content)?;
        Ok(())
    }

    fn list_files(&self) -> anyhow::Result<Vec<String>> {
        let mut files = Vec::new();
        for entry in walkdir::WalkDir::new(&self.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let rel = entry
                .path()
                .strip_prefix(&self.root)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();
            files.push(rel);
        }
        Ok(files)
    }
}
