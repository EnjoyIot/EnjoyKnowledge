//! `enjoyknowledge tree` — recursive tree view of the knowledge base.
use crate::knowledge::KnowledgeSource;

pub fn run(source: &dyn KnowledgeSource, bare: bool) -> anyhow::Result<()> {
    let entries = source.tree_entries(bare)?;

    println!(".enjoyknowledge/");
    print_tree(&entries, "", true, bare);

    Ok(())
}

fn print_tree(
    entries: &[crate::knowledge::KnowledgeEntry],
    prefix: &str,
    _is_last: bool,
    bare: bool,
) {
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let child_prefix = if is_last { "    " } else { "│   " };

        if entry.is_dir {
            let desc = if bare { String::new() } else { entry.path.clone() };
            println!("{prefix}{connector}{desc}/");
            print_tree(&entry.children, &format!("{prefix}{child_prefix}"), is_last, bare);
        } else {
            let desc = if bare {
                entry.path.clone()
            } else {
                match (&entry.description, entry.entry_count) {
                    (Some(d), Some(n)) if n > 1 => {
                        format!("{}  — {} ({} entries)", entry.path, d, n)
                    }
                    (Some(d), _) => format!("{}  — {}", entry.path, d),
                    _ => entry.path.clone(),
                }
            };
            println!("{prefix}{connector}{desc}");
        }
    }
}
