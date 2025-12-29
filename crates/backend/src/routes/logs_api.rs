//! Logs API - Fetch and list logs from private GitHub repository
//!
//! Provides endpoints to browse and search logs stored in GitHub.
//! Access is controlled via config/admins.json in the logs repository.

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};

use super::admin_config;
use super::remote_log::RemoteLogConfig;
use chrono::{Duration, Utc};
use futures::future::join_all;
use serde_json::Value;

/// Query parameters for listing logs
#[derive(Debug, Deserialize)]
pub struct ListLogsQuery {
    /// Date filter in YYYY-MM-DD format
    pub date: Option<String>,
    /// Category filter (e.g., "threat_hunting", "errors")
    pub category: Option<String>,
    /// Maximum number of results
    pub limit: Option<usize>,
    /// Offset for pagination
    pub offset: Option<usize>,
}

/// Log file entry in the list
#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub sha: String,
    pub download_url: Option<String>,
}

/// Response for listing logs
#[derive(Debug, Serialize)]
pub struct ListLogsResponse {
    pub success: bool,
    pub files: Vec<LogEntry>,
    pub total: usize,
    pub message: String,
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
    pub reports: usize,
    pub errors: usize,
    pub th_queries: usize,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub success: bool,
    pub period: String,
    pub total_reports: usize,
    pub total_errors: usize,
    pub daily_stats: Vec<DailyStats>,
    pub message: String,
}

/// List available log files
/// GET /api/logs
pub async fn list_logs(Query(params): Query<ListLogsQuery>) -> impl IntoResponse {
    let Some(config) = RemoteLogConfig::from_env() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ListLogsResponse {
                success: false,
                files: vec![],
                total: 0,
                message: "Remote logging not configured".to_string(),
            }),
        );
    };

    // Build the path to search
    let date_path = params
        .date
        .unwrap_or_else(|| chrono::Utc::now().format("%Y/%m/%d").to_string());

    let base_path = if let Some(ref category) = params.category {
        format!("logs/{}/{}", date_path, category)
    } else {
        format!("logs/{}", date_path)
    };

    // Fetch directory listing from GitHub
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, base_path
    );

    let res = match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-log-viewer")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ListLogsResponse {
                    success: false,
                    files: vec![],
                    total: 0,
                    message: format!("GitHub API error: {}", e),
                }),
            );
        }
    };

    if !res.status().is_success() {
        let status = res.status();
        let err_text = res.text().await.unwrap_or_default();

        // 404 means no logs for that date/category (not an error)
        if status.as_u16() == 404 {
            return (
                StatusCode::OK,
                Json(ListLogsResponse {
                    success: true,
                    files: vec![],
                    total: 0,
                    message: "No logs found for the specified criteria".to_string(),
                }),
            );
        }

        return (
            StatusCode::BAD_GATEWAY,
            Json(ListLogsResponse {
                success: false,
                files: vec![],
                total: 0,
                message: format!("GitHub API error {}: {}", status, err_text),
            }),
        );
    }

    // Parse response - could be array (directory) or object (single file)
    let content: serde_json::Value = match res.json().await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListLogsResponse {
                    success: false,
                    files: vec![],
                    total: 0,
                    message: format!("Failed to parse response: {}", e),
                }),
            );
        }
    };

    let mut files = Vec::new();

    // Handle array (directory listing)
    if let Some(arr) = content.as_array() {
        for item in arr {
            if item.get("type").and_then(|t| t.as_str()) == Some("file") {
                files.push(LogEntry {
                    name: item
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("")
                        .to_string(),
                    path: item
                        .get("path")
                        .and_then(|p| p.as_str())
                        .unwrap_or("")
                        .to_string(),
                    size: item.get("size").and_then(|s| s.as_u64()).unwrap_or(0),
                    sha: item
                        .get("sha")
                        .and_then(|s| s.as_str())
                        .unwrap_or("")
                        .to_string(),
                    download_url: item
                        .get("download_url")
                        .and_then(|u| u.as_str())
                        .map(String::from),
                });
            }
        }
    }

    // Sort by name descending (newest first based on timestamp in name)
    files.sort_by(|a, b| b.name.cmp(&a.name));

    let total = files.len();

    // Apply pagination
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(50);
    let files: Vec<LogEntry> = files.into_iter().skip(offset).take(limit).collect();

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

/// Get specific log file content
/// GET /api/logs/content/*path
pub async fn get_log_content(Path(path): Path<String>) -> impl IntoResponse {
    let Some(config) = RemoteLogConfig::from_env() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(LogContentResponse {
                success: false,
                filename: String::new(),
                content: "Remote logging not configured".to_string(),
                size: 0,
            }),
        );
    };

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        config.owner, config.repo, path
    );

    let res = match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-log-viewer")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(LogContentResponse {
                    success: false,
                    filename: String::new(),
                    content: format!("GitHub API error: {}", e),
                    size: 0,
                }),
            );
        }
    };

    if !res.status().is_success() {
        let status = res.status();
        let err_text = res.text().await.unwrap_or_default();
        return (
            StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
            Json(LogContentResponse {
                success: false,
                filename: String::new(),
                content: format!("GitHub API error {}: {}", status, err_text),
                size: 0,
            }),
        );
    }

    let file_info: serde_json::Value = match res.json().await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LogContentResponse {
                    success: false,
                    filename: String::new(),
                    content: format!("Failed to parse response: {}", e),
                    size: 0,
                }),
            );
        }
    };

    let filename = file_info
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap_or("unknown")
        .to_string();

    let size = file_info.get("size").and_then(|s| s.as_u64()).unwrap_or(0);

    // Decode base64 content
    let encoded_content = file_info
        .get("content")
        .and_then(|c| c.as_str())
        .unwrap_or("");

    // GitHub returns content with newlines, remove them before decoding
    let clean_content = encoded_content.replace('\n', "");

    let content = match BASE64.decode(&clean_content) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LogContentResponse {
                    success: false,
                    filename,
                    content: format!("Failed to decode content: {}", e),
                    size,
                }),
            );
        }
    };

    (
        StatusCode::OK,
        Json(LogContentResponse {
            success: true,
            filename,
            content,
            size,
        }),
    )
}

/// List available dates with logs
/// GET /api/logs/dates
pub async fn list_log_dates() -> impl IntoResponse {
    let Some(config) = RemoteLogConfig::from_env() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "success": false,
                "dates": [],
                "message": "Remote logging not configured"
            })),
        );
    };

    // Get years first
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/logs",
        config.owner, config.repo
    );

    let res = match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-log-viewer")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({
                    "success": false,
                    "dates": [],
                    "message": format!("GitHub API error: {}", e)
                })),
            );
        }
    };

    if !res.status().is_success() {
        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "dates": [],
                "message": "No logs found"
            })),
        );
    }

    let content: Vec<serde_json::Value> = res.json().await.unwrap_or_default();

    // Just return the current year/month structure for now
    let years: Vec<String> = content
        .iter()
        .filter(|item| item.get("type").and_then(|t| t.as_str()) == Some("dir"))
        .filter_map(|item| item.get("name").and_then(|n| n.as_str()).map(String::from))
        .collect();

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
    let Some(config) = RemoteLogConfig::from_env() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "success": false,
                "categories": [],
                "message": "Remote logging not configured"
            })),
        );
    };

    let date_path = params
        .date
        .unwrap_or_else(|| chrono::Utc::now().format("%Y/%m/%d").to_string());

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/logs/{}",
        config.owner, config.repo, date_path
    );

    let res = match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .header("User-Agent", "axur-log-viewer")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({
                    "success": false,
                    "categories": [],
                    "message": format!("GitHub API error: {}", e)
                })),
            );
        }
    };

    if !res.status().is_success() {
        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "categories": [],
                "message": "No logs found for this date"
            })),
        );
    }

    let content: Vec<serde_json::Value> = res.json().await.unwrap_or_default();

    let categories: Vec<String> = content
        .iter()
        .filter(|item| item.get("type").and_then(|t| t.as_str()) == Some("dir"))
        .filter_map(|item| item.get("name").and_then(|n| n.as_str()).map(String::from))
        .collect();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "success": true,
            "categories": categories,
            "date": date_path,
            "message": "OK"
        })),
    )
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
    let Some(config) = RemoteLogConfig::from_env() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(StatsResponse {
                success: false,
                period: format!("{}d", days),
                total_reports: 0,
                total_errors: 0,
                daily_stats: vec![],
                message: "Remote logging not configured".to_string(),
            }),
        );
    };

    let client = reqwest::Client::new();
    let today = Utc::now();
    let mut tasks = Vec::new();

    // Generate tasks for the last N days
    for i in 0..days {
        let date = today - Duration::days(i);
        let date_str = date.format("%Y/%m/%d").to_string();
        let display_date = date.format("%Y-%m-%d").to_string();

        let client = client.clone();
        let owner = config.owner.clone();
        let repo = config.repo.clone();
        let token = config.token.clone();

        tasks.push(tokio::spawn(async move {
            let url = format!(
                "https://api.github.com/repos/{}/{}/contents/logs/{}",
                owner, repo, date_str
            );

            let res = client
                .get(&url)
                .header("Authorization", format!("Bearer {}", token))
                .header("User-Agent", "axur-log-viewer")
                .header("Accept", "application/vnd.github.v3+json")
                .send()
                .await;

            let mut stats = DailyStats {
                date: display_date,
                reports: 0,
                errors: 0,
                th_queries: 0,
                total: 0,
            };

            if let Ok(resp) = res {
                if resp.status().is_success() {
                    if let Ok(content) = resp.json::<Vec<Value>>().await {
                        // Iterate through folders (categories) for this day
                        for item in content {
                            if let (Some(name), Some(item_type)) = (
                                item.get("name").and_then(|n| n.as_str()),
                                item.get("type").and_then(|t| t.as_str()),
                            ) {
                                if item_type == "dir" {
                                    // Make a secondary request to count files in this category
                                    // Optimization: This is expensive, but necessary without a recursive API or cache.
                                    // For a dashboard, we might want to cache this result in memory or Redis later.
                                    // For now, let's assume standard API rate limits.
                                    let sub_url = format!("{}/{}", url, name);
                                    if let Ok(sub_res) = client
                                        .get(&sub_url)
                                        .header("Authorization", format!("Bearer {}", token))
                                        .header("User-Agent", "axur-log-viewer")
                                        .header("Accept", "application/vnd.github.v3+json")
                                        .send()
                                        .await
                                    {
                                        if let Ok(files) = sub_res.json::<Vec<Value>>().await {
                                            let count = files.len();
                                            match name {
                                                "report" | "reports" => stats.reports += count,
                                                "error" | "errors" => stats.errors += count,
                                                "threat_hunting" => stats.th_queries += count,
                                                _ => {}
                                            }
                                            stats.total += count;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            stats
        }));
    }

    // Execute all tasks concurrently
    let results = join_all(tasks).await;

    // Aggregate results
    let mut daily_stats = Vec::new();
    let mut total_reports = 0;
    let mut total_errors = 0;

    for res in results {
        if let Ok(stats) = res {
            total_reports += stats.reports;
            total_errors += stats.errors;
            daily_stats.push(stats);
        }
    }

    // Sort by date ascending
    daily_stats.sort_by(|a, b| a.date.cmp(&b.date));

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
