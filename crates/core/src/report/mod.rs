//! Report generation module

pub mod html;
pub mod template_renderer;

/// Assets for offline report generation (embedded resources)
#[derive(Debug, Clone, Default)]
pub struct OfflineAssets {
    /// Tailwind CSS JS content (minified)
    pub tailwind_js: String,
    /// Chart.js content (minified) for offline charts
    pub chart_js: String,
    /// Office image as base64 encoded string (used in closing slide)
    pub office_image_base64: String,
}

impl OfflineAssets {
    /// Load assets embedded in the binary (or from filesystem relative to source)
    /// This ensures reports can be generated offline without making external requests.
    pub fn load_embedded() -> Self {
        // Use include_str! to embed at compile time
        // Paths are relative to this file (crates/core/src/report/mod.rs)
        Self {
            tailwind_js: include_str!("../../assets/tailwind.js").to_string(),
            chart_js: include_str!("../../assets/chart.min.js").to_string(),
            office_image_base64: include_str!("../../assets/cover_image_base64.txt").to_string(),
        }
    }
}
