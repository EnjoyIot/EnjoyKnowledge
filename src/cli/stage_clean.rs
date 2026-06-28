//! `enjoyknowledge stage clean` — TTL-based cleanup of archived stage tasks.

use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Canonical directory for stage archives.
const STAGE_DIR: &str = ".enjoyknowledge_stage";

pub fn run(
    project_root: &Path,
    dry_run: bool,
    force: bool,
    older_than_days: Option<u64>,
) -> anyhow::Result<()> {
    let ttl_days = older_than_days.unwrap_or(180);
    let archive_dir = project_root.join(STAGE_DIR).join(".archive").join("tasks");

    if !archive_dir.exists() {
        eprintln!("enjoyknowledge: no archived tasks to clean");
        return Ok(());
    }

    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(ttl_days * 24 * 3600))
        .unwrap_or(UNIX_EPOCH);

    let mut to_delete: Vec<(String, SystemTime)> = Vec::new();
    let mut total_dirs = 0usize;

    for entry in std::fs::read_dir(&archive_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        total_dirs += 1;
        let name = entry.file_name().to_string_lossy().to_string();

        // Find the newest file in this task directory
        let newest = find_newest_mtime(&path).unwrap_or(UNIX_EPOCH);

        if newest < cutoff {
            to_delete.push((name, newest));
        }
    }

    if to_delete.is_empty() {
        println!(
            "enjoyknowledge: no archived tasks older than {ttl_days} days ({total_dirs} total)"
        );
        return Ok(());
    }

    // Sort by mtime (oldest first)
    to_delete.sort_by_key(|(_, mtime)| *mtime);

    let count = to_delete.len();
    if dry_run {
        println!("Would delete {count} archived task(s) older than {ttl_days} days:");
        for (name, mtime) in &to_delete {
            let age = age_days(*mtime);
            println!("  {name} (age: ~{age}d)");
        }
        println!("Total: {count} task(s) would be removed.");
        return Ok(());
    }

    if !force {
        eprintln!(
            "enjoyknowledge: {count} archived task(s) older than {ttl_days} days — use --force to delete, or --dry-run to preview"
        );
        for (name, mtime) in &to_delete {
            let age = age_days(*mtime);
            eprintln!("  {name} (age: ~{age}d)");
        }
        return Ok(());
    }

    for (name, _) in &to_delete {
        let task_path = archive_dir.join(name);
        std::fs::remove_dir_all(&task_path)
            .map_err(|e| anyhow::anyhow!("failed to remove {name}: {e}"))?;
    }

    println!("enjoyknowledge: cleaned {count} archived task(s) older than {ttl_days} days");
    Ok(())
}

/// Find the newest modified time of any file in a directory tree.
fn find_newest_mtime(dir: &Path) -> Option<SystemTime> {
    let mut newest: Option<SystemTime> = None;
    for entry in
        walkdir::WalkDir::new(dir).max_depth(10).into_iter().filter_map(std::result::Result::ok)
    {
        if entry.file_type().is_file() {
            if let Ok(meta) = entry.metadata() {
                if let Ok(mtime) = meta.modified() {
                    newest = Some(match newest {
                        Some(cur) if mtime > cur => mtime,
                        Some(cur) => cur,
                        None => mtime,
                    });
                }
            }
        }
    }
    newest
}

/// Calculate approximate age in days from a `SystemTime`.
fn age_days(t: SystemTime) -> u64 {
    SystemTime::now().duration_since(t).map(|d| d.as_secs() / 86400).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn find_newest_mtime_returns_some_for_dir_with_files() {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path();
        std::fs::write(root.join("a.txt"), "a").unwrap();
        sleep(Duration::from_millis(10));
        std::fs::write(root.join("b.txt"), "b").unwrap();

        let newest = find_newest_mtime(root).unwrap();
        // Should be at least as new as the most recent file
        let b_meta = std::fs::metadata(root.join("b.txt")).unwrap();
        let b_mtime = b_meta.modified().unwrap();
        assert!(newest >= b_mtime);
    }

    #[test]
    fn find_newest_mtime_returns_none_for_empty_dir() {
        let tmp = tempfile::TempDir::new().unwrap();
        assert!(find_newest_mtime(tmp.path()).is_none());
    }

    #[test]
    fn age_days_is_zero_for_recent_time() {
        let recent = SystemTime::now();
        assert_eq!(age_days(recent), 0);
    }
}
