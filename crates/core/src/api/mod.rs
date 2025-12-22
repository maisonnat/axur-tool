//! API module for Axur platform interactions

pub mod report;
pub mod retry;

/// Base API URL
pub const API_URL: &str = "https://api.axur.com/gateway/1.0/api";

/// Create HTTP client with default configuration
pub fn create_client() -> anyhow::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()?)
}
