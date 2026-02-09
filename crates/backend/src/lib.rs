//! Axur Backend - Axum Server
//!
//! Auth proxy and report generation API for Axur Web.

pub mod error;
pub mod firebase;
pub mod github_storage;
pub mod google_services;
pub mod injector;
pub mod middleware;
pub mod queue;
pub mod routes;
pub mod services;
pub mod utils;

// Re-export the router creator
pub use crate::routes::create_router;
