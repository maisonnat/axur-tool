//! API client for communicating with backend
// Force rebuild for env var

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

// Use environment variable at compile time
// - Development: Set API_BASE_URL=http://localhost:3001
// - Production: Use empty string for relative URLs (Cloudflare proxies /api/* to Koyeb)
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tenant {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateReportRequest {
    pub tenant_id: String,
    pub from_date: String,
    pub to_date: String,
    pub language: String,
    pub story_tag: Option<String>,
    pub include_threat_intel: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct GenerateReportResponse {
    pub success: bool,
    pub html: Option<String>,
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
    let resp = Request::post(&format!("{}/api/auth/login", API_BASE))
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
        resp.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Login failed: {}", resp.status()))
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
pub async fn validate_session() -> Result<bool, String> {
    let resp = Request::get(&format!("{}/api/auth/validate", API_BASE))
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let data: ValidateResponse = resp.json().await.map_err(|e| e.to_string())?;
        Ok(data.valid)
    } else {
        Ok(false)
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
pub async fn generate_report(
    tenant_id: &str,
    from_date: &str,
    to_date: &str,
    language: &str,
    story_tag: Option<String>,
    include_threat_intel: bool,
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
pub struct LogAccessResponse {
    pub has_access: bool,
    #[allow(dead_code)]
    pub message: String,
}

/// Check if user has access to logs
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
