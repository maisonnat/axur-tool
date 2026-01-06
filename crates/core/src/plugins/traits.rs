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

/// Configuration options for plugins
#[derive(Debug, Clone, Default)]
pub struct PluginConfig {
    /// Whether this is a PoC (static) or production (dynamic) report
    pub is_poc: bool,
    /// Show compliance/regulatory slides
    pub show_compliance: bool,
    /// Custom branding enabled
    pub custom_branding: bool,
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
