/// Core abstractions for enjoyknowledge.
///
/// The `core` module defines the traits that every backend, profile, and
/// provider must implement. Concrete implementations live in sibling
/// modules (`profile/`, `template/`, `knowledge/`).
pub mod profile;
pub mod template;

pub use profile::Profile;
pub use template::TemplateProvider;
