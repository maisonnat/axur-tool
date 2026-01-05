//! Route handlers

pub mod admin_config; // Admin access control
pub mod auth;
pub mod feedback;
pub mod import_export;
pub mod logs_api; // Log viewing API
pub mod marketplace; // Template marketplace
pub mod queue; // Request queue with rate limiting
pub mod remote_log; // Private GitHub log uploads
pub mod report;
pub mod status; // Production health checks
pub mod storage; // GitHub storage for user data
pub mod templates; // Template CRUD

use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Method},
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub google_services: Option<std::sync::Arc<crate::google_services::GoogleServices>>,
    pub pool: Option<sqlx::PgPool>,
}

/// Create the main router with all routes and middleware
pub fn create_router(_state: AppState) -> Router {
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
            // Leapcell
            "https://axur-tool-maisonnat2655-5j70lozi.leapcell.dev"
                .parse::<HeaderValue>()
                .unwrap(),
            "https://axur-tool-maisonnat2655-dc5ya68vc4dbqraqq0.leapcell-async.dev"
                .parse::<HeaderValue>()
                .unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
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
        .route("/api/auth/logout", post(auth::logout))
        // Marketplace (browse is public)
        .route("/api/marketplace", get(marketplace::list_marketplace))
        // Template GET is public (mock templates don't need auth)
        .route("/api/templates/:id", get(templates::get_template));

    // Protected routes (Require Authentication)
    let protected_routes: Router<AppState> = Router::new()
        .route("/api/tenants", get(report::list_tenants))
        .route("/api/report/generate", post(report::generate_report))
        .route("/api/export/inject", post(import_export::inject_pptx))
        .route("/api/import/pptx", post(import_export::import_pptx))
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
        .route("/api/logs/stats", get(logs_api::get_log_stats))
        // Template CRUD API
        .route("/api/templates", get(templates::list_templates))
        .route("/api/templates", post(templates::create_template))
        // GET /api/templates/:id is in public routes (mock templates)
        .route("/api/templates/:id", put(templates::update_template))
        .route("/api/templates/:id", delete(templates::delete_template))
        .route(
            "/api/templates/:id/publish",
            post(marketplace::publish_template),
        )
        // Marketplace user actions
        .route(
            "/api/marketplace/:id/download",
            post(marketplace::download_template),
        )
        .route(
            "/api/marketplace/:id/rate",
            post(marketplace::rate_template),
        )
        // Admin moderation
        .route(
            "/api/admin/marketplace/pending",
            get(marketplace::list_pending_templates),
        )
        .route(
            "/api/admin/marketplace/:id/approve",
            post(marketplace::approve_template),
        )
        .route(
            "/api/admin/marketplace/:id/reject",
            post(marketplace::reject_template),
        )
        .route_layer(axum::middleware::from_fn(crate::middleware::require_auth));

    // Queue routes (public - users need to check their queue status)
    let queue_routes = queue::queue_routes();

    // Storage routes (user templates via GitHub)
    let storage_routes = storage::storage_routes();

    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes.with_state(_state))
        .nest("/api/queue", queue_routes)
        .nest("/api/storage", storage_routes)
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024)) // 50MB limit
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    if let Some(pool) = crate::db::get_db() {
        app.layer(axum::Extension(pool.clone()))
    } else {
        tracing::warn!("Database pool not initialized - some routes may fail");
        app
    }
}

/// Health check endpoint
async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "service": "axur-backend",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
