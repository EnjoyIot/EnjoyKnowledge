//! `enjoyknowledge cat` — read a knowledge file's full content.
use crate::knowledge::KnowledgeSource;

pub fn run(source: &dyn KnowledgeSource, path: &str) {
    match source.read_file(path) {
        Ok(content) => {
            print!("{content}");
        }
        Err(e) => {
            eprintln!("enjoyknowledge: {path}: {e}");
            std::process::exit(2);
        }
    }
}
