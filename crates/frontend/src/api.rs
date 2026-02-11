//! API client for communicating with backend
// Force rebuild for env var

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

// Use environment variable at compile time
// - Development: Set API_BASE_URL=http://localhost:3001
// - Production: Use empty string for relative URLs (Cloudflare proxies /api/* to Leapcell)
const API_BASE: &str = match option_env!("API_BASE_URL") {
    Some(url) => url,
    None => "", // Empty = relative URLs, works with Cloudflare proxy
};

// ========================
// REQUEST/RESPONSE TYPES
// ========================

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LoginResponse {
    pub success: bool,
    pub requires_2fa: bool,
    pub token: Option<String>,
    pub correlation: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TwoFactorRequest {
    pub code: String,
    pub token: String,
    pub correlation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TwoFactorResponse {
    pub success: bool,
    pub token: Option<String>,
    pub correlation: Option<String>,
    pub device_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct FinalizeRequest {
    pub email: String,
    pub password: String,
    pub token: String,
    pub correlation: Option<String>,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FinalizeResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ValidateResponse {
    pub valid: bool,
    pub message: String,
    pub is_admin: bool,
    pub has_log_access: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Tenant {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct GenerateReportRequest {
    pub tenant_id: String,
    pub from_date: String,
    pub to_date: String,
    pub language: String,
    pub story_tag: Option<String>,
    pub include_threat_intel: bool,
    pub template_id: Option<String>,
    pub format: Option<String>,
    /// Use the new plugin-based report generation
    #[serde(default)]
    pub use_plugins: bool,
    /// Theme mode: "dark", "light", or "auto"
    #[serde(default)]
    pub theme: Option<String>,
    /// List of plugin IDs to disable
    #[serde(default)]
    pub disabled_plugins: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct GenerateReportResponse {
    pub success: bool,
    pub html: Option<String>,
    pub pptx_base64: Option<String>,
    pub company_name: Option<String>,
    pub message: String,
    /// Structured error code (e.g., "TI-001", "API-002")
    pub error_code: Option<String>,
    /// User-friendly error message
    pub error_message: Option<String>,
}

// ========================
// API FUNCTIONS
// ========================

/// Step 1: Login with email/password
pub async fn login(email: &str, password: &str) -> Result<LoginResponse, String> {
    let url = format!("{}/api/auth/login", API_BASE);
    let resp = Request::post(&url)
        .header("Content-Type", "application/json")
        .json(&LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let text = resp.text().await.map_err(|e| e.to_string())?;
        serde_json::from_str(&text).map_err(|e| e.to_string())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        // Try to parse JSON error message
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message").and_then(|v| v.as_str()) {
                return Err(msg.to_string());
            }
            if let Some(err) = json.get("error").and_then(|v| v.as_str()) {
                return Err(err.to_string());
            }
        }
        Err(format!("Login failed: {} - {}", status, text))
    }
}

/// Step 2: Verify 2FA code
pub async fn verify_2fa(
    code: &str,
    token: &str,
    correlation: Option<String>,
) -> Result<TwoFactorResponse, String> {
    let resp = Request::post(&format!("{}/api/auth/2fa", API_BASE))
        .header("Content-Type", "application/json")
        .json(&TwoFactorRequest {
            code: code.to_string(),
            token: token.to_string(),
            correlation,
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("2FA failed: {}", resp.status()))
    }
}

/// Step 3: Finalize login
pub async fn finalize(
    email: &str,
    password: &str,
    token: &str,
    correlation: Option<String>,
    device_id: &str,
) -> Result<FinalizeResponse, String> {
    let resp = Request::post(&format!("{}/api/auth/finalize", API_BASE))
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(&FinalizeRequest {
            email: email.to_string(),
            password: password.to_string(),
            token: token.to_string(),
            correlation,
            device_id: device_id.to_string(),
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Finalize failed: {}", resp.status()))
    }
}

/// Validate current session
pub async fn validate_session() -> Result<ValidateResponse, String> {
    let resp = Request::get(&format!("{}/api/auth/validate", API_BASE))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Ok(ValidateResponse {
            valid: false,
            message: "Session invalid".into(),
            is_admin: false,
            has_log_access: false,
        })
    }
}

/// Logout
pub async fn logout() -> Result<(), String> {
    Request::post(&format!("{}/api/auth/logout", API_BASE))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Health check response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct HealthCheckResponse {
    pub status: String,
}

/// Health check - used to detect cold starts
/// Returns elapsed time in milliseconds
pub async fn health_check() -> Result<f64, String> {
    // Use JS Date for timing (WASM compatible)
    let start = js_sys::Date::now();

    let resp = Request::get(&format!("{}/api/health", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let elapsed = js_sys::Date::now() - start;

    if resp.ok() {
        Ok(elapsed)
    } else {
        Err(format!("Health check failed: {}", resp.status()))
    }
}

/// List available tenants
pub async fn list_tenants() -> Result<Vec<Tenant>, String> {
    let resp = Request::get(&format!("{}/api/tenants", API_BASE))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to fetch tenants: {}", resp.status()))
    }
}

/// Generate report
#[allow(dead_code)]
pub async fn generate_report(
    tenant_id: &str,
    from_date: &str,
    to_date: &str,
    language: &str,
    story_tag: Option<String>,
    include_threat_intel: bool,
    template_id: Option<String>,
    format: Option<String>,
    use_plugins: bool,
    theme: Option<String>,
    disabled_plugins: Option<Vec<String>>,
) -> Result<GenerateReportResponse, String> {
    let resp = Request::post(&format!("{}/api/report/generate", API_BASE))
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(&GenerateReportRequest {
            tenant_id: tenant_id.to_string(),
            from_date: from_date.to_string(),
            to_date: to_date.to_string(),
            language: language.to_string(),
            story_tag,
            include_threat_intel,
            template_id,
            format,
            use_plugins,
            theme,
            disabled_plugins,
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to generate report: {}", resp.status()))
    }
}

// ========================
// THREAT HUNTING PREVIEW
// ========================

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct ThreatHuntingPreviewRequest {
    pub tenant_id: String,
    pub story_tag: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[allow(dead_code)]
pub struct ThreatHuntingPreviewResponse {
    pub success: bool,
    pub preview: Option<ThreatHuntingPreview>,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[allow(dead_code)]
pub struct ThreatHuntingPreview {
    pub signal_lake_count: u64,
    pub credential_count: u64,
    pub chat_message_count: u64,
    pub forum_message_count: u64,
    pub total_count: u64,
    pub estimated_credits: u64,
    pub tickets_count: usize,
}

/// Get preview of Threat Hunting results (counts and estimated credits)
#[allow(dead_code)]
pub async fn request_threat_hunting_preview(
    tenant_id: &str,
    story_tag: &str,
) -> Result<ThreatHuntingPreviewResponse, String> {
    let resp = Request::post(&format!("{}/api/threat-hunting/preview", API_BASE))
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(&ThreatHuntingPreviewRequest {
            tenant_id: tenant_id.to_string(),
            story_tag: story_tag.to_string(),
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to get preview: {}", resp.status()))
    }
}

// ========================
// SSE STREAMING TYPES
// ========================

/// SSE Event types from the streaming endpoint
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ThreatHuntingStreamEvent {
    Started {
        total_domains: usize,
        total_tickets: usize,
    },
    DomainProcessing {
        domain: String,
        index: usize,
        source: String,
    },
    DomainComplete {
        domain: String,
        source: String,
        count: u64,
    },
    Finished {
        total_count: u64,
        signal_lake_count: u64,
        chatter_count: u64,
        credential_count: u64,
        estimated_credits: f64,
    },
    Error {
        message: String,
    },
}

/// Get the SSE stream URL for threat hunting preview
pub fn get_threat_hunting_stream_url(
    tenant_id: &str,
    story_tag: &str,
    use_user_credits: bool,
) -> String {
    format!(
        "{}/api/threat-hunting/preview-stream?tenant_id={}&story_tag={}&use_user_credits={}",
        API_BASE,
        urlencoding_encode(tenant_id),
        urlencoding_encode(story_tag),
        use_user_credits
    )
}

/// Simple URL encoding helper
fn urlencoding_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '&' => "%26".to_string(),
            '=' => "%3D".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

// ========================
// SSE REPORT GENERATION TYPES
// ========================

/// SSE Event types from the report generation streaming endpoint
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ReportStreamEvent {
    Started {
        stages: Vec<String>,
    },
    StageProgress {
        stage: String,
        message: String,
        progress_pct: u8,
    },
    StageComplete {
        stage: String,
    },
    Finished {
        html: String,
        company_name: Option<String>,
    },
    Error {
        code: String,
        message: String,
    },
}

/// Get the SSE stream URL for report generation
pub fn get_report_stream_url(
    tenant_id: &str,
    from_date: &str,
    to_date: &str,
    language: &str,
    story_tag: Option<&str>,
    include_threat_intel: bool,
    template_id: Option<&str>,
    use_plugins: bool,
) -> String {
    let mut url = format!(
        "{}/api/reports/generate-stream?tenant_id={}&from_date={}&to_date={}&language={}&include_threat_intel={}&use_plugins={}",
        API_BASE,
        urlencoding_encode(tenant_id),
        urlencoding_encode(from_date),
        urlencoding_encode(to_date),
        urlencoding_encode(language),
        include_threat_intel,
        use_plugins
    );

    if let Some(tag) = story_tag {
        if !tag.is_empty() {
            url.push_str(&format!("&story_tag={}", urlencoding_encode(tag)));
        }
    }

    if let Some(tid) = template_id {
        url.push_str(&format!("&template_id={}", urlencoding_encode(tid)));
    }

    url
}

// ========================
// FEEDBACK API
// ========================

#[derive(Debug, Serialize)]
pub struct FeedbackRequest {
    pub message: String,
    pub screenshot: Option<String>,
    pub url: String,
    pub user_agent: String,
    pub tenant_id: Option<String>,
    pub user_email: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FeedbackResponse {
    pub success: bool,
    pub issue_url: Option<String>,
    pub message: String,
}

/// Helper to submit feedback
pub async fn submit_feedback(
    message: String,
    screenshot: Option<String>,
    url: String,
    user_agent: String,
    tenant_id: Option<String>,
    user_email: Option<String>,
) -> Result<FeedbackResponse, String> {
    let resp = Request::post(&format!("{}/api/feedback", API_BASE))
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(&FeedbackRequest {
            message,
            screenshot,
            url,
            user_agent,
            tenant_id,
            user_email,
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to submit feedback: {}", resp.status()))
    }
}

// ========================
// LOGS API
// ========================

#[derive(Debug, Clone, Deserialize)]
pub struct LogEntry {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub sha: String,
    #[allow(dead_code)]
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListLogsResponse {
    pub success: bool,
    pub files: Vec<LogEntry>,
    pub total: usize,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogContentResponse {
    pub success: bool,
    #[allow(dead_code)]
    pub filename: String,
    pub content: String,
    #[allow(dead_code)]
    pub size: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListCategoriesResponse {
    pub success: bool,
    pub categories: Vec<String>,
    #[allow(dead_code)]
    pub date: Option<String>,
    #[allow(dead_code)]
    pub message: String,
}

/// List log files with optional filters
pub async fn list_logs(
    date: Option<&str>,
    category: Option<&str>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<ListLogsResponse, String> {
    let mut url = format!("{}/api/logs", API_BASE);
    let mut params = Vec::new();

    if let Some(d) = date {
        params.push(format!("date={}", urlencoding_encode(d)));
    }
    if let Some(c) = category {
        params.push(format!("category={}", urlencoding_encode(c)));
    }
    if let Some(l) = limit {
        params.push(format!("limit={}", l));
    }
    if let Some(o) = offset {
        params.push(format!("offset={}", o));
    }

    if !params.is_empty() {
        url = format!("{}?{}", url, params.join("&"));
    }

    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to list logs: {}", resp.status()))
    }
}

/// Get log file content
pub async fn get_log_content(path: &str) -> Result<LogContentResponse, String> {
    let url = format!("{}/api/logs/content/{}", API_BASE, path);

    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to get log content: {}", resp.status()))
    }
}

/// List available log categories for a date
pub async fn list_log_categories(date: Option<&str>) -> Result<ListCategoriesResponse, String> {
    let mut url = format!("{}/api/logs/categories", API_BASE);

    if let Some(d) = date {
        url = format!("{}?date={}", url, urlencoding_encode(d));
    }

    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to list categories: {}", resp.status()))
    }
}

// ========================
// LOG ACCESS CHECK
// ========================

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct LogAccessResponse {
    pub has_access: bool,
    #[allow(dead_code)]
    pub message: String,
}

/// Check if user has access to logs
#[allow(dead_code)]
pub async fn check_log_access(email: &str) -> Result<bool, String> {
    let url = format!(
        "{}/api/logs/access?email={}",
        API_BASE,
        urlencoding_encode(email)
    );

    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let data: LogAccessResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(data.has_access)
    } else {
        // On error, default to no access
        Ok(false)
    }
}

// ========================
// ANALYTICS API
// ========================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub reports: usize,
    pub errors: usize,
    pub th_queries: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub success: bool,
    pub period: String,
    pub total_reports: usize,
    pub total_errors: usize,
    pub daily_stats: Vec<DailyStats>,
    pub message: String,
}

/// Get analytics stats
pub async fn get_log_stats(days: Option<i64>) -> Result<StatsResponse, String> {
    let mut url = format!("{}/api/logs/stats", API_BASE);
    if let Some(d) = days {
        url = format!("{}?days={}", url, d);
    }

    let resp = Request::get(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to get stats: {}", resp.status()))
    }
}

// ========================
// TEMPLATE API
// ========================

#[derive(Debug, Clone, Serialize)]
pub struct SaveTemplateRequest {
    pub name: String,
    pub description: Option<String>,
    pub slides: Vec<serde_json::Value>,
    pub thumbnail: Option<String>, // Base64 encoded image
}

#[derive(Debug, Clone, Deserialize)]
pub struct SaveTemplateResponse {
    pub success: bool,
    pub template_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct TemplateListItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub preview_image_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct ListTemplatesResponse {
    pub success: bool,
    pub templates: Vec<TemplateListItem>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct TemplateDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub slides: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct GetTemplateResponse {
    pub success: bool,
    pub template: Option<TemplateDetail>,
    pub message: String,
}

/// Save a template (create or update)
pub async fn save_template(
    template_id: Option<&str>,
    name: &str,
    description: Option<&str>,
    slides: Vec<serde_json::Value>,
    thumbnail: Option<String>,
    file: Option<web_sys::File>,
) -> Result<SaveTemplateResponse, String> {
    let url = match template_id {
        Some(id) => format!("{}/api/templates/{}", API_BASE, id),
        None => format!("{}/api/templates", API_BASE),
    };

    let method = if template_id.is_some() { "PUT" } else { "POST" };

    let body_struct = SaveTemplateRequest {
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
        slides,
        thumbnail,
    };

    let form_data = web_sys::FormData::new().map_err(|_| "Failed to create FormData")?;

    // Append JSON data as "data" field
    let json_str = serde_json::to_string(&body_struct).map_err(|e| e.to_string())?;
    let _ = form_data.append_with_str("data", &json_str);

    // Append file if present
    if let Some(f) = file {
        let _ = form_data.append_with_blob("file", &f);
    }

    let builder = if method == "PUT" {
        Request::put(&url)
    } else {
        Request::post(&url)
    };

    // Note: Do NOT set Content-Type header for FormData, browser does it with boundary
    let resp = builder
        .credentials(web_sys::RequestCredentials::Include)
        .body(form_data)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to save template: {}", resp.status()))
    }
}

/// List user's templates
#[allow(dead_code)]
pub async fn list_templates() -> Result<ListTemplatesResponse, String> {
    let resp = Request::get(&format!("{}/api/templates", API_BASE))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to list templates: {}", resp.status()))
    }
}

/// Get a template by ID
#[allow(dead_code)]
pub async fn get_template(template_id: &str) -> Result<GetTemplateResponse, String> {
    let resp = Request::get(&format!("{}/api/templates/{}", API_BASE, template_id))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to get template: {}", resp.status()))
    }
}

/// Delete a template
#[allow(dead_code)]
pub async fn delete_template(template_id: &str) -> Result<bool, String> {
    let resp = Request::delete(&format!("{}/api/templates/{}", API_BASE, template_id))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(resp.ok())
}

// ========================
// GOOGLE SLIDES EXPORT
// ========================

#[derive(Debug, Clone, Serialize)]
pub struct ExportSlideData {
    pub title: String,
    pub body: Vec<String>,
    pub layout: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportToSlidesRequest {
    pub title: String,
    pub slides: Vec<ExportSlideData>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct ExportToSlidesResponse {
    pub success: bool,
    pub presentation_id: String,
    pub presentation_url: String,
    pub slides_count: usize,
    pub message: String,
}

/// Export slides to Google Slides
pub async fn export_to_slides(
    title: &str,
    slides: Vec<ExportSlideData>,
) -> Result<ExportToSlidesResponse, String> {
    let resp = Request::post(&format!("{}/api/export/slides", API_BASE))
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(&ExportToSlidesRequest {
            title: title.to_string(),
            slides,
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err(format!(
            "Google Slides export failed ({}): {}",
            status, text
        ))
    }
}

// =================================================================
// PPTX REPORT GENERATION
// =================================================================

use std::collections::HashMap;

/// Response from PPTX generation
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct GeneratePptxResponse {
    pub success: bool,
    pub message: String,
    pub pptx_base64: Option<String>,
}

/// Slide edit structure for PPTX injection
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct PptxSlideEdit {
    pub slide_index: usize,
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub placeholder_key: Option<String>,
}

/// Generate PPTX report with real data
///
/// Takes the template PPTX file, placeholder edits (positions), and actual values
/// Returns the modified PPTX as base64
#[allow(dead_code)]
pub async fn generate_pptx_report(
    pptx_file: web_sys::File,
    edits: Vec<PptxSlideEdit>,
    placeholder_values: HashMap<String, String>,
) -> Result<GeneratePptxResponse, String> {
    let form_data = web_sys::FormData::new().map_err(|_| "Failed to create FormData")?;

    // Add file
    form_data
        .append_with_blob("file", &pptx_file)
        .map_err(|_| "Failed to append file")?;

    // Add edits as JSON
    let edits_json = serde_json::to_string(&edits).map_err(|e| e.to_string())?;
    form_data
        .append_with_str("edits", &edits_json)
        .map_err(|_| "Failed to append edits")?;

    // Add placeholder values as JSON
    let values_json = serde_json::to_string(&placeholder_values).map_err(|e| e.to_string())?;
    form_data
        .append_with_str("placeholder_values", &values_json)
        .map_err(|_| "Failed to append placeholder_values")?;

    let resp = Request::post(&format!("{}/api/export/generate-pptx", API_BASE))
        .credentials(web_sys::RequestCredentials::Include)
        .body(form_data)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err(format!("PPTX generation failed ({}): {}", status, text))
    }
}

// =================================================================
// TEMPLATE PPTX FILE ACCESS
// =================================================================

/// Response for get_template_pptx
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct GetTemplatePptxResponse {
    pub success: bool,
    pub pptx_base64: Option<String>,
    pub size_bytes: Option<usize>,
    pub error: Option<String>,
}

/// Get the base PPTX file of a template
#[allow(dead_code)]
pub async fn get_template_pptx(template_id: &str) -> Result<GetTemplatePptxResponse, String> {
    let resp = Request::get(&format!("{}/api/templates/{}/pptx", API_BASE, template_id))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        Err(format!(
            "Failed to get template PPTX ({}): {}",
            status, text
        ))
    }
}

/// Download a base64-encoded file by triggering browser download
/// Uses JavaScript atob for base64 decoding
#[allow(dead_code)]
pub fn download_base64_file(base64_data: &str, filename: &str, mime_type: &str) {
    use wasm_bindgen::JsCast;

    // Use JavaScript's atob to decode base64
    let Some(window) = web_sys::window() else {
        return;
    };

    let decoded_string = match window.atob(base64_data) {
        Ok(s) => s,
        Err(e) => {
            leptos::logging::error!("Failed to decode base64: {:?}", e);
            return;
        }
    };

    // Convert string to bytes (binary string from atob)
    let bytes: Vec<u8> = decoded_string.chars().map(|c| c as u8).collect();

    // Create Blob
    let array = js_sys::Uint8Array::new_with_length(bytes.len() as u32);
    array.copy_from(&bytes);
    let parts = js_sys::Array::new();
    parts.push(&array.buffer());

    let options = web_sys::BlobPropertyBag::new();
    options.set_type(mime_type);

    let blob = match web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &options) {
        Ok(b) => b,
        Err(e) => {
            leptos::logging::error!("Failed to create Blob: {:?}", e);
            return;
        }
    };

    // Create download URL
    let url = match web_sys::Url::create_object_url_with_blob(&blob) {
        Ok(u) => u,
        Err(e) => {
            leptos::logging::error!("Failed to create URL: {:?}", e);
            return;
        }
    };

    // Trigger download
    if let Some(document) = window.document() {
        if let Ok(element) = document.create_element("a") {
            let anchor = element.unchecked_into::<web_sys::HtmlAnchorElement>();
            anchor.set_href(&url);
            anchor.set_download(filename);
            anchor.click();

            // Cleanup
            let _ = web_sys::Url::revoke_object_url(&url);
        }
    }
}

// ========================
// ADMIN USER MANAGEMENT
// ========================

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct AllowedUser {
    pub email: String,
    pub role: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub added_by: Option<String>,
}

#[derive(Debug, Serialize)]
struct AddUserRequest {
    pub email: String,
    pub role: String,
    pub description: Option<String>,
}

/// List allowed users
pub async fn list_users() -> Result<Vec<AllowedUser>, String> {
    let resp = Request::get(&format!("{}/api/admin/users", API_BASE))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to list users: {}", resp.status()))
    }
}

/// Add a user to the whitelist
pub async fn add_user(email: &str, role: &str, description: Option<String>) -> Result<(), String> {
    let resp = Request::post(&format!("{}/api/admin/users", API_BASE))
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include)
        .json(&AddUserRequest {
            email: email.to_string(),
            role: role.to_string(),
            description,
        })
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        // Try parsing error message
        let text = resp.text().await.unwrap_or_default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message").and_then(|v| v.as_str()) {
                return Err(msg.to_string());
            }
        }
        Err(format!("Failed to add user: {} - {}", resp.status(), text))
    }
}

/// Remove a user from the whitelist
pub async fn remove_user(email: &str) -> Result<(), String> {
    let url = format!("{}/api/admin/users/{}", API_BASE, urlencoding_encode(email));
    let resp = Request::delete(&url)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        // Try parsing error message
        let text = resp.text().await.unwrap_or_default();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(msg) = json.get("message").and_then(|v| v.as_str()) {
                return Err(msg.to_string());
            }
        }
        Err(format!(
            "Failed to remove user: {} - {}",
            resp.status(),
            text
        ))
    }
}
