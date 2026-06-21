/// --scan 功能：扫描已有项目，提取技术栈信息
use std::path::Path;

/// 扫描结果
#[derive(Debug, Default)]
pub struct ScanResult {
    pub tech_stack: Vec<String>,
    pub has_package_json: bool,
    pub has_pom_xml: bool,
    pub has_requirements_txt: bool,
    pub has_cargo_toml: bool,
}

/// 扫描项目根目录，识别技术栈
pub fn scan_project(root: &Path) -> ScanResult {
    let mut result = ScanResult::default();

    result.has_package_json = root.join("package.json").exists();
    if result.has_package_json {
        result.tech_stack.push("Node.js".into());
        // 尝试检测框架
        if let Ok(content) = std::fs::read_to_string(root.join("package.json")) {
            if content.contains("\"next\"") {
                result.tech_stack.push("Next.js".into());
            } else if content.contains("\"vue\"") {
                result.tech_stack.push("Vue".into());
            } else if content.contains("\"react\"") {
                result.tech_stack.push("React".into());
            }
        }
    }

    result.has_pom_xml = root.join("pom.xml").exists();
    if result.has_pom_xml {
        result.tech_stack.push("Java/Maven".into());
    }

    result.has_requirements_txt = root.join("requirements.txt").exists();
    if result.has_requirements_txt {
        result.tech_stack.push("Python".into());
    }

    result.has_cargo_toml = root.join("Cargo.toml").exists();
    if result.has_cargo_toml {
        result.tech_stack.push("Rust".into());
    }

    result
}
