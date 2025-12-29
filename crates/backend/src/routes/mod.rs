//! Route handlers

pub mod admin_config; // Admin access control
pub mod auth;
pub mod feedback;
pub mod logs_api; // Log viewing API
pub mod remote_log; // Private GitHub log uploads
pub mod report;
pub mod status; // Production health checks

use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Create the main router with all routes and middleware
pub fn create_router() -> Router {
    // CORS configuration - allow frontend origins (dev + production)
    let cors = CorsLayer::new()
        .allow_origin([
            // Development
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:8080".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:8080".parse::<HeaderValue>().unwrap(),
            // Production (Cloudflare Pages)
            "https://axtool.pages.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::COOKIE,
            header::ACCEPT,
        ])
        .expose_headers([header::SET_COOKIE, header::CONTENT_TYPE])
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600)); // Cache preflight for 1 hour

    // Public routes (Auth, Health, Status)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/api/status", get(status::full_status))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/2fa", post(auth::verify_2fa))
        .route("/api/auth/finalize", post(auth::finalize))
        .route("/api/auth/validate", get(auth::validate))
        .route("/api/auth/logout", post(auth::logout));

    // Protected routes (Require Authentication)
    let protected_routes = Router::new()
        .route("/api/tenants", get(report::list_tenants))
        .route("/api/report/generate", post(report::generate_report))
        .route(
            "/api/threat-hunting/preview",
            post(report::threat_hunting_preview),
        )
        .route(
            "/api/threat-hunting/preview-stream",
            get(report::threat_hunting_preview_stream),
        )
        .route("/api/feedback", post(feedback::submit_feedback))
        .route("/api/logs/sync", post(remote_log::sync_logs))
        // Log viewer API
        .route("/api/logs", get(logs_api::list_logs))
        .route("/api/logs/dates", get(logs_api::list_log_dates))
        .route("/api/logs/categories", get(logs_api::list_log_categories))
        .route("/api/logs/content/*path", get(logs_api::get_log_content))
        .route("/api/logs/access", get(logs_api::check_log_access))
        .route_layer(axum::middleware::from_fn(crate::middleware::require_auth));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

/// Health check endpoint
async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "service": "axur-backend",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
