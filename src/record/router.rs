/// record 命令的路由逻辑
use crate::knowledge::types::RecordType;

/// 根据记录类型和参数路由到目标文件
pub fn route(
    record_type: RecordType,
    task: Option<&str>,
    _tag: &[String],
    _content: &str,
    root: &std::path::Path,
) -> anyhow::Result<String> {
    let _base = root.join(".enjoyflow");

    let rel_path = match record_type {
        RecordType::Gotcha => {
            if let Some(task_id) = task {
                format!("knowledge-tasks/{}/gotchas.md", task_id)
            } else {
                "knowledge-base/development/GOTCHAS.md".into()
            }
        }
        RecordType::Pattern => "knowledge-base/development/PATTERNS.md".into(),
        RecordType::Decision => {
            let task_id = task.unwrap_or("unknown");
            format!("knowledge-tasks/{}/adr.md", task_id)
        }
    };

    Ok(rel_path)
}
