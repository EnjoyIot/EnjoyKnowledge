/// YAML frontmatter 解析
use serde::{Deserialize, Serialize};

/// 知识文档的 frontmatter 结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    pub class: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub last_modified: Option<String>,
    pub description: Option<String>,
}

/// 从 Markdown 内容中提取 YAML frontmatter
///
/// frontmatter 必须位于文件开头，由 `---` 包裹。
pub fn parse_frontmatter(content: &str) -> Option<Frontmatter> {
    let content = content.trim_start();
    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return None;
    }

    // 找到第二个 ---
    let after_first = &content[3..]; // skip first ---
    let end = after_first.find("\n---")?;

    let yaml_str = &after_first[..end];
    serde_yaml::from_str(yaml_str).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let input = "---\nclass: C1_gotchas\ntags:\n  - export\n  - excel\nlast_modified: 2026-06-20\n---\n\n# Title\nContent";
        let fm = parse_frontmatter(input).unwrap();
        assert_eq!(fm.class.unwrap(), "C1_gotchas");
        assert_eq!(fm.tags, vec!["export", "excel"]);
        assert_eq!(fm.last_modified.unwrap(), "2026-06-20");
    }
}
