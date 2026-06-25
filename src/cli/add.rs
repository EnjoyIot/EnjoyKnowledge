//! `enjoyknowledge add` — append or create a knowledge entry.
use crate::knowledge::KnowledgeSource;
use crate::EK_DIR;
use std::path::Path;

pub fn run(source: &dyn KnowledgeSource, path: &str, content: &str) -> anyhow::Result<()> {
    source.add_entry(path, content)?;

    // Determine action verb for user feedback
    let full = Path::new(EK_DIR).join(path);
    if full.exists() {
        eprintln!("enjoyknowledge: appended to {path}");
    } else {
        eprintln!("enjoyknowledge: created {path}");
    }

    // Sync the AGENTS.md knowledge summary block
    crate::init::skeleton::sync_agents_md_summary(Path::new("."), source)?;

    Ok(())
}
