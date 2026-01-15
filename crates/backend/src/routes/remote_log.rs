//! Remote logging module - Hybrid (Firestore + GitHub)
//!
//! - Writes full log content to GitHub (Unlimited storage)
//! - Writes metadata and indexed fields to Firestore (Fast queries)
//! - Uses daily sharding for Firestore collections to optimize cost/performance.

use axum::http::StatusCode;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::json;
use uuid::Uuid;

/// Configuration for remote logging (GitHub)
pub struct RemoteLogConfig {
    pub token: String,
    pub owner: String,
    pub repo: String,
}

impl RemoteLogConfig {
    /// Load config from environment variables
    pub fn from_env() -> Option<Self> {
        let token = std::env::var("GH_PAT")
            .or_else(|_| std::env::var("GITHUB_TOKEN"))
            .ok()?;

        let owner = std::env::var("GH_OWNER")
            .or_else(|_| std::env::var("GITHUB_OWNER"))
            .ok()?;

        let repo = std::env::var("GH_LOGS_REPO")
            .or_else(|_| std::env::var("GITHUB_LOGS_REPO"))
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

/// Upload a file to the private GitHub repository
async fn upload_to_github(
    config: &RemoteLogConfig,
    path: &str,
    content: &str,
    message: &str,
) -> Result<String, String> {
    let upload_url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    let encoded_content = BASE64.encode(content.as_bytes());

    let perform_upload = |sha: Option<String>| async {
        let mut body_map = serde_json::Map::new();
        body_map.insert("message".to_string(), json!(message));
        body_map.insert("content".to_string(), json!(encoded_content));

        // Committer is optional, GitHub uses authenticated user by default
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
        let client = reqwest::Client::new();

        client
            .put(&upload_url)
            .header("Authorization", format!("Bearer {}", config.token))
            .header("User-Agent", "axur-bot")
            .header("Accept", "application/vnd.github.v3+json")
            .json(&body)
            .send()
            .await
    };

    let res = perform_upload(None).await.map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let resp_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        return Ok(resp_json
            .get("content")
            .and_then(|c| c.get("html_url"))
            .and_then(|u| u.as_str())
            .unwrap_or("")
            .to_string());
    }

    // Handle conflict (update)
    if res.status() == reqwest::StatusCode::CONFLICT
        || res.status() == reqwest::StatusCode::UNPROCESSABLE_ENTITY
    {
        if let Some(sha) = get_file_sha(config, path).await {
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
            }
        }
    }

    Err(format!("GitHub upload failed: {}", res.status()))
}

/// Upload a log entry (Hybrid: GitHub + Firestore)
pub async fn upload_log(category: &str, filename: &str, content: &str) -> Result<String, String> {
    let now = chrono::Utc::now();
    let message = format!("Log: {} - {}", category, filename);

    // 1. Upload to GitHub
    let mut github_url = String::new();
    let mut github_path = String::new();

    if let Some(config) = RemoteLogConfig::from_env() {
        let date_folder = now.format("%Y/%m/%d").to_string();
        github_path = format!("logs/{}/{}/{}", date_folder, category, filename);

        match upload_to_github(&config, &github_path, content, &message).await {
            Ok(url) => github_url = url,
            Err(e) => tracing::warn!("GitHub upload failed: {}, continuing to DB", e),
        }
    }

    // 2. Insert into Firestore
    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => return Err("Firestore not available".to_string()),
    };

    let level = if category.contains("error") {
        "error"
    } else if category.contains("warn") {
        "warn"
    } else {
        "info"
    };

    // Truncate content for DB
    let max_db_content_size = 2000;
    let db_content = if content.len() > max_db_content_size {
        format!(
            "{}... (truncated, see GitHub)",
            &content[..max_db_content_size]
        )
    } else {
        content.to_string()
    };

    let metadata = serde_json::from_str::<serde_json::Value>(content).unwrap_or(json!({}));

    // Sharding by Date: system_logs/{YYYY-MM-DD}/entries/{ID}
    let date_key = now.format("%Y-%m-%d").to_string();
    let uuid_val = Uuid::new_v4();
    let id = format!("{}_{}", date_key, uuid_val); // Prefix with date for easier lookup if needed

    let log_entry = json!({
        "id": id,
        "timestamp": now.to_rfc3339(),
        "category": category,
        "message": message,
        "level": level,
        "content": db_content,
        "metadata": metadata,
        "github_path": github_path,
        "github_html_url": github_url
    });

    // Ensure the daily document exists (creating it is cheap/idempotent with set)
    // Actually, we don't strictly need the parent doc to exist to add to subcollection in Firestore Native?
    // In Datastore mode yes, in Native mode usually yes for listing.
    // We'll create a dummy doc for the date to enable listing dates.
    // Optimistic: Assume it exists or we create it once per day.
    // For now we just write to subcollection. If we want to list dates, we should write a date doc.
    // Let's write the log first.

    let path = format!("system_logs/{}/entries", date_key);
    match firestore.set_doc(&path, &id, &log_entry).await {
        Ok(_) => {
            // Also ensure date doc exists? - optimization: do this only on distinct dates?
            // Too expensive to check every time.
            // We'll assume the client 'list dates' will list root collection `system_logs`.
            // Depending on Firestore, empty docs (only having subcollections) might show up or not.
            Ok(id)
        }
        Err(e) => Err(format!("Firestore error: {}", e)),
    }
}

/// Upload multiple logs in batch (fire-and-forget style)
pub fn upload_log_async(category: &str, filename: &str, content: String) {
    let category = category.to_string();
    let filename = filename.to_string();

    tokio::spawn(async move {
        match upload_log(&category, &filename, &content).await {
            Ok(id) => tracing::debug!("Log saved: {}", id),
            Err(e) => tracing::warn!("Failed to save log: {}", e),
        }
    });
}

/// Upload a generated report (Hybrid)
pub fn upload_report_async(tenant: &str, filename: &str, content: String) {
    let _tenant = tenant.to_string();
    let filename = filename.to_string();
    let category = "reports".to_string();

    tokio::spawn(async move {
        if let Err(e) = upload_log(&category, &filename, &content).await {
            tracing::warn!("Failed to archive report: {}", e);
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
    // Firestore Analytics
    if let Some(firestore) = crate::firebase::get_firestore() {
        let op = operation.to_string();
        let tid = tenant_id.map(|s| s.to_string());
        let props = serde_json::to_value(payload).unwrap_or(json!({}));

        tokio::spawn(async move {
            let now = chrono::Utc::now();
            let date_key = now.format("%Y-%m-%d").to_string();
            let id = Uuid::new_v4().to_string();

            let event = json!({
                "event_type": "api_request",
                "tenant_id": tid,
                "timestamp": now.to_rfc3339(),
                "properties": { "operation": op, "payload": props }
            });

            let _ = firestore
                .set_doc(
                    &format!("analytics_events/{}/events", date_key),
                    &id,
                    &event,
                )
                .await;
        });
    }

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
    if let Some(firestore) = crate::firebase::get_firestore() {
        let op = operation.to_string();
        let tid = tenant_id.map(|s| s.to_string());
        // Avoid cloning large response if not needed? Properties can be simplified.
        let props = json!({
            "operation": op,
            "duration_ms": duration_ms as i64,
            "success": success
        });

        tokio::spawn(async move {
            let now = chrono::Utc::now();
            let date_key = now.format("%Y-%m-%d").to_string();
            let id = Uuid::new_v4().to_string();

            let event = json!({
                "event_type": "api_response",
                "tenant_id": tid,
                "timestamp": now.to_rfc3339(),
                "properties": props
            });

            let _ = firestore
                .set_doc(
                    &format!("analytics_events/{}/events", date_key),
                    &id,
                    &event,
                )
                .await;
        });
    }

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
    if let Some(firestore) = crate::firebase::get_firestore() {
        let op = operation.to_string();
        let code = error_code.to_string();
        let msg = error_message.to_string();
        let tid = tenant_id.map(|s| s.to_string());
        let ctx = context.clone();

        tokio::spawn(async move {
            let now = chrono::Utc::now();
            let date_key = now.format("%Y-%m-%d").to_string();
            let id = Uuid::new_v4().to_string();

            let event = json!({
                "event_type": "error",
                "tenant_id": tid,
                "timestamp": now.to_rfc3339(),
                "properties": {
                   "operation": op,
                   "error_code": code,
                   "error_message": msg,
                   "context": ctx
                }
            });

            let _ = firestore
                .set_doc(
                    &format!("analytics_events/{}/events", date_key),
                    &id,
                    &event,
                )
                .await;
        });
    }

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
    if let Some(firestore) = crate::firebase::get_firestore() {
        let feat = feature.to_string();
        let tid = tenant_id.map(|s| s.to_string());
        let meta = metadata.clone().unwrap_or(json!({}));

        tokio::spawn(async move {
            let now = chrono::Utc::now();
            let date_key = now.format("%Y-%m-%d").to_string();
            let id = Uuid::new_v4().to_string();

            let event = json!({
                "event_type": "feature_usage",
                "tenant_id": tid,
                "timestamp": now.to_rfc3339(),
                "properties": {
                   "feature": feat,
                   "success": success,
                   "metadata": meta
                }
            });

            let _ = firestore
                .set_doc(
                    &format!("analytics_events/{}/events", date_key),
                    &id,
                    &event,
                )
                .await;
        });
    }

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

use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct SyncLogsResponse {
    pub success: bool,
    pub uploaded: usize,
    pub failed: usize,
    pub message: String,
}

/// Endpoint to sync all local debug logs to the database
/// POST /api/logs/sync
pub async fn sync_logs() -> impl IntoResponse {
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
                    match upload_log(category, &filename, &content).await {
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
            message: format!("Synced {} files to DB, {} failed", uploaded, failed),
        }),
    )
}
