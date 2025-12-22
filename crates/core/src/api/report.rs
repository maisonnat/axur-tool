#![allow(dead_code)]
#![allow(unused)]

use crate::api::{create_client, API_URL};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use chrono::Local;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;

// ========================
// IMAGE DOWNLOAD HELPER
// ========================

/// Download image from Axur API with auth and convert to base64 data URI
async fn download_image_as_base64(
    client: &reqwest::Client,
    auth: &str,
    url: &str,
) -> Option<String> {
    let resp = client
        .get(url)
        .header("Authorization", auth)
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let bytes = resp.bytes().await.ok()?;
    let base64_str = general_purpose::STANDARD.encode(&bytes);

    // Detect image type from URL
    let mime = if url.ends_with(".png") {
        "image/png"
    } else if url.ends_with(".gif") {
        "image/gif"
    } else {
        "image/jpeg"
    };

    Some(format!("data:{};base64,{}", mime, base64_str))
}

// ========================
// DEBUG LOGGING
// ========================

fn ensure_debug_dir() {
    let _ = fs::create_dir_all("json_temp");
}

fn log_api_call(endpoint_name: &str, url: &str, status: u16, success: bool, body: &str) {
    // Only log when debug mode is enabled via --debug flag
    if std::env::var("AXUR_DEBUG").is_err() {
        return;
    }

    ensure_debug_dir();

    // Save response JSON
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("json_temp/{}_{}.json", endpoint_name, timestamp);
    if let Ok(mut file) = fs::File::create(&filename) {
        let _ = file.write_all(body.as_bytes());
    }

    // Append to log file
    let log_line = format!(
        "[{}] {} | {} | HTTP {} | {} bytes\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        if success { "✓ OK" } else { "✗ ERR" },
        endpoint_name,
        status,
        body.len()
    );

    if let Ok(mut log_file) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("json_temp/api_calls.log")
    {
        let _ = log_file.write_all(log_line.as_bytes());
        // Also log the URL
        let url_line = format!("    URL: {}\n", url);
        let _ = log_file.write_all(url_line.as_bytes());

        // Log first 500 chars of response for debugging
        let preview = if body.len() > 500 { &body[..500] } else { body };
        let preview_line = format!("    Response preview: {}\n\n", preview.replace('\n', " "));
        let _ = log_file.write_all(preview_line.as_bytes());
    }
}

// ========================
// TENANT LISTING (for UX)
// ========================

/// Represents a tenant available to the current user
#[derive(Debug, Clone)]
pub struct TenantInfo {
    pub key: String,
    pub name: String,
}

/// Fetch all tenants available to the authenticated user.
/// Used for interactive tenant picker when no --tenants flag provided.
pub async fn fetch_available_tenants(token: &str) -> Result<Vec<TenantInfo>> {
    let client = create_client()?;
    let auth_header = format!("Bearer {}", token);
    let url = format!("{}/customers/customers", API_URL);

    let resp = client
        .get(&url)
        .header("Authorization", &auth_header)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Ok(vec![]);
    }

    let body = resp.text().await?;

    // Try parsing as array first (most common format)
    if let Ok(customers) = serde_json::from_str::<Vec<CustomerItem>>(&body) {
        let tenants: Vec<TenantInfo> = customers
            .into_iter()
            .filter_map(|c| {
                let key = c.key?;
                let name = c.name.unwrap_or_else(|| key.clone());
                Some(TenantInfo { key, name })
            })
            .collect();
        return Ok(tenants);
    }

    // Try parsing as object with "customers" array
    if let Ok(wrapper) = serde_json::from_str::<CustomerApiResponse>(&body) {
        let tenants: Vec<TenantInfo> = wrapper
            .customers
            .into_iter()
            .filter_map(|c| {
                let key = c.key?;
                let name = c.name.unwrap_or_else(|| key.clone());
                Some(TenantInfo { key, name })
            })
            .collect();
        return Ok(tenants);
    }

    Ok(vec![])
}

// ========================
// COMPLETE POC REPORT DATA
// ========================

/// Full PoC report data matching the exact structure of axur_presentation_tool
#[derive(Debug, Serialize, Clone)]
pub struct PocReportData {
    // Customer Info
    pub company_name: String,
    pub partner_name: Option<String>,
    pub tlp_level: String,

    // Date Range
    pub start_date: String,
    pub end_date: String,

    // Monitored Assets
    pub brands_count: u32,
    pub brands: Vec<String>,
    pub executives_count: u32,
    pub ips_count: u32,
    pub bins_count: u32,
    pub domains_count: u32,

    // Investigation Access
    pub threat_hunting_credits: u32,
    pub threat_intelligence_assets: u32,

    // General Metrics
    pub total_tickets: u64,
    pub total_threats: u64,
    pub validation_hours: f64,

    // Threats by Type (for chart)
    pub threats_by_type: Vec<ThreatTypeCount>,

    // Infostealer Summary
    pub credentials_total: u64,
    pub unique_hosts: u64,
    pub high_risk_users: u64,
    pub malware_breakdown: Vec<NameValuePair>,
    pub top_services: Vec<NameValuePair>,

    // Code Leaks Summary (from getCodeLeaksSummary)
    pub secrets_total: u64,
    pub unique_repos: u64,
    pub production_secrets: u64,
    pub platform_breakdown: Vec<NameValuePair>,
    pub secret_types: Vec<NameValuePair>,

    // NEW: Credential Leaks Summary (Risk Profiling)
    pub credential_leaks_summary: CredentialLeaksSummary,

    // Incidents by Type (detections vs incidents)
    pub incidents_by_type: Vec<IncidentTypeCount>,

    // Takedowns
    pub takedown_resolved: u64,
    pub takedown_pending: u64,
    pub takedown_aborted: u64,
    pub takedown_unresolved: u64, // NEW

    // NEW: Advanced Takedown Metrics
    pub takedown_success_rate: f64, // Percentage of successful takedowns
    pub takedown_median_time_to_notify: String, // Time to first notification (e.g., "3 min 13 seg")
    pub takedown_median_uptime: String, // How long threats were active (e.g., "7.7 días")
    pub takedowns_by_type: Vec<NameValuePair>, // Breakdown by threat type

    // Evidence Examples
    pub poc_examples: Vec<PocEvidence>,

    // NEW: Takedown Examples (from getTakedownExamples)
    pub takedown_examples: Vec<TakedownExample>,

    // NEW: Resolved Takedowns (from getResolvedTakedowns)
    pub resolved_takedowns: Vec<ResolvedTakedown>,

    // NEW: Latest Incidents (from getLatestIncidents)
    pub latest_incidents: Vec<IncidentExample>,

    // NEW: Deep Analytics (computed insights for smart slides)
    pub deep_analytics: DeepAnalyticsData,

    // NEW: ROI Metrics for COO Executive Slide
    pub roi_metrics: RoiMetrics,

    // UI FLAGS
    pub is_dynamic_window: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct ThreatTypeCount {
    pub threat_type: String,
    pub count: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct IncidentTypeCount {
    pub incident_type: String,
    pub detections: u64,
    pub incidents: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct NameValuePair {
    pub name: String,
    pub value: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct PocEvidence {
    pub evidence_type: String,
    pub ticket_key: String,
    pub reference_url: String,
    pub status: String,
    pub ip: Option<String>,
    pub isp: Option<String>,
    pub domain: Option<String>,
    pub screenshot_url: Option<String>,
    pub reported_date: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TakedownExample {
    pub name: String,
    pub ticket_type: String,
    pub status: String,
    pub request_date: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ResolvedTakedown {
    pub ticket_key: String,
    pub name: String,
    pub ticket_type: String,
    pub status: String,
    pub host: String,
    pub ip: String,
    pub country: String,
    pub request_date: Option<String>,
    pub resolution_date: Option<String>,
    pub url: String,
    pub screenshot_url: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct IncidentExample {
    pub ticket_key: String,
    pub name: String,
    pub ticket_type: String,
    pub status: String,
    pub open_date: Option<String>,
    pub incident_date: Option<String>,
    pub host: String,
    pub ip: String,
    pub isp: String,
    pub url: String,
    pub screenshot_url: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CodeLeaksSummary {
    pub total_secrets: u64,
    pub unique_repos: u64,
    pub high_severity_secrets: u64,
    pub exposure_platforms: Vec<NameValuePair>,
    pub secret_types: Vec<NameValuePair>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CredentialLeaksSummary {
    pub total_credentials: u64,
    pub unique_emails: u64,
    pub sources: Vec<NameValuePair>,
    pub plaintext_passwords: u64,
    pub stealer_logs_count: u64,
}

/// Deep Analytics computed insights - only populated sections are rendered
#[derive(Debug, Serialize, Clone, Default)]
pub struct DeepAnalyticsData {
    // Code Leak Insights
    pub top_repositories: Vec<NameValuePair>, // Top 5 repos with most leaks
    pub secret_types_breakdown: Vec<NameValuePair>, // Types of secrets found

    // Credential Insights
    pub leak_source_breakdown: Vec<NameValuePair>, // STEALER LOG vs COMBOLIST
    pub top_affected_domains: Vec<NameValuePair>,  // gmail.com, hotmail.com, etc.

    // Takedown Efficiency
    pub avg_takedown_time_hours: Option<f64>, // Average time to close takedown
    pub takedowns_by_platform: Vec<NameValuePair>, // Facebook, Instagram, TikTok

    // Flags for conditional rendering
    pub has_code_leak_insights: bool,
    pub has_credential_insights: bool,
    pub has_takedown_insights: bool,
}

impl DeepAnalyticsData {
    pub fn has_any_data(&self) -> bool {
        self.has_code_leak_insights || self.has_credential_insights || self.has_takedown_insights
    }
}

// ========================
// OPERATIONAL IMPACT METRICS FOR COO SLIDE
// ========================

/// Time multipliers for operational calculations
const MINUTES_PER_TICKET_VALIDATION: f64 = 5.0; // Time to manually validate one ticket
const MINUTES_PER_CREDENTIAL_CHECK: f64 = 2.0; // Time to check one credential exposure
const MINUTES_PER_SECRET_REMEDIATION: f64 = 30.0; // Time to remediate one code secret
const MINUTES_PER_TAKEDOWN_MANUAL: f64 = 120.0; // Time to manually process a takedown
const WORKING_HOURS_PER_DAY: f64 = 8.0;
const WORKING_DAYS_PER_MONTH: f64 = 22.0;

/// Operational Impact Metrics for COO Executive Slide
/// Focused on TIME and PEOPLE instead of money (better for LATAM)
#[derive(Debug, Serialize, Clone, Default)]
pub struct OperationalMetrics {
    // ===== TIME SAVINGS =====
    /// Total hours saved in security operations
    pub hours_saved_total: f64,
    /// Hours saved in ticket validation
    pub hours_saved_validation: f64,
    /// Hours saved in credential monitoring
    pub hours_saved_credentials: f64,
    /// Hours saved in takedown processing
    pub hours_saved_takedowns: f64,
    /// Hours that would be needed for manual secret remediation
    pub hours_saved_secrets: f64,

    // ===== PEOPLE EQUIVALENT =====
    /// FTE (Full Time Equivalent) analysts saved per month
    pub analysts_equivalent_monthly: f64,
    /// Person-days saved in the period
    pub person_days_saved: f64,

    // ===== RESPONSE PERFORMANCE =====
    /// Average response time in minutes
    pub avg_response_minutes: f64,
    /// Median time to first notification (formatted string)
    pub median_response_time: String,
    /// Median threat uptime (formatted string)
    pub median_threat_exposure: String,
    /// Takedown success rate percentage
    pub takedown_success_rate: f64,

    // ===== VOLUME METRICS =====
    /// Total tickets processed automatically
    pub tickets_processed: u64,
    /// Credentials monitored
    pub credentials_monitored: u64,
    /// Secrets detected in code
    pub secrets_detected: u64,
    /// Takedowns completed
    pub takedowns_completed: u64,
    /// Threats detected
    pub threats_detected: u64,

    // ===== EFFICIENCY RATIOS =====
    /// Automation rate (percentage of automated vs manual work)
    pub automation_rate_percent: f64,

    // ===== DISPLAY FLAGS =====
    /// Should this slide be shown? (based on data significance)
    pub has_significant_data: bool,
}

// Keep the old name as alias for compatibility
pub type RoiMetrics = OperationalMetrics;

impl OperationalMetrics {
    /// Calculate operational metrics from report data
    pub fn compute(data: &PocReportData) -> Self {
        // ===== TIME CALCULATIONS =====

        // Ticket validation time saved
        let hours_saved_validation =
            (data.total_tickets as f64 * MINUTES_PER_TICKET_VALIDATION) / 60.0;

        // Credential monitoring time saved
        let hours_saved_credentials =
            (data.credentials_total as f64 * MINUTES_PER_CREDENTIAL_CHECK) / 60.0;

        // Secret remediation time (estimated if done manually)
        let hours_saved_secrets =
            (data.secrets_total as f64 * MINUTES_PER_SECRET_REMEDIATION) / 60.0;

        // Takedown processing time saved
        let hours_saved_takedowns =
            (data.takedown_resolved as f64 * MINUTES_PER_TAKEDOWN_MANUAL) / 60.0;

        // Total hours saved
        let hours_saved_total = hours_saved_validation
            + hours_saved_credentials
            + hours_saved_secrets
            + hours_saved_takedowns;

        // ===== PEOPLE EQUIVALENT =====

        // Person-days saved
        let person_days_saved = hours_saved_total / WORKING_HOURS_PER_DAY;

        // FTE analysts equivalent (monthly)
        let hours_per_analyst_month = WORKING_HOURS_PER_DAY * WORKING_DAYS_PER_MONTH; // 176 hours
        let analysts_equivalent_monthly = hours_saved_total / hours_per_analyst_month;

        // ===== RESPONSE TIME =====
        let avg_response_minutes = parse_duration_to_minutes(&data.takedown_median_time_to_notify);

        // ===== EFFICIENCY =====
        // Automation rate: percentage of threats that were auto-processed
        let automation_rate_percent = if data.total_threats > 0 {
            (data.takedown_resolved as f64 / data.total_threats as f64 * 100.0).min(100.0)
        } else {
            0.0
        };

        // ===== SIGNIFICANCE THRESHOLD =====
        // Show slide if we have meaningful operational impact
        let has_significant_data = hours_saved_total >= 10.0 || // At least 10 hours saved
                                   data.credentials_total >= 100 ||
                                   data.takedown_resolved >= 3 ||
                                   data.secrets_total >= 10;

        OperationalMetrics {
            hours_saved_total,
            hours_saved_validation,
            hours_saved_credentials,
            hours_saved_takedowns,
            hours_saved_secrets,
            analysts_equivalent_monthly,
            person_days_saved,
            avg_response_minutes,
            median_response_time: data.takedown_median_time_to_notify.clone(),
            median_threat_exposure: data.takedown_median_uptime.clone(),
            takedown_success_rate: data.takedown_success_rate,
            tickets_processed: data.total_tickets,
            credentials_monitored: data.credentials_total,
            secrets_detected: data.secrets_total,
            takedowns_completed: data.takedown_resolved,
            threats_detected: data.total_threats,
            automation_rate_percent,
            has_significant_data,
        }
    }

    /// Format hours in a human-readable way
    pub fn format_hours(hours: f64) -> String {
        if hours >= 24.0 {
            let days = hours / WORKING_HOURS_PER_DAY;
            if days >= 1.0 {
                format!("{:.1} días", days)
            } else {
                format!("{:.0} horas", hours)
            }
        } else if hours >= 1.0 {
            format!("{:.1} horas", hours)
        } else {
            format!("{:.0} minutos", hours * 60.0)
        }
    }

    /// Format FTE analysts
    pub fn format_analysts(fte: f64) -> String {
        if fte >= 1.0 {
            format!("{:.1} analistas/mes", fte)
        } else if fte >= 0.5 {
            format!("{:.0}% de 1 analista", fte * 100.0)
        } else {
            format!(
                "{:.0} horas/mes",
                fte * WORKING_HOURS_PER_DAY * WORKING_DAYS_PER_MONTH
            )
        }
    }
}

/// Parse duration string like "3m 13s" to total minutes
fn parse_duration_to_minutes(duration: &str) -> f64 {
    let mut total_minutes = 0.0;

    // Parse hours (e.g., "1h ")
    if let Some(h_pos) = duration.find('h') {
        if let Ok(hours) = duration[..h_pos].trim().parse::<f64>() {
            total_minutes += hours * 60.0;
        }
    }

    // Parse minutes (e.g., "3m ")
    if let Some(m_pos) = duration.find('m') {
        let start = duration.find('h').map(|p| p + 2).unwrap_or(0);
        if let Ok(mins) = duration[start..m_pos].trim().parse::<f64>() {
            total_minutes += mins;
        }
    }

    // Parse seconds (e.g., "13s")
    if let Some(s_pos) = duration.find('s') {
        let start = duration.find('m').map(|p| p + 2).unwrap_or(0);
        if let Ok(secs) = duration[start..s_pos].trim().parse::<f64>() {
            total_minutes += secs / 60.0;
        }
    }

    if total_minutes == 0.0 && duration != "N/A" {
        // Fallback: try to parse as raw number
        total_minutes = duration.parse().unwrap_or(0.0);
    }

    total_minutes
}

// ========================
// API RESPONSE STRUCTURES
// ========================

#[derive(Deserialize, Debug)]
struct CustomerApiResponse {
    #[serde(default)]
    customers: Vec<CustomerItem>,
}

#[derive(Deserialize, Debug)]
struct CustomerItem {
    name: Option<String>,
    key: Option<String>,
    #[serde(default)]
    assets: Vec<AssetItem>,
    #[serde(default)]
    properties: Vec<PropertyItem>,
}

#[derive(Deserialize, Debug)]
struct AssetItem {
    name: Option<String>,
    key: Option<String>,
    category: Option<String>,
    #[serde(default)]
    active: bool,
}

#[derive(Deserialize, Debug)]
struct PropertyItem {
    key: Option<String>,
    value: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct StatsCountResponse {
    total: Option<u64>,
}

#[derive(Deserialize)]
struct TicketsResponse {
    #[serde(default)]
    tickets: Vec<TicketItem>,
    pageable: Option<Pageable>,
}

#[derive(Deserialize)]
struct Pageable {
    total: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
struct TicketItem {
    ticket: Option<TicketInfo>,
    current: Option<CurrentInfo>,
    detection: Option<CurrentInfo>, // detection field (sometimes used instead of current)
    #[serde(default)]
    snapshots: Vec<SnapshotItem>,
    #[serde(default)]
    attachments: Vec<AttachmentItem>, // Top-level attachments (contains screenshots)
}

#[derive(Deserialize, Debug, Clone)]
struct TicketInfo {
    #[serde(rename = "ticketKey")]
    ticket_key: Option<String>,
    reference: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct CurrentInfo {
    #[serde(rename = "type")]
    type_: Option<String>,
    status: Option<String>,
    ip: Option<String>,
    isp: Option<String>,
    domain: Option<String>,
    host: Option<String>,
    // Date fields
    #[serde(rename = "open")]
    open: Option<DateInfo>,
    #[serde(rename = "incident")]
    incident: Option<DateInfo>,
    #[serde(rename = "close")]
    close: Option<DateInfo>,
    // Takedown fields
    #[serde(rename = "takedown")]
    takedown: Option<TakedownInfo>,
}

#[derive(Deserialize, Debug, Clone)]
struct DateInfo {
    date: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct TakedownInfo {
    request: Option<DateInfo>,
    done: Option<DateInfo>,
}

impl CurrentInfo {
    fn open_date(&self) -> Option<String> {
        self.open.as_ref().and_then(|o| o.date.clone())
    }
    fn incident_date(&self) -> Option<String> {
        self.incident.as_ref().and_then(|i| i.date.clone())
    }
    fn close_date(&self) -> Option<String> {
        self.close.as_ref().and_then(|c| c.date.clone())
    }
    fn takedown_request_date(&self) -> Option<String> {
        self.takedown
            .as_ref()
            .and_then(|t| t.request.as_ref())
            .and_then(|r| r.date.clone())
    }
}

#[derive(Deserialize, Debug, Clone)]
struct SnapshotItem {
    #[serde(default)]
    attachments: Vec<AttachmentItem>,
}

#[derive(Deserialize, Debug, Clone)]
struct AttachmentItem {
    name: Option<String>,
    url: Option<String>,
    #[serde(rename = "type")]
    type_: Option<String>,
}

#[derive(Deserialize)]
struct CredentialsTotalResponse {
    total: Option<u64>,
}

#[derive(Deserialize)]
struct CredentialsListResponse {
    #[serde(default)]
    detections: Vec<CredentialDetection>,
    pageable: Option<Pageable>,
}

#[derive(Deserialize, Clone)]
struct CredentialDetection {
    credential: Option<CredentialInfo>,
}

#[derive(Deserialize, Clone)]
struct CredentialInfo {
    #[serde(rename = "source.name")]
    source_name: Option<String>,
    #[serde(rename = "access.url")]
    access_url: Option<String>,
}

// ========================
// MAIN FETCH FUNCTION
// ========================

/// Fetch complete PoC report data with parallel API calls
pub async fn fetch_full_report(
    token: &str,
    tenant_id: &str,
    from: &str,
    to: &str,
) -> Result<PocReportData> {
    let client = create_client()?;
    let auth = format!("Bearer {}", token);

    // Phase 1: Fetch customer info and stats in parallel
    let (
        customer_data,
        total_open,
        total_incident,
        total_closed,
        threats_by_type,
        credentials_total,
        code_leaks_total,
        takedown_stats,
    ) = tokio::join!(
        fetch_customer_data(&client, &auth, tenant_id),
        fetch_ticket_count(&client, &auth, from, to, "open", tenant_id),
        fetch_ticket_count(&client, &auth, from, to, "incident", tenant_id),
        fetch_ticket_count(&client, &auth, from, to, "closed", tenant_id),
        fetch_threats_by_type_map(&client, &auth, from, to, tenant_id),
        fetch_credentials_total_all(&client, &auth, from, to, tenant_id),
        fetch_code_leaks_count(&client, &auth, from, to, tenant_id),
        fetch_takedown_stats_full(&client, &auth, from, to, tenant_id),
    );

    let customer = customer_data.unwrap_or_default();
    let threats_map = threats_by_type.unwrap_or_default();
    let tk = takedown_stats.unwrap_or(TakedownStats {
        resolved: 0,
        pending: 0,
        aborted: 0,
        success_rate: 0.0,
        median_time_to_notify: "N/A".to_string(),
        median_uptime: "N/A".to_string(),
        unresolved: 0,
    });

    // Convert threats map to sorted vec
    let mut threats_vec: Vec<ThreatTypeCount> = threats_map
        .into_iter()
        .map(|(k, v)| ThreatTypeCount {
            threat_type: k,
            count: v,
        })
        .collect();
    threats_vec.sort_by(|a, b| b.count.cmp(&a.count));

    // SMART: Only these 6 types generate visual evidence slides (per frontend logic)
    let allowed_types = [
        "phishing",
        "fraudulent-brand-use",
        "paid-search",
        "fake-social-media-profile",
        "similar-domain-name",
        "fake-mobile-app",
    ];

    // Extract top 3 from ALLOWED types only (sorted by count)
    let top_types: Vec<String> = threats_vec
        .iter()
        .filter(|t| t.count > 0 && allowed_types.contains(&t.threat_type.as_str()))
        .take(3)
        .map(|t| t.threat_type.clone())
        .collect();

    // Phase 2: Fetch evidence samples using SMART selection (only top 3 types)
    // and other advanced data
    let (
        smart_evidence,
        code_leaks_summary,
        takedown_examples,
        resolved_takedowns,
        latest_incidents,
        credential_leaks_summary,
    ) = tokio::join!(
        fetch_smart_evidence(&client, &auth, from, to, &top_types, 3, tenant_id),
        fetch_code_leaks_summary(&client, &auth, from, to, tenant_id),
        fetch_takedown_examples(&client, &auth, from, to, 10, tenant_id),
        fetch_resolved_takedowns(&client, &auth, from, to, 10, tenant_id),
        fetch_latest_incidents(&client, &auth, from, to, 5, tenant_id),
        fetch_credential_leaks_summary(&client, &auth, from, to, tenant_id), // NEW
    );

    let evidence = smart_evidence.unwrap_or_default();
    let code_summary = code_leaks_summary.unwrap_or(CodeLeaksSummary {
        total_secrets: 0,
        unique_repos: 0,
        high_severity_secrets: 0,
        exposure_platforms: vec![],
        secret_types: vec![],
    });
    let takedown_ex = takedown_examples.unwrap_or_default();
    let resolved_td = resolved_takedowns.unwrap_or_default();
    let incidents = latest_incidents.unwrap_or_default();
    let cred_summary = credential_leaks_summary.unwrap_or(CredentialLeaksSummary {
        total_credentials: 0,
        unique_emails: 0,
        sources: vec![],
        plaintext_passwords: 0,
        stealer_logs_count: 0,
    });

    // Calculate metrics
    let open_count = total_open.unwrap_or(0);
    let closed_count = total_closed.unwrap_or(0);
    let creds_count = credentials_total.unwrap_or(0);
    let leaks_count = code_leaks_total.unwrap_or(0);

    // FIXED: total_threats should be sum of all incidents by type (from correct endpoint)
    // The fetch_ticket_count with status=incident uses wrong endpoint, threats_vec uses correct one
    let total_threats: u64 = threats_vec.iter().map(|t| t.count).sum();
    let total_tickets = open_count + total_threats + closed_count;
    let validation_hours = (total_tickets as f64 * 5.0) / 60.0; // 5 min per ticket

    // Build incidents by type from threats
    let incidents_by_type: Vec<IncidentTypeCount> = threats_vec
        .iter()
        .take(10)
        .map(|t| IncidentTypeCount {
            incident_type: t.threat_type.clone(),
            detections: t.count,
            incidents: 0, // Would need separate query
        })
        .collect();

    // Compute Deep Analytics BEFORE building the struct (to avoid borrow issues)
    let deep_analytics = compute_deep_analytics(&code_summary, &cred_summary, &resolved_td);

    let report = PocReportData {
        company_name: customer.name,
        partner_name: customer.partner,
        tlp_level: "AMBER".to_string(),
        start_date: from.to_string(),
        end_date: to.to_string(),
        brands_count: customer.brands_count,
        brands: customer.brands,
        executives_count: customer.executives_count,
        ips_count: customer.ips_count,
        bins_count: customer.bins_count,
        domains_count: customer.domains_count,
        threat_hunting_credits: 0,
        threat_intelligence_assets: 0,
        total_tickets,
        total_threats,
        validation_hours,
        threats_by_type: threats_vec,
        credentials_total: creds_count,
        unique_hosts: 0,
        high_risk_users: 0,
        malware_breakdown: vec![],
        top_services: vec![],
        // Code Leaks Summary from advanced function
        secrets_total: if code_summary.total_secrets > 0 {
            code_summary.total_secrets
        } else {
            leaks_count
        },
        unique_repos: code_summary.unique_repos,
        production_secrets: code_summary.high_severity_secrets,
        platform_breakdown: code_summary.exposure_platforms,
        secret_types: code_summary.secret_types,
        credential_leaks_summary: cred_summary,
        incidents_by_type,
        takedown_resolved: tk.resolved,
        takedown_pending: tk.pending,
        takedown_aborted: tk.aborted,
        takedown_success_rate: tk.success_rate,
        takedown_median_time_to_notify: tk.median_time_to_notify,
        takedown_median_uptime: tk.median_uptime,
        takedown_unresolved: tk.unresolved,
        takedowns_by_type: vec![], // Not available in main stats endpoint yet
        poc_examples: evidence,
        // NEW: Advanced data
        takedown_examples: takedown_ex,
        resolved_takedowns: resolved_td.clone(),
        latest_incidents: incidents,
        // Computed Deep Analytics
        deep_analytics,
        // ROI placeholder (will be computed after struct creation)
        roi_metrics: RoiMetrics::default(),

        is_dynamic_window: false, // Default to fixed window
    };

    // Compute ROI metrics now that we have the full data
    let roi = RoiMetrics::compute(&report);
    let mut report_with_roi = report;
    report_with_roi.roi_metrics = roi;

    Ok(report_with_roi)
}

/// Compute Deep Analytics insights from raw API data
/// Only populates sections that have meaningful data (thresholds apply)
fn compute_deep_analytics(
    code_summary: &CodeLeaksSummary,
    cred_summary: &CredentialLeaksSummary,
    takedowns: &[ResolvedTakedown],
) -> DeepAnalyticsData {
    let mut analytics = DeepAnalyticsData::default();

    // ===== Code Leak Insights =====
    // Threshold: at least 3 unique repos for pattern analysis
    if code_summary.unique_repos >= 3 {
        analytics.has_code_leak_insights = true;
        analytics.secret_types_breakdown = code_summary.secret_types.clone();
        // Note: top_repositories would need separate API call for detailed breakdown
    }

    // ===== Credential Insights =====
    // Threshold: at least 100 credentials for statistical significance
    if cred_summary.total_credentials >= 100 {
        analytics.has_credential_insights = true;
        analytics.leak_source_breakdown = cred_summary.sources.clone();
        // top_affected_domains would need parsing from credential data
    }

    // ===== Takedown Efficiency Insights =====
    // Threshold: at least 3 resolved takedowns for meaningful stats
    if takedowns.len() >= 3 {
        analytics.has_takedown_insights = true;

        // Calculate average takedown time from resolved_takedowns
        let mut total_hours: f64 = 0.0;
        let mut count_with_dates: u64 = 0;

        // Count takedowns by platform (Facebook, Instagram, TikTok, etc.)
        let mut platform_counts: HashMap<String, u64> = HashMap::new();

        for td in takedowns {
            // Calculate time difference if both dates are present
            if let (Some(req), Some(res)) = (&td.request_date, &td.resolution_date) {
                // Parse dates (format: "2025-12-01T12:00:00")
                if let (Ok(req_dt), Ok(res_dt)) = (
                    chrono::NaiveDateTime::parse_from_str(req, "%Y-%m-%dT%H:%M:%S"),
                    chrono::NaiveDateTime::parse_from_str(res, "%Y-%m-%dT%H:%M:%S"),
                ) {
                    let hours = (res_dt - req_dt).num_hours() as f64;
                    if hours > 0.0 {
                        total_hours += hours;
                        count_with_dates += 1;
                    }
                }
            }

            // Extract platform from host (e.g., "www.facebook.com" -> "Facebook")
            let platform = match td.host.as_str() {
                h if h.contains("facebook") => "Facebook",
                h if h.contains("instagram") => "Instagram",
                h if h.contains("tiktok") => "TikTok",
                h if h.contains("twitter") || h.contains("x.com") => "Twitter/X",
                h if h.contains("linkedin") => "LinkedIn",
                h if h.contains("youtube") => "YouTube",
                _ => "Other",
            };
            *platform_counts.entry(platform.to_string()).or_insert(0) += 1;
        }

        // Calculate average takedown time
        if count_with_dates > 0 {
            analytics.avg_takedown_time_hours = Some(total_hours / count_with_dates as f64);
        }

        // Convert to sorted Vec<NameValuePair>
        let mut platforms: Vec<NameValuePair> = platform_counts
            .into_iter()
            .map(|(name, value)| NameValuePair { name, value })
            .collect();
        platforms.sort_by(|a, b| b.value.cmp(&a.value));
        analytics.takedowns_by_platform = platforms;
    }

    analytics
}

// ========================
// HELPER STRUCTURES
// ========================

#[derive(Default)]
struct CustomerData {
    name: String,
    partner: Option<String>,
    brands_count: u32,
    brands: Vec<String>,
    executives_count: u32,
    ips_count: u32,
    bins_count: u32,
    domains_count: u32,
}

// ========================
// API FETCH FUNCTIONS
// ========================

async fn fetch_customer_data(
    client: &reqwest::Client,
    auth: &str,
    tenant_id: &str,
) -> Result<CustomerData> {
    let url = format!("{}/customers/customers", API_URL);
    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    if !success {
        log_api_call("fetch_customer_data", &url, status, false, "Request failed");
        return Ok(CustomerData::default());
    }

    // Try to parse as array first (documented format)
    let text = resp.text().await?;
    log_api_call(
        "fetch_customer_data",
        &url,
        status,
        true,
        &format!("{} bytes received", text.len()),
    );

    let customers: Vec<CustomerItem> = serde_json::from_str(&text).unwrap_or_default();

    // Log the number of customers found and list some keys for debugging
    let customer_keys: Vec<String> = customers
        .iter()
        .filter_map(|c| c.key.clone())
        .take(20)
        .collect();
    log_api_call(
        "customer_search",
        &format!("Looking for: {}", tenant_id),
        0,
        true,
        &format!(
            "Found {} customers. First 20 keys: {:?}",
            customers.len(),
            customer_keys
        ),
    );

    // Find the customer matching tenant_id
    let customer = customers
        .into_iter()
        .find(|c| c.key.as_deref() == Some(tenant_id) || c.name.as_deref() == Some(tenant_id));

    if customer.is_none() {
        log_api_call(
            "customer_search",
            tenant_id,
            404,
            false,
            "Customer NOT FOUND in list!",
        );
    }

    if let Some(customer) = customer {
        let mut data = CustomerData {
            name: customer.name.unwrap_or_default(),
            partner: None,
            brands_count: 0,
            brands: vec![],
            executives_count: 0,
            ips_count: 0,
            bins_count: 0,
            domains_count: 0,
        };

        // Count assets by category
        for asset in &customer.assets {
            if !asset.active {
                continue;
            }
            match asset.category.as_deref() {
                Some("BRAND") => {
                    data.brands_count += 1;
                    if let Some(name) = &asset.name {
                        data.brands.push(name.clone());
                    }
                }
                Some("EXECUTIVE") | Some("VIP") => data.executives_count += 1,
                Some("IP") | Some("IP_RANGE") => data.ips_count += 1,
                Some("BIN") => data.bins_count += 1,
                Some("DOMAIN") => data.domains_count += 1,
                _ => {}
            }
        }

        // Look for partner in properties
        for prop in &customer.properties {
            if prop.key.as_deref() == Some("partner") {
                if let Some(val) = &prop.value {
                    data.partner = val.as_str().map(|s| s.to_string());
                }
            }
        }

        return Ok(data);
    }

    Ok(CustomerData::default())
}

async fn fetch_ticket_count(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    status: &str,
    customer: &str,
) -> Result<u64> {
    let url = format!(
        "{}/tickets-api/stats/customer?from={}&to={}&status={}&customer={}",
        API_URL, from, to, status, customer
    );
    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    if !success {
        log_api_call(
            &format!("fetch_ticket_count_{}", status),
            &url,
            status,
            false,
            "Request failed",
        );
        return Ok(0);
    }

    let text = resp.text().await?;
    log_api_call(
        &format!("fetch_ticket_count_{}", status),
        &url,
        status,
        true,
        &text,
    );

    let data: StatsCountResponse =
        serde_json::from_str(&text).unwrap_or(StatsCountResponse { total: None });
    Ok(data.total.unwrap_or(0))
}

async fn fetch_threats_by_type_map(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    customer: &str,
) -> Result<HashMap<String, u64>> {
    // Use stats endpoint for incident count by type
    // Note: This works for most tenants but CABALP seems to have a portal vs API discrepancy
    let url = format!(
        "{}/tickets-api/stats/incident/count/ticket-types?from={}&to={}&customer={}",
        API_URL, from, to, customer
    );
    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    if !success {
        log_api_call(
            "fetch_threats_by_type_map",
            &url,
            status,
            false,
            "Request failed",
        );
        return Ok(HashMap::new());
    }

    let text = resp.text().await?;
    log_api_call("fetch_threats_by_type_map", &url, status, true, &text);

    // Parse the stats response format: {"totalByTicketType":[{"type":"x","totalOnPeriod":N}]}
    #[derive(Deserialize)]
    struct ThreatTypeItem {
        #[serde(rename = "type")]
        threat_type: String,
        #[serde(rename = "totalOnPeriod")]
        total_on_period: u64,
    }
    #[derive(Deserialize)]
    struct ThreatsResponse {
        #[serde(rename = "totalByTicketType")]
        total_by_ticket_type: Option<Vec<ThreatTypeItem>>,
    }

    let parsed: ThreatsResponse = serde_json::from_str(&text).unwrap_or(ThreatsResponse {
        total_by_ticket_type: None,
    });
    let mut data = HashMap::new();
    if let Some(items) = parsed.total_by_ticket_type {
        for item in items {
            if item.total_on_period > 0 {
                data.insert(item.threat_type, item.total_on_period);
            }
        }
    }

    Ok(data)
}

async fn fetch_credentials_total_all(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    customer: &str,
) -> Result<u64> {
    // WORKAROUND: /exposure-api/credentials/total does NOT support the 'customer' param
    // Instead, use /exposure-api/credentials (search) with pageSize=1 and get total from pageable.total
    // Reference: Axur API doc - MSSP users need to use the search endpoint for customer filtering
    let url = format!(
        "{}/exposure-api/credentials?created=ge:{}T00:00:00&created=le:{}T23:59:59&customer={}&pageSize=1&timezone=-03:00",
        API_URL, from, to, customer
    );
    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    let text = resp.text().await?;
    log_api_call("fetch_credentials_total", &url, status, success, &text);

    if !success {
        return Ok(0);
    }

    // Response format: {"detections": [...], "pageable": {"total": N, ...}}
    #[derive(Deserialize)]
    struct Pageable {
        total: Option<u64>,
    }
    #[derive(Deserialize)]
    struct CredentialsSearchResponse {
        pageable: Option<Pageable>,
    }

    let data: CredentialsSearchResponse =
        serde_json::from_str(&text).unwrap_or(CredentialsSearchResponse { pageable: None });
    Ok(data.pageable.and_then(|p| p.total).unwrap_or(0))
}

async fn fetch_code_leaks_count(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    customer: &str,
) -> Result<u64> {
    let url = format!(
        "{}/tickets-api/tickets?type=code-secret-leak&open.date=ge:{}&open.date=le:{}&page=1&pageSize=1&ticket.customer={}",
        API_URL, from, to, customer
    );
    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    if !success {
        log_api_call(
            "fetch_code_leaks_count",
            &url,
            status,
            false,
            "Request failed",
        );
        return Ok(0);
    }

    let text = resp.text().await?;
    log_api_call("fetch_code_leaks_count", &url, status, true, &text);

    let data: TicketsResponse = serde_json::from_str(&text).unwrap_or(TicketsResponse {
        tickets: vec![],
        pageable: None,
    });
    Ok(data.pageable.and_then(|p| p.total).unwrap_or(0))
}

#[derive(Debug, Clone, Serialize)]
pub struct TakedownStats {
    pub resolved: u64,
    pub pending: u64,
    pub aborted: u64,
    pub success_rate: f64,
    pub median_time_to_notify: String,
    pub median_uptime: String,
    pub unresolved: u64, // NEW: Added unresolved
}

async fn fetch_takedown_stats_full(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    customer: &str,
) -> Result<TakedownStats> {
    let url = format!(
        "{}/tickets-api/stats/takedown?from={}&to={}&customer={}",
        API_URL, from, to, customer
    );
    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    if !success {
        log_api_call(
            "fetch_takedown_stats",
            &url,
            status,
            false,
            "Request failed",
        );
        return Ok(TakedownStats {
            resolved: 0,
            pending: 0,
            aborted: 0,
            success_rate: 0.0,
            median_time_to_notify: "N/A".to_string(),
            median_uptime: "N/A".to_string(),
            unresolved: 0,
        });
    }

    #[derive(Deserialize)]
    struct TakedownResp {
        total: Option<TakedownTotals>,
    }
    #[derive(Deserialize)]
    struct TakedownTotals {
        resolved: Option<u64>,
        pending: Option<u64>,
        aborted: Option<u64>,
        #[serde(rename = "successRate")]
        success_rate: Option<f64>,
        #[serde(rename = "medianTimeToFirstNotification")]
        median_time_to_first_notification: Option<String>,
        #[serde(rename = "medianUptime")]
        median_uptime: Option<String>,
        unresolved: Option<u64>,
    }

    let text = resp.text().await?;
    log_api_call("fetch_takedown_stats", &url, status, true, &text);

    let data: TakedownResp = serde_json::from_str(&text).unwrap_or(TakedownResp { total: None });

    let t = data.total.unwrap_or(TakedownTotals {
        resolved: None,
        pending: None,
        aborted: None,
        success_rate: None,
        median_time_to_first_notification: None,
        median_uptime: None,
        unresolved: None,
    });

    // Format duration strings (PT3M13S -> 3m 13s)
    let format_duration = |s: Option<String>| -> String {
        match s {
            Some(d) => d
                .replace("PT", "")
                .replace("H", "h ")
                .replace("M", "m ")
                .replace("S", "s")
                .to_lowercase(),
            None => "N/A".to_string(),
        }
    };

    Ok(TakedownStats {
        resolved: t.resolved.unwrap_or(0),
        pending: t.pending.unwrap_or(0),
        aborted: t.aborted.unwrap_or(0),
        success_rate: t.success_rate.unwrap_or(0.0), // It's already numeric in JSON (99.81...)
        median_time_to_notify: format_duration(t.median_time_to_first_notification),
        median_uptime: format_duration(t.median_uptime),
        unresolved: t.unresolved.unwrap_or(0),
    })
}

async fn fetch_evidence_samples(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    per_type: usize,
    customer: &str,
) -> Result<Vec<PocEvidence>> {
    // Main threat types for evidence samples (most relevant for PoC reports)
    let threat_types = [
        "phishing",
        "fake-social-media-profile",
        "fraudulent-brand-use",
        "fake-mobile-app",
        "similar-domain-name",
        "paid-search",
        "code-secret-leak",
        "corporate-credential-leak",
        "infostealer-credential",
        "malware",
        "ransomware-attack",
        "dw-activity",
    ];

    let mut all_evidence = Vec::new();

    for threat_type in &threat_types {
        let url = format!(
            "{}/tickets-api/tickets?type={}&status=incident&open.date=ge:{}&open.date=le:{}&page=1&pageSize={}&include=fields,attachments&ticket.customer={}",
            API_URL, threat_type, from, to, per_type, customer
        );

        let resp = client.get(&url).header("Authorization", auth).send().await;

        if let Ok(response) = resp {
            if response.status().is_success() {
                if let Ok(data) = response.json::<TicketsResponse>().await {
                    for ticket in data.tickets.into_iter().take(per_type) {
                        let ticket_info = ticket.ticket.as_ref();
                        let current = ticket.current.as_ref();

                        let screenshot = ticket
                            .snapshots
                            .iter()
                            .flat_map(|s| s.attachments.iter())
                            .find(|a| {
                                a.type_.as_deref() == Some("screenshot")
                                    || a.type_.as_deref() == Some("image")
                            })
                            .and_then(|a| a.url.clone());

                        all_evidence.push(PocEvidence {
                            evidence_type: threat_type.to_string(),
                            ticket_key: ticket_info
                                .and_then(|t| t.ticket_key.clone())
                                .unwrap_or_default(),
                            reference_url: ticket_info
                                .and_then(|t| t.reference.clone())
                                .unwrap_or_default(),
                            status: current.and_then(|c| c.status.clone()).unwrap_or_default(),
                            ip: current.and_then(|c| c.ip.clone()),
                            isp: current.and_then(|c| c.isp.clone()),
                            domain: current
                                .and_then(|c| c.domain.clone())
                                .or_else(|| current.and_then(|c| c.host.clone())),
                            screenshot_url: screenshot,
                            reported_date: None,
                        });
                    }
                }
            }
        }
    }

    Ok(all_evidence)
}

/// SMART: Fetch evidence only for top N threat types (based on incident counts)
/// This is the intelligent approach - only query types that actually have incidents
async fn fetch_smart_evidence(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    top_types: &[String], // Pre-sorted by count, e.g. ["phishing", "fraudulent-brand-use"]
    examples_per_type: usize,
    customer: &str,
) -> Result<Vec<PocEvidence>> {
    let mut all_evidence = Vec::new();

    for threat_type in top_types.iter().take(3) {
        // Max 3 types for focused slides
        // Per documentation (edpoints_reporte.md line 65):
        // Use type= (not current.type=) with open.date range for ticket examples
        // Filter by status to show active threats (open, quarantine, incident)
        let url = format!(
            "{}/tickets-api/tickets?type={}&status=open,quarantine,incident&pageSize={}&sortBy=open.date&order=desc&include=fields,attachments&open.date=ge:{}&open.date=le:{}&ticket.customer={}",
            API_URL, threat_type, examples_per_type, from, to, customer
        );

        log_api_call(
            &format!("fetch_smart_evidence_{}", threat_type),
            &url,
            0,
            true,
            "Starting request",
        );

        let resp = client.get(&url).header("Authorization", auth).send().await;

        if let Ok(response) = resp {
            let status = response.status().as_u16();
            if response.status().is_success() {
                if let Ok(text) = response.text().await {
                    log_api_call(
                        &format!("fetch_smart_evidence_{}", threat_type),
                        &url,
                        status,
                        true,
                        &text,
                    );

                    if let Ok(data) = serde_json::from_str::<TicketsResponse>(&text) {
                        for ticket in data.tickets.into_iter().take(examples_per_type) {
                            let ticket_info = ticket.ticket.as_ref();
                            // Use detection field as fallback for current (per colleague's docs)
                            let current = ticket.current.as_ref().or(ticket.detection.as_ref());

                            // Find screenshot from TOP-LEVEL attachments (per colleague's docs)
                            // Look for files ending in .png, .jpg, .jpeg
                            let screenshot_url = ticket
                                .attachments
                                .iter()
                                .find(|a| {
                                    let url = a.url.as_deref().unwrap_or("");
                                    let name = a.name.as_deref().unwrap_or("");
                                    url.ends_with(".png")
                                        || url.ends_with(".jpg")
                                        || url.ends_with(".jpeg")
                                        || name.ends_with(".png")
                                        || name.ends_with(".jpg")
                                        || name.ends_with(".jpeg")
                                })
                                .and_then(|a| a.url.clone());

                            // Download image with auth and convert to base64
                            let screenshot_base64 = if let Some(ref img_url) = screenshot_url {
                                download_image_as_base64(client, auth, img_url).await
                            } else {
                                None
                            };

                            all_evidence.push(PocEvidence {
                                evidence_type: threat_type.to_string(),
                                ticket_key: ticket_info
                                    .and_then(|t| t.ticket_key.clone())
                                    .unwrap_or_default(),
                                reference_url: ticket_info
                                    .and_then(|t| t.reference.clone())
                                    .unwrap_or_default(),
                                status: current.and_then(|c| c.status.clone()).unwrap_or_default(),
                                ip: current.and_then(|c| c.ip.clone()),
                                isp: current.and_then(|c| c.isp.clone()),
                                domain: current
                                    .and_then(|c| c.domain.clone())
                                    .or_else(|| current.and_then(|c| c.host.clone())),
                                screenshot_url: screenshot_base64, // Now contains base64 data URI
                                reported_date: current.and_then(|c| c.open_date()),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(all_evidence)
}

// ========================
// ADVANCED FETCH FUNCTIONS (from PocService.js)
// ========================

/// Fetch code leaks summary with secret type classification using regex
async fn fetch_code_leaks_summary(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    customer: &str,
) -> Result<CodeLeaksSummary> {
    let url = format!(
        "{}/tickets-api/tickets?type=code-secret-leak&open.date=ge:{}&open.date=le:{}&page=1&pageSize=100&include=fields&order=desc&ticket.customer={}",
        API_URL, from, to, customer
    );

    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    if !success {
        log_api_call(
            "fetch_code_leaks_summary",
            &url,
            status,
            false,
            "Request failed",
        );
        return Ok(CodeLeaksSummary {
            total_secrets: 0,
            unique_repos: 0,
            high_severity_secrets: 0,
            exposure_platforms: vec![],
            secret_types: vec![],
        });
    }

    let text = resp.text().await?;
    log_api_call("fetch_code_leaks_summary", &url, status, true, &text);

    let data: TicketsResponse = serde_json::from_str(&text).unwrap_or(TicketsResponse {
        tickets: vec![],
        pageable: None,
    });
    let total = data.pageable.as_ref().and_then(|p| p.total).unwrap_or(0);

    // Regex patterns for secret classification (based on PocService.js)
    let api_key_re = Regex::new(r"(?i)api.?key|apikey|token").unwrap();
    let db_pass_re = Regex::new(r"(?i)db|database|sql|postgres|mysql|mongodb|password").unwrap();
    let cloud_re = Regex::new(r"(?i)aws|gcp|azure|cloud|access.?key|secret.?access").unwrap();
    let ssh_re = Regex::new(r"(?i)ssh|private.?key|rsa|ecdsa").unwrap();
    let jwt_re = Regex::new(r"(?i)jwt|oauth|bearer").unwrap();

    let mut repo_set: HashSet<String> = HashSet::new();
    let mut high_severity = 0u64;
    let mut platform_counts: HashMap<String, u64> = HashMap::new();
    let mut secret_type_counts: HashMap<String, u64> = HashMap::new();

    for ticket in &data.tickets {
        // Extract platform from URL or reference
        if let Some(ref ticket_info) = ticket.ticket {
            let url_str = ticket_info.reference.as_deref().unwrap_or("");
            let platform = hostname_to_platform(url_str);
            *platform_counts.entry(platform.clone()).or_insert(0) += 1;

            // Try to extract repo from URL
            if let Some(repo) = extract_repo_from_url(url_str) {
                repo_set.insert(repo);
            }
        }

        // Classify secret type based on name/fields
        let name = ticket
            .ticket
            .as_ref()
            .and_then(|t| t.reference.clone())
            .unwrap_or_default();

        let secret_type = if api_key_re.is_match(&name) {
            "API Keys"
        } else if db_pass_re.is_match(&name) {
            "Database Passwords"
        } else if cloud_re.is_match(&name) {
            "Cloud Credentials"
        } else if ssh_re.is_match(&name) {
            "SSH Keys"
        } else if jwt_re.is_match(&name) {
            "Access Tokens"
        } else {
            "Other"
        };
        *secret_type_counts
            .entry(secret_type.to_string())
            .or_insert(0) += 1;

        // Check for high severity (simplified - count all as potential high severity)
        if secret_type != "Other" {
            high_severity += 1;
        }
    }

    // Convert to sorted vectors
    let mut exposure_platforms: Vec<NameValuePair> = platform_counts
        .into_iter()
        .map(|(name, value)| NameValuePair { name, value })
        .collect();
    exposure_platforms.sort_by(|a, b| b.value.cmp(&a.value));

    let mut secret_types: Vec<NameValuePair> = secret_type_counts
        .into_iter()
        .map(|(name, value)| NameValuePair { name, value })
        .collect();
    secret_types.sort_by(|a, b| b.value.cmp(&a.value));

    Ok(CodeLeaksSummary {
        total_secrets: total,
        unique_repos: repo_set.len() as u64,
        high_severity_secrets: high_severity,
        exposure_platforms,
        secret_types,
    })
}

fn hostname_to_platform(url: &str) -> String {
    let url_lower = url.to_lowercase();
    if url_lower.contains("github.com") {
        "GitHub".to_string()
    } else if url_lower.contains("gitlab.com") {
        "GitLab".to_string()
    } else if url_lower.contains("bitbucket.org") {
        "Bitbucket".to_string()
    } else if url_lower.contains("pastebin.com") {
        "Pastebin".to_string()
    } else if url_lower.contains("gist.github.com") {
        "GitHub Gist".to_string()
    } else {
        "Other".to_string()
    }
}

fn extract_repo_from_url(url: &str) -> Option<String> {
    // Try to extract owner/repo from GitHub/GitLab/Bitbucket URLs
    let url_lower = url.to_lowercase();
    if url_lower.contains("github.com")
        || url_lower.contains("gitlab.com")
        || url_lower.contains("bitbucket.org")
    {
        let parts: Vec<&str> = url.split('/').filter(|p| !p.is_empty()).collect();
        // URL format: https://github.com/owner/repo/...
        if parts.len() >= 4 {
            return Some(format!("{}/{}", parts[2], parts[3]));
        }
    }
    None
}

/// Fetch takedown examples (requested takedowns)
async fn fetch_takedown_examples(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    page_size: usize,
    customer: &str,
) -> Result<Vec<TakedownExample>> {
    let url = format!(
        "{}/tickets-api/tickets?takedown=true&takedown.request.date=ge:{}&takedown.request.date=le:{}&pageSize={}&sortBy=takedown.request.date&order=desc&include=fields&ticket.customer={}",
        API_URL, from, to, page_size, customer
    );

    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    if !success {
        log_api_call(
            "fetch_takedown_examples",
            &url,
            status,
            false,
            "Request failed",
        );
        return Ok(vec![]);
    }

    let text = resp.text().await?;
    log_api_call("fetch_takedown_examples", &url, status, true, &text);

    let data: TicketsResponse = serde_json::from_str(&text).unwrap_or(TicketsResponse {
        tickets: vec![],
        pageable: None,
    });

    let examples: Vec<TakedownExample> = data
        .tickets
        .into_iter()
        .map(|t| {
            let ticket_info = t.ticket.as_ref();
            let current = t.current.as_ref();

            TakedownExample {
                name: ticket_info
                    .and_then(|ti| ti.reference.clone())
                    .unwrap_or_else(|| "N/A".to_string()),
                ticket_type: current
                    .and_then(|c| c.type_.clone())
                    .unwrap_or_else(|| "unknown".to_string()),
                status: current
                    .and_then(|c| c.status.clone())
                    .unwrap_or_else(|| "unknown".to_string()),
                request_date: current.and_then(|c| c.takedown_request_date()),
                url: ticket_info
                    .and_then(|ti| ti.reference.clone())
                    .unwrap_or_default(),
            }
        })
        .collect();

    Ok(examples)
}

/// Fetch resolved takedowns
async fn fetch_resolved_takedowns(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    page_size: usize,
    customer: &str,
) -> Result<Vec<ResolvedTakedown>> {
    let url = format!(
        "{}/tickets-api/tickets?current.takedown.resolution=resolved&current.resolution=resolved&pageSize={}&sortBy=current.close.date&order=desc&include=fields,attachments&ticket.customer={}",
        API_URL, page_size, customer
    );

    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    if !success {
        log_api_call(
            "fetch_resolved_takedowns",
            &url,
            status,
            false,
            "Request failed",
        );
        return Ok(vec![]);
    }

    let text = resp.text().await?;
    log_api_call("fetch_resolved_takedowns", &url, status, true, &text);

    let data: TicketsResponse = serde_json::from_str(&text).unwrap_or(TicketsResponse {
        tickets: vec![],
        pageable: None,
    });

    // We need to iterate and await inside map, which isn't directly supported by iterator map.
    // So we'll use a loop and build the vector.
    let mut resolved_list = Vec::new();

    for t in data.tickets {
        let ticket_info = t.ticket.as_ref();
        let current = t.current.as_ref();

        // Find screenshot from TOP-LEVEL attachments (ticket.attachments)
        let screenshot_url = t
            .attachments
            .iter()
            .find(|a| {
                let url = a.url.as_deref().unwrap_or("");
                let name = a.name.as_deref().unwrap_or("");
                url.ends_with(".png")
                    || url.ends_with(".jpg")
                    || url.ends_with(".jpeg")
                    || name.ends_with(".png")
                    || name.ends_with(".jpg")
                    || name.ends_with(".jpeg")
            })
            .and_then(|a| a.url.clone());

        // Download image with auth and convert to base64
        let screenshot_base64 = if let Some(ref img_url) = screenshot_url {
            download_image_as_base64(client, auth, img_url).await
        } else {
            None
        };

        resolved_list.push(ResolvedTakedown {
            ticket_key: ticket_info
                .and_then(|ti| ti.ticket_key.clone())
                .unwrap_or_default(),
            name: ticket_info
                .and_then(|ti| ti.reference.clone())
                .unwrap_or_else(|| "N/A".to_string()),
            ticket_type: current
                .and_then(|c| c.type_.clone())
                .unwrap_or_else(|| "unknown".to_string()),
            status: current
                .and_then(|c| c.status.clone())
                .unwrap_or_else(|| "resolved".to_string()),
            host: current.and_then(|c| c.host.clone()).unwrap_or_default(),
            ip: current.and_then(|c| c.ip.clone()).unwrap_or_default(),
            country: String::new(), // Not available directly
            request_date: current.and_then(|c| c.takedown_request_date()),
            resolution_date: current.and_then(|c| c.close_date()),
            url: ticket_info
                .and_then(|ti| ti.reference.clone())
                .unwrap_or_default(),
            screenshot_url: screenshot_base64,
        });
    }

    Ok(resolved_list)
}

/// Fetch latest incidents for ALL threat types (all 30 detection types)
async fn fetch_latest_incidents(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    page_size: usize,
    customer: &str,
) -> Result<Vec<IncidentExample>> {
    // ALL 30 detection/ticket types from Axur API
    let incident_types = [
        // Brand Protection
        "phishing",
        "fake-social-media-profile",
        "fraudulent-brand-use",
        "fake-mobile-app",
        "similar-domain-name",
        "paid-search",
        "malware",
        // Data Exposure
        "code-secret-leak",
        "corporate-credential-leak",
        "database-exposure",
        "other-sensitive-data",
        "infostealer-credential",
        "infrastructure-exposure",
        // Sales & Distribution
        "unauthorized-sale",
        "unauthorized-distribution",
        "data-sale-website",
        "data-sale-message",
        "data-exposure-website",
        "data-exposure-message",
        // Suspicious Activity
        "suspicious-activity-website",
        "suspicious-activity-message",
        "fraud-tool-scheme-website",
        "fraud-tool-scheme-message",
        // Dark Web & Executive
        "dw-activity",
        "ransomware-attack",
        "executive-card-leak",
        "executive-credential-leak",
        "executive-fake-social-media-profile",
        "executive-personalinfo-leak",
    ];
    let mut all_incidents = Vec::new();

    for incident_type in &incident_types {
        // Per API documentation line 220: status field supports [open, quarantine, incident, treatment, closed]
        // Per API documentation line 242: type field is for detection-type filter
        // Using status=incident (not current.status) with type= filter and open.date range
        let url = format!(
            "{}/tickets-api/tickets?type={}&status=incident&open.date=ge:{}&open.date=le:{}&sortBy=open.date&order=desc&pageSize={}&include=fields,attachments&ticket.customer={}",
            API_URL, incident_type, from, to, page_size, customer
        );

        let resp = client.get(&url).header("Authorization", auth).send().await;

        if let Ok(response) = resp {
            let status = response.status().as_u16();
            if response.status().is_success() {
                if let Ok(text) = response.text().await {
                    log_api_call(
                        &format!("fetch_latest_incidents_{}", incident_type),
                        &url,
                        status,
                        true,
                        &text,
                    );

                    if let Ok(data) = serde_json::from_str::<TicketsResponse>(&text) {
                        for t in data.tickets.into_iter().take(page_size) {
                            let ticket_info = t.ticket.as_ref();
                            let current = t.current.as_ref();

                            // Find screenshot from attachments
                            let screenshot = t
                                .snapshots
                                .iter()
                                .flat_map(|s| s.attachments.iter())
                                .find(|a| {
                                    a.url.as_ref().map_or(false, |u| {
                                        u.ends_with(".png")
                                            || u.ends_with(".jpg")
                                            || u.ends_with(".jpeg")
                                    })
                                })
                                .and_then(|a| a.url.clone());

                            all_incidents.push(IncidentExample {
                                ticket_key: ticket_info
                                    .and_then(|ti| ti.ticket_key.clone())
                                    .unwrap_or_default(),
                                name: ticket_info
                                    .and_then(|ti| ti.reference.clone())
                                    .unwrap_or_else(|| "N/A".to_string()),
                                ticket_type: incident_type.to_string(),
                                status: current
                                    .and_then(|c| c.status.clone())
                                    .unwrap_or_else(|| "incident".to_string()),
                                open_date: current.and_then(|c| c.open_date()),
                                incident_date: current.and_then(|c| c.incident_date()),
                                host: current.and_then(|c| c.host.clone()).unwrap_or_default(),
                                ip: current.and_then(|c| c.ip.clone()).unwrap_or_default(),
                                isp: current.and_then(|c| c.isp.clone()).unwrap_or_default(),
                                url: ticket_info
                                    .and_then(|ti| ti.reference.clone())
                                    .unwrap_or_default(),
                                screenshot_url: screenshot,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(all_incidents)
}

/// Fetch credential leaks summary for Risk Profiling (Stealer Logs, Plaintext)
/// Uses /exposure-api/credentials to get detailed leak info
async fn fetch_credential_leaks_summary(
    client: &reqwest::Client,
    auth: &str,
    from: &str,
    to: &str,
    customer: &str,
) -> Result<CredentialLeaksSummary> {
    // We need fields: leak.format, password.type, user
    let url = format!(
        "{}/exposure-api/credentials?created=ge:{}T00:00:00&created=le:{}T23:59:59&fields=leak.format,password.type,user,status&pageSize=100&customer={}&timezone=-03:00",
        API_URL, from, to, customer
    );

    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    let status = resp.status().as_u16();
    let success = resp.status().is_success();

    if !success {
        log_api_call(
            "fetch_credential_leaks_summary",
            &url,
            status,
            false,
            "Request failed",
        );
        return Ok(CredentialLeaksSummary {
            total_credentials: 0,
            unique_emails: 0,
            sources: vec![],
            plaintext_passwords: 0,
            stealer_logs_count: 0,
        });
    }

    let text = resp.text().await?;
    log_api_call("fetch_credential_leaks_summary", &url, status, true, &text);

    #[derive(Deserialize)]
    struct CredDetection {
        #[serde(rename = "leak.format")]
        leak_format: Option<String>,
        #[serde(rename = "password.type")]
        password_type: Option<String>,
        user: Option<String>,
    }

    #[derive(Deserialize)]
    struct ExposureResponse {
        #[serde(default)]
        detections: Vec<CredDetection>,
        pageable: Option<Pageable>,
    }

    let data: ExposureResponse = serde_json::from_str(&text).unwrap_or(ExposureResponse {
        detections: vec![],
        pageable: None,
    });
    let total = data.pageable.as_ref().and_then(|p| p.total).unwrap_or(0);

    // Analyze first 100 for distribution (approximated for total if > 100)
    let sample_size = data.detections.len() as f64;
    let mut stealer_count = 0;
    let mut plain_count = 0;
    let mut unique_emails = HashSet::new();
    let mut sources_map = HashMap::new();

    for d in &data.detections {
        // Count stealer logs
        let fmt = d.leak_format.as_deref().unwrap_or("UNKNOWN");
        *sources_map.entry(fmt.to_string()).or_insert(0) += 1;

        if fmt.to_uppercase().contains("STEALER") {
            stealer_count += 1;
        }

        // Count plaintext
        if let Some(ptype) = &d.password_type {
            if ptype.to_uppercase() == "PLAIN" {
                plain_count += 1;
            }
        }

        // Unique emails
        if let Some(u) = &d.user {
            unique_emails.insert(u.to_string());
        }
    }

    // Extrapolate to total if we have more than page size
    let (final_stealer, final_plain, final_unique) = if total > 0 && sample_size > 0.0 {
        let ratio = total as f64 / sample_size;
        (
            (stealer_count as f64 * ratio) as u64,
            (plain_count as f64 * ratio) as u64,
            (unique_emails.len() as f64 * ratio) as u64,
        )
    } else {
        (stealer_count, plain_count, unique_emails.len() as u64)
    };

    let mut sources_vec: Vec<NameValuePair> = sources_map
        .into_iter()
        .map(|(k, v)| NameValuePair {
            name: k,
            value: (v as f64
                * (if total > 0 {
                    total as f64 / sample_size
                } else {
                    1.0
                })) as u64,
        })
        .collect();
    sources_vec.sort_by(|a, b| b.value.cmp(&a.value));

    Ok(CredentialLeaksSummary {
        total_credentials: total,
        unique_emails: final_unique,
        sources: sources_vec,
        plaintext_passwords: final_plain,
        stealer_logs_count: final_stealer,
    })
}

// ========================
// LEGACY COMPAT (keep old function working)
// ========================

/// Legacy PocReport for backward compatibility
#[derive(Debug, Serialize)]
pub struct PocReport {
    pub tenant_id: String,
    pub customer_name: String,
    pub date_from: String,
    pub date_to: String,
    pub total_signals: u64,
    pub total_incidents: u64,
    pub total_threats: u64,
    pub credentials_employee: u64,
    pub credentials_stealer: u64,
    pub takedown_resolved: u64,
    pub takedown_pending: u64,
    pub takedown_aborted: u64,
    pub incidents_by_type: Vec<LegacyIncidentType>,
    pub code_leak_total: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct LegacyIncidentType {
    pub incident_type: String,
    pub count: u64,
}

/// Legacy function for backward compatibility
pub async fn fetch_report_data(
    token: &str,
    tenant_id: &str,
    from: &str,
    to: &str,
) -> Result<PocReport> {
    let full = fetch_full_report(token, tenant_id, from, to).await?;

    Ok(PocReport {
        tenant_id: tenant_id.to_string(),
        customer_name: full.company_name,
        date_from: full.start_date,
        date_to: full.end_date,
        total_signals: full.total_tickets,
        total_incidents: full.total_threats,
        total_threats: full.total_threats,
        credentials_employee: full.credentials_total / 2,
        credentials_stealer: full.credentials_total / 2,
        takedown_resolved: full.takedown_resolved,
        takedown_pending: full.takedown_pending,
        takedown_aborted: full.takedown_aborted,
        incidents_by_type: full
            .threats_by_type
            .iter()
            .map(|t| LegacyIncidentType {
                incident_type: t.threat_type.clone(),
                count: t.count,
            })
            .collect(),
        code_leak_total: full.secrets_total,
    })
}
