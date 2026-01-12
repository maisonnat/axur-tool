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

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct BetaReq {
    pub id: uuid::Uuid,
    pub email: String,
    pub company: String,
    pub status: String,
    pub requested_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
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
        .route("/beta/requests/:id/approve", post(approve_beta_request))
        .route("/beta/requests/:id/reject", post(reject_beta_request))
}

// ========================
// HANDLERS
// ========================

/// Check if the caller is an admin
async fn require_admin(jar: &CookieJar, pool: &sqlx::PgPool) -> Result<String, ApiError> {
    let user_email = jar
        .get(AUTH_USER_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or_else(|| ApiError::Unauthorized("Not logged in".into()))?;

    let email_lower = user_email.to_lowercase();
    let result: Option<(String,)> =
        sqlx::query_as("SELECT role FROM allowed_users WHERE LOWER(email) = $1")
            .bind(&email_lower)
            .fetch_optional(pool)
            .await
            .unwrap_or(None);

    match result {
        Some((role,)) if role == "admin" => Ok(user_email),
        Some(_) => Err(ApiError::Forbidden("Admin access required".into())),
        None => Err(ApiError::Forbidden("User not in allowed list".into())),
    }
}

/// List all allowed users
async fn list_users(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<Vec<AllowedUser>>, ApiError> {
    let pool = state
        .pool
        .as_ref()
        .ok_or_else(|| ApiError::Internal("Database not available".into()))?;

    require_admin(&jar, pool).await?;

    let rows: Vec<(String, String, Option<String>, Option<chrono::DateTime<chrono::Utc>>, Option<String>)> = sqlx::query_as(
        "SELECT email, role, description, created_at, added_by FROM allowed_users ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    let users: Vec<AllowedUser> = rows
        .into_iter()
        .map(
            |(email, role, description, created_at, added_by)| AllowedUser {
                email,
                role,
                description,
                created_at: created_at.map(|dt| dt.to_rfc3339()),
                added_by,
            },
        )
        .collect();

    Ok(Json(users))
}

/// Add a new user to the allowed list
async fn add_user(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<AddUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let pool = state
        .pool
        .as_ref()
        .ok_or_else(|| ApiError::Internal("Database not available".into()))?;

    let admin_email = require_admin(&jar, pool).await?;

    // Validate email format (basic check)
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

    let result = sqlx::query(
        "INSERT INTO allowed_users (email, role, description, added_by) VALUES ($1, $2, $3, $4) ON CONFLICT (email) DO UPDATE SET role = $2, description = $3"
    )
    .bind(&payload.email.to_lowercase())
    .bind(&payload.role)
    .bind(&payload.description)
    .bind(&admin_email)
    .execute(pool)
    .await;

    match result {
        Ok(_) => {
            tracing::info!(
                admin = %admin_email,
                user = %payload.email,
                role = %payload.role,
                "User added to allowed_users"
            );
            Ok(Json(serde_json::json!({
                "success": true,
                "message": format!("User {} added with role {}", payload.email, payload.role)
            })))
        }
        Err(e) => Err(ApiError::Internal(format!("Failed to add user: {}", e))),
    }
}

/// Remove a user from the allowed list
async fn remove_user(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let pool = state
        .pool
        .as_ref()
        .ok_or_else(|| ApiError::Internal("Database not available".into()))?;

    let admin_email = require_admin(&jar, pool).await?;

    // Prevent removing yourself
    if email.to_lowercase() == admin_email.to_lowercase() {
        return Err(ApiError::BadRequest("Cannot remove yourself".into()));
    }

    let result = sqlx::query("DELETE FROM allowed_users WHERE LOWER(email) = $1")
        .bind(&email.to_lowercase())
        .execute(pool)
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            tracing::info!(
                admin = %admin_email,
                removed_user = %email,
                "User removed from allowed_users"
            );
            Ok(Json(serde_json::json!({
                "success": true,
                "message": format!("User {} removed", email)
            })))
        }
        Ok(_) => Err(ApiError::NotFound(format!("User {} not found", email))),
        Err(e) => Err(ApiError::Internal(format!("Failed to remove user: {}", e))),
    }
}

/// List all beta requests
async fn list_beta_requests(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<Vec<BetaReq>>, ApiError> {
    let pool = state
        .pool
        .as_ref()
        .ok_or_else(|| ApiError::Internal("Database not available".into()))?;

    require_admin(&jar, pool).await?;

    let requests: Vec<BetaReq> = sqlx::query_as(
        "SELECT id, email, company, status, requested_at FROM beta_requests ORDER BY requested_at DESC",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    Ok(Json(requests))
}

/// Approve a beta request
async fn approve_beta_request(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let pool = state
        .pool
        .as_ref()
        .ok_or_else(|| ApiError::Internal("Database not available".into()))?;

    let admin_email = require_admin(&jar, pool).await?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| ApiError::Internal(format!("Transaction error: {}", e)))?;

    // 1. Get request details
    let req: Option<(String, String)> = sqlx::query_as(
        "SELECT email, company FROM beta_requests WHERE id = $1 AND status = 'pending'",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    let (email, company) = match req {
        Some(r) => r,
        None => return Err(ApiError::NotFound("Pending request not found".into())),
    };

    // 2. Insert into allowed_users
    let user_desc = format!("Beta Tester (Company: {})", company);
    sqlx::query(
        "INSERT INTO allowed_users (email, role, description, added_by) VALUES ($1, 'beta_tester', $2, $3) ON CONFLICT (email) DO NOTHING"
    )
    .bind(&email)
    .bind(&user_desc)
    .bind(&admin_email)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to add user: {}", e)))?;

    // 3. Update request status
    sqlx::query(
        "UPDATE beta_requests SET status = 'approved', processed_at = NOW(), processed_by = $1 WHERE id = $2"
    )
    .bind(&admin_email)
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to update request: {}", e)))?;

    tx.commit()
        .await
        .map_err(|e| ApiError::Internal(format!("Commit error: {}", e)))?;

    tracing::info!(admin = %admin_email, user = %email, "Approved beta request");

    Ok(Json(serde_json::json!({
        "success": true,
        "message": format!("Approved {}", email)
    })))
}

/// Reject a beta request
async fn reject_beta_request(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let pool = state
        .pool
        .as_ref()
        .ok_or_else(|| ApiError::Internal("Database not available".into()))?;

    let admin_email = require_admin(&jar, pool).await?;

    let result = sqlx::query(
        "UPDATE beta_requests SET status = 'rejected', processed_at = NOW(), processed_by = $1 WHERE id = $2"
    )
    .bind(&admin_email)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to reject: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Request not found".into()));
    }

    tracing::info!(admin = %admin_email, id = %id, "Rejected beta request");

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Request rejected"
    })))
}
