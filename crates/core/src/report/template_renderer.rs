//! Template Renderer (Simplified)
//!
//! Resolves placeholders in user templates with real report data

use std::collections::HashMap;

/// Resolve all placeholders in a template JSON string
pub fn resolve_placeholders(template_json: &str, values: &HashMap<String, String>) -> String {
    let mut result = template_json.to_string();

    // Replace each placeholder pattern {{key}} with its resolved value
    for (key, value) in values {
        let pattern = format!("{{{{{}}}}}", key);
        result = result.replace(&pattern, value);
    }

    result
}

/// Get placeholder values suitable for template rendering
/// This returns mock data that can be replaced with real data later
pub fn get_mock_placeholder_values() -> HashMap<String, String> {
    let mut values = HashMap::new();

    // Dynamic Text
    values.insert("company_name".to_string(), "ACME Corporation".to_string());
    values.insert(
        "report_date_range".to_string(),
        "Dec 1 - Dec 31, 2024".to_string(),
    );
    values.insert(
        "total_incidents".to_string(),
        r#"<span style="font-size:48px;font-weight:bold;color:#6366F1;">247</span>"#.to_string(),
    );
    values.insert(
        "total_takedowns".to_string(),
        r#"<span style="font-size:48px;font-weight:bold;color:#10B981;">189</span>"#.to_string(),
    );
    values.insert(
        "avg_takedown_time".to_string(),
        r#"<span style="font-size:36px;color:#F59E0B;">4.2h</span>"#.to_string(),
    );

    // Risk Score
    values.insert("risk_score_gauge".to_string(), r#"<div style="text-align:center;padding:20px;">
            <div style="font-size:64px;font-weight:bold;color:#F59E0B;">7.2</div>
            <div style="font-size:14px;color:#94A3B8;text-transform:uppercase;">RISK SCORE</div>
            <div style="margin-top:10px;padding:4px 12px;background:#F59E0B33;color:#F59E0B;border-radius:12px;display:inline-block;">
                Medium Risk
            </div>
        </div>"#.to_string());

    // Charts (simplified)
    values.insert("incidents_chart".to_string(), r#"<div style="padding:20px;background:#1E293B;border-radius:8px;text-align:center;">
            <div style="height:100px;display:flex;align-items:flex-end;justify-content:center;gap:10px;">
                <div style="width:40px;background:#6366F1;border-radius:4px 4px 0 0;height:80%"></div>
                <div style="width:40px;background:#EC4899;border-radius:4px 4px 0 0;height:60%"></div>
                <div style="width:40px;background:#10B981;border-radius:4px 4px 0 0;height:40%"></div>
                <div style="width:40px;background:#F59E0B;border-radius:4px 4px 0 0;height:30%"></div>
            </div>
            <div style="margin-top:10px;color:#94A3B8;font-size:12px;">Incidents by Type</div>
        </div>"#.to_string());

    values.insert("threat_distribution_pie".to_string(), r#"<div style="padding:20px;text-align:center;">
            <div style="width:120px;height:120px;margin:0 auto;border-radius:50%;background:conic-gradient(#6366F1 0% 35%, #EC4899 35% 60%, #10B981 60% 80%, #F59E0B 80% 100%);"></div>
            <div style="margin-top:10px;font-size:12px;color:#94A3B8;">Phishing 35% · Fake Profile 25% · Fraud 20% · Other 20%</div>
        </div>"#.to_string());

    // Tables
    values.insert("top_threats_table".to_string(), r#"<table style="width:100%;border-collapse:collapse;font-size:14px;">
            <thead><tr style="border-bottom:1px solid #334155;">
                <th style="text-align:left;padding:8px;color:#94A3B8;">Type</th>
                <th style="text-align:right;padding:8px;color:#94A3B8;">Count</th>
            </tr></thead>
            <tbody>
                <tr><td style="padding:8px;color:#F8FAFC;">Phishing</td><td style="text-align:right;color:#6366F1;">87</td></tr>
                <tr><td style="padding:8px;color:#F8FAFC;">Fake Profile</td><td style="text-align:right;color:#6366F1;">62</td></tr>
                <tr><td style="padding:8px;color:#F8FAFC;">Fraud</td><td style="text-align:right;color:#6366F1;">48</td></tr>
            </tbody>
        </table>"#.to_string());

    values.insert("takedowns_table".to_string(), r#"<table style="width:100%;border-collapse:collapse;font-size:14px;">
            <thead><tr style="border-bottom:1px solid #334155;">
                <th style="text-align:left;padding:8px;color:#94A3B8;">URL</th>
                <th style="text-align:center;padding:8px;color:#94A3B8;">Status</th>
            </tr></thead>
            <tbody>
                <tr><td style="padding:8px;color:#60A5FA;">fake-acme.com</td><td style="text-align:center;color:#10B981;">✓</td></tr>
                <tr><td style="padding:8px;color:#60A5FA;">acme-promo.net</td><td style="text-align:center;color:#10B981;">✓</td></tr>
            </tbody>
        </table>"#.to_string());

    // Geospatial
    values.insert(
        "geospatial_map".to_string(),
        r#"<div style="padding:20px;background:#1E293B;border-radius:8px;text-align:center;">
            <div style="font-size:48px;margin:20px 0;">🌎</div>
            <div style="display:flex;justify-content:center;gap:20px;font-size:14px;">
                <span>🇧🇷 Brazil <b style="color:#6366F1;">45</b></span>
                <span>🇺🇸 USA <b style="color:#6366F1;">23</b></span>
                <span>🇦🇷 Argentina <b style="color:#6366F1;">12</b></span>
            </div>
        </div>"#.to_string(),
    );

    // Media
    values.insert("evidence_gallery".to_string(), r#"<div style="display:grid;grid-template-columns:repeat(3,1fr);gap:10px;">
            <div style="aspect-ratio:16/9;background:#334155;border-radius:4px;display:flex;align-items:center;justify-content:center;">📷</div>
            <div style="aspect-ratio:16/9;background:#334155;border-radius:4px;display:flex;align-items:center;justify-content:center;">📷</div>
            <div style="aspect-ratio:16/9;background:#334155;border-radius:4px;display:flex;align-items:center;justify-content:center;">📷</div>
        </div>"#.to_string());

    // AI Analysis
    values.insert("ai_executive_summary".to_string(), r#"<div style="padding:20px;background:linear-gradient(135deg,#6366F120,#EC489920);border-radius:8px;border-left:3px solid #6366F1;">
            <div style="display:flex;align-items:center;gap:8px;margin-bottom:10px;">
                <span style="font-size:20px;">🤖</span>
                <span style="color:#6366F1;font-weight:bold;">AI Analysis</span>
            </div>
            <p style="color:#F8FAFC;line-height:1.6;margin:0;">
                Analysis indicates a coordinated phishing campaign targeting financial credentials. 
                Primary attack vector involves fake login pages hosted on newly registered domains. 
                Recommendation: Increase monitoring of domain registrations and implement additional email filtering rules.
            </p>
        </div>"#.to_string());

    values.insert("campaign_summary".to_string(), r#"<div style="padding:15px;background:#1E293B;border-radius:8px;">
            <div style="color:#EC4899;font-weight:bold;margin-bottom:8px;">🎯 Campaign Detected</div>
            <div style="color:#F8FAFC;font-size:18px;margin-bottom:10px;">Phishing Campaign - Financial Credentials</div>
            <div style="display:flex;gap:15px;color:#94A3B8;font-size:14px;">
                <span>📊 87 incidents</span>
                <span>🎯 High severity</span>
            </div>
        </div>"#.to_string());

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_placeholders() {
        let template = r#"{"company": "{{company_name}}", "total": "{{total_incidents}}"}"#;
        let mut values = HashMap::new();
        values.insert("company_name".to_string(), "Test Corp".to_string());
        values.insert("total_incidents".to_string(), "100".to_string());

        let result = resolve_placeholders(template, &values);
        assert!(result.contains("Test Corp"));
        assert!(result.contains("100"));
    }
}
