/// 目录骨架生成
use std::path::Path;

/// 生成 .enjoyflow/ 目录骨架
///
/// 结构：
/// ```text
/// .enjoyflow/
/// ├── config.yaml
/// ├── knowledge-base/
/// │   ├── project/
/// │   ├── contract/
/// │   ├── business/
/// │   ├── development/
/// │   ├── testing/
/// │   ├── deployment/
/// │   ├── shared/
/// │   └── context/
/// ├── knowledge-tasks/
/// ├── AGENTS.md
/// └── .index.json
/// ```
pub fn generate(root: &Path) -> anyhow::Result<()> {
    let ef = root.join(".enjoyflow");
    std::fs::create_dir_all(&ef)?;

    // knowledge-base/ 子目录
    let kb = ef.join("knowledge-base");
    let dirs = [
        "project",
        "contract",
        "business",
        "development",
        "testing",
        "deployment",
        "shared",
        "context",
    ];
    for d in &dirs {
        std::fs::create_dir_all(kb.join(d))?;
    }

    // knowledge-tasks/
    std::fs::create_dir_all(ef.join("knowledge-tasks"))?;

    // 最小模板文件
    write_if_missing(&kb.join("project/ARCHITECTURE.md"), ARCHITECTURE_TEMPLATE)?;
    write_if_missing(
        &kb.join("project/CODE-STANDARDS.md"),
        CODE_STANDARDS_TEMPLATE,
    )?;
    write_if_missing(&kb.join("development/GOTCHAS.md"), GOTCHAS_TEMPLATE)?;

    Ok(())
}

fn write_if_missing(path: &Path, content: &str) -> anyhow::Result<()> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
    }
    Ok(())
}

const ARCHITECTURE_TEMPLATE: &str = "---\nclass: A1_architecture\ntags:\n  - architecture\nlast_modified: \n---\n\n# 项目架构\n\n> 记录项目整体架构、模块拓扑、技术选型。\n\n## 技术栈\n\n## 模块划分\n\n## 部署拓扑\n";

const CODE_STANDARDS_TEMPLATE: &str = "---\nclass: A2_code_standards\ntags:\n  - code-standards\nlast_modified: \n---\n\n# 代码规范\n\n> 记录项目代码规范、命名约定、最佳实践。\n\n## 命名规范\n\n## 目录结构约定\n\n## 代码风格\n";

const GOTCHAS_TEMPLATE: &str = "---\nclass: C1_gotchas\ntags:\n  - gotchas\nlast_modified: \n---\n\n# 踩坑清单\n\n> 记录开发过程中踩过的坑，避免重复。\n";
