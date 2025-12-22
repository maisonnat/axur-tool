//! Authentication routes - Proxy to Axur API
//!
//! Implements the 3-step login flow:
//! 1. POST /login - Email/password → temp token + correlation
//! 2. POST /2fa - 2FA code verification  
//! 3. POST /finalize - Get master token, set httpOnly cookie

use axum::{response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::ApiError;
use crate::middleware::AUTH_COOKIE_NAME;

// Axur API URL
const AXUR_API_URL: &str = "https://api.axur.com/gateway/1.0/api";

// ========================
// REQUEST/RESPONSE TYPES
// ========================

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct TwoFactorRequest {
    pub code: String,
    /// Temp token from login step
    pub token: String,
    /// Correlation token from login step
    pub correlation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FinalizeRequest {
    pub email: String,
    pub password: String,
    pub token: String,
    pub correlation: Option<String>,
    pub device_id: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub requires_2fa: bool,
    pub token: Option<String>,
    pub correlation: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TwoFactorResponse {
    pub success: bool,
    pub token: Option<String>,
    pub correlation: Option<String>,
    pub device_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ValidateResponse {
    pub valid: bool,
    pub message: String,
}

// Internal Axur API response
#[derive(Debug, Deserialize)]
struct AxurAuthResponse {
    correlation: Option<String>,
    token: Option<String>,
    #[serde(rename = "deviceId")]
    device_id: Option<String>,
}

// ========================
// ROUTE HANDLERS
// ========================

/// Step 1: Initial login with email/password
/// Returns temp token and correlation for 2FA step
pub async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, ApiError> {
    // Validate input
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ApiError::BadRequest("Email and password required".into()));
    }

    // Create HTTP client
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // Call Axur auth endpoint
    let url = format!("{}/identity/session", AXUR_API_URL);
    let resp = client
        .post(&url)
        .json(&json!({
            "email": payload.email,
            "password": payload.password
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        tracing::warn!("Axur login failed: {} - {}", status, body);
        return Err(ApiError::Unauthorized("Invalid credentials".into()));
    }

    let data: AxurAuthResponse = resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Axur response: {}", e)))?;

    Ok(Json(LoginResponse {
        success: true,
        requires_2fa: true, // Axur always requires 2FA
        token: data.token,
        correlation: data.correlation,
        message: "Credentials validated. Please complete 2FA.".into(),
    }))
}

/// Step 2: 2FA verification
pub async fn verify_2fa(
    Json(payload): Json<TwoFactorRequest>,
) -> Result<Json<TwoFactorResponse>, ApiError> {
    // Validate input
    if payload.code.is_empty() || payload.token.is_empty() {
        return Err(ApiError::BadRequest("Code and token required".into()));
    }

    let code: u32 = payload
        .code
        .parse()
        .map_err(|_| ApiError::BadRequest("2FA code must be numeric".into()))?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let url = format!("{}/identity/session/tfa", AXUR_API_URL);

    let mut req = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .json(&json!({"code": code}));

    if let Some(ref corr) = payload.correlation {
        req = req.header("oxref-token", corr);
    }

    let resp = req.send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        tracing::warn!("Axur 2FA failed: {} - {}", status, body);
        return Err(ApiError::Unauthorized("Invalid 2FA code".into()));
    }

    let data: AxurAuthResponse = resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Axur response: {}", e)))?;

    Ok(Json(TwoFactorResponse {
        success: true,
        token: data.token,
        correlation: data.correlation,
        device_id: data.device_id,
        message: "2FA verified. Please finalize login.".into(),
    }))
}

/// Step 3: Finalize login and set httpOnly cookie
pub async fn finalize(
    jar: CookieJar,
    Json(payload): Json<FinalizeRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate input
    if payload.token.is_empty() || payload.device_id.is_empty() {
        return Err(ApiError::BadRequest("Token and device_id required".into()));
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let url = format!("{}/identity/session", AXUR_API_URL);

    let mut req = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", payload.token))
        .header("Device-Id", &payload.device_id)
        .json(&json!({
            "email": payload.email,
            "password": payload.password
        }));

    if let Some(ref corr) = payload.correlation {
        req = req.header("oxref-token", corr);
    }

    let resp = req.send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        tracing::warn!("Axur finalize failed: {} - {}", status, body);
        return Err(ApiError::Unauthorized("Failed to finalize session".into()));
    }

    let data: AxurAuthResponse = resp
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to parse Axur response: {}", e)))?;

    let master_token = data
        .token
        .ok_or_else(|| ApiError::Internal("No master token received".into()))?;

    // Create httpOnly secure cookie (OWASP compliant)
    let cookie = Cookie::build((AUTH_COOKIE_NAME, master_token))
        .http_only(true)
        .secure(true) // Requires HTTPS in production
        .same_site(SameSite::None)
        .path("/")
        .max_age(cookie::time::Duration::days(7))
        .build();

    let updated_jar = jar.add(cookie);

    Ok((
        updated_jar,
        Json(json!({
            "success": true,
            "message": "Login complete. Session established."
        })),
    ))
}

/// Validate current session
pub async fn validate(jar: CookieJar) -> Result<Json<ValidateResponse>, ApiError> {
    let token = match jar.get(AUTH_COOKIE_NAME) {
        Some(c) => c.value().to_string(),
        None => {
            return Ok(Json(ValidateResponse {
                valid: false,
                message: "No session found".into(),
            }))
        }
    };

    // Validate token with Axur API
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let url = format!("{}/customers/customers", AXUR_API_URL);
    let resp = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    // 200 or 403 means token is valid (403 = valid but no access to this resource)
    let is_valid = resp.status().is_success() || resp.status().as_u16() == 403;

    Ok(Json(ValidateResponse {
        valid: is_valid,
        message: if is_valid {
            "Session valid".into()
        } else {
            "Session expired".into()
        },
    }))
}

/// Logout - clear session cookie
pub async fn logout(jar: CookieJar) -> impl IntoResponse {
    // Create expired cookie to clear the session
    let cookie = Cookie::build((AUTH_COOKIE_NAME, ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(cookie::time::Duration::seconds(0))
        .build();

    let updated_jar = jar.add(cookie);

    (
        updated_jar,
        Json(json!({
            "success": true,
            "message": "Logged out successfully"
        })),
    )
}
