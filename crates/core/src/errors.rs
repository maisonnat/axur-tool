use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("❌ Authentication Failed: {0}\n   💡 Try: Check your email/password and try again")]
    AuthError(String),

    #[error("❌ Session expired.\n   💡 Try: Run 'logout' then login again")]
    SessionExpired,

    #[error("❌ Network Error: Could not reach Axur API ({0})\n   💡 Try: Check your internet connection")]
    NetworkError(String),

    #[error("❌ Invalid date '{0}'\n   💡 Try: Use YYYY-MM-DD format (e.g., 2025-01-15)")]
    InvalidDate(String),

    #[error("❌ Invalid tenant ID: {0}\n   💡 Try: Run 'quick' to see available tenants")]
    InvalidTenant(String),

    #[error("⚠️ Invalid Input: {0}")]
    InputError(String),

    #[error("📂 File System Error: {0}\n   💡 Try: Check file permissions")]
    IoError(#[from] std::io::Error),

    #[error("⚙️ API Error: {0}\n   💡 Try: Wait a moment and retry")]
    ApiError(String),

    #[error("⏳ API rate limited.\n   💡 Try: Wait 30 seconds and try again")]
    RateLimited,

    #[error("🔄 Request failed after {0} retries.\n   💡 Try: Check your connection and try again later")]
    RetryExhausted(u32),

    #[error("❌ {0}")]
    Unknown(String),
}

/// Convert reqwest errors to user-friendly CliError
impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            CliError::NetworkError("Request timed out".to_string())
        } else if err.is_connect() {
            CliError::NetworkError("Could not connect to server".to_string())
        } else if let Some(status) = err.status() {
            match status.as_u16() {
                401 => CliError::SessionExpired,
                429 => CliError::RateLimited,
                _ => CliError::ApiError(format!("HTTP {}", status)),
            }
        } else {
            CliError::NetworkError(err.to_string())
        }
    }
}
