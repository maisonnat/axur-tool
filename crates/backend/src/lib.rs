//! Axur Backend - Axum Server
//!
//! Auth proxy and report generation API for Axur Web.

pub mod db;
pub mod error;
pub mod middleware;
pub mod routes;

// Re-export the router creator
pub use crate::routes::create_router;
