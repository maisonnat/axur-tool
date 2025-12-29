//! Report generation routes
//!
//! Endpoints for listing tenants and generating HTML reports.
//! Includes structured error codes for better debugging.

use axum::response::sse::{Event, KeepAlive, Sse};
use axum::Json;
use axum_extra::extract::CookieJar;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

use crate::error::ApiError;
use crate::middleware::AUTH_COOKIE_NAME;
use axur_core::api::report::{
    fetch_available_tenants, fetch_full_report, fetch_tagged_tickets_for_preview,
    preview_threat_hunting,
};
use axur_core::error_codes::{self, ErrorCode};
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    /// Structured error code for quick debugging (e.g., "TI-001", "API-002")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// User-friendly error message in the requested language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThreatHuntingPreviewRequest {
    pub tenant_id: String,
    pub story_tag: String,
    #[serde(default)]
    pub use_user_credits: bool,
}

#[derive(Debug, Serialize)]
pub struct ThreatHuntingPreviewResponse {
    pub success: bool,
    pub preview: Option<axur_core::api::report::ThreatHuntingPreview>,
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

/// Generate HTML report for a tenant with structured error handling
pub async fn generate_report(
    jar: CookieJar,
    Json(payload): Json<GenerateReportRequest>,
) -> Result<Json<GenerateReportResponse>, ApiError> {
    // Validate input
    if payload.tenant_id.is_empty() {
        let code = error_codes::report::invalid_date_range();
        return Ok(Json(GenerateReportResponse {
            success: false,
            html: None,
            company_name: None,
            message: "Tenant ID is required".into(),
            error_code: Some(code.code()),
            error_message: Some(get_user_friendly_message(&code)),
        }));
    }
    if payload.from_date.is_empty() || payload.to_date.is_empty() {
        let code = error_codes::report::invalid_date_range();
        return Ok(Json(GenerateReportResponse {
            success: false,
            html: None,
            company_name: None,
            message: "Date range is required".into(),
            error_code: Some(code.code()),
            error_message: Some(get_user_friendly_message(&code)),
        }));
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

    // 📝 Log request
    crate::routes::remote_log::log_request("report_generate", &payload, Some(&payload.tenant_id));

    // ⏱️ Start performance tracking
    let start_time = std::time::Instant::now();

    // Fetch report data using axur-core
    let report_data = match fetch_full_report(
        &token,
        &payload.tenant_id,
        &payload.from_date,
        &payload.to_date,
        payload.story_tag.clone(),
        payload.include_threat_intel,
    )
    .await
    {
        Ok(data) => data,
        Err(e) => {
            let error_code = classify_error(&e.to_string());
            tracing::error!(
                error_code = %error_code.code(),
                context = %e.to_string(),
                "Report generation failed"
            );

            // 📝 Log error
            crate::routes::remote_log::log_error(
                "report_generate",
                &error_code.code(),
                &e.to_string(),
                Some(&payload.tenant_id),
                serde_json::json!({
                    "from_date": &payload.from_date,
                    "to_date": &payload.to_date,
                    "include_threat_intel": payload.include_threat_intel,
                    "story_tag": &payload.story_tag
                }),
            );

            return Ok(Json(GenerateReportResponse {
                success: false,
                html: None,
                company_name: None,
                message: e.to_string(),
                error_code: Some(error_code.code()),
                error_message: Some(get_user_friendly_message(&error_code)),
            }));
        }
    };

    // Get dictionary for selected language
    let language = match payload.language.to_lowercase().as_str() {
        "en" => Language::En,
        "pt" | "pt-br" => Language::PtBr,
        _ => Language::Es,
    };
    let dict = get_dictionary(language);

    // Generate HTML report
    let html = generate_full_report_html(&report_data, None, &dict);

    // ⏱️ Calculate duration
    let duration_ms = start_time.elapsed().as_millis();

    let response = GenerateReportResponse {
        success: true,
        html: Some(html),
        company_name: Some(report_data.company_name.clone()),
        message: "Report generated successfully".into(),
        error_code: None,
        error_message: None,
    };

    // 📝 Log successful response
    crate::routes::remote_log::log_response(
        "report_generate",
        &response,
        duration_ms,
        Some(&payload.tenant_id),
        true,
    );

    // 📊 Log feature usage
    crate::routes::remote_log::log_feature_usage(
        "report_generation",
        Some(&payload.tenant_id),
        true,
        Some(serde_json::json!({
            "include_threat_intel": payload.include_threat_intel,
            "language": &payload.language,
            "has_story_tag": payload.story_tag.is_some(),
            "duration_ms": duration_ms
        })),
    );

    // 📦 Archive report to GitHub (async)
    let company_name = report_data.company_name.clone();
    let filename = format!(
        "{}_{}_report.html",
        company_name.replace(' ', "_"),
        chrono::Utc::now().format("%Y-%m-%d_%H%M%S")
    );
    crate::routes::remote_log::upload_report_async(&company_name, &filename, html.clone());

    Ok(Json(response))
}

/// Preview Threat Hunting results without consuming full credits
/// Returns counts and estimated credits for user confirmation
pub async fn threat_hunting_preview(
    jar: CookieJar,
    Json(payload): Json<ThreatHuntingPreviewRequest>,
) -> Result<Json<ThreatHuntingPreviewResponse>, ApiError> {
    let token = jar
        .get(AUTH_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::Unauthorized("No session found".into()))?;

    tracing::info!(
        tenant = %payload.tenant_id,
        story_tag = %payload.story_tag,
        "Starting Threat Hunting preview"
    );

    // 📝 Log request
    crate::routes::remote_log::log_request("th_preview", &payload, Some(&payload.tenant_id));

    // ⏱️ Start performance tracking
    let start_time = std::time::Instant::now();

    // FIXED: Fetch actual tickets with the story_tag instead of using tag as domain
    let tickets = match axur_core::api::report::fetch_tagged_tickets_for_preview(
        &token,
        &payload.tenant_id,
        &payload.story_tag,
    )
    .await
    {
        Ok(t) => {
            tracing::info!(
                "Fetched {} tickets with tag '{}'",
                t.len(),
                payload.story_tag
            );
            t
        }
        Err(e) => {
            tracing::warn!("Failed to fetch tagged tickets: {}, using empty list", e);
            vec![]
        }
    };

    if tickets.is_empty() {
        return Ok(Json(ThreatHuntingPreviewResponse {
            success: false,
            preview: None,
            message: format!(
                "No tickets found with tag '{}' in tenant {}. Please verify the tag exists.",
                payload.story_tag, payload.tenant_id
            ),
        }));
    }

    match preview_threat_hunting(
        &token,
        &payload.tenant_id,
        &tickets,
        &payload.story_tag,
        payload.use_user_credits,
    )
    .await
    {
        Ok(preview) => {
            tracing::info!(
                total = preview.total_count,
                estimated_credits = preview.estimated_credits,
                tickets_used = tickets.len(),
                "Threat Hunting preview completed"
            );

            // ⏱️ Calculate duration
            let duration_ms = start_time.elapsed().as_millis();

            // 📝 Log successful response (metadata only, not full preview data)
            crate::routes::remote_log::log_response(
                "th_preview",
                &serde_json::json!({
                    "success": true,
                    "total_count": preview.total_count,
                    "estimated_credits": preview.estimated_credits,
                    "signal_lake_count": preview.signal_lake_count,
                    "credential_count": preview.credential_count,
                    "tickets_used": tickets.len()
                }),
                duration_ms,
                Some(&payload.tenant_id),
                true,
            );

            // 📊 Log feature usage
            crate::routes::remote_log::log_feature_usage(
                "preview_generation",
                Some(&payload.tenant_id),
                true,
                Some(serde_json::json!({
                    "story_tag": &payload.story_tag,
                    "tickets_count": tickets.len(),
                    "total_results": preview.total_count,
                    "estimated_credits": preview.estimated_credits,
                    "duration_ms": duration_ms
                })),
            );

            Ok(Json(ThreatHuntingPreviewResponse {
                success: true,
                preview: Some(preview),
                message: format!("Preview ready. Found {} tickets with tag.", tickets.len()),
            }))
        }
        Err(e) => {
            tracing::error!("Threat Hunting preview failed: {}", e);

            // 📝 Log error
            crate::routes::remote_log::log_error(
                "th_preview",
                "TH-ERR",
                &e.to_string(),
                Some(&payload.tenant_id),
                serde_json::json!({
                    "story_tag": &payload.story_tag,
                    "tickets_count": tickets.len()
                }),
            );

            Ok(Json(ThreatHuntingPreviewResponse {
                success: false,
                preview: None,
                message: format!("Preview failed: {}", e),
            }))
        }
    }
}

/// Classify an error string into an appropriate ErrorCode
fn classify_error(error: &str) -> ErrorCode {
    let lower = error.to_lowercase();

    if lower.contains("timeout") {
        if lower.contains("dark") || lower.contains("threat") {
            error_codes::threat_intel::dark_web_timeout()
        } else {
            error_codes::network::connection_timeout()
        }
    } else if lower.contains("cors") {
        error_codes::network::cors_blocked()
    } else if lower.contains("token") || lower.contains("expired") {
        error_codes::api::token_expired()
    } else if lower.contains("tenant") || lower.contains("not found") {
        error_codes::api::tenant_not_found()
    } else if lower.contains("rate") || lower.contains("limit") {
        error_codes::api::rate_limited()
    } else if lower.contains("dns") {
        error_codes::network::dns_failed()
    } else if lower.contains("ssl") || lower.contains("tls") || lower.contains("certificate") {
        error_codes::network::ssl_error()
    } else {
        error_codes::system::internal_error().with_context(error.to_string())
    }
}

/// Get a user-friendly message for an error code (i18n ready)
fn get_user_friendly_message(code: &ErrorCode) -> String {
    match code.code().as_str() {
        "AUTH-001" => "Credenciales inválidas. Verifica tu email y contraseña.".into(),
        "AUTH-002" => "Código 2FA incorrecto. Intenta de nuevo.".into(),
        "AUTH-003" => "Tu sesión ha expirado. Por favor, inicia sesión nuevamente.".into(),
        "AUTH-004" => "No hay sesión activa. Por favor, inicia sesión.".into(),
        "API-001" => "El token de Axur ha expirado. Reconecta tu cuenta.".into(),
        "API-002" => "El tenant seleccionado no fue encontrado.".into(),
        "API-003" => "Demasiadas solicitudes. Espera unos minutos e intenta de nuevo.".into(),
        "RPT-001" => "No hay datos en el período seleccionado.".into(),
        "RPT-004" => "Rango de fechas inválido o tenant no especificado.".into(),
        "TI-001" => {
            "La búsqueda en Dark Web excedió el tiempo de espera. Intenta nuevamente.".into()
        }
        "TI-002" => "El servicio de Threat Intelligence no está disponible temporalmente.".into(),
        "NET-001" => "Error de CORS. Contacta al administrador.".into(),
        "NET-002" => "Timeout de conexión. Verifica tu conexión a internet.".into(),
        "SYS-001" => "Error interno del servidor. Si persiste, contacta soporte.".into(),
        _ => "Ha ocurrido un error inesperado.".into(),
    }
}

// ========================
// SSE STREAMING PREVIEW
// ========================

/// Event types for SSE streaming
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
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

/// SSE endpoint for streaming Threat Hunting preview progress
/// Uses GET with query params for EventSource compatibility
pub async fn threat_hunting_preview_stream(
    jar: CookieJar,
    axum::extract::Query(params): axum::extract::Query<ThreatHuntingPreviewRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    let token = jar
        .get(AUTH_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::Unauthorized("No session found".into()))?;

    tracing::info!(
        tenant = %params.tenant_id,
        story_tag = %params.story_tag,
        "Starting SSE Threat Hunting preview stream"
    );

    // Clone values for the async stream
    let tenant_id = params.tenant_id.clone();
    let story_tag = params.story_tag.clone();

    let stream = async_stream::stream! {
        // Fetch tickets first
        let tickets = match fetch_tagged_tickets_for_preview(&token, &tenant_id, &story_tag).await {
            Ok(t) => t,
            Err(e) => {
                let event = ThreatHuntingStreamEvent::Error {
                    message: format!("Failed to fetch tickets: {}", e),
                };
                if let Ok(json) = serde_json::to_string(&event) {
                    yield Ok(Event::default().data(json));
                }
                return;
            }
        };

        if tickets.is_empty() {
            let event = ThreatHuntingStreamEvent::Error {
                message: format!("No tickets found with tag '{}' in tenant {}", story_tag, tenant_id),
            };
            if let Ok(json) = serde_json::to_string(&event) {
                yield Ok(Event::default().data(json));
            }
            return;
        }

        // Extract unique domains (max 5)
        let unique_domains: Vec<String> = {
            let mut domains: std::collections::HashSet<String> = std::collections::HashSet::new();
            for ticket in &tickets {
                if !ticket.target.is_empty() {
                    domains.insert(ticket.target.clone());
                }
            }
            domains.into_iter().take(5).collect()
        };

        // Send started event
        let started = ThreatHuntingStreamEvent::Started {
            total_domains: unique_domains.len(),
            total_tickets: tickets.len(),
        };
        if let Ok(json) = serde_json::to_string(&started) {
            yield Ok(Event::default().data(json));
        }

        // Process each domain
        let client = match axur_core::api::create_client() {
            Ok(c) => c,
            Err(e) => {
                let event = ThreatHuntingStreamEvent::Error {
                    message: format!("Failed to create HTTP client: {}", e),
                };
                if let Ok(json) = serde_json::to_string(&event) {
                    yield Ok(Event::default().data(json));
                }
                return;
            }
        };

        let auth = format!("Bearer {}", token);
        let mut total_signal_lake: u64 = 0;
        let mut total_chatter: u64 = 0;
        let mut total_credentials: u64 = 0;

        // Determine customer for credits (None = User/Admin, Some = Tenant)
        let use_user_credits = params.use_user_credits;
        let customer_opt = if use_user_credits { None } else { Some(tenant_id.as_str()) };

        for (idx, domain) in unique_domains.iter().enumerate() {
             // Emit processing event
            let processing = ThreatHuntingStreamEvent::DomainProcessing {
                domain: domain.clone(),
                index: idx + 1,
                source: "multi-source".to_string(),
            };
            if let Ok(json) = serde_json::to_string(&processing) {
                yield Ok(Event::default().data(json));
            }

            // 1. Infra Search (Signal Lake)
            let query_infra = format!("domain=\"{}\"", domain);
            if let Ok(count) = axur_core::api::report::start_and_poll_th_search(
                &client, &auth, customer_opt, &query_infra, "signal-lake"
            ).await {
                total_signal_lake += count;
                if count > 0 {
                    let evt = ThreatHuntingStreamEvent::DomainComplete {
                        domain: domain.clone(),
                        source: "signal-lake".to_string(),
                        count,
                    };
                    if let Ok(json) = serde_json::to_string(&evt) { yield Ok(Event::default().data(json)); }
                }
            }

            // 2. Chatter Search
            let query_chatter = format!("content=\"{}\"", domain);
            for source in ["chat-message", "forum-message"] {
                if let Ok(count) = axur_core::api::report::start_and_poll_th_search(
                    &client, &auth, customer_opt, &query_chatter, source
                ).await {
                    total_chatter += count;
                    if count > 0 {
                         let evt = ThreatHuntingStreamEvent::DomainComplete {
                            domain: domain.clone(),
                            source: source.to_string(),
                            count,
                        };
                        if let Ok(json) = serde_json::to_string(&evt) { yield Ok(Event::default().data(json)); }
                    }
                }
            }

            // 3. Credential search is done separately after domain loop
            // (TH doesn't support tag: for credentials, must use Exposure API)

            // Rate limit wait
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        // 4. Fetch credentials via Exposure API using the story_tag
        // This is done once, not per-domain, since credentials are tagged with story_tag
        if !story_tag.is_empty() {
            let exposure_url = format!(
                "https://api.axur.com/gateway/1.0/api/exposure-api/credentials?tags=contains:{}&pageSize=100",
                story_tag
            );

            if let Ok(resp) = client.get(&exposure_url)
                .header("Authorization", &auth)
                .send()
                .await
            {
                if resp.status().is_success() {
                    if let Ok(body) = resp.text().await {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                            // Get total count from pageable.total
                            if let Some(pageable) = json.get("pageable") {
                                if let Some(total) = pageable.get("total").and_then(|t| t.as_u64()) {
                                    total_credentials = total;

                                    // Emit event for credentials
                                    let evt = ThreatHuntingStreamEvent::DomainComplete {
                                        domain: format!("tag:{}", story_tag),
                                        source: "credential".to_string(),
                                        count: total,
                                    };
                                    if let Ok(json_str) = serde_json::to_string(&evt) {
                                        yield Ok(Event::default().data(json_str));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Send finished event
        let total_count = total_signal_lake + total_chatter + total_credentials;
        let estimated_credits = (total_count as f64) * 0.01; // Rough estimate

        let finished = ThreatHuntingStreamEvent::Finished {
            total_count,
            signal_lake_count: total_signal_lake,
            chatter_count: total_chatter,
            credential_count: total_credentials,
            estimated_credits,
        };
        if let Ok(json) = serde_json::to_string(&finished) {
            yield Ok(Event::default().data(json));
        }
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
