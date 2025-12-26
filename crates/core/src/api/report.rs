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
        tracing::warn!("Failed to download screenshot: {} - {}", url, resp.status());
        return None;
    }

    // Get content type from response headers
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| {
            // Fallback: detect from URL extension
            if url.ends_with(".png") {
                "image/png"
            } else if url.ends_with(".gif") {
                "image/gif"
            } else {
                "image/jpeg"
            }
        })
        .to_string();

    let bytes = resp.bytes().await.ok()?;

    // Limit image size to avoid huge HTML files (max 500KB)
    if bytes.len() > 500_000 {
        tracing::warn!(
            "Screenshot too large ({} bytes), skipping base64 encoding",
            bytes.len()
        );
        return None;
    }

    let base64_str = general_purpose::STANDARD.encode(&bytes);
    Some(format!("data:{};base64,{}", content_type, base64_str))
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

    // NEW: Story / Timeline Tickets
    pub story_tickets: Vec<StoryTicket>,

    // NEW: Threat Intelligence from Threat Hunting API
    pub threat_intelligence: ThreatIntelligence,

    // NEW: Deep Investigation results from signal-lake
    pub deep_investigations: Vec<DeepInvestigationResult>,

    // NEW: Enriched Credential Exposures
    pub credential_exposures: Vec<CredentialExposure>,

    // UI FLAGS
    pub is_dynamic_window: bool,
}

/// Aggregated threat intelligence data from Threat Hunting API
#[derive(Debug, Serialize, Clone, Default)]
pub struct ThreatIntelligence {
    // === Dimension 1: Threat Origin (Dark Web) ===
    pub dark_web_mentions: u64,
    pub earliest_dark_web_date: Option<String>,
    pub dark_web_sources: Vec<String>,
    pub days_before_public: Option<i64>, // Days detected before going public

    // === Dimension 2: Virality & Reach ===
    pub chat_group_shares: u64,          // WhatsApp/Telegram/Discord
    pub social_media_mentions: u64,      // Twitter/X
    pub platforms_detected: Vec<String>, // List of platforms

    // === Dimension 3: Credential Quality ===
    pub total_credentials: u64,
    pub stealer_log_count: u64,       // CRITICAL: Active malware
    pub combolist_count: u64,         // Medium: Old/recycled
    pub plain_password_count: u64,    // High risk: plain text
    pub hashed_password_count: u64,   // Lower risk: hashed
    pub stealer_log_percent: f64,     // % that are stealer logs
    pub plain_password_percent: f64,  // % plain text passwords
    pub top_access_urls: Vec<String>, // Target portals

    // === Dimension 4: Attacker Investment ===
    pub paid_ads_detected: u64,
    pub ad_platforms: Vec<String>, // FB, IG, Google etc.

    // === Meta ===
    pub data_available: bool, // False if API unavailable or no results
}

impl ThreatIntelligence {
    /// Generate demo/fake data for previewing the slide design
    pub fn demo() -> Self {
        Self {
            // Dark Web
            dark_web_mentions: 12,
            earliest_dark_web_date: Some("2024-12-19T14:30:00Z".to_string()),
            dark_web_sources: vec![
                "Telegram - Fraud Groups".to_string(),
                "Dark Forum XSS".to_string(),
                "RaidForums Archive".to_string(),
            ],
            days_before_public: Some(3),

            // Virality
            chat_group_shares: 47,
            social_media_mentions: 23,
            platforms_detected: vec![
                "Telegram".to_string(),
                "WhatsApp".to_string(),
                "Twitter/X".to_string(),
            ],

            // Credentials
            total_credentials: 156,
            stealer_log_count: 31,    // 20%
            combolist_count: 94,      // 60%
            plain_password_count: 47, // 30%
            hashed_password_count: 109,
            stealer_log_percent: 19.9,
            plain_password_percent: 30.1,
            top_access_urls: vec![
                "login.empresa.com".to_string(),
                "portal.banco.com.ar".to_string(),
            ],

            // Ads
            paid_ads_detected: 3,
            ad_platforms: vec!["Facebook".to_string(), "Instagram".to_string()],

            data_available: true,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct StoryTicket {
    pub ticket_key: String,
    pub target: String,      // Domain/host
    pub status: String,      // incident, open, closed
    pub threat_type: String, // phishing, fake-social-media-profile, etc.
    pub description: String, // Type + host combined
    pub screenshot_url: Option<String>,

    // Timeline dates (for individual story)
    pub creation_date: Option<String>, // When discovered
    pub open_date: Option<String>,     // When opened
    pub incident_date: Option<String>, // When became incident
    pub close_date: Option<String>,    // When closed (if applicable)

    // Operational impact metrics
    pub isp: Option<String>,           // Hosting provider
    pub ip: Option<String>,            // IP address
    pub risk_score: Option<f64>,       // 0-1 prediction.risk
    pub brand_confidence: Option<f64>, // 0-1 prediction.brand-logo
    pub page_title: Option<String>,    // From snapshots.content.title

    // Computed metrics
    pub time_to_incident_hours: Option<i64>, // Hours from creation to incident
    pub incident_age_hours: Option<i64>,     // Hours since incident started
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

// ========================
// DEEP INVESTIGATION (Signal-Lake)
// ========================

/// Result of deep investigation using signal-lake API
#[derive(Debug, Serialize, Clone, Default)]
pub struct DeepInvestigationResult {
    /// Original ticket key
    pub ticket_key: String,
    /// Target domain/host investigated
    pub target: String,
    /// Ticket status
    pub status: String,
    /// Threat type (phishing, fake-social-media, etc.)
    pub threat_type: String,
    /// Related URLs found in signal-lake
    pub related_urls: Vec<String>,
    /// Historical detection dates from signal-lake
    pub detection_dates: Vec<String>,
    /// Infrastructure information
    pub infrastructure: InfrastructureInfo,
    /// Whether this appears to be a mass campaign
    pub is_mass_campaign: bool,
    /// Number of related signals found
    pub signal_count: u64,
    /// First seen date in signal-lake
    pub first_seen: Option<String>,
    /// Last seen date in signal-lake
    pub last_seen: Option<String>,
    /// Enriched data from AI inspection, geolocation, screenshots
    pub enrichment: EnrichedSignalData,
}

/// Infrastructure details from signal-lake investigation
#[derive(Debug, Serialize, Clone, Default)]
pub struct InfrastructureInfo {
    pub ip: Option<String>,
    pub asn: Option<String>,
    pub hosting_provider: Option<String>,
    pub country: Option<String>,
    pub related_domains: Vec<String>,
}

/// Enriched data from Signal-Lake AI inspection, geolocation, and site scanner
#[derive(Debug, Serialize, Clone, Default)]
pub struct EnrichedSignalData {
    /// Screenshot image as base64 data URL (data:image/jpeg;base64,...)
    pub screenshot_base64: Option<String>,
    /// Original screenshot URL from Axur S3
    pub screenshot_url: Option<String>,
    /// AI classification of content type (Login page, Error page, etc.)
    pub ai_content_type: Option<String>,
    /// Description of the page from AI analysis
    pub ai_image_description: Option<String>,
    /// Brands being impersonated with level (high, medium, low)
    pub impersonated_brands: Vec<ImpersonatedBrand>,
    /// Whether credentials are requested on the page
    pub credential_requested: bool,
    /// Whether payment is requested on the page
    pub payment_requested: bool,
    /// Predominant language detected
    pub predominant_language: Option<String>,
    /// Geolocation info
    pub geolocation: Option<GeoInfo>,
    /// HTTP status code when site was scanned
    pub http_status: Option<u16>,
    /// Detection date (formatted as human-readable string)
    pub detection_date: Option<String>,
    /// Domain creation date (from WHOIS)
    pub domain_created: Option<String>,
    /// Registrar name (from WHOIS)
    pub registrar: Option<String>,
    /// Company logos detected via AI
    pub company_logos: Vec<String>,
}

/// Impersonated brand with confidence level
#[derive(Debug, Serialize, Clone, Default)]
pub struct ImpersonatedBrand {
    pub brand: String,
    pub level: String, // "high", "medium", "low"
    pub explanation: Option<String>,
}

/// Geolocation information
#[derive(Debug, Serialize, Clone, Default)]
pub struct GeoInfo {
    pub ip: Option<String>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub isp: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Signal-Lake search request payload
#[derive(Debug, Serialize)]
struct SignalLakeSearchRequest {
    pub query: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
}

/// Signal-Lake search response with searchId
#[derive(Debug, Deserialize)]
struct SignalLakeSearchInitResponse {
    #[serde(rename = "searchId")]
    pub search_id: Option<String>,
    pub message: Option<String>,
}

/// Signal-Lake poll response
#[derive(Debug, Deserialize)]
struct SignalLakePollResponse {
    pub id: Option<String>,
    pub result: Option<SignalLakePollResult>,
}

#[derive(Debug, Deserialize)]
struct SignalLakePollResult {
    pub status: Option<SignalLakeStatus>,
    pub data: Option<Vec<SignalLakeDataItem>>,
    pub pagination: Option<SignalLakePagination>,
}

#[derive(Debug, Deserialize)]
struct SignalLakeStatus {
    pub running: Option<bool>,
    // we can add other fields if needed
}

#[derive(Debug, Deserialize, Clone)]
struct SignalLakeDataItem {
    pub url: Option<String>,
    pub domain: Option<String>,
    pub ip: Option<String>,
    pub asn: Option<String>,
    #[serde(rename = "hostingProvider")]
    pub hosting_provider: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "firstSeen")]
    pub first_seen: Option<String>,
    #[serde(rename = "lastSeen")]
    pub last_seen: Option<String>,
    #[serde(rename = "detectionDate")]
    pub detection_date: Option<String>,
    pub name: Option<String>,

    // Reference data (flattened from content.hitData)
    #[serde(rename = "content.hitData.reference")]
    pub reference: Option<String>,
    #[serde(rename = "content.hitData.referenceType")]
    pub reference_type: Option<String>,
    #[serde(rename = "content.hitData.hitDetectionDate")]
    pub hit_detection_date: Option<i64>, // Unix timestamp
    #[serde(rename = "content.hitData.hitMetadata.referenceElements.domain")]
    pub reference_domain: Option<String>,

    // Site Scanner data
    #[serde(rename = "content.enrichmentData.enrichmentContent.siteScanner.image.contentLocation")]
    pub screenshot_url: Option<String>,
    #[serde(rename = "content.enrichmentData.enrichmentContent.siteScanner.statusCode")]
    pub http_status: Option<u16>,
    #[serde(rename = "content.enrichmentData.enrichmentContent.siteScanner.error")]
    pub site_scanner_error: Option<bool>,

    // AI Inspection data
    #[serde(rename = "content.enrichmentData.enrichmentContent.aiInspection.contentType.type")]
    pub ai_content_type: Option<String>,
    #[serde(rename = "content.enrichmentData.enrichmentContent.aiInspection.imageDescription")]
    pub ai_image_description: Option<String>,
    #[serde(
        rename = "content.enrichmentData.enrichmentContent.aiInspection.credentialRequested.value"
    )]
    pub credential_requested: Option<String>, // "Yes", "No", ""
    #[serde(
        rename = "content.enrichmentData.enrichmentContent.aiInspection.paymentRequested.value"
    )]
    pub payment_requested: Option<String>, // "Yes", "No", "Possibly", ""
    #[serde(rename = "content.enrichmentData.enrichmentContent.aiInspection.predominantIdiom")]
    pub predominant_language: Option<String>,
    #[serde(rename = "content.enrichmentData.enrichmentContent.aiInspection.companyLogosFlat")]
    pub company_logos: Option<Vec<String>>,
    #[serde(
        rename = "content.enrichmentData.enrichmentContent.aiInspection.impersonatedBrandsHigh"
    )]
    pub impersonated_brands_high: Option<Vec<String>>,
    #[serde(
        rename = "content.enrichmentData.enrichmentContent.aiInspection.impersonatedBrandsMedium"
    )]
    pub impersonated_brands_medium: Option<Vec<String>>,
    #[serde(
        rename = "content.enrichmentData.enrichmentContent.aiInspection.impersonatedBrandsLow"
    )]
    pub impersonated_brands_low: Option<Vec<String>>,

    // WHOIS data
    #[serde(
        rename = "content.enrichmentData.enrichmentContent.whois.whoisData.timestampDomainLong.createdAt"
    )]
    pub domain_created_ts: Option<i64>, // Unix timestamp
    #[serde(rename = "content.enrichmentData.enrichmentContent.whois.whoisData.registrar.name")]
    pub registrar_name: Option<String>,

    // Geolocation data (we'll take the first entry)
    #[serde(rename = "content.enrichmentData.enrichmentContent.countryCodes")]
    pub country_codes: Option<Vec<String>>,
    #[serde(rename = "content.enrichmentData.enrichmentContent.countryNames")]
    pub country_names: Option<Vec<String>>,
    #[serde(rename = "content.enrichmentData.enrichmentContent.isps")]
    pub isps: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct SignalLakePagination {
    pub total: Option<u64>,
    pub page: Option<u32>,
    #[serde(rename = "totalPages")]
    pub total_pages: Option<u32>,
}

// ========================
// THREAT HUNTING PREVIEW
// ========================
// Note: ThreatHuntingSource enum is defined later in this file (line ~2844)

/// Preview of available Threat Hunting data (before consuming credits)
#[derive(Debug, Serialize, Clone, Default)]
pub struct ThreatHuntingPreview {
    /// Count of results in signal-lake (domains, IPs, URLs)
    pub signal_lake_count: u64,
    /// Count of leaked credentials found
    pub credential_count: u64,
    /// Count of chat messages (Telegram, Discord, etc)
    pub chat_message_count: u64,
    /// Count of forum posts (Deep Web)
    pub forum_message_count: u64,
    /// Total count across all sources
    pub total_count: u64,
    /// Estimated credits to consume for full report
    pub estimated_credits: u64,
    /// Number of tickets that will be investigated
    pub tickets_count: usize,
    /// Sample data for preview (first 3 items from each source)
    pub samples: ThreatHuntingSamples,
}

impl ThreatHuntingPreview {
    pub fn compute_estimated_credits(&mut self) {
        // Each page of 10 results = 1 credit
        let pages = |count: u64| -> u64 { (count + 9) / 10 };
        self.estimated_credits = pages(self.signal_lake_count)
            + pages(self.credential_count)
            + pages(self.chat_message_count)
            + pages(self.forum_message_count);
    }
}

/// Sample items for preview display
#[derive(Debug, Serialize, Clone, Default)]
pub struct ThreatHuntingSamples {
    pub signal_lake: Vec<String>,   // First 3 domains/URLs
    pub credentials: Vec<String>,   // First 3 access URLs
    pub chat_messages: Vec<String>, // First 3 message snippets
    pub forum_posts: Vec<String>,   // First 3 post titles
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
    snapshots: serde_json::Value, // Can be object or array depending on API version
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

    // Nested date fields (old API format)
    #[serde(rename = "open")]
    open: Option<DateInfo>,
    #[serde(rename = "incident")]
    incident: Option<DateInfo>,
    #[serde(rename = "close")]
    close: Option<DateInfo>,

    // Flat date fields (new API format) - used by ticket tags API
    #[serde(rename = "open.date")]
    open_date_flat: Option<String>,
    #[serde(rename = "incident.date")]
    incident_date_flat: Option<String>,
    #[serde(rename = "close.date")]
    close_date_flat: Option<String>,
    #[serde(rename = "creation.date")]
    creation_date_flat: Option<String>,

    // Prediction metrics
    #[serde(rename = "prediction.risk")]
    prediction_risk: Option<String>, // String because API returns "0.36"
    #[serde(rename = "prediction.brand-logo")]
    prediction_brand_logo: Option<String>,
    #[serde(rename = "prediction.brand-name")]
    prediction_brand_name: Option<String>,

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
    story_tag: Option<String>,
    include_threat_intel: bool,
) -> Result<PocReportData> {
    // DEBUG: Write to file to trace story_tag
    use std::io::Write;
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("story_debug.log")
    {
        let _ = writeln!(
            file,
            "[{}] fetch_full_report called with story_tag: {:?}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            story_tag
        );
    }

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
        story_tickets_res,
        credential_exposures_res,
    ) = tokio::join!(
        fetch_customer_data(&client, &auth, tenant_id),
        fetch_ticket_count(&client, &auth, from, to, "open", tenant_id),
        fetch_ticket_count(&client, &auth, from, to, "incident", tenant_id),
        fetch_ticket_count(&client, &auth, from, to, "closed", tenant_id),
        fetch_threats_by_type_map(&client, &auth, from, to, tenant_id),
        fetch_credentials_total_all(&client, &auth, from, to, tenant_id),
        fetch_code_leaks_count(&client, &auth, from, to, tenant_id),
        fetch_takedown_stats_full(&client, &auth, from, to, tenant_id),
        fetch_tagged_tickets(&client, &auth, tenant_id, story_tag.as_deref()),
        async {
            if let Some(tag) = story_tag.as_deref() {
                if !tag.is_empty() {
                    // Use the same customer logic as in preview if needed, but here we enforce tenant_id
                    return fetch_tagged_credentials(&client, &auth, tenant_id, tag).await;
                }
            }
            Vec::new()
        }
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

        // NEW: Story Tickets
        story_tickets: story_tickets_res.unwrap_or_default(),

        // NEW: Threat Intelligence - fetched conditionally
        threat_intelligence: ThreatIntelligence::default(), // Placeholder, will be populated below

        // NEW: Deep Investigation results from signal-lake
        deep_investigations: vec![], // Placeholder, will be populated below

        // NEW: Enriched Credential Exposures
        credential_exposures: credential_exposures_res,

        is_dynamic_window: false, // Default to fixed window
    };

    // Fetch Threat Intelligence if enabled (this is async and can take 1-2 min)
    let threat_intel = if include_threat_intel {
        // Use the first brand or company name as the search query
        let query = report
            .brands
            .first()
            .cloned() // brands is Vec<String>, clone the string directly
            .unwrap_or_else(|| report.company_name.clone());

        tracing::info!("Fetching threat intelligence for query: {}", query);
        fetch_threat_intelligence(&client, &auth, &query, from, to).await
    } else {
        ThreatIntelligence::default()
    };

    // Deep Investigation: If we have story tickets, investigate them via signal-lake
    let deep_investigations = if !report.story_tickets.is_empty() && include_threat_intel {
        tracing::info!(
            "Investigating {} tagged tickets via signal-lake",
            report.story_tickets.len()
        );
        investigate_tagged_tickets(&client, &auth, tenant_id, &report.story_tickets).await
    } else {
        vec![]
    };

    // Compute ROI metrics now that we have the full data
    let roi = RoiMetrics::compute(&report);
    let mut report_with_roi = report;
    report_with_roi.roi_metrics = roi;
    report_with_roi.threat_intelligence = threat_intel;
    report_with_roi.deep_investigations = deep_investigations;

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
                            .attachments
                            .iter()
                            .find(|a| {
                                a.name
                                    .as_ref()
                                    .map(|n| n.ends_with(".jpg") || n.ends_with(".png"))
                                    .unwrap_or(false)
                                    || a.type_.as_deref() == Some("screenshot")
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
                                .attachments
                                .iter()
                                .find(|a| {
                                    a.url.as_ref().map_or(false, |u| {
                                        u.ends_with(".png")
                                            || u.ends_with(".jpg")
                                            || u.ends_with(".jpeg")
                                    }) || a.name.as_ref().map_or(false, |n| {
                                        n.ends_with(".png")
                                            || n.ends_with(".jpg")
                                            || n.ends_with(".jpeg")
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
    let full = fetch_full_report(token, tenant_id, from, to, None, false).await?;

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

/// Fetch tickets filtered by tag for the Story Slide
async fn fetch_tagged_tickets(
    client: &reqwest::Client,
    auth: &str,
    tenant_id: &str,
    tag: Option<&str>,
) -> Result<Vec<StoryTicket>> {
    use std::io::Write;
    let mut log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("story_debug.log")
        .ok();

    macro_rules! log_debug {
        ($($arg:tt)*) => {
            if let Some(ref mut f) = log_file {
                let _ = writeln!(f, "[{}] {}", chrono::Local::now().format("%H:%M:%S"), format!($($arg)*));
            }
        };
    }

    log_debug!("fetch_tagged_tickets called with tag: {:?}", tag);

    let tag = match tag {
        Some(t) if !t.is_empty() => t,
        _ => {
            log_debug!("No tag provided, skipping story tickets fetch");
            return Ok(vec![]);
        }
    };

    let url = format!(
        "{}/tickets-api/tickets?ticket.customer={}&ticket.tags={}",
        API_URL, tenant_id, tag
    );

    log_debug!("Fetching from URL: {}", url);

    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    log_debug!("Response status: {}", resp.status());

    if !resp.status().is_success() {
        log_debug!("API returned error status: {}", resp.status());
        return Ok(vec![]);
    }

    let body = resp.text().await?;
    log_debug!("Response body length: {} chars", body.len());
    log_debug!("First 500 chars: {}", &body[..body.len().min(500)]);

    let response: TicketsResponse = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            log_debug!("JSON parsing error: {}", e);
            return Ok(vec![]);
        }
    };
    log_debug!("Parsed {} tickets from response", response.tickets.len());

    let mut story_tickets = Vec::new();

    for item in response.tickets {
        if let (Some(ticket), Some(detection)) = (item.ticket, item.detection) {
            let key = ticket.ticket_key.unwrap_or_default();

            // Determine date (Incident > Open > Created)
            let date = detection
                .incident_date()
                .or_else(|| detection.open_date())
                .unwrap_or_else(|| "N/A".to_string());

            // Format date nicely if possible
            let fmt_date = if date.len() >= 10 {
                format!("{}/{}/{}", &date[8..10], &date[5..7], &date[0..4])
            } else {
                date
            };

            // Determine Target
            let target = detection
                .host
                .clone()
                .or(detection.domain.clone())
                .or(ticket.reference.clone())
                .unwrap_or_else(|| "Unknown Target".to_string());

            // Image - find URL first
            let screenshot_url = item
                .attachments
                .iter()
                .find(|a| {
                    a.name
                        .as_ref()
                        .map(|n| n.ends_with(".jpg") || n.ends_with(".png"))
                        .unwrap_or(false)
                })
                .and_then(|a| a.url.clone());

            // Download and convert to base64 with authentication
            let screenshot_base64 = if let Some(ref img_url) = screenshot_url {
                download_image_as_base64(client, auth, img_url).await
            } else {
                None
            };

            // Extract dates (prefer flat format, fallback to nested)
            let creation_date = detection.creation_date_flat.clone();
            let open_date = detection
                .open_date_flat
                .clone()
                .or_else(|| detection.open_date());
            let incident_date = detection
                .incident_date_flat
                .clone()
                .or_else(|| detection.incident_date());
            let close_date = detection
                .close_date_flat
                .clone()
                .or_else(|| detection.close_date());

            // Parse prediction metrics
            let risk_score = detection
                .prediction_risk
                .as_ref()
                .and_then(|s| s.parse::<f64>().ok());
            let brand_confidence = detection
                .prediction_brand_logo
                .as_ref()
                .and_then(|s| s.parse::<f64>().ok());

            // Extract page title from snapshots if available
            let page_title = item
                .snapshots
                .get("content")
                .and_then(|c| c.get("title"))
                .and_then(|t| t.as_str())
                .map(|s| s.to_string());

            // Compute time metrics
            let time_to_incident_hours = compute_hours_between(&creation_date, &incident_date);
            let incident_age_hours = incident_date.as_ref().and_then(|d| compute_hours_since(d));

            // Description (Type + Host)
            let threat_type = detection.type_.clone().unwrap_or_default();
            let desc = format!("{} on {}", &threat_type, &target);

            story_tickets.push(StoryTicket {
                ticket_key: key,
                target,
                status: detection
                    .status
                    .clone()
                    .unwrap_or_else(|| "open".to_string()),
                threat_type,
                description: desc,
                screenshot_url: screenshot_base64,
                creation_date,
                open_date,
                incident_date,
                close_date,
                isp: detection.isp.clone(),
                ip: detection.ip.clone(),
                risk_score,
                brand_confidence,
                page_title,
                time_to_incident_hours,
                incident_age_hours,
            });
        }
    }

    log_debug!("Returning {} story_tickets", story_tickets.len());
    Ok(story_tickets)
}

/// Public wrapper to fetch tagged tickets for the Threat Hunting preview
/// This is called from the backend handler to get real tickets before TH search
pub async fn fetch_tagged_tickets_for_preview(
    token: &str,
    tenant_id: &str,
    tag: &str,
) -> Result<Vec<StoryTicket>> {
    let client = create_client()?;
    let auth = format!("Bearer {}", token);

    // Log for debugging
    tracing::info!(
        "fetch_tagged_tickets_for_preview: tenant={}, tag={}",
        tenant_id,
        tag
    );

    // Write debug file
    let debug_dir = std::path::Path::new("debug_logs");
    if debug_dir.exists() {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let debug_file = debug_dir.join(format!("fetch_tickets_request_{}.txt", timestamp));
        let content = format!(
            "Fetch Tagged Tickets Request\n\
             Timestamp: {}\n\
             Tenant ID: {}\n\
             Tag: {}\n\
             API URL: {}/tickets-api/tickets?ticket.customer={}&ticket.tags={}\n",
            timestamp, tenant_id, tag, API_URL, tenant_id, tag
        );
        let _ = std::fs::write(&debug_file, &content);
    }

    let tickets = fetch_tagged_tickets(&client, &auth, tenant_id, Some(tag)).await?;

    // Log found tickets and their targets
    if debug_dir.exists() {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let debug_file = debug_dir.join(format!("fetch_tickets_result_{}.json", timestamp));
        let targets: Vec<_> = tickets.iter().map(|t| &t.target).collect();
        let content = format!(
            "Found {} tickets\n\nTargets extracted:\n{:#?}\n",
            tickets.len(),
            targets
        );
        let _ = std::fs::write(&debug_file, &content);
    }

    Ok(tickets)
}
/// Helper to compute hours between two ISO date strings
fn compute_hours_between(from: &Option<String>, to: &Option<String>) -> Option<i64> {
    let from = from.as_ref()?;
    let to = to.as_ref()?;
    let from_dt = chrono::DateTime::parse_from_rfc3339(from).ok()?;
    let to_dt = chrono::DateTime::parse_from_rfc3339(to).ok()?;
    Some((to_dt - from_dt).num_hours())
}

/// Helper to compute hours since an ISO date string
fn compute_hours_since(date: &str) -> Option<i64> {
    let dt = chrono::DateTime::parse_from_rfc3339(date).ok()?;
    let now = chrono::Utc::now();
    Some((now - dt.with_timezone(&chrono::Utc)).num_hours())
}

// ========================
// THREAT HUNTING API
// ========================

/// Source types for Threat Hunting searches
#[derive(Debug, Clone, Copy)]
pub enum ThreatHuntingSource {
    SignalLake,            // URLs/Domains (General)
    SignalLakeSocialMedia, // Social Media Profiles
    SignalLakeAds,         // Social Media Ads
    Credential,            // Leaked credentials
    CreditCard,            // Credit Card data
    ChatMessage,           // WhatsApp/Telegram/Discord
    ForumMessage,          // Dark web forums
    SocialMediaPosts,      // Twitter/X posts
    Tokens,                // Text file tokens (emails, SSNs)
}

impl ThreatHuntingSource {
    fn as_str(&self) -> &'static str {
        match self {
            Self::SignalLake => "signal-lake",
            Self::SignalLakeSocialMedia => "signal-lake-social-media",
            Self::SignalLakeAds => "signal-lake-ads",
            Self::Credential => "credential",
            Self::CreditCard => "credit-card",
            Self::ChatMessage => "chat-message",
            Self::ForumMessage => "forum-message",
            Self::SocialMediaPosts => "social-media-posts",
            Self::Tokens => "tokens",
        }
    }
}

/// Request body for starting a threat hunting search
#[derive(Debug, Serialize)]
struct ThreatSearchRequest {
    query: String,
    source: String,
    #[serde(rename = "dateRange", skip_serializing_if = "Option::is_none")]
    date_range: Option<DateRange>,
}

#[derive(Debug, Serialize)]
struct DateRange {
    from: String,
    to: String,
}

/// Response from starting a search
#[derive(Debug, Deserialize)]
struct ThreatSearchStartResponse {
    #[serde(rename = "searchId")]
    search_id: Option<String>,
    id: Option<String>, // Alternative field name
}

impl ThreatSearchStartResponse {
    fn get_id(&self) -> Option<&str> {
        self.search_id.as_deref().or(self.id.as_deref())
    }
}

/// Response from polling search results
#[derive(Debug, Deserialize)]
struct ThreatSearchResultsResponse {
    status: Option<String>,
    #[serde(default)]
    results: Vec<ThreatSearchResult>,
    #[serde(default)]
    data: Vec<ThreatSearchResult>, // Alternative field name
}

impl ThreatSearchResultsResponse {
    fn get_results(&self) -> &[ThreatSearchResult] {
        if !self.results.is_empty() {
            &self.results
        } else {
            &self.data
        }
    }

    fn is_complete(&self) -> bool {
        self.status
            .as_deref()
            .map(|s| s.eq_ignore_ascii_case("COMPLETED"))
            .unwrap_or(false)
    }
}

/// Individual search result
#[derive(Debug, Deserialize, Clone)]
struct ThreatSearchResult {
    source: Option<String>,
    reference: Option<String>,
    date: Option<String>,
    #[serde(rename = "leakFormat")]
    leak_format: Option<String>,
    #[serde(rename = "passwordType")]
    password_type: Option<String>,
    #[serde(rename = "accessUrl")]
    access_url: Option<String>,
    #[serde(rename = "sourceName")]
    source_name: Option<String>,
    platform: Option<String>,
}

/// Start a threat hunting search (async)
async fn start_threat_search(
    client: &reqwest::Client,
    auth: &str,
    query: &str,
    source: ThreatHuntingSource,
    from: &str,
    to: &str,
) -> Result<Option<String>> {
    let url = format!("{}/threat-hunting-api/external-search", API_URL);

    let request = ThreatSearchRequest {
        query: query.to_string(),
        source: source.as_str().to_string(),
        date_range: Some(DateRange {
            from: from.to_string(),
            to: to.to_string(),
        }),
    };

    let resp = client
        .post(&url)
        .header("Authorization", auth)
        .json(&request)
        .send()
        .await?;

    if !resp.status().is_success() {
        tracing::warn!(
            "Threat hunting search failed for source {:?}: {}",
            source,
            resp.status()
        );
        return Ok(None);
    }

    let response: ThreatSearchStartResponse = resp.json().await?;
    Ok(response.get_id().map(|s| s.to_string()))
}

/// Poll for search results (async with timeout)
async fn poll_threat_search(
    client: &reqwest::Client,
    auth: &str,
    search_id: &str,
    max_attempts: u32,
) -> Result<Vec<ThreatSearchResult>> {
    let url = format!(
        "{}/threat-hunting-api/external-search/{}",
        API_URL, search_id
    );

    for attempt in 0..max_attempts {
        let resp = client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await?;

        if !resp.status().is_success() {
            tracing::warn!("Poll attempt {} failed: {}", attempt, resp.status());
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            continue;
        }

        let response: ThreatSearchResultsResponse = resp.json().await?;

        if response.is_complete() {
            return Ok(response.get_results().to_vec());
        }

        // Wait before next poll
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }

    tracing::warn!(
        "Threat search {} timed out after {} attempts",
        search_id,
        max_attempts
    );
    Ok(vec![])
}

/// Fetch and aggregate all threat intelligence data
pub async fn fetch_threat_intelligence(
    client: &reqwest::Client,
    auth: &str,
    query: &str, // Brand domain or name to search
    from: &str,
    to: &str,
) -> ThreatIntelligence {
    let mut intel = ThreatIntelligence::default();

    // Start all searches in parallel
    let sources = [
        ThreatHuntingSource::ForumMessage,
        ThreatHuntingSource::ChatMessage,
        ThreatHuntingSource::SocialMediaPosts,
        ThreatHuntingSource::Credential,
        ThreatHuntingSource::SignalLakeAds,
    ];

    // Start searches
    let mut search_ids: Vec<(ThreatHuntingSource, Option<String>)> = vec![];

    for source in sources {
        let id = start_threat_search(client, auth, query, source, from, to)
            .await
            .ok()
            .flatten();
        search_ids.push((source, id));
    }

    // Poll for results (with timeout of 10 attempts = ~30 seconds each)
    for (source, search_id) in search_ids {
        let Some(id) = search_id else { continue };

        let results = poll_threat_search(client, auth, &id, 10)
            .await
            .unwrap_or_default();

        // Process results based on source type
        match source {
            ThreatHuntingSource::ForumMessage => {
                intel.dark_web_mentions = results.len() as u64;

                // Find earliest date
                let mut earliest: Option<String> = None;
                for r in &results {
                    if let Some(date) = &r.date {
                        if earliest.as_ref().map(|e| date < e).unwrap_or(true) {
                            earliest = Some(date.clone());
                        }
                    }
                }
                intel.earliest_dark_web_date = earliest;

                // Collect unique sources
                let sources: std::collections::HashSet<_> = results
                    .iter()
                    .filter_map(|r| r.source_name.clone().or(r.source.clone()))
                    .collect();
                intel.dark_web_sources = sources.into_iter().collect();
            }

            ThreatHuntingSource::ChatMessage => {
                intel.chat_group_shares = results.len() as u64;

                // Collect platforms
                let platforms: std::collections::HashSet<_> =
                    results.iter().filter_map(|r| r.platform.clone()).collect();
                intel.platforms_detected.extend(platforms);
            }

            ThreatHuntingSource::SocialMediaPosts => {
                intel.social_media_mentions = results.len() as u64;

                // Add social platforms
                let platforms: std::collections::HashSet<_> =
                    results.iter().filter_map(|r| r.platform.clone()).collect();
                intel.platforms_detected.extend(platforms);
            }

            ThreatHuntingSource::Credential => {
                intel.total_credentials = results.len() as u64;

                for r in &results {
                    // Classify by leak format
                    if let Some(format) = &r.leak_format {
                        let format_lower = format.to_lowercase();
                        if format_lower.contains("stealer") {
                            intel.stealer_log_count += 1;
                        } else if format_lower.contains("combo") {
                            intel.combolist_count += 1;
                        }
                    }

                    // Classify by password type
                    if let Some(pwd_type) = &r.password_type {
                        let type_lower = pwd_type.to_lowercase();
                        if type_lower.contains("plain") {
                            intel.plain_password_count += 1;
                        } else {
                            intel.hashed_password_count += 1;
                        }
                    }

                    // Collect access URLs
                    if let Some(url) = &r.access_url {
                        if intel.top_access_urls.len() < 5 && !intel.top_access_urls.contains(url) {
                            intel.top_access_urls.push(url.clone());
                        }
                    }
                }

                // Compute percentages
                if intel.total_credentials > 0 {
                    intel.stealer_log_percent =
                        (intel.stealer_log_count as f64 / intel.total_credentials as f64) * 100.0;
                    intel.plain_password_percent = (intel.plain_password_count as f64
                        / intel.total_credentials as f64)
                        * 100.0;
                }
            }

            ThreatHuntingSource::SignalLakeAds => {
                intel.paid_ads_detected = results.len() as u64;

                // Collect ad platforms
                let platforms: std::collections::HashSet<_> =
                    results.iter().filter_map(|r| r.platform.clone()).collect();
                intel.ad_platforms = platforms.into_iter().collect();
            }
            _ => {} // Ignore other sources for deep investigation for now
        }
    }

    // Compute days before public (if we have dark web data and incident data)
    // This would require comparing earliest_dark_web_date with earliest public incident
    // For now, we'll leave this as None unless we can compute it

    intel.data_available = intel.dark_web_mentions > 0
        || intel.chat_group_shares > 0
        || intel.social_media_mentions > 0
        || intel.total_credentials > 0
        || intel.paid_ads_detected > 0;

    intel
}

// ========================
// SIGNAL-LAKE DEEP INVESTIGATION
// ========================

/// Maximum number of tickets to investigate (to preserve API credits)
const MAX_TICKETS_TO_INVESTIGATE: usize = 10;

/// Poll interval for signal-lake search results
const SIGNAL_LAKE_POLL_INTERVAL_MS: u64 = 2000;

/// Maximum poll attempts before giving up
const SIGNAL_LAKE_MAX_POLLS: u32 = 30;

/// Start a signal-lake search for a domain
async fn start_signal_lake_search(
    client: &reqwest::Client,
    auth: &str,
    tenant_id: &str,
    domain: &str,
) -> Result<String> {
    let url = format!("{}/threat-hunting-api/external-search", API_URL);

    let request = SignalLakeSearchRequest {
        query: format!("domain=\"{}\"", domain),
        source: "signal-lake".to_string(),
        customer: Some(tenant_id.to_string()),
    };

    let resp = client
        .post(&url)
        .header("Authorization", auth)
        .json(&request)
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Signal-lake search init failed: {}", resp.status());
    }

    let data: SignalLakeSearchInitResponse = resp.json().await?;

    data.search_id
        .ok_or_else(|| anyhow::anyhow!("No searchId in response"))
}

/// Poll signal-lake for search results
async fn poll_signal_lake_results(
    client: &reqwest::Client,
    auth: &str,
    search_id: &str,
) -> Result<SignalLakePollResponse> {
    let url = format!(
        "{}/threat-hunting-api/external-search/{}?page=1",
        API_URL, search_id
    );

    let resp = client
        .get(&url)
        .header("Authorization", auth)
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Signal-lake poll failed: {}", resp.status());
    }

    let data: SignalLakePollResponse = resp.json().await?;
    Ok(data)
}

/// Investigate a single ticket using signal-lake
async fn investigate_ticket(
    client: &reqwest::Client,
    auth: &str,
    tenant_id: &str,
    ticket: &StoryTicket,
) -> Option<DeepInvestigationResult> {
    // Extract domain from target
    let domain = &ticket.target;
    if domain.is_empty() {
        return None;
    }

    // Start the search
    let search_id = match start_signal_lake_search(client, auth, tenant_id, domain).await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to start signal-lake search for {}: {}", domain, e);
            return None;
        }
    };

    // Poll for results
    let mut attempts = 0;
    let mut result_data: Vec<SignalLakeDataItem> = Vec::new();
    let mut total_signals = 0u64;

    loop {
        tokio::time::sleep(std::time::Duration::from_millis(
            SIGNAL_LAKE_POLL_INTERVAL_MS,
        ))
        .await;
        attempts += 1;

        match poll_signal_lake_results(client, auth, &search_id).await {
            Ok(response) => {
                // Check running status from nested result object
                let running = response
                    .result
                    .as_ref()
                    .and_then(|r| r.status.as_ref())
                    .and_then(|s| s.running)
                    .unwrap_or(true); // Default to running if missing

                if !running {
                    if let Some(result) = response.result {
                        if let Some(data) = result.data {
                            result_data = data;
                        }
                        if let Some(pagination) = result.pagination {
                            total_signals = pagination.total.unwrap_or(0);
                        }
                    }
                    break;
                }
                // Still processing, continue polling
                // Still processing, continue polling
            }
            Err(e) => {
                eprintln!("Error polling signal-lake for {}: {}", domain, e);
                if attempts >= SIGNAL_LAKE_MAX_POLLS {
                    return None;
                }
            }
        }

        if attempts >= SIGNAL_LAKE_MAX_POLLS {
            eprintln!("Signal-lake polling timeout for {}", domain);
            return None;
        }
    }

    // Process results into DeepInvestigationResult
    let mut related_urls: Vec<String> = Vec::new();
    let mut detection_dates: Vec<String> = Vec::new();
    let mut related_domains: HashSet<String> = HashSet::new();
    let mut first_seen: Option<String> = None;
    let mut last_seen: Option<String> = None;
    let mut infra = InfrastructureInfo::default();

    for item in &result_data {
        if let Some(url) = &item.url {
            if !related_urls.contains(url) && related_urls.len() < 10 {
                related_urls.push(url.clone());
            }
        }
        if let Some(date) = &item.detection_date {
            if !detection_dates.contains(date) {
                detection_dates.push(date.clone());
            }
        }
        if let Some(d) = &item.domain {
            if d != domain {
                related_domains.insert(d.clone());
            }
        }
        // Get infrastructure info from first item
        if infra.ip.is_none() {
            infra.ip = item.ip.clone();
            infra.asn = item.asn.clone();
            infra.hosting_provider = item.hosting_provider.clone();
            infra.country = item.country.clone();
        }
        // Track first/last seen
        if first_seen.is_none() {
            first_seen = item.first_seen.clone();
        }
        last_seen = item.last_seen.clone().or(last_seen);
    }

    infra.related_domains = related_domains.into_iter().collect();

    // Determine if mass campaign (>5 related domains or >20 signals)
    let is_mass_campaign = infra.related_domains.len() > 5 || total_signals > 20;

    // Extract enrichment data from the first item that has it (prioritize items with screenshots)
    let enrichment = extract_enrichment_data(&result_data);

    Some(DeepInvestigationResult {
        ticket_key: ticket.ticket_key.clone(),
        target: domain.clone(),
        status: ticket.status.clone(),
        threat_type: ticket.threat_type.clone(),
        related_urls,
        detection_dates,
        infrastructure: infra,
        is_mass_campaign,
        signal_count: total_signals,
        first_seen,
        last_seen,
        enrichment,
    })
}

/// Extract enrichment data from Signal-Lake results
/// Prioritizes items with screenshots and AI inspection data
fn extract_enrichment_data(items: &[SignalLakeDataItem]) -> EnrichedSignalData {
    let mut enrichment = EnrichedSignalData::default();

    // Find the "richest" item (prefer ones with screenshots and AI data)
    let best_item = items
        .iter()
        .filter(|item| item.screenshot_url.is_some() || item.ai_content_type.is_some())
        .next()
        .or_else(|| items.first());

    if let Some(item) = best_item {
        // Screenshot URL (will be downloaded later)
        enrichment.screenshot_url = item.screenshot_url.clone();

        // AI Inspection data
        enrichment.ai_content_type = item.ai_content_type.clone();
        enrichment.ai_image_description = item.ai_image_description.clone();
        enrichment.credential_requested = item
            .credential_requested
            .as_ref()
            .map(|v| v.eq_ignore_ascii_case("yes"))
            .unwrap_or(false);
        enrichment.payment_requested = item
            .payment_requested
            .as_ref()
            .map(|v| v.eq_ignore_ascii_case("yes") || v.eq_ignore_ascii_case("possibly"))
            .unwrap_or(false);
        enrichment.predominant_language = item.predominant_language.clone();
        enrichment.company_logos = item.company_logos.clone().unwrap_or_default();

        // Impersonated brands
        let mut brands = Vec::new();
        if let Some(high) = &item.impersonated_brands_high {
            for brand in high {
                brands.push(ImpersonatedBrand {
                    brand: brand.clone(),
                    level: "high".to_string(),
                    explanation: None,
                });
            }
        }
        if let Some(medium) = &item.impersonated_brands_medium {
            for brand in medium {
                brands.push(ImpersonatedBrand {
                    brand: brand.clone(),
                    level: "medium".to_string(),
                    explanation: None,
                });
            }
        }
        if let Some(low) = &item.impersonated_brands_low {
            for brand in low {
                brands.push(ImpersonatedBrand {
                    brand: brand.clone(),
                    level: "low".to_string(),
                    explanation: None,
                });
            }
        }
        enrichment.impersonated_brands = brands;

        // Site scanner data
        enrichment.http_status = item.http_status;

        // Geolocation
        if item.country_codes.is_some() || item.isps.is_some() {
            enrichment.geolocation = Some(GeoInfo {
                ip: item.ip.clone(),
                country_code: item.country_codes.as_ref().and_then(|v| v.first().cloned()),
                country_name: item.country_names.as_ref().and_then(|v| v.first().cloned()),
                isp: item.isps.as_ref().and_then(|v| v.first().cloned()),
                latitude: None,
                longitude: None,
            });
        }

        // Detection date (convert from timestamp)
        if let Some(ts) = item.hit_detection_date {
            enrichment.detection_date = Some(format_unix_timestamp_ms(ts));
        }

        // Domain created date (from WHOIS)
        if let Some(ts) = item.domain_created_ts {
            enrichment.domain_created = Some(format_unix_timestamp_ms(ts));
        }

        // Registrar
        enrichment.registrar = item.registrar_name.clone();
    }

    enrichment
}

/// Format a Unix timestamp (milliseconds) to a human-readable date string
fn format_unix_timestamp_ms(timestamp_ms: i64) -> String {
    use chrono::{TimeZone, Utc};
    let secs = timestamp_ms / 1000;
    match Utc.timestamp_opt(secs, 0) {
        chrono::LocalResult::Single(dt) => dt.format("%b %d, %Y").to_string(),
        _ => "N/A".to_string(),
    }
}

/// Investigate multiple tagged tickets using signal-lake
/// Returns enriched investigation results for each ticket
pub async fn investigate_tagged_tickets(
    client: &reqwest::Client,
    auth: &str,
    tenant_id: &str,
    tickets: &[StoryTicket],
) -> Vec<DeepInvestigationResult> {
    let mut results = Vec::new();

    // Limit to prevent excessive API usage
    let tickets_to_process = tickets.iter().take(MAX_TICKETS_TO_INVESTIGATE);

    for ticket in tickets_to_process {
        if let Some(result) = investigate_ticket(client, auth, tenant_id, ticket).await {
            results.push(result);
        }
    }

    // Download screenshots and convert to base64 for HTML embedding
    for result in &mut results {
        if let Some(screenshot_url) = &result.enrichment.screenshot_url {
            if let Some(base64) = download_image_as_base64(client, auth, screenshot_url).await {
                result.enrichment.screenshot_base64 = Some(base64);
            }
        }
    }

    results
}

// ========================
// CREDENTIAL API INTEGRATION
// ========================

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CredentialExposure {
    pub user: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "access.url")]
    pub access_url: Option<String>,
    #[serde(rename = "access.domain")]
    pub access_domain: Option<String>,
    #[serde(rename = "leak.displayName")]
    pub leak_name: Option<String>,
    #[serde(rename = "source.timestamp")]
    pub leak_date: Option<String>,
    #[serde(rename = "password.length")]
    pub password_length: Option<u32>,
    #[serde(rename = "password.hasLetter")]
    pub has_letter: Option<bool>,
    #[serde(rename = "password.hasNumber")]
    pub has_number: Option<bool>,
    #[serde(rename = "password.hasSpecialChar")]
    pub has_special: Option<bool>,
    #[serde(rename = "password.hasUpperCase")]
    pub has_uppercase: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct CredentialSearchResponse {
    pub detections: Option<Vec<CredentialExposure>>,
    pub pageable: Option<CredentialPageable>,
}

#[derive(Debug, Deserialize)]
struct CredentialPageable {
    pub total: Option<u64>,
}

/// Fetch credentials filtered by a specific tag
pub async fn fetch_tagged_credentials(
    client: &reqwest::Client,
    auth: &str,
    tenant_id: &str,
    tag: &str,
) -> Vec<CredentialExposure> {
    if tag.is_empty() {
        return Vec::new();
    }

    // Assuming tags={tag} works based on user requirement.
    // We request 50 items for the slide.
    let url = format!(
        "https://api.axur.com/gateway/1.0/api/exposure-api/credentials?tags={}&customer={}&pageSize=50",
        tag,
        tenant_id
    );

    tracing::info!("Fetching credentials for tag: {} (URL: {})", tag, url);

    let resp = match client.get(&url).header("Authorization", auth).send().await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to fetch credentials: {}", e);
            return Vec::new();
        }
    };

    if !resp.status().is_success() {
        tracing::error!("Credential API error: {}", resp.status());
        return Vec::new();
    }

    match resp.json::<CredentialSearchResponse>().await {
        Ok(body) => {
            let count = body.pageable.as_ref().and_then(|p| p.total).unwrap_or(0);
            tracing::info!("Found {} credentials for tag '{}'", count, tag);
            body.detections.unwrap_or_default()
        }
        Err(e) => {
            tracing::error!("Failed to parse credential response: {}", e);
            Vec::new()
        }
    }
}

// ========================
// THREAT HUNTING PREVIEW (Multi-Source)
// ========================

/// Start a search on any Threat Hunting source
async fn start_threat_hunting_search(
    client: &reqwest::Client,
    auth: &str,
    customer: Option<&str>,
    query: &str,
    source: ThreatHuntingSource,
) -> Result<String> {
    let url = format!("{}/threat-hunting-api/external-search", API_URL);

    let request = SignalLakeSearchRequest {
        query: query.to_string(),
        source: source.as_str().to_string(),
        customer: customer.map(|s| s.to_string()),
    };

    // Debug logging - save request
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let debug_dir = std::path::Path::new("debug_logs");
    if debug_dir.exists() {
        let req_file = debug_dir.join(format!(
            "th_post_request_{}_{}.json",
            source.as_str(),
            timestamp
        ));
        if let Ok(req_json) = serde_json::to_string_pretty(&request) {
            let _ = std::fs::write(&req_file, &req_json);
            tracing::info!("DEBUG: Saved POST request to {:?}", req_file);
        }
    }

    // Retry logic with exponential backoff for 429 errors
    let mut retry_count = 0;
    let max_retries = 3;
    let mut wait_time = std::time::Duration::from_secs(2);

    loop {
        let resp = client
            .post(&url)
            .header("Authorization", auth)
            .json(&request)
            .send()
            .await?;

        let status = resp.status();
        let resp_text = resp.text().await?;

        // Debug logging - save response
        if debug_dir.exists() {
            let resp_file = debug_dir.join(format!(
                "th_post_response_{}_{}.json",
                source.as_str(),
                timestamp
            ));
            let log_content = format!(
                "HTTP Status: {}\nURL: {}\nRetry: {}\n\nResponse Body:\n{}",
                status, url, retry_count, resp_text
            );
            let _ = std::fs::write(&resp_file, &log_content);
            tracing::info!("DEBUG: Saved POST response to {:?}", resp_file);
        }

        // Handle rate limiting (429)
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            if retry_count >= max_retries {
                tracing::error!(
                    "Rate limited after {} retries for {}",
                    max_retries,
                    source.as_str()
                );
                anyhow::bail!("Rate limited (429) after {} retries", max_retries);
            }
            tracing::warn!(
                "Rate limited (429) for {}, waiting {:?} before retry {}/{}",
                source.as_str(),
                wait_time,
                retry_count + 1,
                max_retries
            );
            tokio::time::sleep(wait_time).await;
            wait_time *= 2; // Exponential backoff
            retry_count += 1;
            continue;
        }

        if !status.is_success() {
            anyhow::bail!(
                "Threat Hunting search init failed for {}: {} - {}",
                source.as_str(),
                status,
                resp_text
            );
        }

        let data: SignalLakeSearchInitResponse = serde_json::from_str(&resp_text)?;
        return data
            .search_id
            .ok_or_else(|| anyhow::anyhow!("No searchId in response"));
    }
}

/// Poll and get count from any source (without consuming too many credits)
async fn poll_threat_hunting_count(
    client: &reqwest::Client,
    auth: &str,
    search_id: &str,
) -> Result<(u64, Vec<String>)> {
    // Just get first page for count and samples
    let url = format!(
        "{}/threat-hunting-api/external-search/{}?page=1",
        API_URL, search_id
    );

    let debug_dir = std::path::Path::new("debug_logs");
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");

    // Debug logging - save GET request info
    if debug_dir.exists() {
        let req_file = debug_dir.join(format!("th_get_request_{}_{}.txt", search_id, timestamp));
        let req_content = format!("GET {}\nSearch ID: {}\n", url, search_id);
        let _ = std::fs::write(&req_file, &req_content);
        tracing::info!("DEBUG: Saved GET request to {:?}", req_file);
    }

    let mut attempts = 0;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        attempts += 1;

        let resp = client
            .get(&url)
            .header("Authorization", auth)
            .send()
            .await?;

        let status_code = resp.status();
        let resp_text = resp.text().await?;

        // Debug logging - save GET response
        if debug_dir.exists() {
            let resp_file = debug_dir.join(format!(
                "th_get_response_{}_{}_attempt{}.json",
                search_id, timestamp, attempts
            ));
            let log_content = format!(
                "HTTP Status: {}\nURL: {}\nAttempt: {}\n\nResponse Body:\n{}",
                status_code, url, attempts, resp_text
            );
            let _ = std::fs::write(&resp_file, &log_content);
            tracing::info!(
                "DEBUG: Saved GET response attempt {} to {:?}",
                attempts,
                resp_file
            );
        }

        if !status_code.is_success() {
            if attempts >= 20 {
                return Ok((0, vec![]));
            }
            continue;
        }

        let json: serde_json::Value = serde_json::from_str(&resp_text)?;

        // Check running status from nested result object
        let running = json
            .get("result")
            .and_then(|r| r.get("status"))
            .and_then(|s| s.get("running"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true); // Default to running if missing

        if !running {
            let total = json
                .get("result")
                .and_then(|r| r.get("pagination"))
                .and_then(|p| p.get("total"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            // Extract samples from first few items
            let samples: Vec<String> = json
                .get("result")
                .and_then(|r| r.get("data"))
                .and_then(|d| d.as_array())
                .map(|items| {
                    items
                        .iter()
                        .take(3)
                        .filter_map(|item| {
                            item.get("url")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                                .or_else(|| {
                                    item.get("domain")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                })
                                .or_else(|| {
                                    item.get("ip")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                })
                                .or_else(|| {
                                    item.get("name")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                })
                                .or_else(|| {
                                    item.get("accessUrl") // Check camelCase first
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                })
                                .or_else(|| {
                                    item.get("access_url") // Check snake_case just in case
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                })
                        })
                        .collect()
                })
                .unwrap_or_default();

            return Ok((total, samples));
        }

        if attempts >= 20 {
            return Ok((0, vec![]));
        }
    }
}

/// Preview Threat Hunting results without consuming full credits
/// Only fetches page 1 of each source to get counts
pub async fn preview_threat_hunting(
    token: &str,
    tenant_id: &str,
    tickets: &[StoryTicket],
    story_tag: &str,
    use_user_credits: bool,
) -> Result<ThreatHuntingPreview> {
    let client = create_client()?;
    let auth = format!("Bearer {}", token);

    let mut preview = ThreatHuntingPreview {
        tickets_count: tickets.len().min(MAX_TICKETS_TO_INVESTIGATE),
        ..Default::default()
    };

    // 1. Fetch Tagged Credentials using Exposure API (correct approach)
    // The Threat Hunting API does NOT support tag: queries for credentials
    // Must use /exposure-api/credentials?tags=contains:{tag} instead
    let mut search_ids: Vec<(ThreatHuntingSource, String, String)> = Vec::new();

    if !story_tag.is_empty() {
        tracing::info!(
            "Fetching credentials tagged '{}' from Exposure API",
            story_tag
        );

        // Build Exposure API URL with tags filter
        let customer_param = if tenant_id.is_empty() || tenant_id.eq_ignore_ascii_case("default") {
            String::new()
        } else {
            format!("&customer={}", tenant_id)
        };

        let exposure_url = format!(
            "{}/exposure-api/credentials?tags=contains:{}&pageSize=100&sortBy=created&order=desc{}",
            API_URL, story_tag, customer_param
        );

        tracing::debug!("Exposure API URL: {}", exposure_url);

        match client
            .get(&exposure_url)
            .header("Authorization", &auth)
            .send()
            .await
        {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    if let Ok(body) = resp.text().await {
                        // Log the response for debugging
                        let debug_dir = std::path::Path::new("debug_logs");
                        if debug_dir.exists() {
                            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                            let debug_file =
                                debug_dir.join(format!("exposure_credentials_{}.json", timestamp));
                            let _ = std::fs::write(&debug_file, &body);
                        }

                        // Parse the response
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                            // Extract detections array
                            if let Some(detections) =
                                json.get("detections").and_then(|d| d.as_array())
                            {
                                preview.credential_count = detections.len() as u64;

                                // Also check pageable.total for full count
                                if let Some(pageable) = json.get("pageable") {
                                    if let Some(total) =
                                        pageable.get("total").and_then(|t| t.as_u64())
                                    {
                                        preview.credential_count = total;
                                    }
                                }

                                // Extract samples for preview
                                for detection in detections.iter().take(5) {
                                    let user = detection
                                        .get("user")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    let access_url = detection
                                        .get("access.url")
                                        .or_else(|| detection.get("accessUrl"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    let domain = detection
                                        .get("user.emailDomain")
                                        .or_else(|| detection.get("access.domain"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    let source_name = detection
                                        .get("source.name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string();
                                    // Push a formatted string sample
                                    let sample = if !access_url.is_empty() {
                                        format!("{} @ {}", user, access_url)
                                    } else if !domain.is_empty() {
                                        format!("{} ({})", user, domain)
                                    } else {
                                        user.clone()
                                    };

                                    if !sample.is_empty() {
                                        preview.samples.credentials.push(sample);
                                    }
                                }

                                tracing::info!(
                                    "Found {} credentials tagged '{}' via Exposure API",
                                    preview.credential_count,
                                    story_tag
                                );
                            }
                        }
                    }
                } else {
                    tracing::warn!("Exposure API returned status {}", status);
                }
            }
            Err(e) => {
                tracing::error!("Failed to fetch credentials from Exposure API: {}", e);
            }
        }
    }

    // For now, only search phishing domains in Signal-Lake
    // Credentials will use a separate API (/exposure-api/credentials)
    let unique_domains: Vec<String> = {
        let mut domains: std::collections::HashSet<String> = std::collections::HashSet::new();
        for ticket in tickets {
            if !ticket.target.is_empty() {
                domains.insert(ticket.target.clone());
            }
        }
        domains.into_iter().take(5).collect() // MAX 5 domains for rate limit
    };

    // Debug log domains
    let debug_dir = std::path::Path::new("debug_logs");
    if debug_dir.exists() {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let debug_file = debug_dir.join(format!("phishing_domains_{}.txt", timestamp));
        let content = format!(
            "Phishing domains for Signal-Lake search:\n{:#?}\n\nTotal: {}\nTickets: {}\n",
            unique_domains,
            unique_domains.len(),
            tickets.len()
        );
        let _ = std::fs::write(&debug_file, &content);
    }

    if unique_domains.is_empty() && search_ids.is_empty() {
        return Ok(preview);
    }

    tracing::info!(
        "Searching {} phishing domains in Signal-Lake only",
        unique_domains.len()
    );

    // Search only in Signal-Lake for phishing domains
    // search_ids initialized above

    let search_customer = if use_user_credits {
        None
    } else {
        Some(tenant_id)
    };

    for domain in &unique_domains {
        // No quotes around domain value - matches Axur web UI format
        let query = format!("domain=\"{}\"", domain);

        match start_threat_hunting_search(
            &client,
            &auth,
            search_customer,
            &query,
            ThreatHuntingSource::SignalLake,
        )
        .await
        {
            Ok(id) => {
                tracing::info!(
                    "Started Signal-Lake search for phishing domain '{}': {}",
                    domain,
                    id
                );
                search_ids.push((ThreatHuntingSource::SignalLake, id, domain.clone()));
            }
            Err(e) => {
                tracing::warn!("Failed to start Signal-Lake search for '{}': {}", domain, e);
            }
        }

        // RATE LIMIT PROTECTION: Wait 1 second between each API call
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    // Poll each search for counts (aggregate results)
    for (source, search_id, domain) in search_ids {
        match poll_threat_hunting_count(&client, &auth, &search_id).await {
            Ok((count, samples)) => {
                tracing::info!(
                    "Got {} results for {} on domain '{}'",
                    count,
                    source.as_str(),
                    domain
                );
                match source {
                    ThreatHuntingSource::SignalLake => {
                        preview.signal_lake_count += count;
                        if preview.samples.signal_lake.len() < 3 {
                            preview.samples.signal_lake.extend(
                                samples
                                    .into_iter()
                                    .take(3 - preview.samples.signal_lake.len()),
                            );
                        }
                    }
                    ThreatHuntingSource::Credential => {
                        preview.credential_count = count;
                        preview.samples.credentials = samples;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to poll {} results for '{}': {}",
                    source.as_str(),
                    domain,
                    e
                );
            }
        }
    }

    // Calculate totals
    preview.total_count = preview.signal_lake_count
        + preview.credential_count
        + preview.chat_message_count
        + preview.forum_message_count;

    preview.compute_estimated_credits();

    Ok(preview)
}

/// Public function for SSE streaming - starts a TH search and polls for count
/// This is a simplified version for the streaming endpoint
pub async fn start_and_poll_th_search(
    client: &reqwest::Client,
    auth: &str,
    customer: Option<&str>,
    query: &str,
    source_str: &str,
) -> Result<u64> {
    // Map source string to enum
    let source = match source_str {
        "credential" => ThreatHuntingSource::Credential,
        "signal-lake" => ThreatHuntingSource::SignalLake,
        "chat-message" => ThreatHuntingSource::ChatMessage,
        "forum-message" => ThreatHuntingSource::ForumMessage,
        _ => ThreatHuntingSource::SignalLake,
    };

    // Start the search
    let search_id = start_threat_hunting_search(client, auth, customer, query, source).await?;

    // Poll for results
    let (count, _samples) = poll_threat_hunting_count(client, auth, &search_id).await?;

    Ok(count)
}
