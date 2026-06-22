/// .enjoyknowledge/config.yaml 配置读写
// stub 阶段，功能尚未接入 CLI
use serde::{Deserialize, Serialize};

/// 项目配置
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: Option<ProjectConfig>,
    pub ai_tool: Option<String>,
    pub sources: Option<Vec<SourceConfig>>,
    pub class_labels: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProjectConfig {
    pub name: Option<String>,
    pub tech_stack: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(dead_code)]
pub enum SourceConfig {
    #[serde(rename = "filesystem")]
    Filesystem { path: String },
    #[serde(rename = "git")]
    Git {
        url: String,
        #[serde(default = "default_branch")]
        branch: String,
    },
    #[serde(rename = "mcp")]
    Mcp { server: String },
}

#[allow(dead_code)]
fn default_branch() -> String {
    "main".into()
}

impl Config {
    #[allow(dead_code)]
    /// 加载项目 .enjoyknowledge/config.yaml
    pub fn load(root: &std::path::Path) -> anyhow::Result<Self> {
        let path = root.join(".enjoyknowledge").join("config.yaml");
        let content = std::fs::read_to_string(&path)?;
        Ok(serde_yaml::from_str(&content)?)
    }

    /// 写回 config.yaml
    #[allow(dead_code)]
    pub fn save(&self, root: &std::path::Path) -> anyhow::Result<()> {
        let path = root.join(".enjoyknowledge").join("config.yaml");
        let content = serde_yaml::to_string(self)?;
        Ok(std::fs::write(path, content)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: None,
            ai_tool: Some("auto".into()),
            sources: Some(vec![SourceConfig::Filesystem {
                path: ".enjoyknowledge/knowledge-base/".into(),
            }]),
            class_labels: None,
        }
    }
}
