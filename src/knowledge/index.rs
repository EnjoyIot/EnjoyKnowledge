/// .enjoyflow/.index.json 惰性索引
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// 索引结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Index {
    /// class → 文件列表
    pub by_class: HashMap<String, Vec<String>>,
    /// tag → 文件列表
    pub by_tag: HashMap<String, Vec<String>>,
    /// 上次重建时间 (ISO 8601)
    pub last_rebuilt: Option<String>,
}

impl Index {
    /// 从 .enjoyflow/.index.json 加载
    pub fn load(root: &Path) -> anyhow::Result<Option<Self>> {
        let path = root.join(".enjoyflow").join(".index.json");
        if !path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(path)?;
        Ok(Some(serde_json::from_str(&content)?))
    }

    /// 保存到 .enjoyflow/.index.json
    pub fn save(&self, root: &Path) -> anyhow::Result<()> {
        let path = root.join(".enjoyflow").join(".index.json");
        let content = serde_json::to_string_pretty(self)?;
        Ok(std::fs::write(path, content)?)
    }

    /// 按 class 查询文件列表
    pub fn files_by_class(&self, class: &str) -> Vec<&str> {
        self.by_class
            .get(class)
            .map(|v| v.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    /// 按 tag 查询文件列表
    pub fn files_by_tag(&self, tag: &str) -> Vec<&str> {
        self.by_tag
            .get(tag)
            .map(|v| v.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }

    /// 增量更新：添加 class → file 映射
    pub fn add_class_file(&mut self, class: &str, file: &str) {
        self.by_class
            .entry(class.to_string())
            .or_default()
            .push(file.to_string());
    }

    /// 增量更新：添加 tag → file 映射
    pub fn add_tag_file(&mut self, tag: &str, file: &str) {
        self.by_tag
            .entry(tag.to_string())
            .or_default()
            .push(file.to_string());
    }
}
