//! Template CRUD API routes (Simplified version)
//!
//! Uses dynamic queries to avoid compile-time database checks

use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::AppState;
use axur_core::editor::{get_user_template_path, PresentationTemplate};

// ==================== TYPES ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateListItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub preview_image_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RawSlide {
    pub id: Option<String>,
    pub name: String,
    pub canvas_json: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub description: Option<String>,
    pub slides: Vec<RawSlide>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub template: Option<PresentationTemplate>,
}

/// Frontend-compatible template detail (uses serde_json::Value for slides)
#[derive(Debug, Clone, Serialize)]
pub struct TemplateDetail {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub slides: Vec<serde_json::Value>,
}

impl TemplateDetail {
    /// Convert from PresentationTemplate to frontend-compatible TemplateDetail
    pub fn from_template(id: &str, template: &PresentationTemplate) -> Self {
        let slides: Vec<serde_json::Value> = template
            .slides
            .iter()
            .enumerate()
            .map(|(_i, slide)| {
                serde_json::json!({
                    "id": slide.id.to_string(),
                    "name": slide.name.clone(),
                    "canvas_json": slide.canvas_json.clone().unwrap_or_default()
                })
            })
            .collect();

        Self {
            id: id.to_string(),
            name: template.name.clone(),
            description: template.description.clone(),
            slides,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub success: bool,
    pub id: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<TemplateDetail>,
}

#[derive(Debug, Deserialize)]
pub struct ListTemplatesQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== GITHUB STORAGE ====================

#[derive(Clone)]
pub struct GitHubConfig {
    pub token: String,
    pub owner: String,
    pub repo: String,
}

impl GitHubConfig {
    pub fn from_env() -> Option<Self> {
        Some(Self {
            token: std::env::var("GITHUB_TOKEN").ok()?,
            owner: std::env::var("GITHUB_OWNER").unwrap_or_else(|_| "maisonnat".to_string()),
            repo: std::env::var("GITHUB_LOGS_REPO")
                .unwrap_or_else(|_| "axur-logs-private".to_string()),
        })
    }
}

async fn get_file_sha(config: &GitHubConfig, path: &str) -> Option<String> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-bot")
        .send()
        .await
        .ok()?;

    if res.status().is_success() {
        let json: serde_json::Value = res.json().await.ok()?;
        json.get("sha")?.as_str().map(|s| s.to_string())
    } else {
        None
    }
}

async fn upload_template_to_github(
    config: &GitHubConfig,
    path: &str,
    template: &PresentationTemplate,
) -> Result<String, String> {
    let content = serde_json::to_string_pretty(template)
        .map_err(|e| format!("Serialization error: {}", e))?;

    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    let encoded = BASE64.encode(content.as_bytes());
    let sha = get_file_sha(config, path).await;

    let mut body = serde_json::json!({
        "message": format!("Update template: {}", template.name),
        "content": encoded,
        "committer": { "name": "Axur Bot", "email": "bot@axur.local" }
    });

    if let Some(s) = sha {
        body["sha"] = serde_json::json!(s);
    }

    let client = reqwest::Client::new();
    let res = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-bot")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        Ok(json
            .get("content")
            .and_then(|c| c.get("html_url"))
            .and_then(|u| u.as_str())
            .unwrap_or("")
            .to_string())
    } else {
        Err(format!("GitHub upload failed: {}", res.status()))
    }
}

async fn upload_file_to_github(
    config: &GitHubConfig,
    path: &str,
    content: &[u8],
    commit_msg: &str,
) -> Result<String, String> {
    let encoded = BASE64.encode(content);
    let sha = get_file_sha(config, path).await;

    let mut body = serde_json::json!({
        "message": commit_msg,
        "content": encoded,
        "committer": { "name": "Axur Bot", "email": "bot@axur.local" }
    });

    if let Some(s) = sha {
        body["sha"] = serde_json::json!(s);
    }

    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    let client = reqwest::Client::new();
    let res = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-bot")
        .header("Accept", "application/vnd.github.v3+json")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        Ok(json
            .get("content")
            .and_then(|c| c.get("html_url"))
            .and_then(|u| u.as_str())
            .unwrap_or("")
            .to_string())
    } else {
        Err(format!("GitHub upload failed: {}", res.status()))
    }
}

/// Fetch raw file content (binary) from GitHub
pub async fn fetch_raw_file_from_github(
    config: &GitHubConfig,
    path: &str,
) -> Result<Vec<u8>, String> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-bot")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("GitHub fetch failed: {}", res.status()));
    }

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

    // Check if it's a file
    let type_ = json.get("type").and_then(|t| t.as_str()).unwrap_or("");
    if type_ != "file" {
        return Err("Not a file".to_string());
    }

    let content_b64 = json
        .get("content")
        .and_then(|c| c.as_str())
        .map(|s| s.replace('\n', ""))
        .ok_or_else(|| "No content found".to_string())?;

    BASE64
        .decode(content_b64)
        .map_err(|e| format!("Base64 decode failed: {}", e))
}

#[allow(dead_code)]
pub async fn fetch_template_from_github(
    config: &GitHubConfig,
    path: &str,
) -> Result<PresentationTemplate, String> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-bot")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Template not found: {}", res.status()));
    }

    let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let encoded = json
        .get("content")
        .and_then(|c| c.as_str())
        .ok_or("Missing content")?;

    let clean: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
    let decoded = BASE64.decode(&clean).map_err(|e| e.to_string())?;
    let content = String::from_utf8(decoded).map_err(|e| e.to_string())?;

    serde_json::from_str(&content).map_err(|e| e.to_string())
}

// ==================== ENDPOINTS ====================

/// GET /api/templates - List user's templates
pub async fn list_templates(
    State(_state): State<AppState>,
    Extension(user_id): Extension<String>,
    Query(_params): Query<ListTemplatesQuery>,
) -> impl IntoResponse {
    // Try Firestore first
    if let Some(firestore) = crate::firebase::get_firestore() {
        // Collection path: user_templates/{user_id} - documents are the templates
        let collection = format!("user_templates/{}/items", user_id);

        match firestore.list_docs::<TemplateListItem>(&collection).await {
            Ok(templates) => {
                tracing::debug!(
                    "Found {} templates for user {} from Firestore",
                    templates.len(),
                    user_id
                );
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "success": true,
                        "templates": templates,
                        "total": templates.len(),
                        "source": "firestore"
                    })),
                );
            }
            Err(crate::firebase::FirestoreError::RateLimited) => {
                tracing::warn!("Firestore rate limited, returning empty list");
            }
            Err(e) => {
                tracing::warn!("Firestore error: {}, returning empty list", e);
            }
        }
    } else {
        tracing::debug!("Firestore not configured, returning empty list");
    }

    // Fallback: return empty list
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "templates": [],
            "total": 0,
            "source": "fallback"
        })),
    )
}

/// POST /api/templates - Create a new template (Multipart)
/// POST /api/templates - Create a new template (Multipart)
pub async fn create_template(
    State(_state): State<AppState>,
    Extension(user_id): Extension<String>,
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    // 1. Parse Multipart Data
    let mut req: Option<CreateTemplateRequest> = None;
    let mut file_data: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "metadata" {
            let data = field.text().await.unwrap_or_default();
            if let Ok(parsed) = serde_json::from_str::<CreateTemplateRequest>(&data) {
                req = Some(parsed);
            }
        } else if name == "file" {
            if let Ok(bytes) = field.bytes().await {
                file_data = Some(bytes.to_vec());
            }
        }
    }

    let req = match req {
        Some(r) => r,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Metadata field required".to_string(),
                    template: None,
                }),
            )
        }
    };

    if req.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(TemplateResponse {
                success: false,
                id: None,
                message: "Name is required".to_string(),
                template: None,
            }),
        );
    }

    // 2. Setup GitHub & Firestore
    let config = match GitHubConfig::from_env() {
        Some(c) => c,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "GitHub not configured".to_string(),
                    template: None,
                }),
            );
        }
    };

    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Storage not available".to_string(),
                    template: None,
                }),
            );
        }
    };

    let template_id = Uuid::new_v4();
    // GitHub Path: templates/{user_id}/{template_id}/metadata.json
    let metadata_path = format!("templates/{}/{}/metadata.json", user_id, template_id);
    let pptx_path = format!("templates/{}/{}/base.pptx", user_id, template_id);

    // 3. Create Template Object
    // Parse slides if available (indirectly via CreateTemplateRequest having slides)
    // We reuse the slide parsing from previous implementation or assume simplified object for metadata
    // The previous implementation for GitHub upload had logic to convert RawSlide to SlideDefinition.
    // If we want to support full editor templates, we need that logic.
    // For now, minimal object to satisfy types:
    let slides: Vec<axur_core::editor::SlideDefinition> = req
        .slides
        .iter()
        .enumerate()
        .map(|(i, s)| axur_core::editor::SlideDefinition {
            id: s
                .id
                .as_ref()
                .and_then(|i| Uuid::parse_str(i).ok())
                .unwrap_or_else(Uuid::new_v4),
            name: s.name.clone(),
            canvas_json: s.canvas_json.as_ref().map(|v| v.to_string()),
            order: i as i32,
            ..Default::default()
        })
        .collect();

    let template_obj = PresentationTemplate {
        id: template_id,
        name: req.name.clone(),
        description: req.description.clone(),
        slides,
        theme: axur_core::editor::Theme::default(),
        version: "1.0.0".to_string(),
    };

    // 4. Upload Metadata to GitHub
    if let Err(e) = upload_template_to_github(&config, &metadata_path, &template_obj).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TemplateResponse {
                success: false,
                id: None,
                message: format!("Metadata upload failed: {}", e),
                template: None,
            }),
        );
    }

    // 5. Upload Base PPTX if provided
    if let Some(bytes) = file_data {
        if let Err(e) = upload_file_to_github(
            &config,
            &pptx_path,
            &bytes,
            &format!("Add base PPTX for {}", req.name),
        )
        .await
        {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: format!("PPTX upload failed: {}", e),
                    template: None,
                }),
            );
        }
    }

    // 6. Save metadata to Firestore
    // Path: user_templates/{user_id}/items/{template_id}
    let created_at = chrono::Utc::now().to_rfc3339();
    let template_doc = serde_json::json!({
        "id": template_id.to_string(),
        "name": req.name,
        "description": req.description,
        "github_path": metadata_path,
        "created_at": created_at,
        "updated_at": created_at
    });

    match firestore
        .set_doc(
            &format!("user_templates/{}/items", user_id),
            &template_id.to_string(),
            &template_doc,
        )
        .await
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(TemplateResponse {
                success: true,
                id: Some(template_id.to_string()),
                message: "Template created".to_string(),
                template: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TemplateResponse {
                success: false,
                id: None,
                message: format!("Failed to save metadata: {}", e),
                template: None,
            }),
        ),
    }
}

/// Helper to get mock templates
pub fn get_mock_template(id: &str) -> Option<PresentationTemplate> {
    // ===== AXUR OFFICIAL DESIGN SYSTEM =====
    // Background: #09090b (zinc-950), #18181b (zinc-900)
    // Cards: #27272a (zinc-800) with rx/ry: 12
    // Accent: #f97316 (orange-500)
    // Text: #ffffff (primary), #a1a1aa (zinc-400 secondary), #52525b (zinc-600 muted)
    // Success: #22c55e (green-500)
    // Warning: #f59e0b (amber-500)
    // Danger: #ef4444 (red-500)

    // Template 1: Axur Official - Cover Slide Style
    let json_official = r##"{
      "version": "5.3.0",
      "objects": [
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 720, "fill": "#09090b", "selectable": false },
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 8, "fill": "#f97316", "selectable": false },
        { "type": "rect", "left": 60, "top": 80, "width": 90, "height": 32, "fill": "#f97316", "rx": 2, "ry": 2 },
        { "type": "text", "left": 75, "top": 85, "fill": "#ffffff", "text": "TLP:AMBER", "fontSize": 14, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 60, "top": 150, "fill": "#ffffff", "text": "INFORME", "fontSize": 64, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "text", "left": 60, "top": 220, "fill": "#ffffff", "text": "EJECUTIVO", "fontSize": 64, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "text", "left": 60, "top": 320, "fill": "#f97316", "text": "Compañía", "fontSize": 16, "fontFamily": "Inter", "fontWeight": "600" },
        { "type": "text", "left": 60, "top": 345, "fill": "#ffffff", "text": "{{company_name}}", "fontSize": 32, "fontFamily": "Inter" },
        { "type": "text", "left": 60, "top": 410, "fill": "#f97316", "text": "Período", "fontSize": 16, "fontFamily": "Inter", "fontWeight": "600" },
        { "type": "text", "left": 60, "top": 435, "fill": "#ffffff", "text": "{{date_range}}", "fontSize": 24, "fontFamily": "Inter" },
        { "type": "text", "left": 60, "top": 660, "fill": "#f97316", "text": "/// AXUR", "fontSize": 24, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 900, "top": 665, "fill": "#52525b", "text": "Digital experiences made safe.", "fontSize": 12, "fontFamily": "Inter" }
      ]
    }"##;

    // Template 2: Executive Summary - KPI Dashboard
    let json_executive = r##"{
      "version": "5.3.0",
      "objects": [
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 720, "fill": "#18181b", "selectable": false },
        { "type": "text", "left": 60, "top": 40, "fill": "#f97316", "text": "Resumen Ejecutivo", "fontSize": 36, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "rect", "left": 60, "top": 110, "width": 360, "height": 180, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 90, "top": 140, "fill": "#a1a1aa", "text": "SEÑALES DETECTADAS", "fontSize": 12, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 90, "top": 180, "fill": "#ffffff", "text": "{{signals}}", "fontSize": 56, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "rect", "left": 460, "top": 110, "width": 360, "height": 180, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 490, "top": 140, "fill": "#a1a1aa", "text": "INCIDENTES CONFIRMADOS", "fontSize": 12, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 490, "top": 180, "fill": "#f97316", "text": "{{incidents}}", "fontSize": 56, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "rect", "left": 860, "top": 110, "width": 360, "height": 180, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 890, "top": 140, "fill": "#a1a1aa", "text": "AMENAZAS ACTIVAS", "fontSize": 12, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 890, "top": 180, "fill": "#ef4444", "text": "{{threats}}", "fontSize": 56, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "rect", "left": 60, "top": 320, "width": 360, "height": 180, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 90, "top": 350, "fill": "#a1a1aa", "text": "CREDENCIALES EXPUESTAS", "fontSize": 12, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 90, "top": 390, "fill": "#f59e0b", "text": "{{credentials}}", "fontSize": 56, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "rect", "left": 460, "top": 320, "width": 360, "height": 180, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 490, "top": 350, "fill": "#a1a1aa", "text": "TAKEDOWNS RESUELTOS", "fontSize": 12, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 490, "top": 390, "fill": "#22c55e", "text": "{{takedowns}}", "fontSize": 56, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "rect", "left": 860, "top": 320, "width": 360, "height": 180, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 890, "top": 350, "fill": "#a1a1aa", "text": "FUGAS DE CÓDIGO", "fontSize": 12, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 890, "top": 390, "fill": "#a855f7", "text": "{{code_leaks}}", "fontSize": 56, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "text", "left": 60, "top": 660, "fill": "#f97316", "text": "/// AXUR", "fontSize": 20, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 1180, "top": 665, "fill": "#52525b", "text": "2", "fontSize": 12, "fontFamily": "Inter" }
      ]
    }"##;

    // Template 3: Risk Assessment - Focus on Risk Score
    let json_risk = r##"{
      "version": "5.3.0",
      "objects": [
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 720, "fill": "#18181b", "selectable": false },
        { "type": "text", "left": 60, "top": 40, "fill": "#f97316", "text": "Evaluación de Riesgo", "fontSize": 36, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "rect", "left": 60, "top": 110, "width": 500, "height": 450, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 230, "top": 140, "fill": "#a1a1aa", "text": "RISK SCORE", "fontSize": 14, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 180, "top": 220, "fill": "#f59e0b", "text": "{{risk_score}}", "fontSize": 140, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "text", "left": 210, "top": 400, "fill": "#ffffff", "text": "{{risk_label}}", "fontSize": 32, "fontFamily": "Inter", "fontWeight": "600" },
        { "type": "text", "left": 150, "top": 480, "fill": "#a1a1aa", "text": "Vectores: Phishing, Brand Abuse", "fontSize": 14, "fontFamily": "Inter" },
        { "type": "rect", "left": 600, "top": 110, "width": 620, "height": 210, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 640, "top": 140, "fill": "#a1a1aa", "text": "PRINCIPALES AMENAZAS", "fontSize": 12, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 640, "top": 180, "fill": "#ef4444", "text": "• Credential Leak (Critical)", "fontSize": 18, "fontFamily": "Inter" },
        { "type": "text", "left": 640, "top": 220, "fill": "#f59e0b", "text": "• Phishing Campaign #129", "fontSize": 18, "fontFamily": "Inter" },
        { "type": "text", "left": 640, "top": 260, "fill": "#a1a1aa", "text": "• Suspicious Domain Registration", "fontSize": 18, "fontFamily": "Inter" },
        { "type": "rect", "left": 600, "top": 350, "width": 620, "height": 210, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 640, "top": 380, "fill": "#a1a1aa", "text": "RECOMENDACIONES", "fontSize": 12, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 640, "top": 420, "fill": "#ffffff", "text": "1. Reset admin passwords", "fontSize": 16, "fontFamily": "Inter" },
        { "type": "text", "left": 640, "top": 460, "fill": "#ffffff", "text": "2. Enable 2FA on VPN", "fontSize": 16, "fontFamily": "Inter" },
        { "type": "text", "left": 640, "top": 500, "fill": "#ffffff", "text": "3. Review access logs", "fontSize": 16, "fontFamily": "Inter" },
        { "type": "text", "left": 60, "top": 660, "fill": "#f97316", "text": "/// AXUR", "fontSize": 20, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 1180, "top": 665, "fill": "#52525b", "text": "3", "fontSize": 12, "fontFamily": "Inter" }
      ]
    }"##;

    // Template 4: Takedowns Performance
    let json_takedowns = r##"{
      "version": "5.3.0",
      "objects": [
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 720, "fill": "#18181b", "selectable": false },
        { "type": "text", "left": 60, "top": 40, "fill": "#f97316", "text": "Takedowns", "fontSize": 36, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "rect", "left": 60, "top": 110, "width": 360, "height": 130, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 90, "top": 140, "fill": "#22c55e", "text": "{{resolved}}", "fontSize": 48, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "text", "left": 200, "top": 155, "fill": "#a1a1aa", "text": "Resueltos", "fontSize": 20, "fontFamily": "Inter" },
        { "type": "rect", "left": 60, "top": 260, "width": 360, "height": 130, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 90, "top": 290, "fill": "#f59e0b", "text": "{{pending}}", "fontSize": 48, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "text", "left": 200, "top": 305, "fill": "#a1a1aa", "text": "Pendientes", "fontSize": 20, "fontFamily": "Inter" },
        { "type": "rect", "left": 60, "top": 410, "width": 360, "height": 130, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 90, "top": 440, "fill": "#ef4444", "text": "{{aborted}}", "fontSize": 48, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "text", "left": 200, "top": 455, "fill": "#a1a1aa", "text": "Cancelados", "fontSize": 20, "fontFamily": "Inter" },
        { "type": "rect", "left": 480, "top": 110, "width": 740, "height": 430, "fill": "#27272a", "rx": 12, "ry": 12 },
        { "type": "text", "left": 750, "top": 200, "fill": "#f97316", "text": "{{success_rate}}%", "fontSize": 120, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "text", "left": 720, "top": 380, "fill": "#a1a1aa", "text": "Tasa de Éxito", "fontSize": 28, "fontFamily": "Inter" },
        { "type": "text", "left": 60, "top": 660, "fill": "#f97316", "text": "/// AXUR", "fontSize": 20, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 1180, "top": 665, "fill": "#52525b", "text": "4", "fontSize": 12, "fontFamily": "Inter" }
      ]
    }"##;

    // Template 5: Closing / Thank You Slide
    let json_closing = r##"{
      "version": "5.3.0",
      "objects": [
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 720, "fill": "#09090b", "selectable": false },
        { "type": "text", "left": 540, "top": 200, "fill": "#f97316", "text": "/// AXUR", "fontSize": 48, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "text", "left": 520, "top": 300, "fill": "#ffffff", "text": "Gracias", "fontSize": 72, "fontFamily": "Inter", "fontWeight": "900" },
        { "type": "text", "left": 450, "top": 400, "fill": "#a1a1aa", "text": "{{company_name}}", "fontSize": 32, "fontFamily": "Inter" },
        { "type": "text", "left": 380, "top": 480, "fill": "#52525b", "text": "Informe generado automáticamente por Axur CLI", "fontSize": 18, "fontFamily": "Inter" },
        { "type": "text", "left": 530, "top": 530, "fill": "#3f3f46", "text": "{{date_range}}", "fontSize": 16, "fontFamily": "Inter" },
        { "type": "text", "left": 340, "top": 660, "fill": "#3f3f46", "text": "Axur. Digital experiences made safe. All rights reserved.", "fontSize": 12, "fontFamily": "Inter" }
      ]
    }"##;

    match id {
        "1" | "axur_official" => Some(PresentationTemplate {
            name: "Axur Official".to_string(),
            slides: vec![axur_core::editor::SlideDefinition {
                canvas_json: Some(json_official.to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        "2" | "executive" => Some(PresentationTemplate {
            name: "Executive Summary".to_string(),
            slides: vec![axur_core::editor::SlideDefinition {
                canvas_json: Some(json_executive.to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        "3" | "risk" => Some(PresentationTemplate {
            name: "Risk Focus".to_string(),
            slides: vec![axur_core::editor::SlideDefinition {
                canvas_json: Some(json_risk.to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        "4" | "technical" => Some(PresentationTemplate {
            name: "Technical Deep Dive".to_string(),
            slides: vec![axur_core::editor::SlideDefinition {
                canvas_json: Some(json_takedowns.to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        "5" | "compliance" => Some(PresentationTemplate {
            name: "Compliance Report".to_string(),
            slides: vec![axur_core::editor::SlideDefinition {
                canvas_json: Some(json_closing.to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        _ => None,
    }
}

/// GET /api/templates/:id
pub async fn get_template(Path(template_id): Path<String>) -> impl IntoResponse {
    // Check for mock templates first (no auth/db needed for these)
    if let Some(mock) = get_mock_template(&template_id) {
        return (
            StatusCode::OK,
            Json(TemplateResponse {
                success: true,
                id: Some(template_id.clone()),
                message: "Template loaded (Mock)".to_string(),
                template: Some(TemplateDetail::from_template(&template_id, &mock)),
            }),
        );
    }

    // For non-mock templates, return not found on public route
    // User templates should be accessed via authenticated route
    (
        StatusCode::NOT_FOUND,
        Json(TemplateResponse {
            success: false,
            id: None,
            message: format!("Template '{}' not found", template_id),
            template: None,
        }),
    )
}

/// PUT /api/templates/:id
pub async fn update_template(
    State(_state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(template_id): Path<String>,
    Json(req): Json<UpdateTemplateRequest>,
) -> impl IntoResponse {
    let config = match GitHubConfig::from_env() {
        Some(c) => c,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "GitHub not configured".to_string(),
                    template: None,
                }),
            );
        }
    };

    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Storage not available".to_string(),
                    template: None,
                }),
            );
        }
    };

    // Get current template metadata from Firestore
    // Path: user_templates/{user_id}/items/{template_id}
    let current_meta: serde_json::Value = match firestore
        .get_doc(&format!("user_templates/{}/items", user_id), &template_id)
        .await
    {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Not found".to_string(),
                    template: None,
                }),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: e.to_string(),
                    template: None,
                }),
            );
        }
    };

    let github_path = current_meta
        .get("github_path")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    // Update GitHub if content changed
    if let Some(ref template) = req.template {
        if !github_path.is_empty() {
            if let Err(e) = upload_template_to_github(&config, &github_path, template).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(TemplateResponse {
                        success: false,
                        id: None,
                        message: e,
                        template: None,
                    }),
                );
            }
        }
    }

    // Update metadata
    let current_name = current_meta
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let current_desc = current_meta.get("description").and_then(|v| v.as_str());

    let new_name = req
        .name
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or(current_name);
    let new_desc = req
        .description
        .as_ref()
        .map(|s| s.as_str())
        .or(current_desc);

    // Merge updates
    let mut update = current_meta.clone();
    if let Some(obj) = update.as_object_mut() {
        obj.insert("name".to_string(), serde_json::json!(new_name));
        obj.insert("description".to_string(), serde_json::json!(new_desc));
        obj.insert(
            "updated_at".to_string(),
            serde_json::json!(chrono::Utc::now().to_rfc3339()),
        );
    }

    // Save back to Firestore
    match firestore
        .update_doc(
            &format!("user_templates/{}/items", user_id),
            &template_id,
            &update,
        )
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(TemplateResponse {
                success: true,
                id: Some(template_id),
                message: "Updated".to_string(),
                template: None,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TemplateResponse {
                success: false,
                id: None,
                message: e.to_string(),
                template: None,
            }),
        ),
    }
}

/// DELETE /api/templates/:id
pub async fn delete_template(
    State(_state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(template_id): Path<String>,
) -> impl IntoResponse {
    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Storage not available".to_string(),
                    template: None,
                }),
            );
        }
    };

    // Delete from Firestore
    // Path: user_templates/{user_id}/items/{template_id}
    match firestore
        .delete_doc(&format!("user_templates/{}/items", user_id), &template_id)
        .await
    {
        Ok(_) => {
            // TODO: Delete from GitHub as well.
            // For now, metadata is gone so it won't show up.
            (
                StatusCode::OK,
                Json(TemplateResponse {
                    success: true,
                    id: Some(template_id),
                    message: "Deleted".to_string(),
                    template: None,
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR, // Or not found?
            Json(TemplateResponse {
                success: false,
                id: None,
                message: e.to_string(),
                template: None,
            }),
        ),
    }
}

// ==================== HELPERS ====================
// ensure_user_exists removed as it was only for SQL users table management

/// GET /api/templates/:id/pptx - Get the base PPTX file of a template
pub async fn get_template_pptx(
    State(_state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(template_id): Path<String>,
) -> impl IntoResponse {
    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": "Storage not available" })),
            );
        }
    };

    let config = match GitHubConfig::from_env() {
        Some(c) => c,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": "GitHub not configured" })),
            );
        }
    };

    // Get template info
    // Path: user_templates/{user_id}/items/{template_id}
    let meta: serde_json::Value = match firestore
        .get_doc(&format!("user_templates/{}/items", user_id), &template_id)
        .await
    {
        Ok(Some(doc)) => doc,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "success": false, "error": "Template not found" })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": e.to_string() })),
            );
        }
    };

    let github_path = meta
        .get("github_path")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    if github_path.is_empty() {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "success": false, "error": "No GitHub path for template" })),
        );
    }

    // Construct PPTX path from metadata path
    // metadata is at: templates/{user_id}/{template_id}/metadata.json
    // PPTX is at: templates/{user_id}/{template_id}/base.pptx
    let pptx_path = github_path.replace("metadata.json", "base.pptx");

    tracing::info!("Fetching PPTX from: {}", pptx_path);

    // Fetch PPTX from GitHub
    match fetch_raw_file_from_github(&config, &pptx_path).await {
        Ok(bytes) => {
            let base64_pptx = BASE64.encode(&bytes);
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "success": true,
                    "pptx_base64": base64_pptx,
                    "size_bytes": bytes.len()
                })),
            )
        }
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "success": false,
                "error": format!("PPTX file not found: {}", e)
            })),
        ),
    }
}

// ==================== AUTO-SAVE ENDPOINTS (Firestore-only, no GitHub) ====================

/// Request for quick save (auto-save feature)
#[derive(Debug, Deserialize)]
pub struct QuickSaveRequest {
    /// Template name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Slides content as JSON (array of slide objects with canvas_json)
    pub slides: serde_json::Value,
}

/// Response for quick-save operations
#[derive(Debug, Serialize)]
pub struct QuickSaveResponse {
    pub success: bool,
    pub id: String,
    pub message: String,
    pub saved_at: String,
}

/// POST /api/templates/quick-save - Create or update template in Firestore (draft)
/// This is the fast path for auto-save functionality
pub async fn quick_save_template(
    State(_state): State<AppState>,
    Extension(user_id): Extension<String>,
    Json(req): Json<QuickSaveRequest>,
) -> impl IntoResponse {
    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(QuickSaveResponse {
                    success: false,
                    id: String::new(),
                    message: "Storage not available".to_string(),
                    saved_at: String::new(),
                }),
            )
        }
    };

    // For quick save, we might need an ID. If specific ID not provided in request (not here yet),
    // we assume it's a new one or based on name?
    // Wait, the original code checked if name exists to update or insert.
    // Firestore scanning for name is expensive.
    // Ideally quick-save should take an ID if it's an update.
    // The request struct doesn't have ID.
    // So we search by name?
    // Firestore `list_docs` can be filtered? No, REST API basic list.
    // We can list all docs and find by name (inefficient) or assume client provides ID.
    // Original implementation: `SELECT id FROM user_templates WHERE user_id = $1 AND name = $2`

    // Compromise: We check if there's a param 'id' or we search.
    // Since we don't have search index easily without extra cost/setup maybe,
    // we will implement a suboptimal "List and Find" for now since user templates count is small per user.
    // Or we rely on client sending ID next time.
    // But the signature is `QuickSaveRequest` without ID.

    let path = format!("user_templates/{}/items", user_id);
    let mut template_id = Uuid::new_v4().to_string();
    let mut is_update = false;

    if let Ok(docs) = firestore.list_docs::<serde_json::Value>(&path).await {
        // Find by name
        for doc in docs {
            if let Some(name) = doc.get("name").and_then(|n| n.as_str()) {
                if name == req.name {
                    if let Some(id) = doc.get("id").and_then(|i| i.as_str()) {
                        template_id = id.to_string();
                        is_update = true;
                        break;
                    }
                }
            }
        }
    }

    let saved_at = chrono::Utc::now().to_rfc3339();

    // We store content directly in Firestore.
    // NOTE: Max 1MB. Large templates might fail.
    // If it fails, we should return error.
    let template_doc = serde_json::json!({
        "id": template_id,
        "name": req.name,
        "description": req.description,
        "content": {
            "slides": req.slides,
            "version": 1,
            "saved_at": saved_at.clone()
        },
        // We set github_path empty or "local" to indicate it's not on GitHub yet?
        // Or we reserve github_path for when it is synced.
        "updated_at": saved_at.clone(),
        // Only set created_at if new
    });

    // We need to merge with existing if update to preserve created_at and github_path
    let final_doc = if is_update {
        // We would ideally merge. `set_doc` overwrites?
        // Firestore REST update (patch) merges if mask present or separate method.
        // `update_doc` in our client uses PATCH.
        template_doc // We use this as partial update payload? `content` field + `updated_at`.
                     // But we want to set `content` field.
                     // Let's use `set_doc` which overwrites, effectively replacing the draft.
                     // BUT if github_path existed, we lose it?
                     // Let's first try to `update_doc` (PATCH).
    } else {
        template_doc
    };

    let result = if is_update {
        firestore.update_doc(&path, &template_id, &final_doc).await
    } else {
        // Add created_at
        let mut full_doc = final_doc;
        if let Some(obj) = full_doc.as_object_mut() {
            obj.insert(
                "created_at".to_string(),
                serde_json::json!(saved_at.clone()),
            );
            if !is_update {
                obj.insert("github_path".to_string(), serde_json::json!("")); // Explicit empty
            }
        }
        firestore.set_doc(&path, &template_id, &full_doc).await
    };

    match result {
        Ok(_) => {
            tracing::info!(
                "[AutoSave] {} template '{}' for user {}",
                if is_update { "Updated" } else { "Created" },
                req.name,
                user_id
            );
            (
                StatusCode::OK,
                Json(QuickSaveResponse {
                    success: true,
                    id: template_id,
                    message: if is_update {
                        "Template updated"
                    } else {
                        "Template created"
                    }
                    .to_string(),
                    saved_at,
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(QuickSaveResponse {
                success: false,
                id: String::new(),
                message: e.to_string(),
                saved_at: String::new(),
            }),
        ),
    }
}

/// GET /api/templates/quick-load/:id - Load template content from Firestore
pub async fn quick_load_template(
    State(_state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(template_id): Path<String>,
) -> impl IntoResponse {
    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": "Storage not available" })),
            )
        }
    };

    match firestore
        .get_doc::<serde_json::Value>(&format!("user_templates/{}/items", user_id), &template_id)
        .await
    {
        Ok(Some(doc)) => {
            let content = doc.get("content").cloned();
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "success": true,
                    "template": {
                         // We reconstruct structure expected by frontend if needed or just return content
                         // Original returned: id, name, description, content, updated_at
                         "id": doc.get("id"),
                         "name": doc.get("name"),
                         "description": doc.get("description"),
                         "content": content,
                         "updated_at": doc.get("updated_at")
                    }
                })),
            )
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "success": false, "error": "Not found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "error": e.to_string() })),
        ),
    }
}
