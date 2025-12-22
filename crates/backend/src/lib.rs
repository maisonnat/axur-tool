//! Axur Backend - Axum Server
//!
//! Auth proxy and report generation API for Axur Web.

pub mod routes;
pub mod middleware;
pub mod error;

#[cfg(feature = "shuttle")]
mod shuttle;

// Re-export the router creator for shuttle entry point
pub use crate::routes::create_router;
