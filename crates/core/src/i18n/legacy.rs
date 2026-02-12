use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Language {
    En,
    Es,
    PtBr,
}

impl Default for Language {
    fn default() -> Self {
        Language::En
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::En => write!(f, "English"),
            Language::Es => write!(f, "Español"),
            Language::PtBr => write!(f, "Português (Brasil)"),
        }
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Ok(Language::En),
            "es" | "spanish" | "espanol" => Ok(Language::Es),
            "pt" | "pt-br" | "portuguese" | "portugues" => Ok(Language::PtBr),
            _ => Err(()),
        }
    }
}

pub trait Dictionary: Send + Sync {
    // General
    fn welcome_message(&self) -> String;
    fn login_prompt_email(&self) -> String;
    fn footer_text(&self) -> String;

    // Labels common
    fn label_partner(&self) -> String;
    fn label_company(&self) -> String;
    fn label_tlp(&self) -> String;
    fn label_tlp_desc(&self) -> String;

    // Cover
    fn cover_title_dynamic(&self) -> String;
    fn cover_title_static(&self) -> String;

    // Intro
    fn intro_title(&self) -> String;
    fn intro_text_dynamic(&self) -> String;
    fn intro_text_static(&self) -> String;
    fn intro_text_closing(&self) -> String;

    // Solutions
    fn solutions_title(&self) -> String;
    fn solutions_subtitle_1(&self) -> String;
    fn solutions_subtitle_2(&self) -> String;
    fn solutions_subtitle_3(&self) -> String;
    // ... skipping individual solution descriptions for brevity, using generic names if needed, but let's be specific for key headers
    fn solution_takedown(&self) -> String;
    fn solution_brand_protection(&self) -> String;
    fn solution_threat_intel(&self) -> String;

    // TOC
    fn toc_title(&self) -> String;
    fn toc_items(&self) -> Vec<String>; // Returns the list of 7 items

    // PoC Data
    fn poc_scope_title(&self) -> String;
    fn poc_assets_title(&self) -> String;
    fn poc_label_brands(&self) -> String;
    fn poc_label_executives(&self) -> String;
    fn poc_label_ips(&self) -> String; // IPs / Ranges
    fn poc_label_bins(&self) -> String;
    fn poc_label_domains(&self) -> String;

    fn poc_period_dynamic_title(&self) -> String;
    fn poc_period_dynamic_text(&self) -> String;
    fn poc_period_static_title(&self) -> String;
    fn poc_period_start(&self) -> String;
    fn poc_period_end(&self) -> String;

    // Metrics
    fn metrics_title(&self) -> String;
    fn metrics_total_tickets(&self) -> String;
    fn metrics_threats_detected(&self) -> String;
    fn metrics_time_saved(&self) -> String;
    fn metrics_desc_tickets(&self) -> String;
    fn metrics_desc_threats(&self) -> String;
    fn metrics_desc_time(&self) -> String;

    // Threats Chart
    fn threats_title(&self) -> String;
    fn threats_desc(&self, total: u64) -> String;

    // Infostealer
    fn stealer_title(&self) -> String;
    fn stealer_subtitle(&self, count: u64) -> String;
    fn stealer_box_creds(&self) -> String;
    fn stealer_box_hosts(&self) -> String;
    fn stealer_box_high_risk(&self) -> String;
    fn stealer_action(&self) -> String;
    fn stealer_critical_title(&self) -> String;
    fn stealer_critical_desc(&self, count: usize) -> String;

    // Code Leak
    fn code_leak_title(&self) -> String;
    fn code_leak_subtitle(&self, count: u64) -> String;
    fn code_leak_box_secrets(&self) -> String;
    fn code_leak_box_repos(&self) -> String;
    fn code_leak_box_prod(&self) -> String;
    fn code_leak_action(&self) -> String;

    // Incidents
    fn incidents_title(&self) -> String;
    fn incidents_desc(&self, total: u64) -> String;

    // Takedowns (Complex)
    fn takedowns_title(&self) -> String;
    fn takedowns_requested(&self) -> String;
    fn takedowns_success_rate(&self) -> String; // "Success Rate"
    fn takedowns_median_notify(&self) -> String; // "Median time for 1st notification"
    fn takedowns_median_uptime(&self) -> String; // "Uptime median"
    fn takedowns_status_title(&self) -> String;
    fn takedowns_solved(&self) -> String;
    fn takedowns_in_progress(&self) -> String;
    fn takedowns_interrupted(&self) -> String;
    fn takedowns_not_solved(&self) -> String;

    // ROI
    fn roi_title(&self) -> String;
    fn roi_efficiency_title(&self) -> String;
    fn roi_efficiency_label(&self) -> String; // "FTEs Saved"
    fn roi_efficiency_desc(&self, hours: u64) -> String;
    fn roi_risk_title(&self) -> String;
    fn roi_risk_label(&self) -> String; // "vs Industry Avg"
    fn roi_risk_desc(&self, notify_time: &str) -> String;
    fn roi_intel_title(&self) -> String;
    fn roi_intel_desc(&self, count: u64) -> String;
    fn roi_intel_stealer(&self) -> String;
    fn roi_intel_plaintext(&self) -> String;

    // Data Exposure (Unified)
    fn exposure_title(&self) -> String;
    fn exposure_sub_code(&self) -> String;
    fn exposure_sub_stealer(&self) -> String;

    // Operational Impact Slide (new)
    fn op_badge(&self) -> String; // "OPERATIONAL IMPACT"
    fn op_time_saved_title(&self) -> String; // "Time Saved"
    fn op_time_saved_desc(&self) -> String;
    fn op_unit_person_days(&self) -> String; // "person-days"
    fn op_unit_hours(&self) -> String; // "hours"
    fn op_breakdown_validation(&self) -> String; // "Validation"
    fn op_breakdown_monitoring(&self) -> String; // "Monitoring"
    fn op_breakdown_takedowns(&self) -> String; // "Takedowns"
    fn op_capacity_title(&self) -> String; // "Freed Capacity"
    fn op_capacity_desc(&self) -> String;
    fn op_tickets_processed(&self) -> String; // "tickets processed"
    fn op_credentials_monitored(&self) -> String; // "credentials monitored"
    fn op_response_title(&self) -> String; // "Response Time"
    fn op_response_desc(&self) -> String;
    fn op_success_rate(&self) -> String; // "Success rate"
    fn op_takedowns_completed(&self) -> String; // "Takedowns completed"

    // Examples
    fn examples_takedowns_title(&self) -> String;
    fn examples_poc_title(&self) -> String;
    fn example_label_type(&self) -> String;
    fn example_label_date(&self) -> String;
    fn example_label_url(&self) -> String;
    fn example_no_image(&self) -> String;
    fn example_no_data(&self) -> String;

    // Closing
    fn closing_title(&self) -> String; // "Ready to thrive?"
    fn closing_subtitle(&self) -> String;
    fn closing_cta_activate(&self) -> String;
    fn closing_cta_activate_desc(&self) -> String;
    fn closing_cta_meet(&self) -> String;
    fn closing_cta_meet_desc(&self) -> String;

    // Deep Analytics
    fn deep_analytics_title(&self) -> String;
    fn deep_analytics_subtitle(&self) -> String;
    fn deep_analytics_code_leak_title(&self) -> String;
    fn deep_analytics_code_leak_subtitle(&self, count: u64) -> String;
    fn deep_analytics_credential_title(&self) -> String;
    fn deep_analytics_credential_subtitle(&self, count: u64) -> String;
    fn deep_analytics_takedown_title(&self) -> String;
    fn deep_analytics_takedown_subtitle(&self, count: usize) -> String;

    // Incident Story
    fn story_title(&self) -> String;
    fn story_subtitle(&self, count: usize) -> String;

    // Context Slides
    fn ctx_risk_title(&self) -> String;
    fn ctx_risk_text(&self) -> String;

    // Advanced Storytelling & Efficiency
    fn eff_title(&self) -> String;
    fn eff_text_hours(&self, hours: u64, analysts: f64) -> String;
    fn eff_text_speed(&self) -> String;

    fn narrative_phishing_title(&self) -> String;
    fn narrative_phishing_pain(&self) -> String;
    fn narrative_phishing_solution(&self) -> String;

    fn narrative_stealer_title(&self) -> String;
    fn narrative_stealer_pain(&self) -> String;
    fn narrative_stealer_solution(&self) -> String;

    fn narrative_takedown_title(&self) -> String;
    fn narrative_takedown_pain(&self) -> String;
    fn narrative_takedown_solution(&self) -> String;

    fn narrative_timeline_title(&self) -> String;
    fn narrative_timeline_text(&self, percent_off_hours: u64) -> String;

    fn closing_value_title(&self) -> String;
    fn closing_value_intro(&self) -> String;
    fn closing_value_item_1_title(&self) -> String;
    fn closing_value_item_1_desc(&self) -> String;
    fn closing_value_item_2_title(&self) -> String;
    fn closing_value_item_2_desc(&self) -> String;
    fn closing_value_item_3_title(&self) -> String;
    fn closing_value_item_3_desc(&self) -> String;

    // === Virality / Chat Narrative ===
    fn narrative_virality_title(&self) -> String;
    // Primary: Used when there are group shares or dark web mentions
    // "{share_count} mentions in {source_count} groups like '{top_source}'"
    fn narrative_virality_pain_primary(
        &self,
        share_count: u64,
        source_count: usize,
        top_source: &str,
    ) -> String;
    fn narrative_virality_solution_primary(&self) -> String;
    // Fallback: Used when no detected viral activity
    fn narrative_virality_pain_fallback(&self) -> String;
    fn narrative_virality_solution_fallback(&self) -> String;

    // === ROI Precision ===
    fn roi_precise_title(&self) -> String;
    fn roi_precise_text_primary(&self, median_minutes: i64) -> String;
    fn roi_precise_text_fallback(&self) -> String;

    // === AI Intent Analysis ===
    fn intent_title(&self) -> String;
    fn intent_fmt_primary(&self, top_intent: &str, percent: u64) -> String;
    fn intent_fmt_fallback(&self) -> String;
    fn intent_cat_credentials(&self) -> String;
    fn intent_cat_infection(&self) -> String;
    fn intent_cat_fraud(&self) -> String;
    fn intent_cat_trust(&self) -> String;
    fn intent_cat_chat(&self) -> String;
    fn intent_cat_compromised(&self) -> String;
    fn intent_cat_data_leak(&self) -> String;
    fn intent_cat_vip(&self) -> String;
    fn intent_cat_dark_web(&self) -> String;

    // === Geospatial Intelligence ===
    fn geo_title(&self) -> String;
    fn geo_fmt_primary(&self, count: usize, top_country: &str) -> String;
    fn geo_fmt_fallback(&self) -> String;
    fn geo_lbl_countries(&self) -> String;
    fn geo_lbl_isps(&self) -> String;
    fn geo_why_important_title(&self) -> String;
    fn geo_why_important_text(&self) -> String;

    fn ctx_stealer_title(&self) -> String;
    fn ctx_stealer_text(&self) -> String;

    fn ctx_leak_title(&self) -> String;
    fn ctx_leak_text(&self) -> String;

    fn ctx_takedown_title(&self) -> String;
    fn ctx_takedown_text(&self) -> String;
}

pub struct English;
pub struct Spanish;
pub struct Portuguese;

// --- Helper macros to reduce boilerplate if we had them, but standard impl is safer for now ---

impl Dictionary for English {
    fn welcome_message(&self) -> String {
        "Welcome to Axur CLI".to_string()
    }
    fn login_prompt_email(&self) -> String {
        "Email".to_string()
    }
    fn footer_text(&self) -> String {
        "Axur. Digital experiences made safe. All rights reserved.".to_string()
    }

    fn label_partner(&self) -> String {
        "PARTNER".to_string()
    }
    fn label_company(&self) -> String {
        "COMPANY".to_string()
    }
    fn label_tlp(&self) -> String {
        "TLP:".to_string()
    }
    fn label_tlp_desc(&self) -> String {
        "Traffic Light Protocol (TLP). Restrictions on how this information can be shared."
            .to_string()
    }

    fn cover_title_dynamic(&self) -> String {
        "Digital Monitoring<br>Report<br>".to_string()
    }
    fn cover_title_static(&self) -> String {
        "Proof of Concept<br>Results Report<br>".to_string()
    }

    fn intro_title(&self) -> String {
        "Digital Protection: From Detection to Resolution".to_string()
    }
    fn intro_text_dynamic(&self) -> String {
        "This report details the results of continuous monitoring for your brand, demonstrating Axur's capability to provide complete visibility and effective response to digital risks.".to_string()
    }
    fn intro_text_static(&self) -> String {
        "This report details the results of our Proof of Concept (PoC), demonstrating Axur's capability to provide complete visibility and effective response to digital risks.".to_string()
    }
    fn intro_text_closing(&self) -> String {
        "The following pages present a detailed analysis of detected threats, actions taken, and the tangible value our platform brings to your brand's security and integrity.".to_string()
    }

    fn solutions_title(&self) -> String {
        "Complete Platform for Your Digital Protection".to_string()
    }
    fn solutions_subtitle_1(&self) -> String {
        "Axur offers a complete solution for managing your digital risks.".to_string()
    }
    fn solutions_subtitle_2(&self) -> String {
        "We protect your brand, your customers, and your business from online fraud and threats."
            .to_string()
    }
    fn solutions_subtitle_3(&self) -> String {
        "From detection to takedown, our platform automates the entire process.".to_string()
    }
    fn solution_takedown(&self) -> String {
        "Takedown".to_string()
    }
    fn solution_brand_protection(&self) -> String {
        "Brand Protection".to_string()
    }
    fn solution_threat_intel(&self) -> String {
        "Threat Intelligence".to_string()
    }

    fn toc_title(&self) -> String {
        "Table of Contents".to_string()
    }
    fn toc_items(&self) -> Vec<String> {
        vec![
            "General Metrics".to_string(),
            "Digital Fraud".to_string(),
            "Data Exposure".to_string(),
            "Executives & VIPs".to_string(),
            "Deep & Dark Web".to_string(),
            "Threat Intelligence & Exposure".to_string(),
            "Operational Efficiency".to_string(),
        ]
    }

    fn poc_scope_title(&self) -> String {
        "Monitoring Scope".to_string()
    }
    fn poc_assets_title(&self) -> String {
        "Monitored Assets".to_string()
    }
    fn poc_label_brands(&self) -> String {
        "Monitored Brand(s)".to_string()
    }
    fn poc_label_executives(&self) -> String {
        "Executives".to_string()
    }
    fn poc_label_ips(&self) -> String {
        "IPs / Ranges".to_string()
    }
    fn poc_label_bins(&self) -> String {
        "BINs".to_string()
    }
    fn poc_label_domains(&self) -> String {
        "Domains".to_string()
    }

    fn poc_period_dynamic_title(&self) -> String {
        "Analysis Period".to_string()
    }
    fn poc_period_dynamic_text(&self) -> String {
        "Last days until today".to_string()
    }
    fn poc_period_static_title(&self) -> String {
        "PoC Duration".to_string()
    }
    fn poc_period_start(&self) -> String {
        "Start".to_string()
    }
    fn poc_period_end(&self) -> String {
        "End".to_string()
    }

    fn metrics_title(&self) -> String {
        "General Metrics".to_string()
    }
    fn metrics_total_tickets(&self) -> String {
        "Total Tickets (Raw Detections)".to_string()
    }
    fn metrics_threats_detected(&self) -> String {
        "Detected Threats".to_string()
    }
    fn metrics_time_saved(&self) -> String {
        "Time Saved in Validation".to_string()
    }
    fn metrics_desc_tickets(&self) -> String {
        "Total number of signals collected by the platform.".to_string()
    }
    fn metrics_desc_threats(&self) -> String {
        "Signals confirmed as real risks after analysis and validation.".to_string()
    }
    fn metrics_desc_time(&self) -> String {
        "Estimated time your team would have spent manually collecting and validating these threats.".to_string()
    }

    fn threats_title(&self) -> String {
        "Potential Threats".to_string()
    }
    fn threats_desc(&self, total: u64) -> String {
        format!("A total of <strong>{}</strong> threats were detected. The chart shows main categories, highlighting the most frequent attack vectors.", total)
    }

    fn stealer_title(&self) -> String {
        "Risk Landscape: Infostealer Credentials".to_string()
    }
    fn stealer_subtitle(&self, count: u64) -> String {
        format!("Analysis of {} detected credentials", count)
    }
    fn stealer_box_creds(&self) -> String {
        "Total Credentials".to_string()
    }
    fn stealer_box_hosts(&self) -> String {
        "Unique Hosts Compromised".to_string()
    }
    fn stealer_box_high_risk(&self) -> String {
        "High Risk Users".to_string()
    }
    fn stealer_action(&self) -> String {
        "Immediate Action: Force password reset for exposed users and prioritize isolating infected hosts.".to_string()
    }
    fn stealer_critical_title(&self) -> String {
        "CRITICAL: Corporate Pattern Detected".to_string()
    }
    fn stealer_critical_desc(&self, count: usize) -> String {
        format!("Found <span class=\"text-white font-bold\">{}</span> identities using variations of the company name in their passwords.", count)
    }

    fn code_leak_title(&self) -> String {
        "Risk Landscape: Code Credential Leaks".to_string()
    }
    fn code_leak_subtitle(&self, count: u64) -> String {
        format!("Analysis of {} public secrets exposed", count)
    }
    fn code_leak_box_secrets(&self) -> String {
        "Total Secrets Exposed".to_string()
    }
    fn code_leak_box_repos(&self) -> String {
        "Unique Public Repos".to_string()
    }
    fn code_leak_box_prod(&self) -> String {
        "Production Secrets".to_string()
    }
    fn code_leak_action(&self) -> String {
        "Immediate Action: Rotate Production keys and API Keys immediately. Implement secret scanning in CI/CD pipeline.".to_string()
    }

    fn incidents_title(&self) -> String {
        "Incidents by Type".to_string()
    }
    fn incidents_desc(&self, total: u64) -> String {
        format!("A total of <strong>{}</strong> incidents were created. This chart details the relationship between raw detections and confirmed incidents.", total)
    }

    fn takedowns_title(&self) -> String {
        "Takedowns".to_string()
    }
    fn takedowns_requested(&self) -> String {
        "takedowns requested".to_string()
    }
    fn takedowns_success_rate(&self) -> String {
        "success rate".to_string()
    }
    fn takedowns_median_notify(&self) -> String {
        "median time for<br>1st notification".to_string()
    }
    fn takedowns_median_uptime(&self) -> String {
        "uptime median".to_string()
    }
    fn takedowns_status_title(&self) -> String {
        "Takedown status".to_string()
    }
    fn takedowns_solved(&self) -> String {
        "Solved".to_string()
    }
    fn takedowns_in_progress(&self) -> String {
        "In progress".to_string()
    }
    fn takedowns_interrupted(&self) -> String {
        "Interrupted".to_string()
    }
    fn takedowns_not_solved(&self) -> String {
        "Not solved".to_string()
    }

    fn roi_title(&self) -> String {
        "Impact & ROI".to_string()
    }
    fn roi_efficiency_title(&self) -> String {
        "Operational Efficiency".to_string()
    }
    fn roi_efficiency_label(&self) -> String {
        "FTEs Saved".to_string()
    }
    fn roi_efficiency_desc(&self, hours: u64) -> String {
        format!(
            "Equivalent to {} hours of manual analysis saved for your security team.",
            hours
        )
    }
    fn roi_risk_title(&self) -> String {
        "Risk Reduction".to_string()
    }
    fn roi_risk_label(&self) -> String {
        "vs Industry Avg".to_string()
    }
    fn roi_risk_desc(&self, notify_time: &str) -> String {
        format!(
            "Axur Detection & Notification Time ({}) vs 48h (Manual).",
            notify_time
        )
    }
    fn roi_intel_title(&self) -> String {
        "Critical Intelligence".to_string()
    }
    fn roi_intel_desc(&self, count: u64) -> String {
        format!(
            "Advanced risk profile based on {} analyzed credentials.",
            count
        )
    }
    fn roi_intel_stealer(&self) -> String {
        "Malware (Stealer Logs)".to_string()
    }
    fn roi_intel_plaintext(&self) -> String {
        "Plaintext Passwords".to_string()
    }

    fn exposure_title(&self) -> String {
        "Sensitive Data Exposure".to_string()
    }
    fn exposure_sub_code(&self) -> String {
        "Source Code Leaks".to_string()
    }
    fn exposure_sub_stealer(&self) -> String {
        "Infostealer Credentials".to_string()
    }

    // Operational Impact Slide
    fn op_badge(&self) -> String {
        "OPERATIONAL IMPACT".to_string()
    }
    fn op_time_saved_title(&self) -> String {
        "Time Saved".to_string()
    }
    fn op_time_saved_desc(&self) -> String {
        "Hours your team would have spent manually processing these signals and threats."
            .to_string()
    }
    fn op_unit_person_days(&self) -> String {
        "person-days".to_string()
    }
    fn op_unit_hours(&self) -> String {
        "hours".to_string()
    }
    fn op_breakdown_validation(&self) -> String {
        "Validation".to_string()
    }
    fn op_breakdown_monitoring(&self) -> String {
        "Monitoring".to_string()
    }
    fn op_breakdown_takedowns(&self) -> String {
        "Takedowns".to_string()
    }
    fn op_capacity_title(&self) -> String {
        "Freed Capacity".to_string()
    }
    fn op_capacity_desc(&self) -> String {
        "Equivalent capacity of security analysts that Axur frees for strategic tasks.".to_string()
    }
    fn op_tickets_processed(&self) -> String {
        "tickets processed".to_string()
    }
    fn op_credentials_monitored(&self) -> String {
        "credentials monitored".to_string()
    }
    fn op_response_title(&self) -> String {
        "Response Time".to_string()
    }
    fn op_response_desc(&self) -> String {
        "Median time from detection to first provider notification.".to_string()
    }
    fn op_success_rate(&self) -> String {
        "Success rate".to_string()
    }
    fn op_takedowns_completed(&self) -> String {
        "Takedowns completed".to_string()
    }

    fn examples_takedowns_title(&self) -> String {
        "Resolved Takedowns: Examples".to_string()
    }
    fn examples_poc_title(&self) -> String {
        "Examples of Detected Threats".to_string()
    }
    fn example_label_type(&self) -> String {
        "Type:".to_string()
    }
    fn example_label_date(&self) -> String {
        "Date:".to_string()
    }
    fn example_label_url(&self) -> String {
        "URL:".to_string()
    }
    fn example_no_image(&self) -> String {
        "No Image Available".to_string()
    }
    fn example_no_data(&self) -> String {
        "No evidence examples found in the selected period.".to_string()
    }

    fn closing_title(&self) -> String {
        "Protect What Matters Most".to_string()
    }
    fn closing_subtitle(&self) -> String {
        "You've seen the risks. Now turn these results into continuous, 24/7 protection for your brand and customers.".to_string()
    }
    fn closing_cta_activate(&self) -> String {
        "Activate Production Environment".to_string()
    }
    fn closing_cta_activate_desc(&self) -> String {
        "Seamless transition from PoC to full protection.".to_string()
    }
    fn closing_cta_meet(&self) -> String {
        "Meet your Customer Success Manager".to_string()
    }
    fn closing_cta_meet_desc(&self) -> String {
        "Personalized onboarding and strategic planning.".to_string()
    }

    // Deep Analytics -> Executive Summary
    fn deep_analytics_title(&self) -> String {
        "🔍 Executive Threat Summary".to_string()
    }
    fn deep_analytics_subtitle(&self) -> String {
        "Advanced insights computed from your threat data".to_string()
    }
    fn deep_analytics_code_leak_title(&self) -> String {
        "Code Leak Insights".to_string()
    }
    fn deep_analytics_code_leak_subtitle(&self, count: u64) -> String {
        format!("{} unique repositories analyzed", count)
    }
    fn deep_analytics_credential_title(&self) -> String {
        "Credential Insights".to_string()
    }
    fn deep_analytics_credential_subtitle(&self, count: u64) -> String {
        format!("{} credentials analyzed", count)
    }
    fn deep_analytics_takedown_title(&self) -> String {
        "Takedown Efficiency".to_string()
    }
    fn deep_analytics_takedown_subtitle(&self, count: usize) -> String {
        format!("{} takedowns by platform", count)
    }

    fn story_title(&self) -> String {
        "Incident Story".to_string()
    }
    fn story_subtitle(&self, count: usize) -> String {
        format!("Timeline of {} related incidents", count)
    }

    // Context Slides - English
    fn ctx_risk_title(&self) -> String {
        "The Digital Risk Landscape".to_string()
    }
    fn ctx_risk_text(&self) -> String {
        "Digital risk monitoring is essential in today's interconnected world. This section provides an overview of the threats detected across the open, deep, and dark web. By identifying these risks early, we enable proactive mitigation strategies to protect your brand reputation and digital assets.".to_string()
    }

    // Advanced Storytelling - English
    fn eff_title(&self) -> String {
        "Operational Efficiency".to_string()
    }
    fn eff_text_hours(&self, hours: u64, analysts: f64) -> String {
        format!("To manually analyze the threats processed by Axur, your team would have needed <strong>{} hours</strong>. This is equivalent to <strong>{:.1} full-time analysts</strong> dedicated solely to detection.", hours, analysts)
    }
    fn eff_text_speed(&self) -> String {
        "Threat management with Axur was <strong>180x faster</strong> than the industry average. While a human takes ~30 mins per alert, our AI correlates threats in seconds.".to_string()
    }

    fn narrative_phishing_title(&self) -> String {
        "Phishing & Brand Abuse".to_string()
    }
    fn narrative_phishing_pain(&self) -> String {
        "Modern phishing evades traditional filters. Attempting to cover the volume of 40 million new URLs daily is humanly impossible.".to_string()
    }
    fn narrative_phishing_solution(&self) -> String {
        "Our AI visually inspects every site (Computer Vision), detecting fraud even without explicit brand mentions. We don't just detect; we take down in under 4 minutes.".to_string()
    }

    fn narrative_stealer_title(&self) -> String {
        "Understanding Infostealers".to_string()
    }
    fn narrative_stealer_pain(&self) -> String {
        "Stolen credentials from infected devices (Infostealers) allow attackers to bypass MFA by mimicking the user's digital fingerprint.".to_string()
    }
    fn narrative_stealer_solution(&self) -> String {
        "Manually searching the Deep Web is inefficient. Axur scans 42 billion credentials to find only those that pose a real risk to your active sessions.".to_string()
    }

    fn narrative_takedown_title(&self) -> String {
        "The Takedown Process".to_string()
    }
    fn narrative_takedown_pain(&self) -> String {
        "Managing takedowns across different platforms requires handling endless bureaucracy and forms for each provider.".to_string()
    }
    fn narrative_takedown_solution(&self) -> String {
        "Axur has trusted integrations with these platforms. What takes a lawyer days, we request en masse with a 98.9% success rate.".to_string()
    }

    fn narrative_timeline_title(&self) -> String {
        "Attack Timeline vs. Human Capacity".to_string()
    }
    fn narrative_timeline_text(&self, percent: u64) -> String {
        format!("Cybercriminals don't respect office hours. <strong>{}%</strong> of critical threats were detected outside business hours. Without 24/7 automation, these would remain active for days.", percent)
    }

    fn closing_value_title(&self) -> String {
        "Your New Extended Team".to_string()
    }
    fn closing_value_intro(&self) -> String {
        "Acquiring Axur isn't just buying software; it's adding an army of virtual analysts to your SOC.".to_string()
    }
    fn closing_value_item_1_title(&self) -> String {
        "Fatigue Reduction".to_string()
    }
    fn closing_value_item_1_desc(&self) -> String {
        "We eliminate noise so your analysts only see what matters.".to_string()
    }
    fn closing_value_item_2_title(&self) -> String {
        "Reaction Speed".to_string()
    }
    fn closing_value_item_2_desc(&self) -> String {
        "From days to minutes. We drastically minimize exposure time.".to_string()
    }
    fn closing_value_item_3_title(&self) -> String {
        "Total Visibility".to_string()
    }
    fn closing_value_item_3_desc(&self) -> String {
        "We provide full context: 'Who, How, and Methodology', eliminating false positives so your team focuses only on real threats.".to_string()
    }

    fn narrative_virality_title(&self) -> String {
        "DEEP WEB & VIRALITY".to_string()
    }
    fn narrative_virality_pain_primary(
        &self,
        share_count: u64,
        source_count: usize,
        top_source: &str,
    ) -> String {
        format!("Your brand was shared **{} times** in {} different fraud communities, including high-activity groups like '**{}**'. This 'chatter' often precedes a massive wave of attacks.", share_count, source_count, top_source)
    }
    fn narrative_virality_solution_primary(&self) -> String {
        "Axur has infiltrated these invite-only channels (Telegram, Discord, Dark Web). We detect the threat at the *planning* stage, before it reaches your customers.".to_string()
    }
    fn narrative_virality_pain_fallback(&self) -> String {
        "Attacks often incubate silently in closed groups on Telegram and the Dark Web, completely invisible to standard perimeter defenses.".to_string()
    }
    fn narrative_virality_solution_fallback(&self) -> String {
        "Our Deep Web surveillance is active 24/7. In this period, **no critical viral campaigns** were detected, validating that your brand is currently not a 'trending topic' for fraudsters.".to_string()
    }

    fn roi_precise_title(&self) -> String {
        "Velocity Wins".to_string()
    }
    fn roi_precise_text_primary(&self, median_minutes: i64) -> String {
        format!("Real Data: Your median takedown time was **{} minutes**. This speed neutralizes phishing campaigns before they can claim victims.", median_minutes)
    }
    fn roi_precise_text_fallback(&self) -> String {
        "Managing threats with Axur is **180x faster** than industry average. AI correlation happens in seconds, replacing 30-minute manual triage per alert.".to_string()
    }

    fn intent_title(&self) -> String {
        "AI Intent Analysis".to_string()
    }
    fn intent_fmt_primary(&self, top_intent: &str, percent: u64) -> String {
        format!("Our AI Classifiers reveal that **{}%** of attacks aim for **{}**, indicating a specific campaign against your customer base.", percent, top_intent)
    }
    fn intent_fmt_fallback(&self) -> String {
        "Attacks are categorized by technical vector. Phishing remains the dominant method for initiating fraud.".to_string()
    }
    fn intent_cat_credentials(&self) -> String {
        "Credential Theft".to_string()
    }
    fn intent_cat_infection(&self) -> String {
        "Infection & Access".to_string()
    }
    fn intent_cat_fraud(&self) -> String {
        "Fraud & Reputation".to_string()
    }
    fn intent_cat_trust(&self) -> String {
        "Brand Trust & Phishing".to_string()
    }
    fn intent_cat_chat(&self) -> String {
        "Chat Intelligence".to_string()
    }
    fn intent_cat_compromised(&self) -> String {
        "Compromised Devices".to_string()
    }
    fn intent_cat_data_leak(&self) -> String {
        "Data Leakage".to_string()
    }
    fn intent_cat_vip(&self) -> String {
        "VIP Protection".to_string()
    }
    fn intent_cat_dark_web(&self) -> String {
        "Deep & Dark Web".to_string()
    }

    fn geo_title(&self) -> String {
        "Global Attack Origins".to_string()
    }
    fn geo_fmt_primary(&self, count: usize, top_country: &str) -> String {
        format!("We detected attacks originating from **{} countries**. The primary source of hostile infrastructure is **{}**.", count, top_country)
    }
    fn geo_fmt_fallback(&self) -> String {
        "Geographic attribution reveals jurisdiction overlaps. Monitoring international infrastructure helps predict future campaigns.".to_string()
    }
    fn geo_lbl_countries(&self) -> String {
        "Top Origin Countries".to_string()
    }
    fn geo_lbl_isps(&self) -> String {
        "Top ISPs (Networks)".to_string()
    }

    fn geo_why_important_title(&self) -> String {
        "Why is this important?".to_string()
    }
    fn geo_why_important_text(&self) -> String {
        "The geographic origin of attacks helps identify global infrastructure used by cybercriminals. This data enables strategic geo-blocking and aids in attribution, allowing for more effective defensive measures against specific regional threats.".to_string()
    }

    fn ctx_stealer_title(&self) -> String {
        "Understanding Infostealers".to_string()
    }
    fn ctx_stealer_text(&self) -> String {
        "Infostealers are malware designed to harvest sensitive information from infected devices. They collect login credentials, cookies, and system details, often sold on dark web marketplaces. Detecting these logs allows us to identify compromised employee or customer accounts before they are used for unauthorized access.".to_string()
    }

    fn ctx_leak_title(&self) -> String {
        "The Danger of Exposed Secrets".to_string()
    }
    fn ctx_leak_text(&self) -> String {
        "Developers sometimes inadvertently commit sensitive keys, tokens, or credentials to public repositories. These 'secrets' can grant attackers access to cloud infrastructure, databases, or internal services. Continuous monitoring of public code repositories is crucial to detect and revoke these keys immediately.".to_string()
    }

    fn ctx_takedown_title(&self) -> String {
        "The Takedown Process".to_string()
    }
    fn ctx_takedown_text(&self) -> String {
        "Takedown is the process of removing malicious or infringing content from the internet. When a threat is confirmed, our automated systems and legal team interact with hosting providers, registrars, and social media platforms to enforce removal, neutralizing the threat at its source.".to_string()
    }
}

impl Dictionary for Spanish {
    fn welcome_message(&self) -> String {
        "Bienvenido a Axur CLI".to_string()
    }
    fn login_prompt_email(&self) -> String {
        "Correo Electrónico".to_string()
    }
    fn footer_text(&self) -> String {
        "Axur. Digital experiences made safe. Todos los derechos reservados.".to_string()
    }

    fn label_partner(&self) -> String {
        "PARTNER".to_string()
    }
    fn label_company(&self) -> String {
        "EMPRESA".to_string()
    }
    fn label_tlp(&self) -> String {
        "TLP:".to_string()
    }
    fn label_tlp_desc(&self) -> String {
        "Protocolo de Semáforo (TLP). Restricciones sobre cómo se puede compartir esta información."
            .to_string()
    }

    fn cover_title_dynamic(&self) -> String {
        "Informe de<br>Monitoreo Digital<br>".to_string()
    }
    fn cover_title_static(&self) -> String {
        "Informe de Resultados<br>Prueba de Concepto<br>".to_string()
    }

    fn intro_title(&self) -> String {
        "Protección Digital: De la Detección a la Resolución".to_string()
    }
    fn intro_text_dynamic(&self) -> String {
        "Este informe detalla los resultados del monitoreo continuo de su marca, demostrando la capacidad de Axur para ofrecer visibilidad completa y respuesta eficaz ante los riesgos digitales.".to_string()
    }
    fn intro_text_static(&self) -> String {
        "Este informe detalla los resultados de nuestra Prueba de Concepto (PoC), demostrando la capacidad de Axur para ofrecer visibilidad completa y respuesta eficaz ante los riesgos digitales.".to_string()
    }
    fn intro_text_closing(&self) -> String {
        "Las siguientes páginas presentan un análisis detallado de las amenazas detectadas, las acciones tomadas y el valor tangible que nuestra plataforma aporta a la seguridad e integridad de su marca.".to_string()
    }

    fn solutions_title(&self) -> String {
        "Plataforma Completa para su Protección Digital".to_string()
    }
    fn solutions_subtitle_1(&self) -> String {
        "Axur ofrece una solución completa para la gestión de sus riesgos digitales.".to_string()
    }
    fn solutions_subtitle_2(&self) -> String {
        "Protegemos su marca, sus clientes y su negocio de fraudes y amenazas en línea.".to_string()
    }
    fn solutions_subtitle_3(&self) -> String {
        "Desde la detección hasta el takedown, nuestra plataforma automatiza todo el proceso."
            .to_string()
    }
    fn solution_takedown(&self) -> String {
        "Takedown".to_string()
    }
    fn solution_brand_protection(&self) -> String {
        "Protección de Marca".to_string()
    }
    fn solution_threat_intel(&self) -> String {
        "Inteligencia de Amenazas".to_string()
    }

    fn toc_title(&self) -> String {
        "Índice de Contenidos".to_string()
    }
    fn toc_items(&self) -> Vec<String> {
        vec![
            "Métricas Generales".to_string(),
            "Fraude Digital".to_string(),
            "Exposición de Datos".to_string(),
            "Ejecutivos y VIPs".to_string(),
            "Web Profunda y Oscura".to_string(),
            "Inteligencia de Amenazas y Exposición".to_string(),
            "Eficiencia Operativa".to_string(),
        ]
    }

    fn poc_scope_title(&self) -> String {
        "Alcance del Monitoreo".to_string()
    }
    fn poc_assets_title(&self) -> String {
        "Activos Monitoreados".to_string()
    }
    fn poc_label_brands(&self) -> String {
        "Marcas Monitoreadas".to_string()
    }
    fn poc_label_executives(&self) -> String {
        "Ejecutivos".to_string()
    }
    fn poc_label_ips(&self) -> String {
        "IPs / Rangos".to_string()
    }
    fn poc_label_bins(&self) -> String {
        "BINs".to_string()
    }
    fn poc_label_domains(&self) -> String {
        "Dominios".to_string()
    }

    fn poc_period_dynamic_title(&self) -> String {
        "Período de Análisis".to_string()
    }
    fn poc_period_dynamic_text(&self) -> String {
        "Últimos días hasta hoy".to_string()
    }
    fn poc_period_static_title(&self) -> String {
        "Duración de la PoC".to_string()
    }
    fn poc_period_start(&self) -> String {
        "Inicio".to_string()
    }
    fn poc_period_end(&self) -> String {
        "Fin".to_string()
    }

    fn metrics_title(&self) -> String {
        "Métricas Generales".to_string()
    }
    fn metrics_total_tickets(&self) -> String {
        "Total de Tickets (Detecciones Crudas)".to_string()
    }
    fn metrics_threats_detected(&self) -> String {
        "Amenazas Detectadas".to_string()
    }
    fn metrics_time_saved(&self) -> String {
        "Ahorro de Tiempo en Validación".to_string()
    }
    fn metrics_desc_tickets(&self) -> String {
        "El número total de señales que la plataforma ha recopilado.".to_string()
    }
    fn metrics_desc_threats(&self) -> String {
        "Señales que, tras el análisis y validación, se confirmaron como riesgos reales."
            .to_string()
    }
    fn metrics_desc_time(&self) -> String {
        "Tiempo estimado que su equipo habría dedicado a recolectar, analizar y validar manualmente estas mismas amenazas.".to_string()
    }

    fn threats_title(&self) -> String {
        "Amenazas Potenciales".to_string()
    }
    fn threats_desc(&self, total: u64) -> String {
        format!("Se detectaron un total de <strong>{}</strong> amenazas. El gráfico muestra las principales categorías, destacando los vectores de ataque más frecuentes contra su marca.", total)
    }

    fn stealer_title(&self) -> String {
        "Panorama de Riesgo: Credenciales por Infostealer".to_string()
    }
    fn stealer_subtitle(&self, count: u64) -> String {
        format!("Análisis de {} credenciales detectadas", count)
    }
    fn stealer_box_creds(&self) -> String {
        "Credenciales Totales".to_string()
    }
    fn stealer_box_hosts(&self) -> String {
        "Hosts Únicos Comprometidos".to_string()
    }
    fn stealer_box_high_risk(&self) -> String {
        "Usuarios de Alto Riesgo".to_string()
    }
    fn stealer_action(&self) -> String {
        "Acción Inmediata: Forzar el reseteo de contraseña para los usuarios expuestos y priorizar el aislamiento de los hosts infectados.".to_string()
    }
    fn stealer_critical_title(&self) -> String {
        "CRÍTICO: Patrón Corporativo Detectado".to_string()
    }
    fn stealer_critical_desc(&self, count: usize) -> String {
        format!("Se encontraron <span class=\"text-white font-bold\">{}</span> identidades usando variaciones del nombre de la empresa en sus contraseñas.", count)
    }

    fn code_leak_title(&self) -> String {
        "Panorama de Riesgo: Fugas de Credenciales en Código".to_string()
    }

    fn exposure_title(&self) -> String {
        "Exposición de Datos Sensibles".to_string()
    }
    fn exposure_sub_code(&self) -> String {
        "Fugas en Código Fuente".to_string()
    }
    fn exposure_sub_stealer(&self) -> String {
        "Credenciales (Infostealer)".to_string()
    }
    fn code_leak_subtitle(&self, count: u64) -> String {
        format!("Análisis de {} secretos expuestos públicamente", count)
    }
    fn code_leak_box_secrets(&self) -> String {
        "Secretos Totales Expuestos".to_string()
    }
    fn code_leak_box_repos(&self) -> String {
        "Repositorios Públicos Únicos".to_string()
    }
    fn code_leak_box_prod(&self) -> String {
        "Secretos de Producción".to_string()
    }
    fn code_leak_action(&self) -> String {
        "Acción Inmediata: Rotar las claves de 'Producción' y las Claves de API inmediatamente. Implementar un escáner de secretos en el pipeline de CI/CD.".to_string()
    }

    fn incidents_title(&self) -> String {
        "Incidentes por Tipo".to_string()
    }
    fn incidents_desc(&self, total: u64) -> String {
        format!("Se crearon un total de <strong>{}</strong> incidentes. Este gráfico detalla la relación entre detecciones brutas e incidentes confirmados.", total)
    }

    fn takedowns_title(&self) -> String {
        "Takedowns".to_string()
    }
    fn takedowns_requested(&self) -> String {
        "solicitudes de takedown".to_string()
    }

    // Context Slides - Spanish
    fn ctx_risk_title(&self) -> String {
        "El Panorama de Riesgo Digital".to_string()
    }
    fn ctx_risk_text(&self) -> String {
        "El monitoreo de riesgos digitales es esencial en el mundo interconectado de hoy. Esta sección ofrece una visión general de las amenazas detectadas en la web abierta, profunda y oscura. Al identificar estos riesgos temprano, permitimos estrategias de mitigación proactivas para proteger la reputación de su marca y sus activos digitales.".to_string()
    }

    fn ctx_stealer_title(&self) -> String {
        "Entendiendo los Infostealers".to_string()
    }
    fn ctx_stealer_text(&self) -> String {
        "Los Infostealers son malware diseñado para recolectar información sensible de dispositivos infectados. Roban credenciales de inicio de sesión, cookies y detalles del sistema, que a menudo se venden en mercados de la dark web. Detectar estos registros nos permite identificar cuentas de empleados o clientes comprometidas antes de que sean utilizadas para accesos no autorizados.".to_string()
    }

    fn ctx_leak_title(&self) -> String {
        "El Peligro de Secretos Expuestos".to_string()
    }
    fn ctx_leak_text(&self) -> String {
        "Los desarrolladores a veces publican inadvertidamente claves, tokens o credenciales sensibles en repositorios públicos. Estos 'secretos' pueden dar a los atacantes acceso a infraestructura en la nube, bases de datos o servicios internos. El monitoreo continuo de repositorios de código público es crucial para detectar y revocar estas claves de inmediato.".to_string()
    }

    fn ctx_takedown_title(&self) -> String {
        "El Proceso de Takedown".to_string()
    }
    fn ctx_takedown_text(&self) -> String {
        "El Takedown es el proceso de eliminación de contenido malicioso o infractor de internet. Cuando se confirma una amenaza, nuestros sistemas automatizados y equipo legal interactúan con proveedores de alojamiento, registradores y plataformas de redes sociales para forzar su eliminación, neutralizando la amenaza en su origen.".to_string()
    }
    fn takedowns_success_rate(&self) -> String {
        "tasa de éxito".to_string()
    }
    fn takedowns_median_notify(&self) -> String {
        "tiempo medio para<br>1ª notificación".to_string()
    }
    fn takedowns_median_uptime(&self) -> String {
        "uptime medio".to_string()
    }
    fn takedowns_status_title(&self) -> String {
        "Estado de takedown".to_string()
    }
    fn takedowns_solved(&self) -> String {
        "Resuelto".to_string()
    }
    fn takedowns_in_progress(&self) -> String {
        "En progreso".to_string()
    }
    fn takedowns_interrupted(&self) -> String {
        "Interrumpido".to_string()
    }
    fn takedowns_not_solved(&self) -> String {
        "No resuelto".to_string()
    }

    // Advanced Storytelling - Spanish
    fn eff_title(&self) -> String {
        "Eficiencia Operativa".to_string()
    }
    fn eff_text_hours(&self, hours: u64, analysts: f64) -> String {
        format!("Para analizar manualmente las amenazas procesadas por Axur, su equipo habría necesitado <strong>{} horas</strong>. Esto equivale a <strong>{:.1} analistas a tiempo completo</strong> dedicados solo a la detección.", hours, analysts)
    }
    fn eff_text_speed(&self) -> String {
        "La gestión de amenazas con Axur fue <strong>180 veces más rápida</strong> que el promedio de la industria. Mientras un humano tarda ~30 mins por alerta, nuestra IA correlaciona amenazas en segundos.".to_string()
    }

    fn narrative_phishing_title(&self) -> String {
        "Phishing y Abuso de Marca".to_string()
    }
    fn narrative_phishing_pain(&self) -> String {
        "El phishing moderno evita los filtros tradicionales. Intentar cubrir el volumen de 40 millones de URLs nuevas diariamente es humanamente imposible.".to_string()
    }
    fn narrative_phishing_solution(&self) -> String {
        "Nuestra IA inspecciona visualmente cada sitio (Computer Vision), detectando fraudes incluso sin menciones explícitas de la marca. No solo detectamos; eliminamos en menos de 4 minutos.".to_string()
    }

    fn narrative_stealer_title(&self) -> String {
        "Entendiendo los Infostealers".to_string()
    }
    fn narrative_stealer_pain(&self) -> String {
        "Las credenciales robadas de dispositivos infectados (Infostealers) permiten a los atacantes eludir el MFA imitando la huella digital del usuario.".to_string()
    }
    fn narrative_stealer_solution(&self) -> String {
        "Buscar manualmente en la Deep Web es ineficiente. Axur escanea 42 mil millones de credenciales para encontrar solo aquellas que representan un riesgo real para sus sesiones activas.".to_string()
    }

    fn narrative_takedown_title(&self) -> String {
        "El Proceso de Takedown".to_string()
    }
    fn narrative_takedown_pain(&self) -> String {
        "Gestionar takedowns en diferentes plataformas requiere manejar burocracia interminable y formularios específicos para cada proveedor.".to_string()
    }
    fn narrative_takedown_solution(&self) -> String {
        "Axur tiene integraciones de confianza con estas plataformas. Lo que a un abogado le toma días, nosotros lo solicitamos masivamente con una tasa de éxito del 98.9%.".to_string()
    }

    fn narrative_timeline_title(&self) -> String {
        "Línea de Tiempo vs. Capacidad Humana".to_string()
    }
    fn narrative_timeline_text(&self, percent: u64) -> String {
        format!("Los ciberdelincuentes no respetan el horario de oficina. El <strong>{}%</strong> de las amenazas críticas se detectaron fuera del horario laboral. Sin la automatización 24/7, estas habrían permanecido activas durante días.", percent)
    }

    fn closing_value_title(&self) -> String {
        "Su Nuevo Equipo Extendido".to_string()
    }
    fn closing_value_intro(&self) -> String {
        "Adquirir Axur no es solo comprar software; es sumar un ejército de analistas virtuales a su SOC.".to_string()
    }
    fn closing_value_item_1_title(&self) -> String {
        "Reducción de Fatiga".to_string()
    }
    fn closing_value_item_1_desc(&self) -> String {
        "Eliminamos el ruido para que sus analistas solo vean lo que importa.".to_string()
    }
    fn closing_value_item_2_title(&self) -> String {
        "Velocidad de Reacción".to_string()
    }
    fn closing_value_item_2_desc(&self) -> String {
        "De días a minutos. Minimizamos drásticamente el tiempo de exposición.".to_string()
    }
    fn closing_value_item_3_title(&self) -> String {
        "Visibilidad Total".to_string()
    }
    fn closing_value_item_3_desc(&self) -> String {
        "Entregamos el contexto completo: 'Quién, Cómo y Metodología', eliminando falsos positivos para que tu equipo se enfoque solo en amenazas reales.".to_string()
    }

    fn narrative_virality_title(&self) -> String {
        "DEEP WEB & VIRALIDAD".to_string()
    }
    fn narrative_virality_pain_primary(
        &self,
        share_count: u64,
        source_count: usize,
        top_source: &str,
    ) -> String {
        format!("Tu marca fue compartida **{} veces** en {} comunidades de fraude, incluyendo grupos activos como '**{}**'. Este 'ruido' suele preceder una ola masiva de ataques.", share_count, source_count, top_source)
    }
    fn narrative_virality_solution_primary(&self) -> String {
        "Axur está infiltrado en estos canales cerrados (Telegram, Discord, Dark Web). Detectamos la amenaza en la etapa de *planeación*, antes de que llegue a tus clientes.".to_string()
    }
    fn narrative_virality_pain_fallback(&self) -> String {
        "Los ataques suelen incubarse silenciosamente en grupos cerrados de Telegram y Dark Web, invisibles para las defensas perimetrales estándar.".to_string()
    }
    fn narrative_virality_solution_fallback(&self) -> String {
        "Nuestro monitoreo de Deep Web está activo 24/7. En este periodo, **no detectamos campañas virales críticas**, validando que tu marca no es actualmente un 'tema de moda' para los defraudadores.".to_string()
    }

    fn roi_precise_title(&self) -> String {
        "La Velocidad Gana".to_string()
    }
    fn roi_precise_text_primary(&self, median_minutes: i64) -> String {
        format!("Dato Real: Tu tiempo mediano de baja fue de **{} minutos**. Esta velocidad neutraliza campañas de phishing antes de que cobren víctimas.", median_minutes)
    }
    fn roi_precise_text_fallback(&self) -> String {
        "Gestionar amenazas con Axur es **180x más rápido** que el promedio. Nuestra IA correlaciona en segundos, reemplazando los 30 min de triaje manual.".to_string()
    }

    fn intent_title(&self) -> String {
        "Análisis de Intención (AI)".to_string()
    }
    fn intent_fmt_primary(&self, top_intent: &str, percent: u64) -> String {
        format!("Nuestra IA revela que el **{}%** de los ataques buscan **{}**, indicando una campaña específica contra sus usuarios.", percent, top_intent)
    }
    fn intent_fmt_fallback(&self) -> String {
        "Los ataques se categorizan por vector técnico. El Phishing sigue siendo el método dominante para iniciar fraudes.".to_string()
    }
    fn intent_cat_credentials(&self) -> String {
        "Robo de Credenciales".to_string()
    }
    fn intent_cat_infection(&self) -> String {
        "Infección y Acceso".to_string()
    }
    fn intent_cat_fraud(&self) -> String {
        "Fraude y Reputación".to_string()
    }
    fn intent_cat_trust(&self) -> String {
        "Fraude de Marca / Phishing".to_string()
    }
    fn intent_cat_chat(&self) -> String {
        "Chat Intelligence".to_string()
    }
    fn intent_cat_compromised(&self) -> String {
        "Dispositivos Comprometidos".to_string()
    }
    fn intent_cat_data_leak(&self) -> String {
        "Fuga de Datos y Accesos".to_string()
    }
    fn intent_cat_vip(&self) -> String {
        "Protección VIP".to_string()
    }
    fn intent_cat_dark_web(&self) -> String {
        "Deep & Dark Web".to_string()
    }

    fn geo_title(&self) -> String {
        "Orígenes Globales de Ataque".to_string()
    }
    fn geo_fmt_primary(&self, count: usize, top_country: &str) -> String {
        format!("Detectamos ataques originados en **{} países**. La fuente principal de infraestructura hostil es **{}**.", count, top_country)
    }
    fn geo_fmt_fallback(&self) -> String {
        "La atribución geográfica revela superposiciones de jurisdicción. Monitorear infraestructura internacional ayuda a predecir futuras campañas.".to_string()
    }
    fn geo_lbl_countries(&self) -> String {
        "Principales Países de Origen".to_string()
    }
    fn geo_lbl_isps(&self) -> String {
        "Top ISPs (Redes)".to_string()
    }

    fn geo_why_important_title(&self) -> String {
        "¿Por qué es importante esto?".to_string()
    }
    fn geo_why_important_text(&self) -> String {
        "El origen geográfico de los ataques revela la infraestructura global de los ciberdelincuentes. Este dato permite bloqueos estratégicos por región y mejora la atribución, facilitando defensas más efectivas contra amenazas localizadas.".to_string()
    }

    fn roi_title(&self) -> String {
        "Impacto y ROI".to_string()
    }
    fn roi_efficiency_title(&self) -> String {
        "Eficiencia Operativa".to_string()
    }
    fn roi_efficiency_label(&self) -> String {
        "FTEs Ahorrados".to_string()
    }
    fn roi_efficiency_desc(&self, hours: u64) -> String {
        format!(
            "Equivalente a {} horas de análisis manual ahorradas a su equipo.",
            hours
        )
    }
    fn roi_risk_title(&self) -> String {
        "Reducción de Riesgo".to_string()
    }
    fn roi_risk_label(&self) -> String {
        "vs Promedio Industria".to_string()
    }
    fn roi_risk_desc(&self, notify_time: &str) -> String {
        format!(
            "Tiempo de Detección Axur ({}) vs 48h (Manual).",
            notify_time
        )
    }
    fn roi_intel_title(&self) -> String {
        "Inteligencia Crítica".to_string()
    }
    fn roi_intel_desc(&self, count: u64) -> String {
        format!(
            "Perfil de riesgo basado en {} credenciales analizadas.",
            count
        )
    }
    fn roi_intel_stealer(&self) -> String {
        "Malware (Stealer Logs)".to_string()
    }
    fn roi_intel_plaintext(&self) -> String {
        "Password Texto Plano".to_string()
    }

    // Operational Impact Slide
    fn op_badge(&self) -> String {
        "IMPACTO OPERACIONAL".to_string()
    }
    fn op_time_saved_title(&self) -> String {
        "Tiempo Ahorrado".to_string()
    }
    fn op_time_saved_desc(&self) -> String {
        "Horas que tu equipo hubiera dedicado a procesar manualmente estas señales y amenazas."
            .to_string()
    }
    fn op_unit_person_days(&self) -> String {
        "días persona".to_string()
    }
    fn op_unit_hours(&self) -> String {
        "horas".to_string()
    }
    fn op_breakdown_validation(&self) -> String {
        "Validación".to_string()
    }
    fn op_breakdown_monitoring(&self) -> String {
        "Monitoreo".to_string()
    }
    fn op_breakdown_takedowns(&self) -> String {
        "Takedowns".to_string()
    }
    fn op_capacity_title(&self) -> String {
        "Capacidad Liberada".to_string()
    }
    fn op_capacity_desc(&self) -> String {
        "Capacidad equivalente de analistas de seguridad que Axur libera para tareas estratégicas."
            .to_string()
    }
    fn op_tickets_processed(&self) -> String {
        "tickets procesados".to_string()
    }
    fn op_credentials_monitored(&self) -> String {
        "credenciales monitoreadas".to_string()
    }
    fn op_response_title(&self) -> String {
        "Tiempo de Respuesta".to_string()
    }
    fn op_response_desc(&self) -> String {
        "Tiempo mediano desde detección hasta la primera notificación al proveedor.".to_string()
    }
    fn op_success_rate(&self) -> String {
        "Tasa de éxito".to_string()
    }
    fn op_takedowns_completed(&self) -> String {
        "Takedowns realizados".to_string()
    }

    fn examples_takedowns_title(&self) -> String {
        "Takedowns Resueltos: Ejemplos".to_string()
    }
    fn examples_poc_title(&self) -> String {
        "Ejemplos de Amenazas Detectadas".to_string()
    }
    fn example_label_type(&self) -> String {
        "Tipo:".to_string()
    }
    fn example_label_date(&self) -> String {
        "Fecha:".to_string()
    }
    fn example_label_url(&self) -> String {
        "URL:".to_string()
    }
    fn example_no_image(&self) -> String {
        "Imagen no disponible".to_string()
    }
    fn example_no_data(&self) -> String {
        "No se encontraron ejemplos de evidencia en el período seleccionado.".to_string()
    }

    fn closing_title(&self) -> String {
        "Protege lo que más importa".to_string()
    }
    fn closing_subtitle(&self) -> String {
        "Has visto los riesgos. Ahora convierte estos resultados en protección continua 24/7."
            .to_string()
    }
    fn closing_cta_activate(&self) -> String {
        "Activar Entorno de Producción".to_string()
    }
    fn closing_cta_activate_desc(&self) -> String {
        "Transición fluida de PoC a protección total.".to_string()
    }
    fn closing_cta_meet(&self) -> String {
        "Reunirse con Customer Success".to_string()
    }
    fn closing_cta_meet_desc(&self) -> String {
        "Onboarding personalizado y planificación estratégica.".to_string()
    }

    // Deep Analytics -> Executive Summary
    fn deep_analytics_title(&self) -> String {
        "🔍 Resumen Ejecutivo de Amenazas".to_string()
    }
    fn deep_analytics_subtitle(&self) -> String {
        "Insights avanzados computados a partir de tus datos de amenazas".to_string()
    }
    fn deep_analytics_code_leak_title(&self) -> String {
        "Insights de Fugas de Código".to_string()
    }
    fn deep_analytics_code_leak_subtitle(&self, count: u64) -> String {
        format!("{} repositorios únicos analizados", count)
    }
    fn deep_analytics_credential_title(&self) -> String {
        "Insights de Credenciales".to_string()
    }
    fn deep_analytics_credential_subtitle(&self, count: u64) -> String {
        format!("{} credenciales analizadas", count)
    }
    fn deep_analytics_takedown_title(&self) -> String {
        "Eficiencia de Takedowns".to_string()
    }
    fn deep_analytics_takedown_subtitle(&self, count: usize) -> String {
        format!("{} takedowns por plataforma", count)
    }

    fn story_title(&self) -> String {
        "Historia del Incidente".to_string()
    }
    fn story_subtitle(&self, count: usize) -> String {
        format!("Línea de tiempo de {} incidentes relacionados", count)
    }
}

impl Dictionary for Portuguese {
    fn welcome_message(&self) -> String {
        "Bem-vindo ao Axur CLI".to_string()
    }
    fn login_prompt_email(&self) -> String {
        "E-mail".to_string()
    }
    fn footer_text(&self) -> String {
        "Axur. Digital experiences made safe. Todos os direitos reservados.".to_string()
    }

    fn label_partner(&self) -> String {
        "PARCEIRO".to_string()
    }
    fn label_company(&self) -> String {
        "EMPRESA".to_string()
    }
    fn label_tlp(&self) -> String {
        "TLP:".to_string()
    }
    fn label_tlp_desc(&self) -> String {
        "Protocolo de Semáforo (TLP). Restrições sobre como esta informação pode ser compartilhada."
            .to_string()
    }

    fn cover_title_dynamic(&self) -> String {
        "Relatório de<br>Monitoramento Digital<br>".to_string()
    }
    fn cover_title_static(&self) -> String {
        "Relatório de Resultados<br>Prova de Conceito<br>".to_string()
    }

    fn intro_title(&self) -> String {
        "Proteção Digital: Da Detecção à Resolução".to_string()
    }
    fn intro_text_dynamic(&self) -> String {
        "Este relatório detalha os resultados do monitoramento contínuo da sua marca, demonstrando a capacidade da Axur de oferecer visibilidade completa e resposta eficaz aos riscos digitais.".to_string()
    }
    fn intro_text_static(&self) -> String {
        "Este relatório detalha os resultados da nossa Prova de Conceito (PoC), demonstrando a capacidade da Axur de oferecer visibilidade completa e resposta eficaz aos riscos digitais.".to_string()
    }
    fn intro_text_closing(&self) -> String {
        "As páginas a seguir apresentam uma análise detalhada das ameaças detectadas, ações tomadas e o valor tangível que nossa plataforma traz para a segurança e integridade da sua marca.".to_string()
    }

    fn solutions_title(&self) -> String {
        "Plataforma Completa para sua Proteção Digital".to_string()
    }
    fn solutions_subtitle_1(&self) -> String {
        "A Axur oferece uma solução completa para a gestão dos seus riscos digitais.".to_string()
    }
    fn solutions_subtitle_2(&self) -> String {
        "Protegemos sua marca, seus clientes e seu negócio contra fraudes e ameaças online."
            .to_string()
    }
    fn solutions_subtitle_3(&self) -> String {
        "Da detecção à remoção (takedown), nossa plataforma automatiza todo o processo.".to_string()
    }
    fn solution_takedown(&self) -> String {
        "Takedown".to_string()
    }
    fn solution_brand_protection(&self) -> String {
        "Proteção de Marca".to_string()
    }
    fn solution_threat_intel(&self) -> String {
        "Inteligência de Ameaças".to_string()
    }

    fn toc_title(&self) -> String {
        "Índice de Conteúdos".to_string()
    }
    fn toc_items(&self) -> Vec<String> {
        vec![
            "Métricas Gerais".to_string(),
            "Fraude Digital".to_string(),
            "Exposição de Dados".to_string(),
            "Executivos e VIPs".to_string(),
            "Deep & Dark Web".to_string(),
            "Inteligência e Exposição".to_string(),
            "Eficiência Operacional".to_string(),
        ]
    }

    fn poc_scope_title(&self) -> String {
        "Escopo do Monitoramento".to_string()
    }
    fn poc_assets_title(&self) -> String {
        "Ativos Monitorados".to_string()
    }
    fn poc_label_brands(&self) -> String {
        "Marcas Monitoradas".to_string()
    }
    fn poc_label_executives(&self) -> String {
        "Executivos".to_string()
    }
    fn poc_label_ips(&self) -> String {
        "IPs / Ranges".to_string()
    }
    fn poc_label_bins(&self) -> String {
        "BINs".to_string()
    }
    fn poc_label_domains(&self) -> String {
        "Domínios".to_string()
    }

    fn poc_period_dynamic_title(&self) -> String {
        "Período de Análise".to_string()
    }
    fn poc_period_dynamic_text(&self) -> String {
        "Últimos dias até hoje".to_string()
    }
    fn poc_period_static_title(&self) -> String {
        "Duração da PoC".to_string()
    }
    fn poc_period_start(&self) -> String {
        "Início".to_string()
    }
    fn poc_period_end(&self) -> String {
        "Fim".to_string()
    }

    fn metrics_title(&self) -> String {
        "Métricas Gerais".to_string()
    }
    fn metrics_total_tickets(&self) -> String {
        "Total de Tickets (Detecções Brutas)".to_string()
    }
    fn metrics_threats_detected(&self) -> String {
        "Ameaças Detectadas".to_string()
    }
    fn metrics_time_saved(&self) -> String {
        "Economia de Tempo na Validação".to_string()
    }
    fn metrics_desc_tickets(&self) -> String {
        "O número total de sinais que a plataforma coletou.".to_string()
    }
    fn metrics_desc_threats(&self) -> String {
        "Sinais que, após análise e validação, foram confirmados como riscos reais.".to_string()
    }
    fn metrics_desc_time(&self) -> String {
        "Tempo estimado que sua equipe teria gasto coletando e validando manualmente essas mesmas ameaças.".to_string()
    }

    fn threats_title(&self) -> String {
        "Ameaças Potenciais".to_string()
    }
    fn threats_desc(&self, total: u64) -> String {
        format!("Foram detectadas um total de <strong>{}</strong> ameaças. O gráfico mostra as principais categorias, destacando os vetores de ataque mais frequentes contra sua marca.", total)
    }

    fn stealer_title(&self) -> String {
        "Cenário de Risco: Credenciais por Infostealer".to_string()
    }
    fn stealer_subtitle(&self, count: u64) -> String {
        format!("Análise de {} credenciais detectadas", count)
    }
    fn stealer_box_creds(&self) -> String {
        "Credenciais Totais".to_string()
    }
    fn stealer_box_hosts(&self) -> String {
        "Hosts Únicos Comprometidos".to_string()
    }
    fn stealer_box_high_risk(&self) -> String {
        "Usuários de Alto Risco".to_string()
    }
    fn stealer_action(&self) -> String {
        "Ação Imediata: Forçar a redefinição de senha para usuários expostos e priorizar o isolamento de hosts infectados.".to_string()
    }
    fn stealer_critical_title(&self) -> String {
        "CRÍTICO: Padrão Corporativo Detectado".to_string()
    }
    fn stealer_critical_desc(&self, count: usize) -> String {
        format!("Foram encontradas <span class=\"text-white font-bold\">{}</span> identidades usando variações do nome da empresa em suas senhas.", count)
    }

    fn code_leak_title(&self) -> String {
        "Panorama de Risco: Vazamento de Credenciais em Código".to_string()
    }

    fn exposure_title(&self) -> String {
        "Exposição de Dados Sensíveis".to_string()
    }
    fn exposure_sub_code(&self) -> String {
        "Vazamentos em Código Fonte".to_string()
    }
    fn exposure_sub_stealer(&self) -> String {
        "Credenciais (Infostealer)".to_string()
    }
    fn code_leak_subtitle(&self, count: u64) -> String {
        format!("Análise de {} segredos expostos publicamente", count)
    }
    fn code_leak_box_secrets(&self) -> String {
        "Segredos Totais Expostos".to_string()
    }
    fn code_leak_box_repos(&self) -> String {
        "Repositórios Públicos Únicos".to_string()
    }
    fn code_leak_box_prod(&self) -> String {
        "Segredos de Produção".to_string()
    }
    fn code_leak_action(&self) -> String {
        "Ação Imediata: Rotacionar chaves de 'Produção' e chaves de API imediatamente. Implementar scanner de segredos no pipeline CI/CD.".to_string()
    }

    fn incidents_title(&self) -> String {
        "Incidentes por Tipo".to_string()
    }
    fn incidents_desc(&self, total: u64) -> String {
        format!("Foram criados um total de <strong>{}</strong> incidentes. Este gráfico detalha a relação entre detecções brutas e incidentes confirmados.", total)
    }

    fn takedowns_title(&self) -> String {
        "Takedowns".to_string()
    }
    fn takedowns_requested(&self) -> String {
        "takedowns solicitados".to_string()
    }
    fn takedowns_success_rate(&self) -> String {
        "taxa de sucesso".to_string()
    }
    fn takedowns_median_notify(&self) -> String {
        "tempo médio para<br>1ª notificação".to_string()
    }
    fn takedowns_median_uptime(&self) -> String {
        "uptime médio".to_string()
    }
    fn takedowns_status_title(&self) -> String {
        "Status do takedown".to_string()
    }
    fn takedowns_solved(&self) -> String {
        "Resolvido".to_string()
    }
    fn takedowns_in_progress(&self) -> String {
        "Em progresso".to_string()
    }
    fn takedowns_interrupted(&self) -> String {
        "Interrompido".to_string()
    }
    fn takedowns_not_solved(&self) -> String {
        "Não resolvido".to_string()
    }

    fn roi_title(&self) -> String {
        "Impacto e ROI".to_string()
    }
    fn roi_efficiency_title(&self) -> String {
        "Eficiência Operacional".to_string()
    }
    fn roi_efficiency_label(&self) -> String {
        "FTEs Economizados".to_string()
    }
    fn roi_efficiency_desc(&self, hours: u64) -> String {
        format!(
            "Equivalente a {} horas de análise manual economizadas para sua equipe.",
            hours
        )
    }
    fn roi_risk_title(&self) -> String {
        "Redução de Risco".to_string()
    }
    fn roi_risk_label(&self) -> String {
        "vs Média de Mercado".to_string()
    }
    fn roi_risk_desc(&self, notify_time: &str) -> String {
        format!("Tempo de Detecção Axur ({}) vs 48h (Manual).", notify_time)
    }
    fn roi_intel_title(&self) -> String {
        "Inteligência Crítica".to_string()
    }
    fn roi_intel_desc(&self, count: u64) -> String {
        format!(
            "Perfil de risco baseado em {} credenciais analisadas.",
            count
        )
    }
    fn roi_intel_stealer(&self) -> String {
        "Malware (Stealer Logs)".to_string()
    }
    fn roi_intel_plaintext(&self) -> String {
        "Senhas em Texto Plano".to_string()
    }

    // Operational Impact Slide
    fn op_badge(&self) -> String {
        "IMPACTO OPERACIONAL".to_string()
    }
    fn op_time_saved_title(&self) -> String {
        "Tempo Economizado".to_string()
    }
    fn op_time_saved_desc(&self) -> String {
        "Horas que sua equipe teria dedicado para processar manualmente esses sinais e ameaças."
            .to_string()
    }
    fn op_unit_person_days(&self) -> String {
        "dias pessoa".to_string()
    }
    fn op_unit_hours(&self) -> String {
        "horas".to_string()
    }
    fn op_breakdown_validation(&self) -> String {
        "Validação".to_string()
    }
    fn op_breakdown_monitoring(&self) -> String {
        "Monitoramento".to_string()
    }
    fn op_breakdown_takedowns(&self) -> String {
        "Takedowns".to_string()
    }
    fn op_capacity_title(&self) -> String {
        "Capacidade Liberada".to_string()
    }
    fn op_capacity_desc(&self) -> String {
        "Capacidade equivalente de analistas de segurança que a Axur libera para tarefas estratégicas.".to_string()
    }
    fn op_tickets_processed(&self) -> String {
        "tickets processados".to_string()
    }
    fn op_credentials_monitored(&self) -> String {
        "credenciais monitoradas".to_string()
    }
    fn op_response_title(&self) -> String {
        "Tempo de Resposta".to_string()
    }
    fn op_response_desc(&self) -> String {
        "Tempo mediano desde detecção até a primeira notificação ao provedor.".to_string()
    }
    fn op_success_rate(&self) -> String {
        "Taxa de sucesso".to_string()
    }
    fn op_takedowns_completed(&self) -> String {
        "Takedowns realizados".to_string()
    }

    fn examples_takedowns_title(&self) -> String {
        "Takedowns Resolvidos: Exemplos".to_string()
    }
    fn examples_poc_title(&self) -> String {
        "Exemplos de Ameaças Detectadas".to_string()
    }
    fn example_label_type(&self) -> String {
        "Tipo:".to_string()
    }
    fn example_label_date(&self) -> String {
        "Data:".to_string()
    }
    fn example_label_url(&self) -> String {
        "URL:".to_string()
    }
    fn example_no_image(&self) -> String {
        "Imagem indisponível".to_string()
    }
    fn example_no_data(&self) -> String {
        "Nenhum exemplo de evidência encontrado no período selecionado.".to_string()
    }

    fn closing_title(&self) -> String {
        "Proteja o que mais importa".to_string()
    }
    fn closing_subtitle(&self) -> String {
        "Você viu os riscos. Agora transforme esses resultados em proteção contínua 24/7."
            .to_string()
    }
    fn closing_cta_activate(&self) -> String {
        "Ativar Ambiente de Produção".to_string()
    }
    fn closing_cta_activate_desc(&self) -> String {
        "Transição fluida da PoC para proteção total.".to_string()
    }
    fn closing_cta_meet(&self) -> String {
        "Falar com Customer Success".to_string()
    }
    fn closing_cta_meet_desc(&self) -> String {
        "Onboarding personalizado e planejamento estratégico.".to_string()
    }

    // Deep Analytics
    // Deep Analytics -> Executive Summary
    fn deep_analytics_title(&self) -> String {
        "🔍 Resumo Executivo de Ameaças".to_string()
    }
    fn deep_analytics_subtitle(&self) -> String {
        "Insights avançados computados a partir dos seus dados de ameaças".to_string()
    }
    fn deep_analytics_code_leak_title(&self) -> String {
        "Insights de Vazamento de Código".to_string()
    }
    fn deep_analytics_code_leak_subtitle(&self, count: u64) -> String {
        format!("{} repositórios únicos analisados", count)
    }
    fn deep_analytics_credential_title(&self) -> String {
        "Insights de Credenciais".to_string()
    }
    fn deep_analytics_credential_subtitle(&self, count: u64) -> String {
        format!("{} credenciais analisadas", count)
    }
    fn deep_analytics_takedown_title(&self) -> String {
        "Eficiência de Takedowns".to_string()
    }
    fn deep_analytics_takedown_subtitle(&self, count: usize) -> String {
        format!("{} takedowns por plataforma", count)
    }

    fn story_title(&self) -> String {
        "Histórico do Incidente".to_string()
    }
    fn story_subtitle(&self, count: usize) -> String {
        format!("Linha do tempo de {} incidentes relacionados", count)
    }

    // Context Slides - Portuguese
    fn ctx_risk_title(&self) -> String {
        "O Panorama de Risco Digital".to_string()
    }
    fn ctx_risk_text(&self) -> String {
        "O monitoramento de riscos digitais é essencial no mundo interconectado de hoje. Esta seção oferece uma visão geral das ameaças detectadas na web aberta, profunda e dark. Ao identificar esses riscos antecipadamente, possibilitamos estratégias de mitigação proativas para proteger a reputação da sua marca e seus ativos digitais.".to_string()
    }

    fn ctx_stealer_title(&self) -> String {
        "Entendendo os Infostealers".to_string()
    }
    fn ctx_stealer_text(&self) -> String {
        "Infostealers são malwares projetados para coletar informações sensíveis de dispositivos infectados. Eles roubam credenciais de login, cookies e detalhes do sistema, frequentemente vendidos em mercados da dark web. Detectar esses registros nos permite identificar contas de funcionários ou clientes comprometidas antes que sejam usadas para acessos não autorizados.".to_string()
    }

    fn ctx_leak_title(&self) -> String {
        "O Perigo de Segredos Expostos".to_string()
    }
    fn ctx_leak_text(&self) -> String {
        "Desenvolvedores às vezes publicam inadvertidamente chaves, tokens ou credenciais sensíveis em repositórios públicos. Esses 'segredos' podem dar aos atacantes acesso à infraestrutura em nuvem, banco de dados ou serviços internos. O monitoramento contínuo de repositórios de código público é crucial para detectar e revogar essas chaves imediatamente.".to_string()
    }

    fn ctx_takedown_title(&self) -> String {
        "O Processo de Takedown".to_string()
    }
    fn ctx_takedown_text(&self) -> String {
        "Takedown é o processo de remoção de conteúdo malicioso ou infrator da internet. Quando uma ameaça é confirmada, nossos sistemas automatizados e equipe jurídica interagem com provedores de hospedagem, registradores e plataformas de redes sociais para forçar a remoção, neutralizando a ameaça em sua origem.".to_string()
    }

    // Advanced Storytelling - Portuguese
    fn eff_title(&self) -> String {
        "Eficiência Operacional".to_string()
    }
    fn eff_text_hours(&self, hours: u64, analysts: f64) -> String {
        format!("Para analisar manualmente as ameaças processadas pela Axur, sua equipe precisaria dedicar <strong>{} horas</strong>. Isso equivale a <strong>{:.1} analistas em tempo integral</strong> dedicados apenas à detecção.", hours, analysts)
    }
    fn eff_text_speed(&self) -> String {
        "A gestão de ameaças com a Axur foi <strong>180x mais rápida</strong> que a média da indústria. Enquanto um humano leva ~30 min por alerta, nossa IA correlaciona ameaças em segundos.".to_string()
    }

    fn narrative_phishing_title(&self) -> String {
        "Phishing & Abuso de Marca".to_string()
    }
    fn narrative_phishing_pain(&self) -> String {
        "O phishing moderno evita filtros tradicionais. Tentar cobrir o volume de 40 milhões de novas URLs diariamente é humanamente impossível.".to_string()
    }
    fn narrative_phishing_solution(&self) -> String {
        "Nossa IA inspeciona visualmente cada site (Visão Computacional), detectando fraudes mesmo sem menções explícitas à marca. Não apenas detectamos; removemos em menos de 4 minutos.".to_string()
    }

    fn narrative_stealer_title(&self) -> String {
        "Entendendo os Infostealers".to_string()
    }
    fn narrative_stealer_pain(&self) -> String {
        "Credenciais roubadas de dispositivos infectados (Infostealers) permitem que atacantes ignorem o MFA imitando a impressão digital do usuário.".to_string()
    }
    fn narrative_stealer_solution(&self) -> String {
        "Buscar manualmente na Deep Web é ineficiente. A Axur escaneia 42 bilhões de credenciais para encontrar apenas aquelas que representam risco real para suas sessões ativas.".to_string()
    }

    fn narrative_takedown_title(&self) -> String {
        "O Processo de Takedown".to_string()
    }
    fn narrative_takedown_pain(&self) -> String {
        "Gerenciar takedowns em diferentes plataformas exige lidar com burocracia sem fim e formulários específicos para cada provedor.".to_string()
    }
    fn narrative_takedown_solution(&self) -> String {
        "A Axur possui integrações confiáveis com essas plataformas. O que leva dias para um advogado, solicitamos em massa com uma taxa de sucesso de 98,9%.".to_string()
    }

    fn narrative_timeline_title(&self) -> String {
        "Linha do Tempo de Ataques vs. Capacidade Humana".to_string()
    }
    fn narrative_timeline_text(&self, percent: u64) -> String {
        format!("Cibercriminosos não respeitam horário comercial. <strong>{}%</strong> das ameaças críticas foram detectadas fora do expediente. Sem a automação 24/7, elas permaneceriam ativas por dias.", percent)
    }

    fn closing_value_title(&self) -> String {
        "Sua Nova Equipe Estendida".to_string()
    }
    fn closing_value_intro(&self) -> String {
        "Adquirir a Axur não é apenas comprar software; é adicionar um exército de analistas virtuais ao seu SOC.".to_string()
    }
    fn closing_value_item_1_title(&self) -> String {
        "Redução de Fadiga".to_string()
    }
    fn closing_value_item_1_desc(&self) -> String {
        "Eliminamos o ruído para que seus analistas vejam apenas o que importa.".to_string()
    }
    fn closing_value_item_2_title(&self) -> String {
        "Velocidade de Reação".to_string()
    }
    fn closing_value_item_2_desc(&self) -> String {
        "De dias para minutos. Minimizamos drasticamente o tempo de exposição.".to_string()
    }
    fn closing_value_item_3_title(&self) -> String {
        "Visibilidade Total".to_string()
    }
    fn closing_value_item_3_desc(&self) -> String {
        "Entregamos contexto completo: 'Quem, Como e Metodologia', eliminando falsos positivos para que seu time foque apenas em ameaças reais.".to_string()
    }

    fn narrative_virality_title(&self) -> String {
        "DEEP WEB & VIRALIDADE".to_string()
    }
    fn narrative_virality_pain_primary(
        &self,
        share_count: u64,
        source_count: usize,
        top_source: &str,
    ) -> String {
        format!("Sua marca foi compartilhada **{} vezes** em {} comunidades de fraude, incluindo grupos como '**{}**'. Esse 'ruído' geralmente precede uma onda massiva de ataques.", share_count, source_count, top_source)
    }
    fn narrative_virality_solution_primary(&self) -> String {
        "A Axur está infiltrada nesses canais fechados (Telegram, Discord, Dark Web). Detectamos a ameaça no estágio de *planejamento*, antes que atinja seus clientes.".to_string()
    }
    fn narrative_virality_pain_fallback(&self) -> String {
        "Ataques costumam ser incubados silenciosamente em grupos fechados do Telegram e Dark Web, invisíveis para defesas de perímetro padrão.".to_string()
    }
    fn narrative_virality_solution_fallback(&self) -> String {
        "Nosso monitoramento de Deep Web está ativo 24/7. Neste período, **não detectamos campanhas virais críticas**, validando que sua marca não é atualmente um 'tópico de tendência' para fraudadores.".to_string()
    }

    fn roi_precise_title(&self) -> String {
        "Velocidade é Segurança".to_string()
    }
    fn roi_precise_text_primary(&self, median_minutes: i64) -> String {
        format!("Dado Real: Seu tempo mediano de takedown foi de **{} minutos**. Essa velocidade neutraliza campanhas de phishing antes de fazerem vítimas.", median_minutes)
    }
    fn roi_precise_text_fallback(&self) -> String {
        "Gerenciar ameaças com a Axur é **180x mais rápido** que a média. Nossa IA correlaciona em segundos, substituindo 30 min de triagem manual.".to_string()
    }

    fn intent_title(&self) -> String {
        "Análise de Intenção (IA)".to_string()
    }
    fn intent_fmt_primary(&self, top_intent: &str, percent: u64) -> String {
        format!("Nossa IA revela que **{}%** dos ataques visam **{}**, indicando uma campanha específica contra sua base.", percent, top_intent)
    }
    fn intent_fmt_fallback(&self) -> String {
        "Ataques são categorizados por vetor técnico. Phishing continua sendo o método dominante para iniciar fraudes.".to_string()
    }
    fn intent_cat_credentials(&self) -> String {
        "Roubo de Credenciais".to_string()
    }
    fn intent_cat_infection(&self) -> String {
        "Infecção e Acesso".to_string()
    }
    fn intent_cat_fraud(&self) -> String {
        "Fraude e Reputação".to_string()
    }
    fn intent_cat_trust(&self) -> String {
        "Proteção de Marca e Clientes".to_string()
    }
    fn intent_cat_chat(&self) -> String {
        "Chat Intelligence".to_string()
    }
    fn intent_cat_compromised(&self) -> String {
        "Dispositivos Comprometidos".to_string()
    }
    fn intent_cat_data_leak(&self) -> String {
        "Fuga de Dados e Acessos".to_string()
    }
    fn intent_cat_vip(&self) -> String {
        "Proteção VIP".to_string()
    }
    fn intent_cat_dark_web(&self) -> String {
        "Deep & Dark Web".to_string()
    }

    fn geo_title(&self) -> String {
        "Origens Globais de Ataque".to_string()
    }
    fn geo_fmt_primary(&self, count: usize, top_country: &str) -> String {
        format!("Detectamos ataques originados em **{} países**. A principal fonte de infraestrutura hostil é **{}**.", count, top_country)
    }
    fn geo_fmt_fallback(&self) -> String {
        "A atribuição geográfica revela sobreposições de jurisdição. Monitorar infraestrutura internacional ajuda a prever campanhas futuras.".to_string()
    }
    fn geo_lbl_countries(&self) -> String {
        "Principais Países de Origem".to_string()
    }
    fn geo_lbl_isps(&self) -> String {
        "Top ISPs (Redes)".to_string()
    }

    fn geo_why_important_title(&self) -> String {
        "Por que isso é importante?".to_string()
    }
    fn geo_why_important_text(&self) -> String {
        "A origem geográfica dos ataques revela a infraestrutura global usada por cibercriminosos. Esses dados permitem bloqueio geográfico estratégico e auxiliam na atribuição, permitindo defesas mais eficazes contra ameaças regionais.".to_string()
    }
}

pub fn get_dictionary(lang: Language) -> Box<dyn Dictionary> {
    match lang {
        Language::En => Box::new(English),
        Language::Es => Box::new(Spanish),
        Language::PtBr => Box::new(Portuguese),
    }
}
