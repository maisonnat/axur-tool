//! Route handlers

pub mod auth;
pub mod report;

use axum::{
    Router,
    routing::{get, post},
    http::{header, Method, HeaderValue},
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Create the main router with all routes and middleware
pub fn create_router() -> Router {
    // CORS configuration - allow frontend origins
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:8080".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:8080".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::COOKIE])
        .allow_credentials(true);

    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Auth routes
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/2fa", post(auth::verify_2fa))
        .route("/api/auth/finalize", post(auth::finalize))
        .route("/api/auth/validate", get(auth::validate))
        .route("/api/auth/logout", post(auth::logout))
        
        // Report routes
        .route("/api/tenants", get(report::list_tenants))
        .route("/api/report/generate", post(report::generate_report))
        
        // Middleware
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
