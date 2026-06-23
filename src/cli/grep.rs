//! `enjoyknowledge grep` — structure-aware search.
use crate::knowledge::{KnowledgeSource, SearchQuery};

pub fn run(
    source: &dyn KnowledgeSource,
    pattern: &str,
    type_filter: &[String],
    tags: &[String],
    path: Option<&str>,
    include_archive: bool,
    req: Option<&str>,
) -> anyhow::Result<()> {
    let query = SearchQuery {
        pattern: pattern.to_string(),
        type_filter: type_filter.to_vec(),
        tags: tags.to_vec(),
        path: path.map(String::from),
        include_archive: include_archive || req.is_some(),
        req: req.map(String::from),
    };

    let results = source.search(&query)?;

    if results.is_empty() {
        std::process::exit(1);
    }

    for result in &results {
        if result.section.is_empty() {
            println!("{}", result.file);
        } else {
            println!("{}##{}", result.file, result.section);
        }
        // Print snippet indented
        for line in result.snippet.lines() {
            println!("  {line}");
        }
    }

    Ok(())
}
