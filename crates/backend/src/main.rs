//! Axur Backend Server Entry Point

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axur_backend::create_router;

#[tokio::main]
async fn main() {
    // Load .env definitions
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axur_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database
    if let Err(e) = axur_backend::db::init_db_pool().await {
        tracing::error!("Failed to initialize database: {}", e);
        // We continue even if DB fails, but logs won't be saved to DB
    } else {
        tracing::info!("Database connection established");
    }

    // Build router (from routes module)
    let app = create_router();

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
