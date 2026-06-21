/// .enjoyflow/config.yaml 配置读写
use serde::{Deserialize, Serialize};

/// 项目配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: Option<ProjectConfig>,
    pub ai_tool: Option<String>,
    pub sources: Option<Vec<SourceConfig>>,
    pub class_labels: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: Option<String>,
    pub tech_stack: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
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

fn default_branch() -> String {
    "main".into()
}

impl Config {
    /// 加载项目 .enjoyflow/config.yaml
    pub fn load(root: &std::path::Path) -> anyhow::Result<Self> {
        let path = root.join(".enjoyflow").join("config.yaml");
        let content = std::fs::read_to_string(&path)?;
        Ok(serde_yaml::from_str(&content)?)
    }

    /// 写回 config.yaml
    pub fn save(&self, root: &std::path::Path) -> anyhow::Result<()> {
        let path = root.join(".enjoyflow").join("config.yaml");
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
                path: ".enjoyflow/knowledge-base/".into(),
            }]),
            class_labels: None,
        }
    }
}
