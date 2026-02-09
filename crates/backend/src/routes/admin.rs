//! Admin API - User Management for Beta Access Control
//!
//! Endpoints for managing the `allowed_users` whitelist.
//! Only admins (role = 'admin' in allowed_users) can use these endpoints.

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::AUTH_USER_COOKIE_NAME;
use crate::routes::AppState;

// ========================
// TYPES
// ========================

#[derive(Debug, Serialize, Deserialize)]
pub struct BetaReq {
    pub id: String,
    pub email: String,
    pub company: String,
    pub status: String,
    pub requested_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllowedUser {
    pub email: String,
    pub role: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub added_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddUserRequest {
    pub email: String,
    #[serde(default = "default_role")]
    pub role: String,
    pub description: Option<String>,
}

fn default_role() -> String {
    "beta_tester".to_string()
}

// ========================
// ROUTES
// ========================

/// Create admin user management routes
pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users", post(add_user))
        .route("/users/:email", delete(remove_user))
        .route("/beta/requests", get(list_beta_requests))
        .route(
            "/beta/requests/pending-count",
            get(super::beta::get_pending_count),
        )
        // Unified action endpoint: POST /beta/requests/:email/action { "action": "approve"|"reject" }
        .route(
            "/beta/requests/:email/action",
            post(handle_beta_request_action),
        )
}

// ========================
// HANDLERS
// ========================

/// Check if the caller is an admin
/// Check if the caller is an admin
async fn require_admin(jar: &CookieJar) -> Result<String, ApiError> {
    let user_email = jar
        .get(AUTH_USER_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::Unauthorized("Not logged in".into()))?;

    let email_lower = user_email.to_lowercase();
    let mut is_admin = false;

    // 1. Check Firestore
    if let Some(firestore) = crate::firebase::get_firestore() {
        let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");
        match firestore
            .get_doc::<serde_json::Value>("allowed_users", &doc_id)
            .await
        {
            Ok(Some(doc)) => {
                if let Some(role) = doc.get("role").and_then(|v| v.as_str()) {
                    if role == "admin" {
                        is_admin = true;
                        tracing::debug!("Firestore check: {} is admin", user_email);
                    }
                }
            }
            Ok(None) => {}
            Err(e) => tracing::warn!("Firestore admin check failed: {}", e),
        }
    }

    if is_admin {
        return Ok(user_email);
    }

    // 2. Check GitHub storage (fallback)
    if let Some(storage) = crate::github_storage::get_github_storage() {
        match storage.is_admin(&user_email).await {
            Ok(true) => return Ok(user_email),
            Ok(false) => {
                tracing::warn!("Admin check failed for {}", user_email);
                return Err(ApiError::Forbidden("Admin access required".into()));
            }
            Err(e) => {
                tracing::error!("GitHub storage admin check failed: {}", e);
                // Fallthrough to error
            }
        }
    } else {
        tracing::error!("GitHub storage not configured");
    }

    Err(ApiError::Internal("Admin check unavailable".into()))
}

/// List all allowed users
/// List all allowed users
async fn list_users(
    State(_state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<Vec<AllowedUser>>, ApiError> {
    require_admin(&jar).await?;

    // Try Firestore first
    if let Some(firestore) = crate::firebase::get_firestore() {
        match firestore.list_docs::<AllowedUser>("allowed_users").await {
            Ok(users) => return Ok(Json(users)),
            Err(e) => tracing::error!("Firestore error listing users: {}", e),
        }
    }

    // Fallback: empty list (or error if we want strict)
    tracing::warn!("Firestore unavailable for list_users");
    Ok(Json(vec![]))
}

/// Add a new user to the allowed list (and GitHub storage)
async fn add_user(
    State(_state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<AddUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let admin_email = require_admin(&jar).await?;

    // Validate email format
    if !payload.email.contains('@') {
        return Err(ApiError::BadRequest("Invalid email format".into()));
    }

    // Validate role
    let valid_roles = ["admin", "beta_tester"];
    if !valid_roles.contains(&payload.role.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Invalid role. Must be one of: {:?}",
            valid_roles
        )));
    }

    let email_lower = payload.email.to_lowercase();
    let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");

    // Create user object
    let user = AllowedUser {
        email: email_lower.clone(),
        role: payload.role.clone(),
        description: payload.description.clone(),
        created_at: Some(chrono::Utc::now().to_rfc3339()),
        added_by: Some(admin_email.clone()),
    };

    // 1. Save to Firestore
    if let Some(firestore) = crate::firebase::get_firestore() {
        let doc_json =
            serde_json::to_value(&user).map_err(|e| ApiError::Internal(e.to_string()))?;
        if let Err(e) = firestore.set_doc("allowed_users", &doc_id, &doc_json).await {
            tracing::error!("Failed to add user to Firestore: {}", e);
            return Err(ApiError::Internal("Failed to save user".into()));
        }
    } else {
        return Err(ApiError::Internal("Storage not available".into()));
    }

    // 2. Also update GitHub storage if configured (for redundancy/CDN)
    if let Some(_storage) = crate::github_storage::get_github_storage() {
        // Implement logic to update allowed_users.json in GitHub if needed
        // For now, we rely on Firestore as primary source of truth for writes
        // and could implement a background sync or dual-write.
        // Assuming dual-write for now if method exists, otherwise skip.
        tracing::info!(
            "User added to Firestore. GitHub sync not yet implemented for individual users."
        );
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("User {} added with role {}", payload.email, payload.role)
    })))
}

/// Remove a user from the allowed list
async fn remove_user(
    State(_state): State<AppState>,
    jar: CookieJar,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let admin_email = require_admin(&jar).await?;

    // Prevent removing yourself
    if email.to_lowercase() == admin_email.to_lowercase() {
        return Err(ApiError::BadRequest("Cannot remove yourself".into()));
    }

    let email_lower = email.to_lowercase();
    let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");

    // 1. Remove from Firestore
    if let Some(firestore) = crate::firebase::get_firestore() {
        if let Err(e) = firestore.delete_doc("allowed_users", &doc_id).await {
            tracing::error!("Failed to remove user from Firestore: {}", e);
            return Err(ApiError::Internal("Failed to remove user".into()));
        }
    } else {
        return Err(ApiError::Internal("Storage not available".into()));
    }

    // 2. Also remove from GitHub storage if configured
    if let Some(_storage) = crate::github_storage::get_github_storage() {
        // Implement logic to remove from allowed_users.json in GitHub if needed
        // For now, logging.
        tracing::info!("User removed from Firestore. GitHub sync not yet implemented.");
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("User {} removed", email)
    })))
}

/// List all beta requests
async fn list_beta_requests(
    State(_state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<Vec<BetaReq>>, ApiError> {
    require_admin(&jar).await?;

    // Try Firestore first
    if let Some(firestore) = crate::firebase::get_firestore() {
        match firestore.list_docs::<BetaRequestDoc>("beta_requests").await {
            Ok(docs) => {
                let requests: Vec<BetaReq> = docs
                    .into_iter()
                    .map(|d| BetaReq {
                        id: d.email.clone(), // Use email as ID
                        email: d.email,
                        company: d.company.unwrap_or_default(),
                        status: d.status,
                        requested_at: d.requested_at.map(|t| {
                            chrono::DateTime::parse_from_rfc3339(&t)
                                .unwrap_or_default()
                                .with_timezone(&chrono::Utc)
                        }),
                    })
                    .collect();
                return Ok(Json(requests));
            }
            Err(e) => tracing::error!("Firestore error listing beta requests: {}", e),
        }
    }

    // Fallback
    tracing::warn!("Firestore unavailable for list_beta_requests");
    Ok(Json(vec![]))
}

// Helper structs for Firestore
#[derive(Debug, serde::Deserialize)]
struct BetaRequestDoc {
    email: String,
    company: Option<String>,
    status: String,
    requested_at: Option<String>,
}

/// Admin Action on Beta Request (Approve/Reject)
#[derive(Debug, serde::Deserialize)]
pub struct BetaActionRequest {
    pub action: String, // "approve" or "reject"
}

/// Handle beta request action (approve/reject)
/// Path param is the email (sanitized or raw, we'll handle it)
async fn handle_beta_request_action(
    State(_state): State<AppState>,
    jar: CookieJar,
    Path(email): Path<String>,
    Json(payload): Json<BetaActionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let admin_email = require_admin(&jar).await?;

    let email_lower = email.to_lowercase();
    let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");

    if let Some(firestore) = crate::firebase::get_firestore() {
        if payload.action == "approve" {
            // 1. Add to allowed_users
            let user = AllowedUser {
                email: email_lower.clone(),
                role: "beta_tester".to_string(),
                description: Some("Approved from beta request".to_string()),
                created_at: Some(chrono::Utc::now().to_rfc3339()),
                added_by: Some(admin_email.clone()),
            };
            let doc_json =
                serde_json::to_value(&user).map_err(|e| ApiError::Internal(e.to_string()))?;
            firestore
                .set_doc("allowed_users", &doc_id, &doc_json)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;

            // 2. Update beta_requests status
            let update = serde_json::json!({ "status": "approved" });
            firestore
                .update_doc("beta_requests", &doc_id, &update)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;

            tracing::info!(admin = %admin_email, user = %email_lower, "Approved beta request");
        } else if payload.action == "reject" {
            // Update beta_requests status
            let update = serde_json::json!({ "status": "rejected" });
            firestore
                .update_doc("beta_requests", &doc_id, &update)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;

            tracing::info!(admin = %admin_email, user = %email_lower, "Rejected beta request");
        } else {
            return Err(ApiError::BadRequest(
                "Invalid action. Use 'approve' or 'reject'".into(),
            ));
        }
    } else {
        return Err(ApiError::Internal("Storage not available".into()));
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Action {} completed for {}", payload.action, email_lower)
    })))
}
