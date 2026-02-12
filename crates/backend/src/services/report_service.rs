use crate::error::ApiError;
use crate::routes::templates::{self, GitHubConfig};
use axur_core::api::report::fetch_full_report;
use axur_core::error_codes::{self, ErrorCode};
use axur_core::i18n::{get_dictionary, Language, Translations};
use axur_core::plugins::{PluginConfig, ThemeMode};
use axur_core::report::html::{generate_full_report_html, generate_report_with_plugins};
use axur_core::report::OfflineAssets;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

// ========================
// TYPES (Moved from routes/report.rs)
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
    pub template_id: Option<String>,
    #[serde(default)]
    pub use_plugins: bool,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub disabled_plugins: Option<Vec<String>>,
    #[serde(default)]
    pub mock: bool,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

pub struct ReportService;

impl ReportService {
    /// Generate HTML report safely (No Panic)
    pub async fn generate_report(
        payload: &GenerateReportRequest,
        token: &str,
        user_id: &str,
    ) -> Result<GenerateReportResponse, ApiError> {
        let _start_time = Instant::now();

        // 1. Fetch Data
        // 1. Fetch Data
        let report_data = if payload.mock {
            tracing::info!("Using MOCK DATA for report generation");
            axur_core::api::report::PocReportData::demo()
        } else {
            match fetch_full_report(
                token,
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

                    // 📝 Log error (Assuming log_error is handled by caller or we inject logger)
                    // Ideally logging relates to HTTP layer, but Service can log domain errors.

                    return Ok(GenerateReportResponse {
                        success: false,
                        html: None,
                        company_name: None,
                        message: e.to_string(),
                        error_code: Some(error_code.code()),
                        error_message: Some(get_user_friendly_message(&error_code)),
                    });
                }
            }
        };

        // 2. Load Language Safely
        let language = match payload.language.to_lowercase().as_str() {
            "en" => Language::En,
            "pt" | "pt-br" => Language::PtBr,
            _ => Language::Es,
        };

        let lang_code = match language {
            Language::En => "en",
            Language::PtBr => "pt-br",
            Language::Es => "es",
        };

        // SAFETY FIX: Handle Translations::load gracefully
        let translations = Translations::load(lang_code)
            .or_else(|_| Translations::load("en"))
            .map_err(|e| ApiError::Internal(format!("Failed to load translations: {}", e)))?;

        let dict = get_dictionary(language);

        // 3. Template Logic (Simplified Migration)
        let mut custom_template_slides: Option<Vec<String>> = None;
        if let Some(tid) = &payload.template_id {
            // Mock Templates
            if let Some(tmpl) = crate::routes::templates::get_mock_template(tid) {
                let slides: Vec<String> = tmpl
                    .slides
                    .iter()
                    .filter_map(|s| s.canvas_json.clone())
                    .collect();
                if !slides.is_empty() {
                    // SAFETY FIX: removed unwrap().len() logging to keep concise or use safe access
                    tracing::info!(
                        "Using mock template '{}' with {} slides",
                        tmpl.name,
                        slides.len()
                    );
                    custom_template_slides = Some(slides);
                }
            }
            // Firestore Templates (if not mock)
            if custom_template_slides.is_none() {
                if let Ok(uuid) = Uuid::parse_str(tid) {
                    if let Some(firestore) = crate::firebase::get_firestore() {
                        let doc_path = format!("user_templates/{}/items", user_id);
                        let doc_id = uuid.to_string();
                        if let Ok(Some(doc)) = firestore
                            .get_doc::<serde_json::Value>(&doc_path, &doc_id)
                            .await
                        {
                            if let Some(path) = doc.get("github_path").and_then(|s| s.as_str()) {
                                if let Some(config) = GitHubConfig::from_env() {
                                    match templates::fetch_template_from_github(&config, path).await
                                    {
                                        Ok(tmpl) => {
                                            let slides: Vec<String> = tmpl
                                                .slides
                                                .iter()
                                                .filter_map(|s| s.canvas_json.clone())
                                                .collect();
                                            if !slides.is_empty() {
                                                custom_template_slides = Some(slides);
                                                tracing::info!(
                                                    "Using custom private template: {}",
                                                    tmpl.name
                                                );
                                            }
                                        }
                                        Err(e) => tracing::error!("Failed from GitHub: {}", e),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 4. Generate HTML
        let offline_assets = OfflineAssets::load_embedded();

        let html = if payload.use_plugins && custom_template_slides.is_none() {
            let theme_mode = match payload.theme.as_deref() {
                Some("light") => ThemeMode::Light,
                Some("auto") => ThemeMode::Auto,
                _ => ThemeMode::Dark, // Default
            };
            let config = PluginConfig::default()
                .with_theme(theme_mode)
                .disable_plugins(payload.disabled_plugins.clone().unwrap_or_default());

            generate_report_with_plugins(
                &report_data,
                &translations,
                Some(&offline_assets),
                Some(config),
            )
        } else {
            generate_full_report_html(
                &report_data,
                custom_template_slides,
                Some(&offline_assets),
                &dict,
            )
        };

        Ok(GenerateReportResponse {
            success: true,
            html: Some(html),
            company_name: Some(report_data.company_name),
            message: "Report generated successfully".into(),
            error_code: None,
            error_message: None,
        })
    }
}

// ========================
// HELPERS (Moved from routes/report.rs)
// ========================

pub fn classify_error(error: &str) -> ErrorCode {
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

pub fn get_user_friendly_message(code: &ErrorCode) -> String {
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
