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
use sqlx::{PgPool, Row};
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
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Query(params): Query<ListTemplatesQuery>,
) -> impl IntoResponse {
    let pool = match &state.pool {
        Some(p) => p,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": "Database not available" })),
            )
        }
    };
    let limit = params.limit.unwrap_or(50).min(100) as i64;
    let offset = params.offset.unwrap_or(0) as i64;

    // Ensure user exists and get UUID
    tracing::debug!("Listing templates for user: {}", user_id);
    let user_uuid = match ensure_user_exists(pool, &user_id).await {
        Some(id) => id,
        None => {
            return (
                StatusCode::OK,
                Json(serde_json::json!({ "success": true, "templates": [], "total": 0 })),
            );
        }
    };

    let result = sqlx::query(
        r#"
        SELECT id::text, name, description, preview_image_url, 
               created_at::text, updated_at::text
        FROM user_templates WHERE user_id = $1
        ORDER BY updated_at DESC LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_uuid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await;

    match result {
        Ok(rows) => {
            tracing::debug!("Found {} templates for user uuid {}", rows.len(), user_uuid);
            let templates: Vec<TemplateListItem> = rows
                .iter()
                .map(|row| TemplateListItem {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    preview_image_url: row.get("preview_image_url"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                })
                .collect();
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "success": true,
                    "templates": templates,
                    "total": templates.len()
                })),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "success": false, "error": e.to_string() })),
        ),
    }
}

/// POST /api/templates - Create a new template (Multipart)
/// POST /api/templates - Create a new template (Multipart)
pub async fn create_template(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    let pool = match &state.pool {
        Some(p) => p,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Database not available".to_string(),
                    template: None,
                }),
            )
        }
    };
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

    let user_uuid = match ensure_user_exists(pool, &user_id).await {
        Some(id) => id,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "User creation failed".to_string(),
                    template: None,
                }),
            );
        }
    };

    // Parse Multipart
    let mut req: Option<CreateTemplateRequest> = None;
    let mut file_data: Option<Vec<u8>> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        tracing::debug!("Received multipart field: {}", name);
        if name == "data" {
            if let Ok(text) = field.text().await {
                tracing::debug!("Received data JSON: {}", text);
                match serde_json::from_str::<CreateTemplateRequest>(&text) {
                    Ok(parsed) => req = Some(parsed),
                    Err(e) => tracing::error!("Failed to parse data JSON: {}", e),
                }
            } else {
                tracing::error!("Failed to read text from data field");
            }
        } else if name == "file" {
            if let Ok(bytes) = field.bytes().await {
                file_data = Some(bytes.to_vec());
                tracing::debug!(
                    "Received file with {} bytes",
                    file_data.as_ref().unwrap().len()
                );
            }
        }
    }

    let Some(req) = req else {
        return (
            StatusCode::BAD_REQUEST,
            Json(TemplateResponse {
                success: false,
                id: None,
                message: "Missing 'data' JSON field".to_string(),
                template: None,
            }),
        );
    };

    let template_id = Uuid::new_v4();
    // Path structure: templates/{user_id}/{template_id}/metadata.json
    // We store the folder path effectively. get_user_template_path usually returns a single file path ?
    // Let's redefine path usage.
    // get_user_template_path returned "templates/{user_id}_{template_id}.json".
    // We will now use "templates/{user_id}/{template_id}/metadata.json"

    let base_folder = format!("templates/{}/{}", user_id, template_id);
    let metadata_path = format!("{}/metadata.json", base_folder);
    let pptx_path = format!("{}/base.pptx", base_folder);

    // Convert RawSlides to PresentationTemplate
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

    // Upload Metadata to GitHub
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

    // Upload Base PPTX if provided
    if let Some(bytes) = file_data {
        if let Err(e) = upload_file_to_github(
            &config,
            &pptx_path,
            &bytes,
            &format!("Add base PPTX for {}", req.name),
        )
        .await
        {
            // Log error but stick with success? No, this is critical now.
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
    } else {
        // Warn: No base file?
        // For MVP we might accept it, but it breaks generation.
    }

    // Save metadata to DB
    // Note: 'github_path' in DB should probably point to the metadata file or the folder.
    // Existing logic used full path. Let's use metadata_path.
    let result = sqlx::query(
        "INSERT INTO user_templates (id, user_id, name, description, github_path) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(template_id)
    .bind(user_uuid)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&metadata_path)
    .bind(&metadata_path)
    .execute(pool)
    .await;

    match result {
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
                message: e.to_string(),
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
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(template_id): Path<String>,
    Json(req): Json<UpdateTemplateRequest>,
) -> impl IntoResponse {
    let pool = match &state.pool {
        Some(p) => p,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Database not available".to_string(),
                    template: None,
                }),
            )
        }
    };
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

    let template_uuid = match Uuid::parse_str(&template_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Invalid ID".to_string(),
                    template: None,
                }),
            );
        }
    };

    // Get current template
    let current = sqlx::query(
        r#"
        SELECT ut.github_path, ut.name, ut.description
        FROM user_templates ut JOIN users u ON ut.user_id = u.id
        WHERE ut.id = $1 AND u.axur_tenant_id = $2
        "#,
    )
    .bind(template_uuid)
    .bind(&user_id)
    .fetch_optional(pool)
    .await;

    let current = match current {
        Ok(Some(row)) => row,
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

    let github_path: String = current.get("github_path");

    // Update GitHub if content changed
    if let Some(ref template) = req.template {
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

    // Update metadata
    let current_name: String = current.get("name");
    let current_desc: Option<String> = current.get("description");
    let new_name = req.name.as_ref().unwrap_or(&current_name);
    let new_desc = req.description.as_ref().or(current_desc.as_ref());

    let _ = sqlx::query(
        "UPDATE user_templates SET name = $1, description = $2, updated_at = NOW() WHERE id = $3",
    )
    .bind(new_name)
    .bind(new_desc)
    .bind(template_uuid)
    .execute(pool)
    .await;

    (
        StatusCode::OK,
        Json(TemplateResponse {
            success: true,
            id: Some(template_id),
            message: "Updated".to_string(),
            template: None,
        }),
    )
}

/// DELETE /api/templates/:id
pub async fn delete_template(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(template_id): Path<String>,
) -> impl IntoResponse {
    let pool = match &state.pool {
        Some(p) => p,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Database not available".to_string(),
                    template: None,
                }),
            )
        }
    };
    let template_uuid = match Uuid::parse_str(&template_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(TemplateResponse {
                    success: false,
                    id: None,
                    message: "Invalid ID".to_string(),
                    template: None,
                }),
            );
        }
    };

    // Verify ownership and delete
    let result = sqlx::query(
        r#"
        DELETE FROM user_templates ut
        USING users u
        WHERE ut.user_id = u.id AND ut.id = $1 AND u.axur_tenant_id = $2
        "#,
    )
    .bind(template_uuid)
    .bind(&user_id)
    .execute(pool)
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => (
            StatusCode::OK,
            Json(TemplateResponse {
                success: true,
                id: Some(template_id),
                message: "Deleted".to_string(),
                template: None,
            }),
        ),
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(TemplateResponse {
                success: false,
                id: None,
                message: "Not found".to_string(),
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

// ==================== HELPERS ====================

async fn ensure_user_exists(pool: &PgPool, axur_tenant_id: &str) -> Option<Uuid> {
    // Try to find existing
    let result = sqlx::query("SELECT id FROM users WHERE axur_tenant_id = $1")
        .bind(axur_tenant_id)
        .fetch_optional(pool)
        .await
        .ok()?;

    if let Some(row) = result {
        return Some(row.get("id"));
    }

    // Create new
    let result = sqlx::query(
        "INSERT INTO users (axur_tenant_id) VALUES ($1) ON CONFLICT (axur_tenant_id) DO UPDATE SET axur_tenant_id = $1 RETURNING id",
    )
    .bind(axur_tenant_id)
    .fetch_one(pool)
    .await
    .ok()?;

    Some(result.get("id"))
}

/// GET /api/templates/:id/pptx - Get the base PPTX file of a template
pub async fn get_template_pptx(
    State(state): State<AppState>,
    Extension(user_id): Extension<String>,
    Path(template_id): Path<String>,
) -> impl IntoResponse {
    let pool = match &state.pool {
        Some(p) => p,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": "Database not available" })),
            )
        }
    };

    let config = match GitHubConfig::from_env() {
        Some(c) => c,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": "GitHub not configured" })),
            )
        }
    };

    let template_uuid = match Uuid::parse_str(&template_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "success": false, "error": "Invalid template ID" })),
            )
        }
    };

    // Get template info to verify ownership and get path
    let result = sqlx::query(
        r#"
        SELECT ut.github_path
        FROM user_templates ut JOIN users u ON ut.user_id = u.id
        WHERE ut.id = $1 AND u.axur_tenant_id = $2
        "#,
    )
    .bind(template_uuid)
    .bind(&user_id)
    .fetch_optional(pool)
    .await;

    let github_path: String = match result {
        Ok(Some(row)) => row.get("github_path"),
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "success": false, "error": "Template not found" })),
            )
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "success": false, "error": e.to_string() })),
            )
        }
    };

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
