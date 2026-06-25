//! Markdown 文档结构化
//!
/// search 以 ## 为段界，返回 文件路径##段标题 格式的引用。
/// 解析文档中的 ## 段标题列表
#[allow(dead_code)]
pub fn extract_sections(content: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.starts_with("## ") && !line.starts_with("### "))
        .map(|line| line[3..].trim().to_string())
        .collect()
}

/// 提取 ## 段标题的锚点（用于 search 输出 文件路径##标题）
///
/// `line_num` 是 1-indexed，查找该行及之前的最后一个 ## 标题
pub fn find_section_at_line(content: &str, line_num: usize) -> Option<String> {
    content
        .lines()
        .take(line_num)
        .filter(|l| l.starts_with("## ") && !l.starts_with("### "))
        .last()
        .map(|l| l[3..].trim().to_string())
}

/// Locate the byte offset where the Markdown body begins (after frontmatter `---` block).
pub fn find_body_start(content: &str) -> usize {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---\n") && !trimmed.starts_with("---\r\n") {
        return content.len() - trimmed.len();
    }
    let after_first = &trimmed[3..];
    if let Some(end) = after_first.find("\n---").or_else(|| after_first.find("\r\n---")) {
        let delimiter_len = if after_first[end..].starts_with("\r\n---") { 5 } else { 4 };
        return content.len() - trimmed.len() + 3 + end + delimiter_len;
    }
    content.len() - trimmed.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_sections() {
        let input = "# Title\n\n## Section A\ncontent\n## Section B\nmore\n### Sub B";
        let sections = extract_sections(input);
        assert_eq!(sections, vec!["Section A", "Section B"]);
    }
}
