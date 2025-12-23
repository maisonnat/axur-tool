//! API client for communicating with backend

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

// Use environment variable at compile time, default to localhost for development
const API_BASE: &str = match option_env!("API_BASE_URL") {
    Some(url) => url,
    None => "http://localhost:3001",
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GenerateReportResponse {
    pub success: bool,
    pub html: Option<String>,
    pub company_name: Option<String>,
    pub message: String,
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
