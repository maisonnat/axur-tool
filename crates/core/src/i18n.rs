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

pub trait Dictionary {
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

    // Deep Analytics
    fn deep_analytics_title(&self) -> String {
        "🔍 Deep Analytics".to_string()
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

    fn code_leak_title(&self) -> String {
        "Panorama de Riesgo: Fugas de Credenciales en Código".to_string()
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

    // Deep Analytics
    fn deep_analytics_title(&self) -> String {
        "🔍 Análisis Profundo".to_string()
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
        "Ação Imediata: Forçar a redefinição de senha para usuários expostos e priorizar o isolamento dos hosts infectados.".to_string()
    }

    fn code_leak_title(&self) -> String {
        "Cenário de Risco: Vazamento de Credenciais em Código".to_string()
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
    fn deep_analytics_title(&self) -> String {
        "🔍 Análise Profunda".to_string()
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
}

pub fn get_dictionary(lang: Language) -> Box<dyn Dictionary> {
    match lang {
        Language::En => Box::new(English),
        Language::Es => Box::new(Spanish),
        Language::PtBr => Box::new(Portuguese),
    }
}
