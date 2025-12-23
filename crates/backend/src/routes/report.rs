//! Report generation routes
//!
//! Endpoints for listing tenants and generating HTML reports.

use axum::Json;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::AUTH_COOKIE_NAME;
use axur_core::api::report::{fetch_available_tenants, fetch_full_report};
use axur_core::i18n::{get_dictionary, Language};
use axur_core::report::html::generate_full_report_html;

// ========================
// REQUEST/RESPONSE TYPES
// ========================

#[derive(Debug, Serialize)]
pub struct TenantResponse {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateReportRequest {
    pub tenant_id: String,
    pub from_date: String,
    pub to_date: String,
    #[serde(default = "default_language")]
    pub language: String,
    pub story_tag: Option<String>,
    #[serde(default)]
    pub include_threat_intel: bool,
}

fn default_language() -> String {
    "es".to_string()
}

#[derive(Debug, Serialize)]
pub struct GenerateReportResponse {
    pub success: bool,
    pub html: Option<String>,
    pub company_name: Option<String>,
    pub message: String,
}

// ========================
// ROUTE HANDLERS
// ========================

/// List available tenants for the authenticated user
pub async fn list_tenants(jar: CookieJar) -> Result<Json<Vec<TenantResponse>>, ApiError> {
    let token = jar
        .get(AUTH_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::Unauthorized("No session found".into()))?;

    let tenants = fetch_available_tenants(&token)
        .await
        .map_err(|e| ApiError::ExternalApi(format!("Failed to fetch tenants: {}", e)))?;

    let response: Vec<TenantResponse> = tenants
        .into_iter()
        .map(|t| TenantResponse {
            key: t.key,
            name: t.name,
        })
        .collect();

    Ok(Json(response))
}

/// Generate HTML report for a tenant
pub async fn generate_report(
    jar: CookieJar,
    Json(payload): Json<GenerateReportRequest>,
) -> Result<Json<GenerateReportResponse>, ApiError> {
    // Validate input
    if payload.tenant_id.is_empty() {
        return Err(ApiError::BadRequest("tenant_id required".into()));
    }
    if payload.from_date.is_empty() || payload.to_date.is_empty() {
        return Err(ApiError::BadRequest(
            "from_date and to_date required".into(),
        ));
    }

    let token = jar
        .get(AUTH_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::Unauthorized("No session found".into()))?;

    tracing::info!(
        "Generating report for tenant {} from {} to {} with story_tag: {:?}",
        payload.tenant_id,
        payload.from_date,
        payload.to_date,
        payload.story_tag
    );

    // Fetch report data using axur-core
    let report_data = fetch_full_report(
        &token,
        &payload.tenant_id,
        &payload.from_date,
        &payload.to_date,
        payload.story_tag,
        payload.include_threat_intel,
    )
    .await
    .map_err(|e| ApiError::ExternalApi(format!("Failed to fetch report data: {}", e)))?;

    // Get dictionary for selected language
    let language = match payload.language.to_lowercase().as_str() {
        "en" => Language::En,
        "pt" | "pt-br" => Language::PtBr,
        _ => Language::Es,
    };
    let dict = get_dictionary(language);

    // Generate HTML report
    let html = generate_full_report_html(&report_data, None, &dict);

    Ok(Json(GenerateReportResponse {
        success: true,
        html: Some(html),
        company_name: Some(report_data.company_name),
        message: "Report generated successfully".into(),
    }))
}
