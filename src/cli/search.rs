//! enjoyflow search — 知识检索

use crate::knowledge::source::KnowledgeSource;

pub fn run(
    query: &str,
    class: &[String],
    tag: &[String],
    archive: bool,
    root: &std::path::Path,
) -> anyhow::Result<()> {
    let search_query = crate::knowledge::types::SearchQuery {
        text: query.to_string(),
        class: class.to_vec(),
        tags: tag.to_vec(),
        include_archive: archive,
    };

    let source = crate::knowledge::filesystem::FilesystemSource::new(
        root.join(".enjoyflow").join("knowledge-base"),
    );
    // TODO: 同时搜索 knowledge-tasks/

    let results = source.search(&search_query)?;

    for r in &results {
        if r.section.is_empty() {
            println!("{}", r.file);
        } else {
            println!("{}##{}", r.file, r.section);
        }
        if !r.snippet.is_empty() {
            println!("  {}", r.snippet);
        }
    }

    if results.is_empty() {
        println!("(无匹配结果)");
    }

    Ok(())
}
