//! Report generation routes
//!
//! Endpoints for listing tenants and generating HTML reports.
//! Includes structured error codes for better debugging.

use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::{Extension, Json};
use axum_extra::extract::CookieJar;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use uuid::Uuid;

use crate::error::ApiError;
use crate::middleware::AUTH_COOKIE_NAME;
use crate::routes::templates::{self, GitHubConfig};
use crate::routes::AppState;
use crate::services::report_service::{
    classify_error, get_user_friendly_message, GenerateReportRequest, GenerateReportResponse,
    ReportService, TenantResponse,
};
use axur_core::api::report::{
    fetch_available_tenants, fetch_full_report, fetch_tagged_tickets_for_preview,
    preview_threat_hunting,
};
use axur_core::error_codes::{self, ErrorCode};
use axur_core::i18n::{get_dictionary, Language, Translations};
use axur_core::report::html::{generate_full_report_html, generate_report_with_plugins};
use axur_core::report::OfflineAssets;
use std::time::Instant;

fn default_language() -> String {
    "es".to_string()
}

// ========================
// REQUEST/RESPONSE TYPES
// ========================

// REQUEST/RESPONSE TYPES MOVED TO services::report_service
// Kept ThreatHunting types here for now as they are not migrated yet.

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
    State(_state): State<AppState>,
    Extension(user_id): Extension<String>,
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

    let start_time = Instant::now();

    // Delegate to ReportService (Safe & Modular)
    let response = ReportService::generate_report(&payload, &token, &user_id).await?;

    // 📝 Log successful response (Service handles logic, Handler handles HTTP logging)
    crate::routes::remote_log::log_response(
        "report_generate",
        &response,
        start_time.elapsed().as_millis(),
        Some(&payload.tenant_id),
        true,
    );

    // 📊 Log feature usage (simplified)
    crate::routes::remote_log::log_feature_usage(
        "report_generation",
        Some(&payload.tenant_id),
        true,
        Some(serde_json::json!({
            "include_threat_intel": payload.include_threat_intel,
            "language": &payload.language,
            "has_story_tag": payload.story_tag.is_some(),
        })),
    );

    // 📦 Archive report to GitHub (async)
    if let (Some(html), Some(company_name)) = (&response.html, &response.company_name) {
        let filename = format!(
            "{}_{}_report.html",
            company_name.replace(' ', "_"),
            chrono::Utc::now().format("%Y-%m-%d_%H%M%S")
        );
        crate::routes::remote_log::upload_report_async(company_name, &filename, html.clone());
    }

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

// Helpers moved to ReportService

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

// ========================
// SSE STREAMING REPORT GENERATION
// ========================

/// Event types for SSE report generation streaming
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReportStreamEvent {
    /// Report generation started
    Started { stages: Vec<String> },
    /// Progress update for current stage
    StageProgress {
        stage: String,
        message: String,
        progress_pct: u8,
    },
    /// Stage completed
    StageComplete { stage: String },
    /// Report generation finished with HTML
    Finished {
        html: String,
        company_name: Option<String>,
    },
    /// Error occurred
    Error { code: String, message: String },
}

/// Request params for streaming report generation (GET for EventSource)
#[derive(Debug, Deserialize)]
pub struct GenerateReportStreamParams {
    pub tenant_id: String,
    pub from_date: String,
    pub to_date: String,
    #[serde(default = "default_language")]
    pub language: String,
    pub story_tag: Option<String>,
    #[serde(default)]
    pub include_threat_intel: bool,
    pub template_id: Option<String>,
    #[serde(default)]
    pub use_plugins: bool,
    pub plugin_theme: Option<String>,
    pub disabled_slides: Option<String>, // Comma-separated list
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

/// SSE endpoint for streaming report generation with progress events
/// Uses GET with query params for EventSource compatibility
pub async fn generate_report_stream(
    State(_state): State<AppState>,
    jar: CookieJar,
    axum::extract::Query(params): axum::extract::Query<GenerateReportStreamParams>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, ApiError> {
    let token = jar
        .get(AUTH_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::Unauthorized("No session found".into()))?;

    tracing::info!(
        tenant = %params.tenant_id,
        from = %params.from_date,
        to = %params.to_date,
        "Starting SSE Report generation stream"
    );

    // Clone all values for the async stream
    let tenant_id = params.tenant_id.clone();
    let from_date = params.from_date.clone();
    let to_date = params.to_date.clone();
    let language_str = params.language.clone();
    let story_tag = params.story_tag.clone();
    let include_threat_intel = params.include_threat_intel;
    let template_id = params.template_id.clone();
    let use_plugins = params.use_plugins;
    let _plugin_theme = params.plugin_theme.clone();
    let _disabled_slides: Option<Vec<String>> = params
        .disabled_slides
        .as_ref()
        .map(|s| s.split(',').map(|x| x.trim().to_string()).collect());
    // let pool = state.pool.clone(); // Removed: No longer using SQL pool

    let stream = async_stream::stream! {
        // Define stages
        let stages = vec![
            "validating".to_string(),
            "fetching_data".to_string(),
            "processing".to_string(),
            "generating_html".to_string(),
        ];

        // Emit started event
        let started = ReportStreamEvent::Started { stages: stages.clone() };
        if let Ok(json) = serde_json::to_string(&started) {
            yield Ok(Event::default().data(json));
        }

        // Stage 1: Validating
        let progress = ReportStreamEvent::StageProgress {
            stage: "validating".into(),
            message: "Validating request parameters...".into(),
            progress_pct: 10,
        };
        if let Ok(json) = serde_json::to_string(&progress) {
            yield Ok(Event::default().data(json));
        }

        if tenant_id.is_empty() || from_date.is_empty() || to_date.is_empty() {
            let err = ReportStreamEvent::Error {
                code: "RPT-001".into(),
                message: "Invalid request parameters".into(),
            };
            if let Ok(json) = serde_json::to_string(&err) {
                yield Ok(Event::default().data(json));
            }
            return;
        }

        let complete = ReportStreamEvent::StageComplete { stage: "validating".into() };
        if let Ok(json) = serde_json::to_string(&complete) {
            yield Ok(Event::default().data(json));
        }

        // Stage 2: Fetching data
        let progress = ReportStreamEvent::StageProgress {
            stage: "fetching_data".into(),
            message: "Fetching incidents and metrics from Axur API...".into(),
            progress_pct: 25,
        };
        if let Ok(json) = serde_json::to_string(&progress) {
            yield Ok(Event::default().data(json));
        }

        let report_data = match fetch_full_report(
            &token,
            &tenant_id,
            &from_date,
            &to_date,
            story_tag.clone(),
            include_threat_intel,
        )
        .await
        {
            Ok(data) => data,
            Err(e) => {
                let error_code = classify_error(&e.to_string());
                let err = ReportStreamEvent::Error {
                    code: error_code.code(),
                    message: e.to_string(),
                };
                if let Ok(json) = serde_json::to_string(&err) {
                    yield Ok(Event::default().data(json));
                }
                return;
            }
        };

        let complete = ReportStreamEvent::StageComplete { stage: "fetching_data".into() };
        if let Ok(json) = serde_json::to_string(&complete) {
            yield Ok(Event::default().data(json));
        }

        // Stage 3: Processing
        let progress = ReportStreamEvent::StageProgress {
            stage: "processing".into(),
            message: "Processing report data...".into(),
            progress_pct: 60,
        };
        if let Ok(json) = serde_json::to_string(&progress) {
            yield Ok(Event::default().data(json));
        }

        // Get dictionary for selected language
        let language = match language_str.to_lowercase().as_str() {
            "en" => Language::En,
            "pt" | "pt-br" => Language::PtBr,
            _ => Language::Es,
        };
        let dict = get_dictionary(language);

        // Handle custom template if provided
        let mut custom_template_slides: Option<Vec<String>> = None;
        if let Some(ref tid) = template_id {
            // Try mock templates first
            if let Some(tmpl) = crate::routes::templates::get_mock_template(tid) {
                let slides: Vec<String> = tmpl
                    .slides
                    .iter()
                    .filter_map(|s| s.canvas_json.clone())
                    .collect();
                if !slides.is_empty() {
                    custom_template_slides = Some(slides);
                }
            }
            // Try DB templates (Firestore)
            if custom_template_slides.is_none() {
                if let Some(firestore) = crate::firebase::get_firestore() {
                    // Try to finding user_id? We don't have user_id easily here in this context unless we lookup the template globally or search all users?
                    // The original SQL looked up by template ID without user ID?
                    // Wait, original SQL was: `SELECT content FROM user_templates WHERE id = $1`
                    // In Firestore, our schema is `user_templates/{user_id}/items/{template_id}`.
                    // We CANNOT efficienty get a document by ID if we don't know the parent collection path (the user_id).
                    // This is a schema mismatch.

                    // HOWEVER, `user_templates` table in Postgres likely had `id` as primary key global.
                    // In Firestore, we sharded by user.
                    // If we want to support "public" or "shared" templates by ID without knowing user, we need a global index or a "lookup" collection.

                    // For now, let's assume `template_id` might be a "system" template or we have to search.
                    // BUT, `marketplace.rs` has `marketplace_templates`.

                    // Workaround: Since we don't have the user_id of the template owner here (we only have `params`),
                    // we might fail to find private user templates here unless we change the API to pass owner_id.
                    // But `list_docs` on group collection `items`? No, `items` is subcollection.
                    // Firestore Group Query: `firestore.collectionGroup('items').where('id', '==', tid)`
                    // My `firebase.rs` client handles simple paths. It doesn't support collectionGroup queries easily yet.

                    // Quick fix: For now, we only support MOCK templates or we logging a warning that DB template loading is temporarily limited by ID only.
                    // Or, if the User is logged in (we have `token`), we *could* try to use the *current* user's ID as the owner?
                    // In `generate_report_stream`, we decode the session cookie to `token`. We can validate it to get `user_id`.

                    // Let's get the user_id from the cookie
                    let user_id_opt = jar
                        .get(crate::middleware::AUTH_USER_COOKIE_NAME)
                        .map(|c| c.value().to_string())
                        .map(|email| crate::github_storage::GitHubStorage::hash_user_id(&email));

                    if let Some(uid) = user_id_opt {
                         // Try fetching from THIS user's templates
                         let path = format!("user_templates/{}/items", uid);
                         if let Ok(Some(doc)) = firestore.get_doc::<serde_json::Value>(&path, tid).await {
                              if let Some(content) = doc.get("content").and_then(|c| c.as_array()) {
                                  let slides: Vec<String> = content
                                      .iter()
                                      .filter_map(|s| s.get("canvas_json").and_then(|c| c.as_str()).map(|s| s.to_string()))
                                      .collect();
                                   if !slides.is_empty() {
                                       custom_template_slides = Some(slides);
                                   }
                              }
                         }
                    } else {
                        // User not found or invalid session, can't look up private template
                    }
                }
            }
        }

        let complete = ReportStreamEvent::StageComplete { stage: "processing".into() };
        if let Ok(json) = serde_json::to_string(&complete) {
            yield Ok(Event::default().data(json));
        }

        // Stage 4: Generating HTML
        let progress = ReportStreamEvent::StageProgress {
            stage: "generating_html".into(),
            message: "Generating HTML report...".into(),
            progress_pct: 85,
        };
        if let Ok(json) = serde_json::to_string(&progress) {
            yield Ok(Event::default().data(json));
        }

        // Load offline assets (embedded for self-contained HTML)
        let offline_assets = OfflineAssets::load_embedded();

        // Generate HTML
        let html = if use_plugins {
            let lang_code = match language {
                Language::En => "en",
                Language::PtBr => "pt-br",
                Language::Es => "es",
            };
            let translations = match Translations::load(lang_code) {
                Ok(t) => t,
                Err(_) => Translations::load("en").expect("CRITICAL: Default English translations missing"),
            };
            generate_report_with_plugins(
                &report_data,
                &translations,
                Some(&offline_assets),
                None, // No custom config
            )
        } else {
            generate_full_report_html(
                &report_data,
                custom_template_slides,
                Some(&offline_assets),
                &dict,
            )
        };

        let complete = ReportStreamEvent::StageComplete { stage: "generating_html".into() };
        if let Ok(json) = serde_json::to_string(&complete) {
            yield Ok(Event::default().data(json));
        }

        // Finished!
        let finished = ReportStreamEvent::Finished {
            html,
            company_name: Some(report_data.company_name.clone()),
        };
        if let Ok(json) = serde_json::to_string(&finished) {
            yield Ok(Event::default().data(json));
        }
    };

    Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}
