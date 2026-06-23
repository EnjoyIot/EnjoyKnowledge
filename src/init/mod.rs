/// Init module — wires core traits to the CLI handler.
///
/// # Extension points
///
/// - **Profile**: create a new struct implementing `core::Profile` in `src/profile/`
///   and register it here for `enjoyknowledge init --profile <name>`.
/// - **`TemplateProvider`**: implement `core::TemplateProvider` in `src/template/`
///   to load templates from git, a registry, or any other source.
pub mod ai_tools;
pub mod skeleton;
pub mod templates;

use crate::core::Profile;

/// Return the default profile (`for-coding`).
#[allow(dead_code)]
pub const fn default_profile() -> crate::profile::coding::CodingProfile {
    crate::profile::coding::CodingProfile
}

/// Return the default template provider (filesystem).
pub const fn default_template_provider() -> crate::template::filesystem::FilesystemTemplateProvider
{
    crate::template::filesystem::FilesystemTemplateProvider
}

/// Resolve a named profile.
///
/// Currently only `for-coding` is shipped. Future profiles can be added here.
pub fn resolve_profile(name: &str) -> Option<Box<dyn Profile>> {
    match name.to_lowercase().as_str() {
        "for-coding" | "coding" => Some(Box::new(crate::profile::coding::CodingProfile)),
        _ => None,
    }
}
