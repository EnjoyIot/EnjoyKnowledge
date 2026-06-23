//! `enjoyknowledge ls` — list knowledge entries with descriptions.
use crate::knowledge::KnowledgeSource;

pub fn run(source: &dyn KnowledgeSource, path: Option<&str>, bare: bool) -> anyhow::Result<()> {
    let entries = source.list_entries(path, bare)?;

    let mut current_dir = String::new();
    let mut dir_files: Vec<&crate::knowledge::KnowledgeEntry> = Vec::new();

    for entry in &entries {
        if entry.is_dir {
            // Flush previous directory group
            if !dir_files.is_empty() {
                for f in &dir_files {
                    print_file(f, bare);
                }
                dir_files.clear();
            }
            current_dir.clone_from(&entry.path);
            if !bare {
                println!("{current_dir}/");
            }
        } else if !current_dir.is_empty() {
            // File under a directory, use directory context from listing
            print_file(entry, bare);
        }
    }

    // Flush remaining
    for f in &dir_files {
        print_file(f, bare);
    }

    Ok(())
}

fn print_file(entry: &crate::knowledge::KnowledgeEntry, bare: bool) {
    if bare {
        println!("  {}", entry.path);
    } else {
        match (&entry.description, entry.entry_count) {
            (Some(desc), Some(n)) if n > 1 => {
                println!("  {}  — {} ({} entries)", entry.path, desc, n);
            }
            (Some(desc), _) => {
                println!("  {}  — {}", entry.path, desc);
            }
            _ => {
                println!("  {}", entry.path);
            }
        }
    }
}
