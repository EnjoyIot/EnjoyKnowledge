/// YAML frontmatter parsing (OKF-compatible).
use serde::{Deserialize, Serialize};

/// Frontmatter metadata block for a knowledge file.
///
/// Classification is derived from the directory name, not stored in frontmatter.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    /// Human-readable title (defaults to filename when absent).
    pub title: Option<String>,
    /// One-line summary ≤ 200 chars — the most important index field.
    pub description: Option<String>,
    /// Cross-category tags (lowercase + hyphens).
    #[serde(default)]
    pub tags: Vec<String>,
    /// ISO 8601 date (YYYY-MM-DD).
    pub timestamp: Option<String>,
}

/// Extract YAML frontmatter from Markdown content.
/// Frontmatter must be at the very start, wrapped with `---`.
pub fn parse_frontmatter(content: &str) -> Option<Frontmatter> {
    let content = content.trim_start();
    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return None;
    }

    let after_first = &content[3..];
    let end = after_first.find("\n---")?;

    let yaml_str = &after_first[..end];
    serde_yaml::from_str(yaml_str).ok()
}

/// Generate minimal frontmatter for a newly-created file.
pub fn generate_frontmatter(description: &str) -> String {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    format!("---\ndescription: {description}\ntimestamp: {today}\n---\n\n")
}

/// Update the `timestamp` field in existing frontmatter in-place.
/// Returns the new full content, or the original if no frontmatter exists.
pub fn update_timestamp(content: &str) -> String {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let re = regex::Regex::new(r"(?m)^timestamp:\s*.*$").unwrap();
    if re.is_match(content) {
        re.replace(content, format!("timestamp: {today}")).to_string()
    } else {
        content.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let input = "---\ndescription: export gotchas\ntags:\n  - export\n  - excel\ntimestamp: 2026-06-20\n---\n\n# Title\nContent";
        let fm = parse_frontmatter(input).unwrap();
        assert_eq!(fm.description.unwrap(), "export gotchas");
        assert_eq!(fm.tags, vec!["export", "excel"]);
        assert_eq!(fm.timestamp.unwrap(), "2026-06-20");
    }

    #[test]
    fn test_generate_frontmatter() {
        let fm = generate_frontmatter("test description");
        assert!(fm.contains("description: test description"));
        assert!(fm.contains("timestamp:"));
    }
}
