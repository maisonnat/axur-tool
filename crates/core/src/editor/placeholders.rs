//! Placeholder definitions with mock data for preview
//!
//! Each placeholder includes:
//! - A unique key
//! - Description of what data it provides
//! - Mock data for live preview in the editor

use serde::{Deserialize, Serialize};

/// Placeholder definition with metadata and mock data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceholderDefinition {
    /// Unique identifier key (e.g., "risk_score")
    pub key: String,
    /// Display name in the library
    pub display_name: String,
    /// Category for grouping in library
    pub category: PlaceholderCategory,
    /// Description of what this placeholder shows
    pub description: String,
    /// Mock HTML for preview in editor
    pub mock_html: String,
    /// Icon for the library (emoji or icon name)
    pub icon: String,
}

/// Categories for organizing placeholders in the library
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderCategory {
    /// Text values (company name, dates, counts)
    DynamicText,
    /// Visual charts and graphs
    DataVisualization,
    /// Data tables
    Tables,
    /// Maps and geographic data
    Geospatial,
    /// Image galleries
    Media,
    /// AI-generated content
    AiAnalysis,
}

impl PlaceholderCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::DynamicText => "🔤 Dynamic Text",
            Self::DataVisualization => "📊 Data Visualizations",
            Self::Tables => "📋 Tables",
            Self::Geospatial => "🗺️ Geospatial",
            Self::Media => "🖼️ Media",
            Self::AiAnalysis => "🤖 AI Analysis",
        }
    }
}

/// Get all available placeholder definitions with mock data
pub fn get_all_placeholders() -> Vec<PlaceholderDefinition> {
    vec![
        // ==================
        // DYNAMIC TEXT
        // ==================
        PlaceholderDefinition {
            key: "company_name".to_string(),
            display_name: "Company Name".to_string(),
            category: PlaceholderCategory::DynamicText,
            description: "The client's company name from the report".to_string(),
            mock_html: r#"<span class="placeholder-text" style="font-size: 24px; font-weight: bold; color: #F8FAFC;">ACME Corporation</span>"#.to_string(),
            icon: "🏢".to_string(),
        },
        PlaceholderDefinition {
            key: "report_date_range".to_string(),
            display_name: "Report Date Range".to_string(),
            category: PlaceholderCategory::DynamicText,
            description: "The date range covered by this report (e.g., 'Dec 1 - Dec 31, 2024')".to_string(),
            mock_html: r#"<span class="placeholder-text" style="color: #94A3B8;">Dec 1 - Dec 31, 2024</span>"#.to_string(),
            icon: "📅".to_string(),
        },
        PlaceholderDefinition {
            key: "total_incidents".to_string(),
            display_name: "Total Incidents".to_string(),
            category: PlaceholderCategory::DynamicText,
            description: "Total number of incidents detected in the period".to_string(),
            mock_html: r#"<span class="placeholder-metric" style="font-size: 48px; font-weight: bold; color: #6366F1;">247</span>"#.to_string(),
            icon: "🔢".to_string(),
        },
        PlaceholderDefinition {
            key: "total_takedowns".to_string(),
            display_name: "Total Takedowns".to_string(),
            category: PlaceholderCategory::DynamicText,
            description: "Total number of takedowns completed in the period".to_string(),
            mock_html: r#"<span class="placeholder-metric" style="font-size: 48px; font-weight: bold; color: #10B981;">189</span>"#.to_string(),
            icon: "✅".to_string(),
        },
        PlaceholderDefinition {
            key: "avg_takedown_time".to_string(),
            display_name: "Avg. Takedown Time".to_string(),
            category: PlaceholderCategory::DynamicText,
            description: "Average time to complete a takedown (hours)".to_string(),
            mock_html: r#"<span class="placeholder-metric" style="font-size: 36px; color: #F59E0B;">4.2h</span>"#.to_string(),
            icon: "⏱️".to_string(),
        },

        // ==================
        // DATA VISUALIZATIONS
        // ==================
        PlaceholderDefinition {
            key: "risk_score_gauge".to_string(),
            display_name: "Risk Score Gauge".to_string(),
            category: PlaceholderCategory::DataVisualization,
            description: "Visual gauge showing the calculated Risk Score (0-10) with status indicator".to_string(),
            mock_html: r#"
                <div class="placeholder-gauge" style="text-align: center; padding: 20px;">
                    <div style="font-size: 64px; font-weight: bold; color: #F59E0B;">7.2</div>
                    <div style="font-size: 14px; color: #94A3B8; text-transform: uppercase;">Risk Score</div>
                    <div style="margin-top: 10px; padding: 4px 12px; background: #F59E0B33; color: #F59E0B; border-radius: 12px; display: inline-block;">
                        ⚠️ Alto Riesgo
                    </div>
                </div>
            "#.to_string(),
            icon: "🎯".to_string(),
        },
        PlaceholderDefinition {
            key: "incidents_chart".to_string(),
            display_name: "Incidents Timeline".to_string(),
            category: PlaceholderCategory::DataVisualization,
            description: "Bar chart showing incident counts over time by threat type".to_string(),
            mock_html: r#"
                <div class="placeholder-chart" style="padding: 20px; background: #1E293B; border-radius: 8px;">
                    <div style="color: #F8FAFC; font-weight: bold; margin-bottom: 15px;">Incidents by Month</div>
                    <div style="display: flex; align-items: flex-end; height: 100px; gap: 8px;">
                        <div style="width: 30px; height: 40%; background: #6366F1; border-radius: 4px;"></div>
                        <div style="width: 30px; height: 60%; background: #6366F1; border-radius: 4px;"></div>
                        <div style="width: 30px; height: 80%; background: #6366F1; border-radius: 4px;"></div>
                        <div style="width: 30px; height: 45%; background: #6366F1; border-radius: 4px;"></div>
                        <div style="width: 30px; height: 70%; background: #EC4899; border-radius: 4px;"></div>
                        <div style="width: 30px; height: 90%; background: #EC4899; border-radius: 4px;"></div>
                    </div>
                </div>
            "#.to_string(),
            icon: "📈".to_string(),
        },
        PlaceholderDefinition {
            key: "threat_distribution_pie".to_string(),
            display_name: "Threat Distribution".to_string(),
            category: PlaceholderCategory::DataVisualization,
            description: "Pie/donut chart showing distribution of threat types".to_string(),
            mock_html: r#"
                <div class="placeholder-chart" style="padding: 20px; text-align: center;">
                    <div style="width: 120px; height: 120px; margin: 0 auto; border-radius: 50%; background: conic-gradient(#6366F1 0% 35%, #EC4899 35% 60%, #10B981 60% 80%, #F59E0B 80% 100%);"></div>
                    <div style="margin-top: 10px; font-size: 12px; color: #94A3B8;">
                        <span style="color: #6366F1;">● Phishing 35%</span> · 
                        <span style="color: #EC4899;">● Fraud 25%</span> · 
                        <span style="color: #10B981;">● Leaks 20%</span>
                    </div>
                </div>
            "#.to_string(),
            icon: "🥧".to_string(),
        },

        // ==================
        // TABLES
        // ==================
        PlaceholderDefinition {
            key: "top_threats_table".to_string(),
            display_name: "Top Threats Table".to_string(),
            category: PlaceholderCategory::Tables,
            description: "Table showing top threat categories with counts and percentages".to_string(),
            mock_html: r#"
                <table class="placeholder-table" style="width: 100%; border-collapse: collapse; font-size: 14px;">
                    <thead>
                        <tr style="border-bottom: 1px solid #334155;">
                            <th style="text-align: left; padding: 8px; color: #94A3B8;">Threat Type</th>
                            <th style="text-align: right; padding: 8px; color: #94A3B8;">Count</th>
                            <th style="text-align: right; padding: 8px; color: #94A3B8;">%</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr><td style="padding: 8px; color: #F8FAFC;">Phishing</td><td style="text-align: right; color: #6366F1;">87</td><td style="text-align: right; color: #94A3B8;">35%</td></tr>
                        <tr><td style="padding: 8px; color: #F8FAFC;">Brand Fraud</td><td style="text-align: right; color: #6366F1;">62</td><td style="text-align: right; color: #94A3B8;">25%</td></tr>
                        <tr><td style="padding: 8px; color: #F8FAFC;">Data Leaks</td><td style="text-align: right; color: #6366F1;">49</td><td style="text-align: right; color: #94A3B8;">20%</td></tr>
                    </tbody>
                </table>
            "#.to_string(),
            icon: "📊".to_string(),
        },
        PlaceholderDefinition {
            key: "takedowns_table".to_string(),
            display_name: "Recent Takedowns".to_string(),
            category: PlaceholderCategory::Tables,
            description: "Table of recently completed takedowns with URLs and dates".to_string(),
            mock_html: r#"
                <table class="placeholder-table" style="width: 100%; border-collapse: collapse; font-size: 14px;">
                    <thead>
                        <tr style="border-bottom: 1px solid #334155;">
                            <th style="text-align: left; padding: 8px; color: #94A3B8;">URL</th>
                            <th style="text-align: right; padding: 8px; color: #94A3B8;">Date</th>
                            <th style="text-align: center; padding: 8px; color: #94A3B8;">Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr><td style="padding: 8px; color: #60A5FA;">fake-acme.com</td><td style="text-align: right; color: #94A3B8;">Dec 28</td><td style="text-align: center;"><span style="color: #10B981;">✓</span></td></tr>
                        <tr><td style="padding: 8px; color: #60A5FA;">acme-login.xyz</td><td style="text-align: right; color: #94A3B8;">Dec 27</td><td style="text-align: center;"><span style="color: #10B981;">✓</span></td></tr>
                    </tbody>
                </table>
            "#.to_string(),
            icon: "📋".to_string(),
        },

        // ==================
        // GEOSPATIAL
        // ==================
        PlaceholderDefinition {
            key: "geospatial_map".to_string(),
            display_name: "Geographic Distribution".to_string(),
            category: PlaceholderCategory::Geospatial,
            description: "World map showing threat origin countries with heat intensity".to_string(),
            mock_html: r#"
                <div class="placeholder-map" style="padding: 20px; background: #1E293B; border-radius: 8px; text-align: center;">
                    <div style="color: #F8FAFC; font-weight: bold; margin-bottom: 10px;">Threat Origins</div>
                    <div style="font-size: 32px; margin: 20px 0;">🌎</div>
                    <div style="display: flex; justify-content: center; gap: 20px; font-size: 14px;">
                        <span>🇧🇷 Brazil <b style="color: #6366F1;">42%</b></span>
                        <span>🇺🇸 USA <b style="color: #6366F1;">28%</b></span>
                        <span>🇷🇺 Russia <b style="color: #6366F1;">15%</b></span>
                    </div>
                </div>
            "#.to_string(),
            icon: "🗺️".to_string(),
        },

        // ==================
        // MEDIA
        // ==================
        PlaceholderDefinition {
            key: "evidence_gallery".to_string(),
            display_name: "Evidence Gallery".to_string(),
            category: PlaceholderCategory::Media,
            description: "Grid of screenshot evidence from detected incidents".to_string(),
            mock_html: r#"
                <div class="placeholder-gallery" style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; padding: 10px;">
                    <div style="aspect-ratio: 16/9; background: #334155; border-radius: 4px; display: flex; align-items: center; justify-content: center; color: #64748B;">📷</div>
                    <div style="aspect-ratio: 16/9; background: #334155; border-radius: 4px; display: flex; align-items: center; justify-content: center; color: #64748B;">📷</div>
                    <div style="aspect-ratio: 16/9; background: #334155; border-radius: 4px; display: flex; align-items: center; justify-content: center; color: #64748B;">📷</div>
                </div>
            "#.to_string(),
            icon: "🖼️".to_string(),
        },

        // ==================
        // AI ANALYSIS
        // ==================
        PlaceholderDefinition {
            key: "ai_executive_summary".to_string(),
            display_name: "AI Executive Summary".to_string(),
            category: PlaceholderCategory::AiAnalysis,
            description: "AI-generated executive summary of the threat landscape".to_string(),
            mock_html: r#"
                <div class="placeholder-ai" style="padding: 20px; background: linear-gradient(135deg, #6366F120, #EC489920); border-radius: 8px; border-left: 3px solid #6366F1;">
                    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 10px;">
                        <span style="font-size: 20px;">🤖</span>
                        <span style="color: #6366F1; font-weight: bold;">AI Analysis</span>
                    </div>
                    <p style="color: #F8FAFC; line-height: 1.6; margin: 0;">
                        Durante el período analizado, se detectó un incremento del 23% en intentos de phishing 
                        dirigidos al sector financiero. La mayoría de los ataques originaron en Brasil y utilizaron 
                        dominios similares al oficial de la empresa...
                    </p>
                </div>
            "#.to_string(),
            icon: "🤖".to_string(),
        },
        PlaceholderDefinition {
            key: "campaign_summary".to_string(),
            display_name: "Campaign Detection".to_string(),
            category: PlaceholderCategory::AiAnalysis,
            description: "Summary of detected attack campaigns with common infrastructure".to_string(),
            mock_html: r#"
                <div class="placeholder-campaign" style="padding: 15px; background: #1E293B; border-radius: 8px;">
                    <div style="color: #EC4899; font-weight: bold; margin-bottom: 8px;">🎯 Campaña Detectada</div>
                    <div style="color: #F8FAFC; font-size: 18px; margin-bottom: 10px;">Phishing Bancario Q4</div>
                    <div style="display: flex; gap: 15px; color: #94A3B8; font-size: 14px;">
                        <span>📍 IP: 185.xxx.xxx.xxx</span>
                        <span>🏠 ISP: DigitalOcean</span>
                        <span>📊 12 incidents</span>
                    </div>
                </div>
            "#.to_string(),
            icon: "🎯".to_string(),
        },
    ]
}

/// Get placeholders by category
pub fn get_placeholders_by_category(category: PlaceholderCategory) -> Vec<PlaceholderDefinition> {
    get_all_placeholders()
        .into_iter()
        .filter(|p| p.category == category)
        .collect()
}

/// Get a single placeholder by key
pub fn get_placeholder(key: &str) -> Option<PlaceholderDefinition> {
    get_all_placeholders().into_iter().find(|p| p.key == key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_placeholders_have_mock() {
        let placeholders = get_all_placeholders();
        assert!(!placeholders.is_empty());

        for p in &placeholders {
            assert!(
                !p.mock_html.is_empty(),
                "Placeholder {} missing mock",
                p.key
            );
            assert!(
                !p.description.is_empty(),
                "Placeholder {} missing description",
                p.key
            );
        }
    }

    #[test]
    fn test_get_by_category() {
        let text_placeholders = get_placeholders_by_category(PlaceholderCategory::DynamicText);
        assert!(text_placeholders.len() >= 3);

        for p in &text_placeholders {
            assert_eq!(p.category, PlaceholderCategory::DynamicText);
        }
    }

    #[test]
    fn test_get_single_placeholder() {
        let risk = get_placeholder("risk_score_gauge");
        assert!(risk.is_some());
        assert_eq!(
            risk.unwrap().category,
            PlaceholderCategory::DataVisualization
        );
    }
}
