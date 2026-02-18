//! Industry Templates for Report Generation
//!
//! Pre-configured slide sets and styling for different industry verticals.
//! Each template defines which plugins to show, color schemes, and terminology.

use crate::plugins::ThemeMode;
use serde::{Deserialize, Serialize};

/// Industry-specific report template
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IndustryTemplate {
    /// General purpose template (default)
    #[default]
    General,
    /// Financial services: credentials, BINs, fraud focus
    Fintech,
    /// Retail/E-commerce: brand abuse, phishing, counterfeits
    Retail,
    /// Healthcare: data leaks, compliance focus
    Healthcare,
}

impl IndustryTemplate {
    /// Parse from string (case-insensitive)
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "fintech" | "finance" | "banking" => Self::Fintech,
            "retail" | "ecommerce" | "e-commerce" => Self::Retail,
            "healthcare" | "health" | "medical" => Self::Healthcare,
            _ => Self::General,
        }
    }

    /// Human-readable name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Fintech => "🏦 Fintech",
            Self::Retail => "🛒 Retail",
            Self::Healthcare => "🏥 Healthcare",
        }
    }

    /// Default plugins to ENABLE for this template
    /// (all others will be disabled)
    pub fn enabled_plugins(&self) -> Vec<&'static str> {
        match self {
            Self::General => vec![
                "builtin.cover",
                "builtin.toc",
                "builtin.metrics",
                "builtin.threats",
                "builtin.takedowns",
                "builtin.roi",
                "builtin.exposure",
                "builtin.geospatial",
                "builtin.evidence",
            ],
            Self::Fintech => vec![
                "builtin.cover",
                "builtin.toc",
                "builtin.metrics",
                "builtin.threats",
                "builtin.exposure", // Credentials are critical
                "builtin.takedowns",
                "builtin.roi",
                // Omit: geospatial (less relevant)
            ],
            Self::Retail => vec![
                "builtin.cover",
                "builtin.toc",
                "builtin.metrics",
                "builtin.threats",
                "builtin.takedowns",  // Brand abuse focus
                "builtin.evidence",   // Screenshots important
                "builtin.geospatial", // Track attack origins
                "builtin.roi",
            ],
            Self::Healthcare => vec![
                "builtin.cover",
                "builtin.toc",
                "builtin.metrics",
                "builtin.exposure", // PHI leaks critical
                "builtin.threats",
                "builtin.takedowns",
                "builtin.roi",
                // Omit: evidence (privacy concerns)
            ],
        }
    }

    /// Plugins to DISABLE for this template
    pub fn disabled_plugins(&self) -> Vec<String> {
        let all_plugins = vec![
            "builtin.cover",
            "builtin.toc",
            "builtin.metrics",
            "builtin.threats",
            "builtin.takedowns",
            "builtin.roi",
            "builtin.exposure",
            "builtin.geospatial",
            "builtin.evidence",
        ];

        let enabled = self.enabled_plugins();
        all_plugins
            .into_iter()
            .filter(|p| !enabled.contains(p))
            .map(|s| s.to_string())
            .collect()
    }

    /// Preferred theme mode for this template
    pub fn theme(&self) -> ThemeMode {
        match self {
            Self::Healthcare => ThemeMode::Light, // Professional/clinical look
            _ => ThemeMode::Dark,                 // Axur brand default
        }
    }

    /// Primary accent color (hex)
    pub fn accent_color(&self) -> &'static str {
        match self {
            Self::General => "#FF671F",    // Axur orange
            Self::Fintech => "#10B981",    // Emerald green (trust/money)
            Self::Retail => "#8B5CF6",     // Purple (brand)
            Self::Healthcare => "#0EA5E9", // Blue (medical)
        }
    }

    /// Description for UI tooltip
    pub fn description(&self) -> &'static str {
        match self {
            Self::General => "Standard report with all sections",
            Self::Fintech => "Focus on credentials, BINs, and fraud indicators",
            Self::Retail => "Emphasis on brand abuse, phishing, and counterfeits",
            Self::Healthcare => "Prioritizes data exposure and compliance metrics",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            IndustryTemplate::from_str("fintech"),
            IndustryTemplate::Fintech
        );
        assert_eq!(
            IndustryTemplate::from_str("RETAIL"),
            IndustryTemplate::Retail
        );
        assert_eq!(
            IndustryTemplate::from_str("unknown"),
            IndustryTemplate::General
        );
    }

    #[test]
    fn test_disabled_plugins() {
        let fintech = IndustryTemplate::Fintech;
        let disabled = fintech.disabled_plugins();
        assert!(disabled.contains(&"builtin.geospatial".to_string()));
        assert!(!disabled.contains(&"builtin.exposure".to_string()));
    }
}
