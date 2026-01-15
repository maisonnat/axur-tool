//! Axur Backend Server Entry Point

use std::net::SocketAddr;

use axur_backend::create_router;

#[tokio::main]
async fn main() {
    // Load .env definitions
    dotenv::dotenv().ok();

    // Initialize tracing with explicit stdout and debug level
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    // Backend initialization

    // Start background queue worker
    axur_backend::queue::start_worker();
    tracing::info!("Queue worker started");

    // Initialize Google Services
    // Priority: 1) Environment variables (production), 2) Local files (development)
    let google_services = if std::env::var("GOOGLE_CLIENT_ID").is_ok() {
        // Production: use environment variables
        match axur_backend::google_services::GoogleServices::from_env().await {
            Ok(service) => {
                tracing::info!("Google Services initialized (Environment Variables)");
                Some(std::sync::Arc::new(service))
            }
            Err(e) => {
                tracing::error!("Failed to initialize Google Services from env: {}", e);
                None
            }
        }
    } else {
        // Development: use local token.json file
        let client_secret_path = "config/client_secret.json";
        let token_path = "config/token.json";

        if std::path::Path::new(token_path).exists() {
            match axur_backend::google_services::GoogleServices::new(client_secret_path, token_path)
                .await
            {
                Ok(service) => {
                    tracing::info!("Google Services initialized (Local Files)");
                    Some(std::sync::Arc::new(service))
                }
                Err(e) => {
                    tracing::error!("Failed to initialize Google Services: {}", e);
                    None
                }
            }
        } else {
            tracing::warn!("Google credentials not found. Google integration disabled.");
            None
        }
    };

    let app_state = axur_backend::routes::AppState { google_services };

    // Build router (from routes module)
    let app = create_router(app_state);

    // Run server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Axur Backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
