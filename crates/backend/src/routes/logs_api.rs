//! Logs API - Fetch and list logs from Firestore (Hybrid)
//!
//! - Provides endpoints to browse and search logs.
//! - Fetches metadata from Firestore (Daily sharding) and content from GitHub if truncated.

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use super::admin_config;
use super::remote_log::RemoteLogConfig;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{DateTime, Duration, TimeZone, Utc};

use crate::firebase::Firestore;

/// Query parameters for listing logs
#[derive(Debug, Deserialize)]
pub struct ListLogsQuery {
    /// Date filter in YYYY-MM-DD format
    pub date: Option<String>,
    /// Category filter
    pub category: Option<String>,
    /// Maximum number of results
    pub limit: Option<i64>,
    /// Offset for pagination
    pub offset: Option<i64>,
}

/// Log file entry in the list
#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub category: String,
    pub message: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub github_path: Option<String>,
}

/// Response for listing logs
#[derive(Debug, Serialize)]
pub struct ListLogsResponse {
    pub success: bool,
    pub files: Vec<LogEntryInternal>,
    pub total: i64,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct LogEntryInternal {
    pub name: String, // timestamp + category
    pub path: String, // ID
    pub size: u64,
    pub sha: String, // ID
    pub download_url: Option<String>,
}

/// Response for getting log content
#[derive(Debug, Serialize)]
pub struct LogContentResponse {
    pub success: bool,
    pub filename: String,
    pub content: String,
    pub size: u64,
}

#[derive(Debug, Serialize)]
pub struct DailyStats {
    pub date: String,
    pub reports: i64,
    pub errors: i64,
    pub th_queries: i64,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub success: bool,
    pub period: String,
    pub total_reports: i64,
    pub total_errors: i64,
    pub daily_stats: Vec<DailyStats>,
    pub message: String,
}

/// List available log files
/// GET /api/logs
pub async fn list_logs(Query(params): Query<ListLogsQuery>) -> impl IntoResponse {
    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ListLogsResponse {
                    success: false,
                    files: vec![],
                    total: 0,
                    message: "Firestore not available".to_string(),
                }),
            )
        }
    };

    let limit = params.limit.unwrap_or(50) as usize;
    let offset = params.offset.unwrap_or(0) as usize;

    // Build date range
    let date_str = params
        .date
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    // Sharding path: system_logs/{date}/entries
    let path = format!("system_logs/{}/entries", date_str);

    // Fetch logs from Firestore
    let logs_res = firestore.list_docs::<LogEntry>(&path).await;

    match logs_res {
        Ok(mut logs) => {
            // Filter by category if needed
            if let Some(cat) = &params.category {
                logs.retain(|l| &l.category == cat);
            }

            // Sort by timestamp DESC
            logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

            let total = logs.len() as i64;

            // Pagination
            let logs_page: Vec<LogEntry> = logs.into_iter().skip(offset).take(limit).collect();

            let files: Vec<LogEntryInternal> = logs_page
                .into_iter()
                .map(|row| {
                    let ts = DateTime::parse_from_rfc3339(&row.timestamp)
                        .map(|dt| dt.format("%H:%M:%S").to_string())
                        .unwrap_or_else(|_| row.timestamp.clone());

                    LogEntryInternal {
                        name: format!("{} - {}", ts, row.message),
                        path: row.id.clone(),
                        size: 0, // Unknown size without content
                        sha: row.id.clone(),
                        download_url: None,
                    }
                })
                .collect();

            (
                StatusCode::OK,
                Json(ListLogsResponse {
                    success: true,
                    files,
                    total,
                    message: "OK".to_string(),
                }),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ListLogsResponse {
                success: false,
                files: vec![],
                total: 0,
                message: format!("Firestore error: {}", e),
            }),
        ),
    }
}

/// Helper to fetch content from GitHub
async fn fetch_from_github(path: &str) -> Option<String> {
    let config = RemoteLogConfig::from_env()?;
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-log-viewer")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .ok()?;

    if !res.status().is_success() {
        return None;
    }

    let file_info: serde_json::Value = res.json().await.ok()?;

    let encoded = file_info.get("content").and_then(|c| c.as_str())?;
    let clean = encoded.replace('\n', "");

    match BASE64.decode(&clean) {
        Ok(bytes) => Some(String::from_utf8_lossy(&bytes).to_string()),
        Err(_) => None,
    }
}

/// Get specific log file content
/// GET /api/logs/content/*path
pub async fn get_log_content(Path(id_str): Path<String>) -> impl IntoResponse {
    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(LogContentResponse {
                    success: false,
                    filename: String::new(),
                    content: "Firestore not available".to_string(),
                    size: 0,
                }),
            )
        }
    };

    // Extract date from ID: YYYY-MM-DD_UUID
    let parts: Vec<&str> = id_str.splitn(2, '_').collect();
    if parts.len() != 2 {
        return (
            StatusCode::BAD_REQUEST,
            Json(LogContentResponse {
                success: false,
                filename: String::new(),
                content: "Invalid ID format (missing date prefix)".to_string(),
                size: 0,
            }),
        );
    }
    let date_str = parts[0];
    let path = format!("system_logs/{}/entries", date_str);

    let res = firestore.get_doc::<LogEntry>(&path, &id_str).await;

    match res {
        Ok(Some(mut row)) => {
            // Check if content is truncated/missing
            let is_truncated = row.content.contains("... (truncated");

            if (row.content.is_empty() || is_truncated) && row.github_path.is_some() {
                if let Some(gh_path) = row.github_path {
                    if let Some(full_content) = fetch_from_github(&gh_path).await {
                        row.content = full_content;
                    }
                }
            }

            (
                StatusCode::OK,
                Json(LogContentResponse {
                    success: true,
                    filename: row.message,
                    content: row.content.clone(),
                    size: row.content.len() as u64,
                }),
            )
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(LogContentResponse {
                success: false,
                filename: String::new(),
                content: "Log not found".to_string(),
                size: 0,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LogContentResponse {
                success: false,
                filename: String::new(),
                content: format!("Firestore error: {}", e),
                size: 0,
            }),
        ),
    }
}

/// List available dates with logs
/// GET /api/logs/dates
pub async fn list_log_dates() -> impl IntoResponse {
    // For now, let's just return current year structure to match frontend expectations
    let years = vec!["2024".to_string(), "2025".to_string()];

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "years": years,
            "message": "OK"
        })),
    )
}

/// List categories for a specific date
/// GET /api/logs/categories?date=YYYY/MM/DD
pub async fn list_log_categories(Query(params): Query<ListLogsQuery>) -> impl IntoResponse {
    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "success": false,
                    "categories": [],
                    "message": "Firestore not available"
                })),
            )
        }
    };

    let date_str = params
        .date
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    let path = format!("system_logs/{}/entries", date_str);

    // Efficiently, we should just list and collect distinct categories.
    // Since we fetch all metadata for a day anyway (usually < 100 logs?), this is fine.
    match firestore.list_docs::<LogEntry>(&path).await {
        Ok(logs) => {
            let mut categories: Vec<String> = logs.into_iter().map(|l| l.category).collect();
            categories.sort();
            categories.dedup();

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "success": true,
                    "categories": categories,
                    "date": date_str,
                    "message": "OK"
                })),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "success": false,
                "categories": [],
                "message": format!("Firestore error: {}", e)
            })),
        ),
    }
}

/// Query parameters for access check
#[derive(Debug, Deserialize)]
pub struct AccessCheckQuery {
    pub email: String,
}

/// Response for access check
#[derive(Debug, Serialize)]
pub struct AccessCheckResponse {
    pub has_access: bool,
    pub message: String,
}

/// Check if user has access to logs
/// GET /api/logs/access?email=user@example.com
pub async fn check_log_access(Query(params): Query<AccessCheckQuery>) -> impl IntoResponse {
    let has_access = admin_config::has_log_access(&params.email).await;

    (
        StatusCode::OK,
        Json(AccessCheckResponse {
            has_access,
            message: if has_access {
                "Access granted".to_string()
            } else {
                "Access denied - not in admin list".to_string()
            },
        }),
    )
}

/// Get analytics stats for the dashboard (last N days)
/// GET /api/logs/stats?days=7
#[derive(Deserialize)]
pub struct StatsQuery {
    pub days: Option<i64>,
}

pub async fn get_log_stats(Query(params): Query<StatsQuery>) -> impl IntoResponse {
    let days = params.days.unwrap_or(7);
    let firestore = match crate::firebase::get_firestore() {
        Some(fs) => fs,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(StatsResponse {
                    success: false,
                    period: format!("{}d", days),
                    total_reports: 0,
                    total_errors: 0,
                    daily_stats: vec![],
                    message: "Firestore not available".to_string(),
                }),
            )
        }
    };

    use std::collections::HashMap;
    let mut daily_stats = Vec::new();
    let mut total_reports = 0;
    let mut total_errors = 0;

    // Parallel fetch for last N days? Or sequential to be nice to rate limit?
    // Sequential for safety. 7 days = 7 writes/reads. 2000 reads/hour limit.
    // 7 reads is fine.

    for i in (0..days).rev() {
        let d = Utc::now() - Duration::days(i);
        let date_str = d.format("%Y-%m-%d").to_string();
        let path = format!("system_logs/{}/entries", date_str);

        let mut stats = DailyStats {
            date: date_str.clone(),
            reports: 0,
            errors: 0,
            th_queries: 0,
            total: 0,
        };

        if let Ok(logs) = firestore.list_docs::<LogEntry>(&path).await {
            stats.total = logs.len() as i64;
            for log in logs {
                if log.category.contains("report") {
                    stats.reports += 1;
                    total_reports += 1;
                } else if log.category.contains("error") {
                    stats.errors += 1;
                    total_errors += 1;
                } else if log.category.contains("threat") {
                    stats.th_queries += 1;
                }
            }
        }

        daily_stats.push(stats);
    }

    (
        StatusCode::OK,
        Json(StatsResponse {
            success: true,
            period: format!("{}d", days),
            total_reports,
            total_errors,
            daily_stats,
            message: "OK".to_string(),
        }),
    )
}
