//! Beta Access Routes - Public Registration
//!
//! Handles public requests to join the beta program.

use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::routes::AppState;

// ========================
// TYPES
// ========================

#[derive(Debug, Deserialize)]
pub struct BetaRequestPayload {
    pub email: String,
    pub company: String,
}

#[derive(Debug, Serialize)]
pub struct BetaRequestResponse {
    pub success: bool,
    pub message: String,
}

// ========================
// HANDLERS
// ========================

/// Submit a new beta access request
pub async fn submit_beta_request(
    State(state): State<AppState>,
    Json(payload): Json<BetaRequestPayload>,
) -> Result<impl IntoResponse, ApiError> {
    let pool = state
        .pool
        .as_ref()
        .ok_or_else(|| ApiError::Internal("Database not available".into()))?;

    // Validate inputs
    if payload.email.trim().is_empty() || payload.company.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Email and Company are required".into(),
        ));
    }

    if !payload.email.contains('@') {
        return Err(ApiError::BadRequest("Invalid email format".into()));
    }

    let email_lower = payload.email.to_lowercase();

    // 1. Check if already active
    let exists_active: Option<(String,)> =
        sqlx::query_as("SELECT email FROM allowed_users WHERE LOWER(email) = $1")
            .bind(&email_lower)
            .fetch_optional(pool)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?;

    if exists_active.is_some() {
        return Ok(Json(BetaRequestResponse {
            success: true,
            message: "You are already a registered beta user! Please log in.".into(),
        }));
    }

    // 2. Check if already requested (prevent duplicates)
    // We treat "pending" or "rejected" as "received". If rejected, we might want to let them apply again?
    // for now, just say "received".
    let exists_request: Option<(String,)> = sqlx::query_as(
        "SELECT status FROM beta_requests WHERE LOWER(email) = $1 AND status = 'pending'",
    )
    .bind(&email_lower)
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::Internal(e.to_string()))?;

    if exists_request.is_some() {
        return Ok(Json(BetaRequestResponse {
            success: true,
            message: "We already have your request! We'll allow access shortly.".into(),
        }));
    }

    // 3. Insert new request
    sqlx::query("INSERT INTO beta_requests (email, company, status) VALUES ($1, $2, 'pending')")
        .bind(&email_lower)
        .bind(&payload.company)
        .execute(pool)
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to save request: {}", e)))?;

    tracing::info!(email = %email_lower, company = %payload.company, "New beta request submitted");

    Ok(Json(BetaRequestResponse {
        success: true,
        message: "Request received! We will notify you when your access is ready.".into(),
    }))
}

pub async fn check_beta_status(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<String, ApiError> {
    let email = params
        .get("email")
        .ok_or(ApiError::BadRequest("Email required".into()))?;

    let email_lower = email.to_lowercase();

    // Check allowed_users first (Approved)
    if let Some(pool) = &state.pool {
        let allowed: Option<(String,)> =
            sqlx::query_as("SELECT email FROM allowed_users WHERE LOWER(email) = $1")
                .bind(&email_lower)
                .fetch_optional(pool)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;

        if allowed.is_some() {
            return Ok("approved".to_string());
        }

        // Check beta_requests (Pending)
        let request: Option<(String,)> =
            sqlx::query_as("SELECT status FROM beta_requests WHERE LOWER(email) = $1")
                .bind(&email_lower)
                .fetch_optional(pool)
                .await
                .map_err(|e| ApiError::Internal(e.to_string()))?;

        if let Some((status,)) = request {
            return Ok(status); // "pending", "rejected", etc.
        }
    }

    Ok("not_found".to_string())
}

/// Get count of pending beta requests (for admin notification badge)
pub async fn get_pending_count(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let pool = match state.pool.as_ref() {
        Some(p) => p,
        None => {
            // Database unavailable - return 0 count silently
            tracing::warn!("Database not available for pending_count, returning 0");
            return Ok(Json(
                serde_json::json!({ "count": 0, "db_available": false }),
            ));
        }
    };

    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM beta_requests WHERE status = 'pending'")
            .fetch_one(pool)
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(serde_json::json!({ "count": count.0 })))
}
