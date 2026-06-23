/// `TemplateProvider` trait — resolves named templates so init can scaffold
/// from a pre-built knowledge base skeleton.
///
/// # Implementing a custom provider
///
/// ```rust
/// use std::path::PathBuf;
/// use enjoyknowledge::core::TemplateProvider;
///
/// struct GitTemplateProvider;
///
/// impl TemplateProvider for GitTemplateProvider {
///     fn resolve(&self, name: &str) -> Option<PathBuf> {
///         // Clone a template repo and return the local path
///         None
///     }
///     fn list_all(&self) -> Vec<String> { vec![] }
/// }
/// ```
#[allow(dead_code)]
pub trait TemplateProvider {
    /// Return the filesystem path of `name`, or `None` if not found.
    fn resolve(&self, name: &str) -> Option<std::path::PathBuf>;

    /// List all available template names.
    fn list_all(&self) -> Vec<String>;
}
