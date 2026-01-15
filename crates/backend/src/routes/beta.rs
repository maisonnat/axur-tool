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
    State(_state): State<AppState>,
    Json(payload): Json<BetaRequestPayload>,
) -> Result<impl IntoResponse, ApiError> {
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
    let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");

    // Try Firestore
    if let Some(firestore) = crate::firebase::get_firestore() {
        // 1. Check if already in allowed_users
        match firestore
            .get_doc::<serde_json::Value>("allowed_users", &doc_id)
            .await
        {
            Ok(Some(_)) => {
                return Ok(Json(BetaRequestResponse {
                    success: true,
                    message: "You are already a registered beta user! Please log in.".into(),
                }));
            }
            _ => {}
        }

        // 2. Check if already requested
        match firestore
            .get_doc::<BetaRequestDoc>("beta_requests", &doc_id)
            .await
        {
            Ok(Some(req)) if req.status == "pending" => {
                return Ok(Json(BetaRequestResponse {
                    success: true,
                    message: "We already have your request! We'll allow access shortly.".into(),
                }));
            }
            _ => {}
        }

        // 3. Create new beta request
        let request_doc = serde_json::json!({
            "email": email_lower,
            "company": payload.company,
            "status": "pending",
            "requested_at": chrono::Utc::now().to_rfc3339()
        });

        match firestore
            .set_doc("beta_requests", &doc_id, &request_doc)
            .await
        {
            Ok(()) => {
                tracing::info!(email = %email_lower, company = %payload.company, "New beta request submitted to Firestore");
                return Ok(Json(BetaRequestResponse {
                    success: true,
                    message: "Request received! We will notify you when your access is ready."
                        .into(),
                }));
            }
            Err(e) => {
                tracing::error!("Failed to save beta request to Firestore: {}", e);
                return Err(ApiError::Internal(format!("Failed to save request: {}", e)));
            }
        }
    }

    // Fallback error if Firestore not configured
    Err(ApiError::Internal("Storage not available".into()))
}

pub async fn check_beta_status(
    State(_state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<String, ApiError> {
    let email = params
        .get("email")
        .ok_or(ApiError::BadRequest("Email required".into()))?;

    let email_lower = email.to_lowercase();
    let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");

    // Try Firestore
    if let Some(firestore) = crate::firebase::get_firestore() {
        // 1. Check allowed_users (Approved)
        match firestore
            .get_doc::<serde_json::Value>("allowed_users", &doc_id)
            .await
        {
            Ok(Some(_)) => return Ok("approved".to_string()),
            Err(e) => tracing::warn!("Firestore error checking allowed_users: {}", e),
            _ => {}
        }

        // 2. Check beta_requests (Pending/Rejected)
        match firestore
            .get_doc::<BetaRequestDoc>("beta_requests", &doc_id)
            .await
        {
            Ok(Some(req)) => return Ok(req.status),
            Err(e) => tracing::warn!("Firestore error checking beta_requests: {}", e),
            _ => {}
        }
    }

    // Fallback or Unknown
    Ok("unknown".to_string())
}

/// Get count of pending beta requests (for admin notification badge)
pub async fn get_pending_count(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // Try Firestore first
    if let Some(firestore) = crate::firebase::get_firestore() {
        // List all beta_requests and count pending ones
        match firestore.list_docs::<BetaRequestDoc>("beta_requests").await {
            Ok(requests) => {
                let pending = requests.iter().filter(|r| r.status == "pending").count();
                return Ok(Json(serde_json::json!({
                    "count": pending,
                    "source": "firestore"
                })));
            }
            Err(e) => {
                tracing::warn!("Firestore error getting pending count: {}", e);
            }
        }
    }

    // Fallback: return 0
    Ok(Json(
        serde_json::json!({ "count": 0, "source": "fallback" }),
    ))
}

/// Helper struct for deserializing beta requests from Firestore
#[derive(Debug, serde::Deserialize)]
struct BetaRequestDoc {
    #[allow(dead_code)]
    email: Option<String>,
    #[allow(dead_code)]
    company: Option<String>,
    status: String,
}
