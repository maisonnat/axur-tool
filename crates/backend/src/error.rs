//! API error types

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// API error type with proper HTTP status codes
#[derive(Debug)]
pub enum ApiError {
    /// Bad request (400)
    BadRequest(String),
    /// Unauthorized (401)  
    Unauthorized(String),
    /// Forbidden (403)
    Forbidden(String),
    /// Not found (404)
    NotFound(String),
    /// Internal server error (500)
    Internal(String),
    /// External API error
    ExternalApi(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::ExternalApi(msg) => (StatusCode::BAD_GATEWAY, msg),
        };

        let body = Json(json!({
            "error": message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::ExternalApi(format!("API request failed: {}", err))
    }
}
