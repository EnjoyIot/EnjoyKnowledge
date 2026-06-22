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

    /// 构建不区分大小写的匹配正则
    fn build_pattern(query: &str) -> Regex {
        Regex::new(&format!("(?i){}", regex::escape(query)))
            .unwrap_or_else(|_| Regex::new(".*").unwrap())
    }
}

/// 找到正文起始位置（跳过 YAML frontmatter）
fn find_body_start(content: &str) -> usize {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---\n") && !trimmed.starts_with("---\r\n") {
        return 0;
    }
    // 跳过第一个 ---
    let after_first = &trimmed[3..];
    // 找第二个 ---
    if let Some(end) = after_first.find("\n---") {
        // +3 for first ---, + end for position, +4 for \n---
        let offset = content.len() - trimmed.len();
        return offset + 3 + end + 4;
    }
    0
}

impl KnowledgeSource for FilesystemSource {
    fn search(&self, query: &SearchQuery) -> anyhow::Result<Vec<SearchResult>> {
        let pattern = Self::build_pattern(&query.text);
        let mut results = Vec::new();

        for entry in walkdir::WalkDir::new(&self.root)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        {
            // 跳过归档（除非明确要求）
            if !query.include_archive && entry.path().to_string_lossy().contains("archive") {
                continue;
            }

            let content = std::fs::read_to_string(entry.path())?;

            // class / tag 过滤（在 frontmatter 中匹配）
            if !query.class.is_empty() || !query.tags.is_empty() {
                if let Some(fm) = crate::format::frontmatter::parse_frontmatter(&content) {
                    if !query.class.is_empty()
                        && !query.class.iter().any(|c| fm.class.as_deref() == Some(c))
                    {
                        continue;
                    }
                    if !query.tags.is_empty()
                        && !query.tags.iter().all(|t| fm.tags.contains(&t.clone()))
                    {
                        continue;
                    }
                }
            }

            // 只在正文中搜索（跳过 frontmatter），以产生正确的 section/snippet
            let body_start = find_body_start(&content);
            let body = &content[body_start..];
            let body_line_offset = content[..body_start].lines().count();

            if let Some((body_line, _)) =
                body.lines().enumerate().find(|(_, line)| pattern.is_match(line))
            {
                let actual_line = body_line + body_line_offset;
                let rel_path = entry
                    .path()
                    .strip_prefix(&self.root)
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

        results.sort_by(|a, b| a.file.cmp(&b.file));
        Ok(results)
    }

    fn read_file(&self, path: &str) -> anyhow::Result<String> {
        let full = self.root.join(path);
        Ok(std::fs::read_to_string(full)?)
    }

    fn append_to_file(&self, path: &str, content: &str) -> anyhow::Result<()> {
        use std::io::Write;
        let full = self.root.join(path);
        let mut file = std::fs::OpenOptions::new().append(true).create(true).open(full)?;
        writeln!(file, "{content}")?;
        Ok(())
    }

    fn list_files(&self) -> anyhow::Result<Vec<String>> {
        let mut files = Vec::new();
        for entry in walkdir::WalkDir::new(&self.root)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        {
            let rel = entry
                .path()
                .strip_prefix(&self.root)
                .unwrap_or_else(|_| entry.path())
                .to_string_lossy()
                .to_string();
            files.push(rel);
        }
        Ok(files)
    }
}
