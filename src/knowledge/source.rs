/// 知识源统一接口
use super::types::{SearchQuery, SearchResult};

/// 所有知识源必须实现此 trait
pub trait KnowledgeSource {
    /// 按条件搜索，返回匹配结果列表
    fn search(&self, query: &SearchQuery) -> anyhow::Result<Vec<SearchResult>>;

    /// 读取文件全部内容
    fn read_file(&self, path: &str) -> anyhow::Result<String>;

    /// 追加内容到文件末尾
    fn append_to_file(&self, path: &str, content: &str) -> anyhow::Result<()>;

    /// 列出所有文件路径
    fn list_files(&self) -> anyhow::Result<Vec<String>>;
}
