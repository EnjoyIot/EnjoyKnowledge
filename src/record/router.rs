/// record 命令的路由逻辑
use crate::knowledge::types::RecordType;

/// 根据记录类型和参数路由到目标文件
#[allow(clippy::unnecessary_wraps)]
pub fn route(
    record_type: RecordType,
    task: Option<&str>,
    _tag: &[String],
    _content: &str,
    root: &std::path::Path,
) -> anyhow::Result<String> {
    let _base = root.join(".enjoyknowledge");

    let rel_path = match record_type {
        RecordType::Gotcha => task.map_or_else(
            || "knowledge-base/development/GOTCHAS.md".into(),
            |task_id| format!("knowledge-tasks/{task_id}/gotchas.md"),
        ),
        RecordType::Pattern => "knowledge-base/development/PATTERNS.md".into(),
        RecordType::Decision => {
            let task_id = task.unwrap_or("unknown");
            format!("knowledge-tasks/{task_id}/adr.md")
        }
    };

    Ok(rel_path)
}
