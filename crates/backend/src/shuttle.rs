//! Shuttle.rs Entry Point
//!
//! This file provides the entry point for Shuttle deployment.
//! Build with: cargo shuttle deploy

#[cfg(feature = "shuttle")]
use shuttle_axum::ShuttleAxum;

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn shuttle_main() -> ShuttleAxum {
    // Initialize tracing for Shuttle
    tracing_subscriber::fmt()
        .with_env_filter("axur_backend=info,tower_http=info")
        .init();

    tracing::info!("Starting Axur Backend on Shuttle.rs");

    // Create the router (same as local)
    let router = axur_backend::create_router();

    Ok(router.into())
}
