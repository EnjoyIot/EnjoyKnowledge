/// 搜索查询与结果类型
/// 搜索条件
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// 自由文本查询（匹配标题 + 正文前 200 字符 + frontmatter description）
    pub text: String,
    /// class 过滤（AND 逻辑，空 = 不过滤）
    pub class: Vec<String>,
    /// tag 过滤（AND 逻辑，空 = 不过滤）
    pub tags: Vec<String>,
    /// 是否包含归档目录
    pub include_archive: bool,
}

/// 单条搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 文件路径
    pub file: String,
    /// ## 段标题
    pub section: String,
    /// 匹配行 ± 3 行上下文
    pub snippet: String,
}

/// record 命令的记录类型
#[derive(Debug, Clone, Copy)]
pub enum RecordType {
    Gotcha,
    Pattern,
    Decision,
}
