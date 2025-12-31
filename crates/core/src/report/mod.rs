//! Report generation module

pub mod html;
pub mod template_renderer;

/// Assets for offline report generation (embedded resources)
#[derive(Debug, Clone, Default)]
pub struct OfflineAssets {
    /// Tailwind CSS JS content (minified)
    pub tailwind_js: String,
    /// Office image as base64 encoded string
    pub office_image_base64: String,
}
