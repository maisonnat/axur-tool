//! Remote logging module - uploads debug logs to private GitHub repository
//!
//! This module provides functionality to upload debug logs to a private
//! GitHub repository for production monitoring and debugging.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::json;
use std::env;

/// Configuration for remote logging
pub struct RemoteLogConfig {
    pub token: String,
    pub owner: String,
    pub repo: String,
}

impl RemoteLogConfig {
    /// Load config from environment variables
    /// Checks for both GH_* (production) and GITHUB_* (local dev) naming conventions
    pub fn from_env() -> Option<Self> {
        // Try GH_PAT first (production), then GITHUB_TOKEN (local dev)
        let token = env::var("GH_PAT")
            .or_else(|_| env::var("GITHUB_TOKEN"))
            .ok()?;

        // Try GH_OWNER first, then GITHUB_OWNER
        let owner = env::var("GH_OWNER")
            .or_else(|_| env::var("GITHUB_OWNER"))
            .ok()?;

        // Try GH_LOGS_REPO first, then GITHUB_LOGS_REPO
        let repo = env::var("GH_LOGS_REPO")
            .or_else(|_| env::var("GITHUB_LOGS_REPO"))
            .unwrap_or_else(|_| "axur-logs-private".to_string());

        Some(Self { token, owner, repo })
    }
}

/// Get the SHA of an existing file (if it exists)
async fn get_file_sha(config: &RemoteLogConfig, path: &str) -> Option<String> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-bot")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .ok()?;

    if res.status().is_success() {
        let json: serde_json::Value = res.json().await.ok()?;
        json.get("sha")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
    } else {
        None
    }
}

/// Generic function to upload a file to the private GitHub repository
/// Handles "Upsert" (Create or Update) logic by checking for 409/422 errors
pub async fn upload_to_github(
    config: &RemoteLogConfig,
    path: &str,
    content: &str,
    message: &str,
) -> Result<String, String> {
    let upload_url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    // Base64 encode the content
    let encoded_content = BASE64.encode(content.as_bytes());

    // Helper to perform the PUT request
    let perform_upload = |sha: Option<String>| async {
        let mut body_map = serde_json::Map::new();
        body_map.insert("message".to_string(), json!(message));
        body_map.insert("content".to_string(), json!(encoded_content));
        body_map.insert(
            "committer".to_string(),
            json!({
                "name": "Axur Bot",
                "email": "bot@axur.local"
            }),
        );

        if let Some(s) = sha {
            body_map.insert("sha".to_string(), json!(s));
        }

        let body = serde_json::Value::Object(body_map);
        let client = reqwest::Client::new(); // Re-create client to avoid move issues in retry

        client
            .put(&upload_url)
            .header("Authorization", format!("Bearer {}", config.token))
            .header("User-Agent", "axur-bot")
            .header("Accept", "application/vnd.github.v3+json")
            .json(&body)
            .send()
            .await
    };

    // 1. Try to create (no SHA)
    let res = perform_upload(None).await.map_err(|e| e.to_string())?;

    // 2. If success, return URL
    if res.status().is_success() {
        let resp_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        return Ok(resp_json
            .get("content")
            .and_then(|c| c.get("html_url"))
            .and_then(|u| u.as_str())
            .unwrap_or("")
            .to_string());
    }

    // 3. If Conflict (409) or Unprocessable (422) -> File likely exists
    if res.status() == StatusCode::CONFLICT || res.status() == StatusCode::UNPROCESSABLE_ENTITY {
        // Fetch current SHA
        if let Some(sha) = get_file_sha(config, path).await {
            // Retry with SHA (Update)
            let retry_res = perform_upload(Some(sha)).await.map_err(|e| e.to_string())?;

            if retry_res.status().is_success() {
                let resp_json: serde_json::Value =
                    retry_res.json().await.map_err(|e| e.to_string())?;
                return Ok(resp_json
                    .get("content")
                    .and_then(|c| c.get("html_url"))
                    .and_then(|u| u.as_str())
                    .unwrap_or("")
                    .to_string());
            } else {
                let status = retry_res.status();
                let err_text = retry_res.text().await.unwrap_or_default();
                return Err(format!("Retry update failed {}: {}", status, err_text));
            }
        }
    }

    // 4. Fallback error
    let status = res.status();
    let err_text = res.text().await.unwrap_or_default();
    Err(format!("GitHub API error {}: {}", status, err_text))
}

/// Upload a log file to the private GitHub repository
pub async fn upload_log(
    config: &RemoteLogConfig,
    category: &str,
    filename: &str,
    content: &str,
) -> Result<String, String> {
    // Organize logs by date and category
    let now = chrono::Utc::now();
    let date_folder = now.format("%Y/%m/%d").to_string();
    let path = format!("logs/{}/{}/{}", date_folder, category, filename);
    let message = format!("Log: {} - {}", category, filename);

    upload_to_github(config, &path, content, &message).await
}

/// Upload multiple logs in batch (fire-and-forget style)
/// Spawns a background task to avoid blocking the main flow
pub fn upload_log_async(category: &str, filename: &str, content: String) {
    let Some(config) = RemoteLogConfig::from_env() else {
        // Remote logging not configured, skip silently
        return;
    };

    let category = category.to_string();
    let filename = filename.to_string();

    tokio::spawn(async move {
        match upload_log(&config, &category, &filename, &content).await {
            Ok(url) => {
                tracing::debug!("Log uploaded to GitHub: {}", url);
            }
            Err(e) => {
                tracing::warn!("Failed to upload log to GitHub: {}", e);
            }
        }
    });
}

/// Upload a generated report to the private GitHub repository
/// Spawns a background task (fire-and-forget)
pub fn upload_report_async(tenant: &str, filename: &str, content: String) {
    let Some(config) = RemoteLogConfig::from_env() else {
        return;
    };

    let tenant = tenant.to_string();
    let filename = filename.to_string();

    tokio::spawn(async move {
        // Organize reports by date and tenant
        let now = chrono::Utc::now();
        let date_folder = now.format("%Y/%m/%d").to_string();
        // Sanitize tenant name for path
        let safe_tenant = tenant.replace(['/', '\\', ':', '.'], "_");

        let path = format!("reports/{}/{}/{}", date_folder, safe_tenant, filename);
        let message = format!("Report: {} - {}", tenant, filename);

        match upload_to_github(&config, &path, &content, &message).await {
            Ok(url) => {
                tracing::info!("Report archived to GitHub: {}", url);
            }
            Err(e) => {
                tracing::warn!("Failed to archive report to GitHub: {}", e);
            }
        }
    });
}

/// Convenience function for uploading JSON debug logs
pub fn upload_debug_json(category: &str, data: &serde_json::Value) {
    let filename = format!(
        "{}_{}.json",
        category,
        chrono::Utc::now().format("%H%M%S_%3f")
    );

    let content = serde_json::to_string_pretty(data).unwrap_or_default();
    upload_log_async(category, &filename, content);
}

/// Log API request with metadata
pub fn log_request<T: serde::Serialize>(operation: &str, payload: &T, tenant_id: Option<&str>) {
    let log_data = serde_json::json!({
        "type": "request",
        "operation": operation,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "tenant_id": tenant_id,
        "payload": payload
    });

    upload_debug_json(&format!("{}_requests", operation), &log_data);
}

/// Log API response with performance data
pub fn log_response<T: serde::Serialize>(
    operation: &str,
    response: &T,
    duration_ms: u128,
    tenant_id: Option<&str>,
    success: bool,
) {
    let log_data = serde_json::json!({
        "type": "response",
        "operation": operation,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "duration_ms": duration_ms,
        "tenant_id": tenant_id,
        "success": success,
        "response": response
    });

    upload_debug_json(&format!("{}_responses", operation), &log_data);
}

/// Log error with context
pub fn log_error(
    operation: &str,
    error_code: &str,
    error_message: &str,
    tenant_id: Option<&str>,
    context: serde_json::Value,
) {
    let log_data = serde_json::json!({
        "type": "error",
        "operation": operation,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "error_code": error_code,
        "error_message": error_message,
        "tenant_id": tenant_id,
        "context": context
    });

    upload_debug_json("errors", &log_data);
}

/// Log performance metrics with breakdown
pub fn log_performance(
    operation: &str,
    total_duration_ms: u128,
    breakdown: serde_json::Value,
    tenant_id: Option<&str>,
) {
    let log_data = serde_json::json!({
        "type": "performance",
        "operation": operation,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "total_duration_ms": total_duration_ms,
        "breakdown": breakdown,
        "tenant_id": tenant_id
    });

    upload_debug_json("performance_metrics", &log_data);
}

/// Log feature usage event
pub fn log_feature_usage(
    feature: &str,
    tenant_id: Option<&str>,
    success: bool,
    metadata: Option<serde_json::Value>,
) {
    let log_data = serde_json::json!({
        "type": "feature_usage",
        "feature": feature,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "tenant_id": tenant_id,
        "success": success,
        "metadata": metadata
    });

    upload_debug_json("feature_usage", &log_data);
}

// ==================== HTTP ENDPOINT ====================

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct SyncLogsResponse {
    pub success: bool,
    pub uploaded: usize,
    pub failed: usize,
    pub message: String,
}

/// Endpoint to sync all local debug logs to the private GitHub repository
/// POST /api/logs/sync
pub async fn sync_logs() -> impl IntoResponse {
    let Some(config) = RemoteLogConfig::from_env() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(SyncLogsResponse {
                success: false,
                uploaded: 0,
                failed: 0,
                message: "Remote logging not configured. Set GITHUB_TOKEN, GITHUB_OWNER, GITHUB_LOGS_REPO".to_string(),
            }),
        );
    };

    let debug_dir = std::path::Path::new("debug_logs");
    if !debug_dir.exists() {
        return (
            StatusCode::OK,
            Json(SyncLogsResponse {
                success: true,
                uploaded: 0,
                failed: 0,
                message: "No debug_logs directory found".to_string(),
            }),
        );
    }

    let mut uploaded = 0;
    let mut failed = 0;

    // Read all files in debug_logs
    if let Ok(entries) = std::fs::read_dir(debug_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                // Determine category from filename
                let category = if filename.starts_with("th_") {
                    "threat_hunting"
                } else if filename.starts_with("exposure_") {
                    "exposure_api"
                } else if filename.starts_with("fetch_") {
                    "fetch_operations"
                } else {
                    "misc"
                };

                // Read and upload
                if let Ok(content) = std::fs::read_to_string(&path) {
                    match upload_log(&config, category, &filename, &content).await {
                        Ok(_) => {
                            uploaded += 1;
                            // Delete local file after successful upload
                            let _ = std::fs::remove_file(&path);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to upload {}: {}", filename, e);
                            failed += 1;
                        }
                    }
                }
            }
        }
    }

    (
        StatusCode::OK,
        Json(SyncLogsResponse {
            success: failed == 0,
            uploaded,
            failed,
            message: format!("Synced {} files, {} failed", uploaded, failed),
        }),
    )
}
