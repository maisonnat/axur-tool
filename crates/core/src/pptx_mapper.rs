//! PPTX Placeholder Mapper
//!
//! Maps placeholder keys to real values from PocReportData

use crate::api::report::PocReportData;
use std::collections::HashMap;

/// Helper to format numbers with thousand separators
fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

/// Map all placeholder keys to their real values from report data
pub fn map_placeholders(data: &PocReportData) -> HashMap<String, String> {
    let mut map = HashMap::new();

    // ============================================================
    // GENERAL & COVER
    // ============================================================
    map.insert("company_name".into(), data.company_name.clone());
    map.insert(
        "partner_name".into(),
        data.partner_name.clone().unwrap_or_default(),
    );
    map.insert("tlp_level".into(), data.tlp_level.clone());
    map.insert(
        "date_range".into(),
        format!("{} - {}", data.start_date, data.end_date),
    );
    map.insert("start_date".into(), data.start_date.clone());
    map.insert("end_date".into(), data.end_date.clone());

    // ============================================================
    // MÉTRICAS PRINCIPALES
    // ============================================================
    map.insert("total_tickets".into(), format_number(data.total_tickets));
    map.insert("total_threats".into(), format_number(data.total_threats));

    let hours_saved = (data.total_tickets * 15) / 60;
    let analysts = (hours_saved as f64) / 160.0;
    map.insert("hours_saved".into(), format!("{}", hours_saved));
    map.insert("analysts_equivalent".into(), format!("{:.1}", analysts));
    map.insert(
        "validation_hours".into(),
        format!("{:.1}h", data.validation_hours),
    );

    // ============================================================
    // AMENAZAS
    // ============================================================
    if let Some(t1) = data.threats_by_type.first() {
        map.insert("top_threat_1_name".into(), t1.threat_type.clone());
        map.insert("top_threat_1_count".into(), format_number(t1.count));
    }
    if let Some(t2) = data.threats_by_type.get(1) {
        map.insert("top_threat_2_name".into(), t2.threat_type.clone());
        map.insert("top_threat_2_count".into(), format_number(t2.count));
    }
    if let Some(t3) = data.threats_by_type.get(2) {
        map.insert("top_threat_3_name".into(), t3.threat_type.clone());
        map.insert("top_threat_3_count".into(), format_number(t3.count));
    }

    // Threats by type as text list
    let threats_list: String = data
        .threats_by_type
        .iter()
        .take(5)
        .map(|t| format!("{}: {}", t.threat_type, format_number(t.count)))
        .collect::<Vec<_>>()
        .join("\n");
    map.insert("threats_by_type".into(), threats_list);

    // ============================================================
    // CREDENCIALES
    // ============================================================
    map.insert(
        "credentials_total".into(),
        format_number(data.credentials_total),
    );
    map.insert(
        "credentials_critical".into(),
        format!("{}", data.critical_credentials.len()),
    );
    map.insert(
        "stealer_log_count".into(),
        format_number(data.threat_intelligence.stealer_log_count),
    );
    map.insert(
        "stealer_log_percent".into(),
        format!("{:.1}%", data.threat_intelligence.stealer_log_percent),
    );
    map.insert(
        "plain_password_count".into(),
        format_number(data.threat_intelligence.plain_password_count),
    );
    map.insert(
        "plain_password_percent".into(),
        format!("{:.1}%", data.threat_intelligence.plain_password_percent),
    );
    map.insert("unique_hosts".into(), format_number(data.unique_hosts));
    map.insert(
        "high_risk_users".into(),
        format_number(data.high_risk_users),
    );

    // ============================================================
    // TAKEDOWNS
    // ============================================================
    let takedown_total = data.takedown_resolved
        + data.takedown_pending
        + data.takedown_aborted
        + data.takedown_unresolved;
    map.insert("takedown_total".into(), format_number(takedown_total));
    map.insert(
        "takedown_resolved".into(),
        format_number(data.takedown_resolved),
    );
    map.insert(
        "takedown_pending".into(),
        format_number(data.takedown_pending),
    );
    map.insert(
        "takedown_aborted".into(),
        format_number(data.takedown_aborted),
    );
    map.insert(
        "takedown_unresolved".into(),
        format_number(data.takedown_unresolved),
    );
    map.insert(
        "takedown_success_rate".into(),
        format!("{:.1}%", data.takedown_success_rate),
    );
    map.insert(
        "takedown_median_notify".into(),
        data.takedown_median_time_to_notify.clone(),
    );
    map.insert(
        "takedown_median_uptime".into(),
        data.takedown_median_uptime.clone(),
    );

    // ============================================================
    // ROI & IMPACTO
    // ============================================================
    map.insert(
        "roi_hours_total".into(),
        format!("{:.0}", data.roi_metrics.hours_saved_total),
    );
    map.insert(
        "roi_person_days".into(),
        format!("{:.0}", data.roi_metrics.person_days_saved),
    );
    map.insert(
        "roi_hours_validation".into(),
        format!("{:.0}h", data.roi_metrics.hours_saved_validation),
    );
    map.insert(
        "roi_hours_monitoring".into(),
        format!("{:.0}h", data.roi_metrics.hours_saved_credentials),
    );
    map.insert(
        "roi_hours_takedowns".into(),
        format!("{:.0}h", data.roi_metrics.hours_saved_takedowns),
    );
    map.insert(
        "roi_analysts_monthly".into(),
        format!("{:.1}", data.roi_metrics.analysts_equivalent_monthly),
    );

    // ============================================================
    // RISK SCORE
    // ============================================================
    map.insert(
        "risk_score_value".into(),
        format!("{:.0}", data.risk_score.current),
    );
    map.insert("risk_score_label".into(), data.risk_score.label.clone());

    // ============================================================
    // CODE LEAKS
    // ============================================================
    map.insert("secrets_total".into(), format_number(data.secrets_total));
    map.insert("unique_repos".into(), format_number(data.unique_repos));
    map.insert(
        "production_secrets".into(),
        format_number(data.production_secrets),
    );

    // ============================================================
    // THREAT INTELLIGENCE
    // ============================================================
    map.insert(
        "dark_web_mentions".into(),
        format_number(data.threat_intelligence.dark_web_mentions),
    );
    map.insert(
        "chat_group_shares".into(),
        format_number(data.threat_intelligence.chat_group_shares),
    );
    map.insert(
        "social_media_mentions".into(),
        format_number(data.threat_intelligence.social_media_mentions),
    );
    map.insert(
        "paid_ads_detected".into(),
        format_number(data.threat_intelligence.paid_ads_detected),
    );

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(100), "100");
        assert_eq!(format_number(1234), "1,234");
        assert_eq!(format_number(1234567), "1,234,567");
    }
}
