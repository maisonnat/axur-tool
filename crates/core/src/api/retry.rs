//! Retry utilities with exponential backoff for network resilience

use std::future::Future;
use std::time::Duration;

/// Default retry configuration
pub const DEFAULT_MAX_RETRIES: u32 = 3;
pub const DEFAULT_BASE_DELAY_MS: u64 = 500;

/// Callback type for retry status updates (for UI integration)
pub type RetryCallback = Option<Box<dyn Fn(&str) + Send + Sync>>;

/// Executes an async operation with exponential backoff retry logic.
///
/// # Arguments
/// * `operation` - Async closure that returns Result<T, E>
/// * `max_retries` - Maximum number of retry attempts
/// * `operation_name` - Name for logging purposes
///
/// # Returns
/// The result of the operation, or the last error after all retries exhausted
pub async fn with_retry<F, Fut, T, E>(
    operation: F,
    max_retries: u32,
    operation_name: &str,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    with_retry_callback(operation, max_retries, operation_name, None).await
}

/// Executes an async operation with exponential backoff retry logic and optional callback.
pub async fn with_retry_callback<F, Fut, T, E>(
    operation: F,
    max_retries: u32,
    operation_name: &str,
    callback: RetryCallback,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut attempt = 0;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < max_retries => {
                attempt += 1;
                let delay_ms = DEFAULT_BASE_DELAY_MS * 2u64.pow(attempt);
                let delay = Duration::from_millis(delay_ms);

                // Notify via callback if provided
                if let Some(ref cb) = callback {
                    let msg = format!(
                        "⟳ {} failed, retrying... (attempt {}/{}) - waiting {:.1}s",
                        operation_name,
                        attempt,
                        max_retries,
                        delay.as_secs_f32()
                    );
                    cb(&msg);
                }

                tokio::time::sleep(delay).await;
            }
            Err(e) => {
                // All retries exhausted - notify via callback if provided
                if let Some(ref cb) = callback {
                    let msg = format!(
                        "✗ {} failed after {} attempts: {}",
                        operation_name, max_retries, e
                    );
                    cb(&msg);
                }
                return Err(e);
            }
        }
    }
}

/// Simpler version that uses anyhow::Result and default retries
pub async fn retry_api_call<F, Fut, T>(operation: F, operation_name: &str) -> anyhow::Result<T>
where
    F: Fn() -> Fut,
    Fut: Future<Output = anyhow::Result<T>>,
{
    with_retry(operation, DEFAULT_MAX_RETRIES, operation_name).await
}
