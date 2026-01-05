//! User Storage Routes
//!
//! API endpoints for user template storage using GitHub backend

use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::github_storage::get_github_storage;

/// Create storage routes
pub fn storage_routes() -> Router {
    Router::new()
        .route("/templates", get(list_templates))
        .route("/templates", post(save_template))
        .route("/templates/:name", get(load_template))
        .route("/templates/:name", delete(delete_template))
}

/// Request body for saving template
#[derive(Debug, Deserialize)]
pub struct SaveTemplateRequest {
    pub user_id: String,
    pub name: String,
    pub content: String,
}

/// List user's templates
async fn list_templates(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    let user_id = payload
        .get("user_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if user_id.is_empty() {
        return Json(serde_json::json!({
            "error": "user_id required"
        }));
    }

    match get_github_storage() {
        Some(storage) => match storage.list_templates(user_id).await {
            Ok(templates) => Json(serde_json::json!({
                "templates": templates
            })),
            Err(e) => Json(serde_json::json!({
                "error": e
            })),
        },
        None => Json(serde_json::json!({
            "error": "Storage not configured",
            "templates": []
        })),
    }
}

/// Save a template
async fn save_template(Json(req): Json<SaveTemplateRequest>) -> impl IntoResponse {
    if req.user_id.is_empty() || req.name.is_empty() {
        return Json(serde_json::json!({
            "error": "user_id and name required"
        }));
    }

    match get_github_storage() {
        Some(storage) => {
            match storage
                .save_template(&req.user_id, &req.name, &req.content)
                .await
            {
                Ok(()) => Json(serde_json::json!({
                    "success": true,
                    "message": format!("Template '{}' saved", req.name)
                })),
                Err(e) => Json(serde_json::json!({
                    "error": e
                })),
            }
        }
        None => Json(serde_json::json!({
            "error": "Storage not configured"
        })),
    }
}

/// Load a template by name
async fn load_template(
    Path(name): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let user_id = payload
        .get("user_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if user_id.is_empty() {
        return Json(serde_json::json!({
            "error": "user_id required"
        }));
    }

    match get_github_storage() {
        Some(storage) => match storage.load_template(user_id, &name).await {
            Ok(content) => Json(serde_json::json!({
                "name": name,
                "content": content
            })),
            Err(e) => Json(serde_json::json!({
                "error": e
            })),
        },
        None => Json(serde_json::json!({
            "error": "Storage not configured"
        })),
    }
}

/// Delete a template
async fn delete_template(
    Path(name): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let user_id = payload
        .get("user_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if user_id.is_empty() {
        return Json(serde_json::json!({
            "error": "user_id required"
        }));
    }

    match get_github_storage() {
        Some(storage) => match storage.delete_template(user_id, &name).await {
            Ok(()) => Json(serde_json::json!({
                "success": true,
                "message": format!("Template '{}' deleted", name)
            })),
            Err(e) => Json(serde_json::json!({
                "error": e
            })),
        },
        None => Json(serde_json::json!({
            "error": "Storage not configured"
        })),
    }
}
