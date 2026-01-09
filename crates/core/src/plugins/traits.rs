//! Plugin Traits for Extensible Report Generation
//!
//! This module defines the interfaces for extending report functionality
//! without modifying core code.

use crate::api::report::PocReportData;
use crate::i18n::Translations;

/// Output from a slide plugin
#[derive(Debug, Clone)]
pub struct SlideOutput {
    /// Unique identifier for the slide (e.g., "metrics", "takedowns")
    pub id: String,
    /// HTML content of the slide
    pub html: String,
}

/// Context available to all plugins during generation
pub struct PluginContext<'a> {
    /// Report data from API
    pub data: &'a PocReportData,
    /// Translations (new i18n system)
    pub translations: &'a Translations,
    /// Tenant name for customization
    pub tenant_name: &'a str,
    /// Report configuration options
    pub config: PluginConfig,
}

/// Theme mode for report generation
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ThemeMode {
    /// Dark theme (default) - Axur.com style
    #[default]
    Dark,
    /// Light theme
    Light,
    /// Auto detection based on data
    Auto,
}

/// Configuration options for plugins
#[derive(Debug, Clone, Default)]
pub struct PluginConfig {
    /// Whether this is a PoC (static) or production (dynamic) report
    pub is_poc: bool,
    /// Show compliance/regulatory slides
    pub show_compliance: bool,
    /// Custom branding enabled
    pub custom_branding: bool,
    /// Theme mode (dark/light/auto)
    pub theme: ThemeMode,
    /// List of plugin IDs to disable
    pub disabled_plugins: Vec<String>,
    /// Custom CSS to inject
    pub custom_css: Option<String>,
}

impl PluginConfig {
    /// Create a new config with default dark theme
    pub fn new() -> Self {
        Self::default()
    }

    /// Set theme mode
    pub fn with_theme(mut self, theme: ThemeMode) -> Self {
        self.theme = theme;
        self
    }

    /// Disable specific plugins by ID
    pub fn disable_plugins(mut self, plugins: Vec<String>) -> Self {
        self.disabled_plugins = plugins;
        self
    }

    /// Check if a plugin is enabled
    pub fn is_plugin_enabled(&self, plugin_id: &str) -> bool {
        !self.disabled_plugins.contains(&plugin_id.to_string())
    }
}

/// Plugin that generates one or more slides for the report
pub trait SlidePlugin: Send + Sync {
    /// Unique identifier for this plugin (e.g., "builtin.metrics")
    fn id(&self) -> &'static str;

    /// Human-readable name
    fn name(&self) -> &'static str;

    /// Generate slides based on report data
    /// Returns empty vec if no slides should be generated
    fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput>;

    /// Priority determines ordering (higher = earlier in report)
    /// Default priorities:
    /// - 100: Cover/intro
    /// - 90: Metrics/overview
    /// - 80: Main content
    /// - 50: Details
    /// - 20: Examples
    /// - 10: Closing
    fn priority(&self) -> i32 {
        50
    }

    /// Check if this plugin should be active for the current context
    /// Override to conditionally enable/disable based on data or config
    fn is_enabled(&self, _ctx: &PluginContext) -> bool {
        true
    }
}

/// Plugin that transforms report data before slide generation
pub trait DataPlugin: Send + Sync {
    /// Unique identifier
    fn id(&self) -> &'static str;

    /// Transform/enrich the report data
    fn transform(&self, data: &mut PocReportData);

    /// Priority (higher = runs first)
    fn priority(&self) -> i32 {
        0
    }
}

/// Plugin that provides custom export formats
pub trait ExportPlugin: Send + Sync {
    /// Unique identifier
    fn id(&self) -> &'static str;

    /// Format identifier (e.g., "pdf", "pptx")
    fn format(&self) -> &'static str;

    /// Export slides to bytes
    fn export(&self, slides: &[SlideOutput]) -> Result<Vec<u8>, String>;
}

// =====================================================
// CLOUD EXPORT PLUGINS
// =====================================================

/// Output from a cloud export plugin
#[derive(Debug, Clone)]
pub struct CloudExportOutput {
    /// Public URL to the exported presentation
    pub url: String,
    /// Cloud provider identifier (e.g., "google_slides", "onedrive")
    pub provider: String,
    /// Provider's resource ID (e.g., Google Slides presentation ID)
    pub resource_id: String,
    /// Number of slides exported
    pub slides_count: usize,
}

/// Plugin that exports to cloud services (returns URLs, not bytes)
///
/// This trait is designed for integrations with cloud presentation services
/// like Google Slides, Microsoft OneDrive/PowerPoint Online, Canva, etc.
///
/// Unlike `ExportPlugin` which returns raw bytes, cloud exports return
/// a URL to the created presentation.
pub trait CloudExportPlugin: Send + Sync {
    /// Unique identifier (e.g., "builtin.export.google_slides")
    fn id(&self) -> &'static str;

    /// Cloud provider name (e.g., "google_slides", "onedrive")
    fn provider(&self) -> &'static str;

    /// Human-readable name for UI display
    fn display_name(&self) -> &'static str;

    /// Export slides to the cloud service
    ///
    /// # Arguments
    /// * `slides` - The generated slide outputs from SlidePlugins
    /// * `title` - Title for the presentation
    /// * `services` - Optional service context (injected by backend)
    ///
    /// # Returns
    /// * `Ok(CloudExportOutput)` - URL and metadata of created presentation
    /// * `Err(String)` - Error message if export failed
    fn export(&self, slides: &[SlideOutput], title: &str) -> Result<CloudExportOutput, String>;
}
