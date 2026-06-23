/// Filesystem-backed template provider.
///
/// Resolve order:
/// 1. `~/.enjoyknowledge/templates/<name>/` (global, shared across projects)
/// 2. `.enjoyknowledge/templates/<name>/`   (local, project-specific)
use crate::core::TemplateProvider;
use std::path::{Path, PathBuf};

pub struct FilesystemTemplateProvider;

impl TemplateProvider for FilesystemTemplateProvider {
    fn resolve(&self, name: &str) -> Option<PathBuf> {
        // 1. Global: ~/.enjoyknowledge/templates/<name>/
        if let Some(home) = dirs::home_dir() {
            let global = home.join(".enjoyknowledge").join("templates").join(name);
            if global.is_dir() {
                return Some(global);
            }
        }

        // 2. Local: .enjoyknowledge/templates/<name>/
        let local = Path::new(".enjoyknowledge").join("templates").join(name);
        if local.is_dir() {
            return Some(local);
        }

        None
    }

    fn list_all(&self) -> Vec<String> {
        let mut names = Vec::new();

        // Global templates
        if let Some(home) = dirs::home_dir() {
            let global = home.join(".enjoyknowledge").join("templates");
            if let Ok(entries) = std::fs::read_dir(&global) {
                for entry in entries.flatten() {
                    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        names.push(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
        }

        // Local templates
        let local = Path::new(".enjoyknowledge").join("templates");
        if let Ok(entries) = std::fs::read_dir(&local) {
            for entry in entries.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !names.contains(&name) {
                        names.push(name);
                    }
                }
            }
        }

        names.sort();
        names
    }
}
