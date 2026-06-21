//! enjoyflow init — 项目初始化

use crate::init::{ai_tools, skeleton};
use std::path::Path;

pub fn run(
    path: Option<&str>,
    scan: bool,
    describe: Option<&str>,
    link: Option<&str>,
    ai: Option<&str>,
) -> anyhow::Result<()> {
    let root = Path::new(path.unwrap_or("."));

    if let Some(link_path) = link {
        // --link 模式：只生成 AI 工具文件，不创建 .enjoyflow/
        let tool = ai_tools::AiTool::from_str(ai.unwrap_or("auto"));
        ai_tools::generate_agents_md(root)?;
        ai_tools::generate_tool_files(root, tool)?;
        println!("✓ 已链接到 {}", link_path);
        return Ok(());
    }

    // 标准 init
    if scan {
        let result = crate::init::scan::scan_project(root);
        println!("扫描结果: {:?}", result.tech_stack);
    }

    if let Some(_desc) = describe {
        #[cfg(feature = "llm")]
        {
            let proposal = crate::init::describe::describe_to_proposal(desc, root)?;
            println!("{}", proposal);
        }
        #[cfg(not(feature = "llm"))]
        {
            anyhow::bail!("需要启用 'llm' feature 才能使用 --describe");
        }
    }

    skeleton::generate(root)?;
    let tool = ai_tools::AiTool::from_str(ai.unwrap_or("auto"));
    ai_tools::generate_agents_md(root)?;
    ai_tools::generate_tool_files(root, tool)?;
    ai_tools::update_gitignore(root)?;

    println!("✓ EnjoyFlow 已初始化");
    Ok(())
}
