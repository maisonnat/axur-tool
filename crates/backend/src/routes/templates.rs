//! Template CRUD API routes (Simplified version)
//!
//! Uses dynamic queries to avoid compile-time database checks

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

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
pub struct CreateTemplateRequest {
    pub name: String,
    pub description: Option<String>,
    pub template: PresentationTemplate,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTemplateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub template: Option<PresentationTemplate>,
}

#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub success: bool,
    pub id: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<PresentationTemplate>,
}

#[derive(Debug, Deserialize)]
pub struct ListTemplatesQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ==================== GITHUB STORAGE ====================

struct GitHubConfig {
    token: String,
    owner: String,
    repo: String,
}

impl GitHubConfig {
    fn from_env() -> Option<Self> {
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

async fn fetch_template_from_github(
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
    Extension(pool): Extension<PgPool>,
    Extension(user_id): Extension<String>,
    Query(params): Query<ListTemplatesQuery>,
) -> impl IntoResponse {
    let limit = params.limit.unwrap_or(50).min(100) as i64;
    let offset = params.offset.unwrap_or(0) as i64;

    // Ensure user exists and get UUID
    let user_uuid = match ensure_user_exists(&pool, &user_id).await {
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
    .fetch_all(&pool)
    .await;

    match result {
        Ok(rows) => {
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

/// POST /api/templates - Create a new template
pub async fn create_template(
    Extension(pool): Extension<PgPool>,
    Extension(user_id): Extension<String>,
    Json(req): Json<CreateTemplateRequest>,
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

    let user_uuid = match ensure_user_exists(&pool, &user_id).await {
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

    let template_id = Uuid::new_v4();
    let github_path = get_user_template_path(&user_id, &template_id.to_string());

    // Upload to GitHub
    if let Err(e) = upload_template_to_github(&config, &github_path, &req.template).await {
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

    // Save metadata to DB
    let result = sqlx::query(
        "INSERT INTO user_templates (id, user_id, name, description, github_path) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(template_id)
    .bind(user_uuid)
    .bind(&req.name)
    .bind(&req.description)
    .bind(&github_path)
    .execute(&pool)
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
    // 1. Executive Summary - High level, branded
    let json_executive = r##"{
      "version": "5.3.0",
      "objects": [
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 720, "fill": "#0f172a", "selectable": false },
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 12, "fill": "#f97316", "selectable": false },
        { "type": "i-text", "left": 60, "top": 80, "fill": "#ffffff", "text": "Executive Summary", "fontSize": 18, "fontFamily": "Inter", "fontWeight": "bold", "opacity": 0.6 },
        { "type": "i-text", "left": 60, "top": 120, "fill": "#ffffff", "text": "Reporte de Ciberinteligencia", "fontSize": 48, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "i-text", "left": 60, "top": 180, "fill": "#f97316", "text": "para {{company_name}}", "fontSize": 32, "fontFamily": "Inter", "fontWeight": "600" },
        
        // Risk Score Card
        { "type": "rect", "left": 60, "top": 280, "width": 300, "height": 300, "fill": "#1e293b", "rx": 12, "ry": 12 },
        { "type": "i-text", "left": 90, "top": 310, "fill": "#94a3b8", "text": "RISK SCORE", "fontSize": 14, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "i-text", "left": 90, "top": 350, "fill": "{{risk_color}}", "text": "{{risk_score}}", "fontSize": 96, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "i-text", "left": 90, "top": 460, "fill": "#ffffff", "text": "{{risk_label}}", "fontSize": 24, "fontFamily": "Inter", "fontWeight": "600" },

        // Metrics Grid
        { "type": "rect", "left": 400, "top": 280, "width": 820, "height": 300, "fill": "#1e293b", "rx": 12, "ry": 12 },
        
        { "type": "i-text", "left": 440, "top": 320, "fill": "#94a3b8", "text": "INCIDENTS", "fontSize": 14, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "i-text", "left": 440, "top": 350, "fill": "#ffffff", "text": "{{total_incidents}}", "fontSize": 48, "fontFamily": "Inter", "fontWeight": "bold" },
        
        { "type": "i-text", "left": 700, "top": 320, "fill": "#94a3b8", "text": "TAKEDOWNS", "fontSize": 14, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "i-text", "left": 700, "top": 350, "fill": "#10b981", "text": "{{total_takedowns}}", "fontSize": 48, "fontFamily": "Inter", "fontWeight": "bold" },
        
        { "type": "i-text", "left": 960, "top": 320, "fill": "#94a3b8", "text": "AVG TIME", "fontSize": 14, "fontFamily": "Inter", "fontWeight": "bold" },
        { "type": "i-text", "left": 960, "top": 350, "fill": "#f59e0b", "text": "{{avg_takedown_time}}", "fontSize": 48, "fontFamily": "Inter", "fontWeight": "bold" },

        { "type": "i-text", "left": 60, "top": 620, "fill": "#475569", "text": "Periodo: {{date}}", "fontSize": 14, "fontFamily": "Inter" },
        { "type": "i-text", "left": 1000, "top": 620, "fill": "#475569", "text": "Confidencial - Axur CTI", "fontSize": 14, "fontFamily": "Inter" }
      ]
    }"##;

    // 2. Technical Deep Dive - Data focused
    let json_technical = r##"{
      "version": "5.3.0",
      "objects": [
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 720, "fill": "#0f172a", "selectable": false },
        // Header Strip
        { "type": "rect", "left": 60, "top": 60, "width": 5, "height": 60, "fill": "#38bdf8" },
        { "type": "i-text", "left": 80, "top": 60, "fill": "#ffffff", "text": "Technical Analysis", "fontSize": 24, "fontFamily": "Roboto Mono", "fontWeight": "bold" },
        { "type": "i-text", "left": 80, "top": 92, "fill": "#94a3b8", "text": "Scope: {{company_name}}", "fontSize": 16, "fontFamily": "Roboto Mono" },
        
        // Main Console Area
        { "type": "rect", "left": 60, "top": 160, "width": 1160, "height": 450, "fill": "#1e293b", "stroke": "#334155", "strokeWidth": 1, "rx": 4, "ry": 4 },
        { "type": "rect", "left": 60, "top": 160, "width": 1160, "height": 32, "fill": "#334155", "rx": 4, "ry": 4 },
        { "type": "circle", "left": 75, "top": 170, "radius": 6, "fill": "#fb7185" },
        { "type": "circle", "left": 95, "top": 170, "radius": 6, "fill": "#fbbf24" },
        { "type": "circle", "left": 115, "top": 170, "radius": 6, "fill": "#4ade80" },
        { "type": "i-text", "left": 140, "top": 168, "fill": "#cbd5e1", "text": "root@axur-cti:~/analysis", "fontSize": 14, "fontFamily": "Roboto Mono" },

        // Terminal Content
        { "type": "i-text", "left": 80, "top": 210, "fill": "#38bdf8", "text": "> ./scan_threats.sh --target \"{{company_name}}\"", "fontSize": 16, "fontFamily": "Roboto Mono" },
        { "type": "i-text", "left": 80, "top": 240, "fill": "#4ade80", "text": "[OK] Scanning infrastructure...", "fontSize": 16, "fontFamily": "Roboto Mono" },
        { "type": "i-text", "left": 80, "top": 270, "fill": "#e2e8f0", "text": "Found {{total_incidents}} incidents in wild", "fontSize": 16, "fontFamily": "Roboto Mono" },
        { "type": "i-text", "left": 80, "top": 300, "fill": "#e2e8f0", "text": "Resolved {{total_takedowns}} threats successfully", "fontSize": 16, "fontFamily": "Roboto Mono" },
        { "type": "i-text", "left": 80, "top": 330, "fill": "#fb7185", "text": "Current Risk Status: {{risk_label}} ({{risk_score}})", "fontSize": 16, "fontFamily": "Roboto Mono" },
        
        { "type": "i-text", "left": 80, "top": 380, "fill": "#94a3b8", "text": "Active Campaigns:", "fontSize": 16, "fontFamily": "Roboto Mono", "fontWeight": "bold" },
        { "type": "i-text", "left": 80, "top": 410, "fill": "#fde047", "text": "{{campaign_summary}}", "fontSize": 16, "fontFamily": "Roboto Mono" },
        
        { "type": "i-text", "left": 80, "top": 570, "fill": "#38bdf8", "text": "> _", "fontSize": 16, "fontFamily": "Roboto Mono", "fontWeight": "bold" }, // blinking cursor effect static
        
        { "type": "i-text", "left": 1050, "top": 650, "fill": "#64748b", "text": "{{date}}", "fontSize": 14, "fontFamily": "Roboto Mono" }
      ]
    }"##;

    // 3. Compliance - Clean, white paper style
    let json_compliance = r##"{
      "version": "5.3.0",
      "objects": [
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 720, "fill": "#ffffff", "selectable": false },
        { "type": "rect", "left": 0, "top": 0, "width": 1280, "height": 80, "fill": "#1e293b", "selectable": false },
        { "type": "i-text", "left": 50, "top": 25, "fill": "#ffffff", "text": "AXUR COMPLIANCE MONITOR", "fontSize": 20, "fontFamily": "Helvetica", "fontWeight": "bold" },
        
        { "type": "i-text", "left": 50, "top": 150, "fill": "#1e293b", "text": "Informe de Cumplimiento Normativo", "fontSize": 36, "fontFamily": "Helvetica", "fontWeight": "bold" },
        { "type": "rect", "left": 50, "top": 200, "width": 100, "height": 4, "fill": "#10b981" },
        
        { "type": "i-text", "left": 50, "top": 240, "fill": "#475569", "text": "Empresa Auditada:", "fontSize": 14, "fontFamily": "Helvetica", "fontWeight": "bold" },
        { "type": "i-text", "left": 50, "top": 260, "fill": "#1e293b", "text": "{{company_name}}", "fontSize": 24, "fontFamily": "Helvetica" },
        
        { "type": "i-text", "left": 400, "top": 240, "fill": "#475569", "text": "Fecha de Emisión:", "fontSize": 14, "fontFamily": "Helvetica", "fontWeight": "bold" },
        { "type": "i-text", "left": 400, "top": 260, "fill": "#1e293b", "text": "{{date}}", "fontSize": 24, "fontFamily": "Helvetica" },
        
        // Status Box
        { "type": "rect", "left": 50, "top": 350, "width": 1180, "height": 200, "fill": "#f8fafc", "stroke": "#e2e8f0", "strokeWidth": 2, "rx": 8, "ry": 8 },
        { "type": "i-text", "left": 90, "top": 390, "fill": "#64748b", "text": "ESTADO GENERAL DE CIBERSEGURIDAD", "fontSize": 14, "fontFamily": "Helvetica", "fontWeight": "bold" },
        
        { "type": "i-text", "left": 90, "top": 440, "fill": "#0f172a", "text": "Nivel de Exposición:", "fontSize": 24, "fontFamily": "Helvetica" },
        { "type": "i-text", "left": 350, "top": 440, "fill": "{{risk_color}}", "text": "{{risk_label}}", "fontSize": 24, "fontFamily": "Helvetica", "fontWeight": "bold" },
        
        { "type": "i-text", "left": 90, "top": 490, "fill": "#0f172a", "text": "Incidentes Activos:", "fontSize": 24, "fontFamily": "Helvetica" },
        { "type": "i-text", "left": 350, "top": 490, "fill": "#ef4444", "text": "{{total_incidents}}", "fontSize": 24, "fontFamily": "Helvetica", "fontWeight": "bold" },

        { "type": "i-text", "left": 50, "top": 650, "fill": "#94a3b8", "text": "Generado autom\u00e1ticamente por Axur One Platform", "fontSize": 12, "fontFamily": "Helvetica", "fontStyle": "italic" }
      ]
    }"##;

    match id {
        "1" | "executive" => Some(PresentationTemplate {
            name: "Executive Summary".to_string(),
            slides: vec![axur_core::editor::SlideDefinition {
                canvas_json: Some(json_executive.to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        "2" | "technical" => Some(PresentationTemplate {
            name: "Technical Deep Dive".to_string(),
            slides: vec![axur_core::editor::SlideDefinition {
                canvas_json: Some(json_technical.to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        "3" | "risk" => Some(PresentationTemplate {
            name: "Risk Focus".to_string(),
            slides: vec![axur_core::editor::SlideDefinition {
                canvas_json: Some(json_executive.to_string()), // Reuse for now
                ..Default::default()
            }],
            ..Default::default()
        }),
        "5" | "compliance" => Some(PresentationTemplate {
            name: "Compliance Report".to_string(),
            slides: vec![axur_core::editor::SlideDefinition {
                canvas_json: Some(json_compliance.to_string()),
                ..Default::default()
            }],
            ..Default::default()
        }),
        _ => None,
    }
}

/// GET /api/templates/:id
pub async fn get_template(
    Extension(pool): Extension<PgPool>,
    Extension(user_id): Extension<String>,
    Path(template_id): Path<String>,
) -> impl IntoResponse {
    // Check for mock templates first
    if let Some(mock) = get_mock_template(&template_id) {
        return (
            StatusCode::OK,
            Json(TemplateResponse {
                success: true,
                id: Some(template_id),
                message: "Template loaded (Mock)".to_string(),
                template: Some(mock),
            }),
        );
    }

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

    // ... rest of the code ...
    let result = sqlx::query(
        r#"
        SELECT ut.github_path FROM user_templates ut
        JOIN users u ON ut.user_id = u.id
        WHERE ut.id = $1 AND u.axur_tenant_id = $2
        "#,
    )
    .bind(template_uuid)
    .bind(&user_id)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(row)) => {
            let github_path: String = row.get("github_path");
            match fetch_template_from_github(&config, &github_path).await {
                Ok(template) => (
                    StatusCode::OK,
                    Json(TemplateResponse {
                        success: true,
                        id: Some(template_id),
                        message: "Template loaded".to_string(),
                        template: Some(template),
                    }),
                ),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(TemplateResponse {
                        success: false,
                        id: None,
                        message: e,
                        template: None,
                    }),
                ),
            }
        }
        Ok(None) => (
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

/// PUT /api/templates/:id
pub async fn update_template(
    Extension(pool): Extension<PgPool>,
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
    .fetch_optional(&pool)
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
    .execute(&pool)
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
    Extension(pool): Extension<PgPool>,
    Extension(user_id): Extension<String>,
    Path(template_id): Path<String>,
) -> impl IntoResponse {
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
    .execute(&pool)
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
