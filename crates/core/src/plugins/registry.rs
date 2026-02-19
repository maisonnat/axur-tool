//! Plugin Registry
//!
//! Manages registration and execution of plugins.

use super::traits::{
    CloudExportPlugin, DataPlugin, ExportPlugin, PluginContext, SlideOutput, SlidePlugin,
};
use crate::api::report::PocReportData;

/// Central registry for all plugins
pub struct PluginRegistry {
    slide_plugins: Vec<Box<dyn SlidePlugin>>,
    data_plugins: Vec<Box<dyn DataPlugin>>,
    export_plugins: Vec<Box<dyn ExportPlugin>>,
    cloud_export_plugins: Vec<Box<dyn CloudExportPlugin>>,
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginRegistry {
    /// Create an empty registry
    pub fn new() -> Self {
        Self {
            slide_plugins: Vec::new(),
            data_plugins: Vec::new(),
            export_plugins: Vec::new(),
            cloud_export_plugins: Vec::new(),
        }
    }

    /// Create a registry with builtin plugins
    pub fn with_builtins() -> Self {
        use super::builtin::*;

        let mut registry = Self::new();
        registry.register_slide(Box::new(CoverSlidePlugin));
        registry.register_slide(Box::new(IntroSlidePlugin));
        registry.register_slide(Box::new(SolutionsSlidePlugin));
        registry.register_slide(Box::new(TocSlidePlugin));
        registry.register_slide(Box::new(PocDataSlidePlugin));
        registry.register_slide(Box::new(MetricsSlidePlugin));
        registry.register_slide(Box::new(ComparativeSlidePlugin)); // NEW: Comparative analysis
        registry.register_slide(Box::new(TimelineSlidePlugin));
        registry.register_slide(Box::new(ThreatsSlidePlugin));
        registry.register_slide(Box::new(ViralitySlidePlugin));
        registry.register_slide(Box::new(AiIntentSlidePlugin));
        registry.register_slide(Box::new(DataExposureSlidePlugin));
        registry.register_slide(Box::new(GeospatialSlidePlugin));
        registry.register_slide(Box::new(HeatmapSlidePlugin)); // NEW: Attack heatmap
        registry.register_slide(Box::new(RadarSlidePlugin)); // NEW: Threat radar
        registry.register_slide(Box::new(IncidentsSlidePlugin));
        registry.register_slide(Box::new(TakedownsSlidePlugin));
        registry.register_slide(Box::new(KillChainSlidePlugin)); // NEW: Kill Chain Timeline
        registry.register_slide(Box::new(VelocitySlidePlugin)); // NEW: Takedown Velocity
        registry.register_slide(Box::new(CredentialsSlidePlugin));
        registry.register_slide(Box::new(RoiSlidePlugin));
        registry.register_slide(Box::new(ThreatIntelSlidePlugin));
        registry.register_slide(Box::new(TakedownExamplesSlidePlugin));
        registry.register_slide(Box::new(PocExamplesSlidePlugin));
        registry.register_slide(Box::new(InsightsSlidePlugin)); // NEW: Insights & Recommendations
        registry.register_slide(Box::new(StyleShowcasePlugin)); // NEW: Style Showcase
        registry.register_slide(Box::new(ClosingSlidePlugin));
        registry
    }

    /// Register a slide plugin
    pub fn register_slide(&mut self, plugin: Box<dyn SlidePlugin>) {
        self.slide_plugins.push(plugin);
        // Sort by priority (descending) for correct ordering
        self.slide_plugins.sort_by_key(|p| -p.priority());
    }

    /// Register a data transformation plugin
    pub fn register_data(&mut self, plugin: Box<dyn DataPlugin>) {
        self.data_plugins.push(plugin);
        self.data_plugins.sort_by_key(|p| -p.priority());
    }

    /// Register an export format plugin
    pub fn register_export(&mut self, plugin: Box<dyn ExportPlugin>) {
        self.export_plugins.push(plugin);
    }

    /// Register a cloud export plugin
    pub fn register_cloud_export(&mut self, plugin: Box<dyn CloudExportPlugin>) {
        self.cloud_export_plugins.push(plugin);
    }

    /// Get list of registered slide plugins
    pub fn slide_plugins(&self) -> &[Box<dyn SlidePlugin>] {
        &self.slide_plugins
    }

    /// Get list of registered data plugins
    pub fn data_plugins(&self) -> &[Box<dyn DataPlugin>] {
        &self.data_plugins
    }

    /// Get export plugin by format
    pub fn export_plugin(&self, format: &str) -> Option<&Box<dyn ExportPlugin>> {
        self.export_plugins.iter().find(|p| p.format() == format)
    }

    /// Get cloud export plugin by provider name
    pub fn cloud_export_plugin(&self, provider: &str) -> Option<&Box<dyn CloudExportPlugin>> {
        self.cloud_export_plugins
            .iter()
            .find(|p| p.provider() == provider)
    }

    /// Get list of available cloud export providers
    pub fn available_cloud_providers(&self) -> Vec<(&'static str, &'static str)> {
        self.cloud_export_plugins
            .iter()
            .map(|p| (p.provider(), p.display_name()))
            .collect()
    }

    /// Run all data plugins to transform report data
    pub fn transform_data(&self, data: &mut PocReportData) {
        for plugin in &self.data_plugins {
            plugin.transform(data);
        }
    }

    /// Generate all slides from registered plugins
    pub fn generate_slides(&self, ctx: &PluginContext) -> Vec<SlideOutput> {
        self.slide_plugins
            .iter()
            .filter(|p| p.is_enabled(ctx))
            .flat_map(|p| p.generate_slides(ctx))
            .collect()
    }

    /// Get count of registered plugins
    pub fn stats(&self) -> RegistryStats {
        RegistryStats {
            slide_plugins: self.slide_plugins.len(),
            data_plugins: self.data_plugins.len(),
            export_plugins: self.export_plugins.len(),
            cloud_export_plugins: self.cloud_export_plugins.len(),
        }
    }
}

/// Statistics about registered plugins
#[derive(Debug)]
pub struct RegistryStats {
    pub slide_plugins: usize,
    pub data_plugins: usize,
    pub export_plugins: usize,
    pub cloud_export_plugins: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_registry() {
        let registry = PluginRegistry::new();
        let stats = registry.stats();
        assert_eq!(stats.slide_plugins, 0);
        assert_eq!(stats.data_plugins, 0);
        assert_eq!(stats.export_plugins, 0);
    }

    #[test]
    fn test_registry_with_builtins() {
        let registry = PluginRegistry::with_builtins();
        // 27 builtin slide plugins registered (incl. StyleShowcase + KillChain + Velocity)
        assert_eq!(registry.slide_plugins().len(), 27);
        // Verify ordering by priority (StyleShowcase=999 should be first, cover=100 second)
        assert_eq!(registry.slide_plugins()[0].id(), "builtin.style_showcase");
        assert_eq!(registry.slide_plugins()[1].id(), "builtin.cover");
        assert_eq!(registry.slide_plugins()[26].id(), "builtin.closing");
    }
}
