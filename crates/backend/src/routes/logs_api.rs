//! Logs API - Fetch and list logs from database (Hybrid)
//!
//! Provides endpoints to browse and search logs.
//! Fetches metadata from PostgreSQL and content from GitHub if truncated.

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use super::admin_config;
use super::remote_log::RemoteLogConfig;
use crate::db::get_db;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use chrono::{DateTime, Duration, TimeZone, Utc};

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
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct LogEntry {
    pub id: uuid::Uuid,
    pub timestamp: DateTime<Utc>,
    pub category: String,
    pub message: String,
    #[sqlx(default)]
    pub content: String,
    #[sqlx(default)]
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
    let pool = match get_db() {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ListLogsResponse {
                    success: false,
                    files: vec![],
                    total: 0,
                    message: "Database not available".to_string(),
                }),
            )
        }
    };

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    // Build date range
    let date_str = params
        .date
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    // Parse date safely
    let start_date = match chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .or_else(|_| chrono::NaiveDate::parse_from_str(&date_str, "%Y/%m/%d"))
    {
        Ok(d) => Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap()),
        Err(_) => Utc::now(), // Fallback
    };
    let end_date = start_date + Duration::days(1);

    let category_filter = params.category.as_deref();

    // Query logs
    // Optimization: Don't select 'content' for listing, it's heavy
    let logs = if let Some(cat) = category_filter {
        sqlx::query_as::<_, LogEntry>(
            r#"
            SELECT id, timestamp, category, message, '' as content, github_path
            FROM system_logs 
            WHERE timestamp >= $1 AND timestamp < $2 AND category = $3
            ORDER BY timestamp DESC
            LIMIT $4 OFFSET $5
            "#,
        )
        .bind(start_date)
        .bind(end_date)
        .bind(cat)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, LogEntry>(
            r#"
            SELECT id, timestamp, category, message, '' as content, github_path
            FROM system_logs 
            WHERE timestamp >= $1 AND timestamp < $2
            ORDER BY timestamp DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(start_date)
        .bind(end_date)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    };

    match logs {
        Ok(rows) => {
            let files: Vec<LogEntryInternal> = rows
                .into_iter()
                .map(|row| {
                    LogEntryInternal {
                        name: format!("{} - {}", row.timestamp.format("%H:%M:%S"), row.message),
                        path: row.id.to_string(),
                        size: 0, // Unknown size without content
                        sha: row.id.to_string(),
                        download_url: None,
                    }
                })
                .collect();

            let total: i64 = files.len() as i64;

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
                message: format!("Database error: {}", e),
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
    let pool = match get_db() {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(LogContentResponse {
                    success: false,
                    filename: String::new(),
                    content: "Database not available".to_string(),
                    size: 0,
                }),
            )
        }
    };

    let Ok(id) = uuid::Uuid::parse_str(&id_str) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(LogContentResponse {
                success: false,
                filename: String::new(),
                content: "Invalid ID format".to_string(),
                size: 0,
            }),
        );
    };

    let res = sqlx::query_as::<_, LogEntry>(
        "SELECT id, timestamp, category, message, content, github_path FROM system_logs WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await;

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
                content: format!("Database error: {}", e),
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
    let pool = match get_db() {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "success": false,
                    "categories": [],
                    "message": "Database not available"
                })),
            )
        }
    };

    let date_str = params
        .date
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());
    // Parse date
    let start_date = match chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .or_else(|_| chrono::NaiveDate::parse_from_str(&date_str, "%Y/%m/%d"))
    {
        Ok(d) => Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap()),
        Err(_) => Utc::now(), // Fallback
    };
    let end_date = start_date + Duration::days(1);

    #[derive(sqlx::FromRow)]
    struct CatRow {
        category: String,
    }

    let res = sqlx::query_as::<_, CatRow>(
        "SELECT DISTINCT category FROM system_logs WHERE timestamp >= $1 AND timestamp < $2",
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await;

    match res {
        Ok(rows) => {
            let categories: Vec<String> = rows.into_iter().map(|r| r.category).collect();
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
                "message": format!("Database error: {}", e)
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
    let pool = match get_db() {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(StatsResponse {
                    success: false,
                    period: format!("{}d", days),
                    total_reports: 0,
                    total_errors: 0,
                    daily_stats: vec![],
                    message: "Database not available".to_string(),
                }),
            )
        }
    };

    let start_date = Utc::now() - Duration::days(days);

    #[derive(sqlx::FromRow)]
    struct StatRow {
        day: Option<chrono::NaiveDate>, // date_trunc returns timestamp, we cast to date
        category: String,
        count: Option<i64>,
    }

    // Efficient aggregation query
    let res = sqlx::query_as::<_, StatRow>(
        r#"
        SELECT 
            DATE(timestamp) as day, 
            category, 
            COUNT(*) as count 
        FROM system_logs 
        WHERE timestamp >= $1 
        GROUP BY 1, 2
        ORDER BY 1 ASC
        "#,
    )
    .bind(start_date)
    .fetch_all(pool)
    .await;

    match res {
        Ok(rows) => {
            // Process rows in memory to group by day struct
            use std::collections::HashMap;
            let mut day_map: HashMap<String, DailyStats> = HashMap::new();
            let mut total_reports = 0;
            let mut total_errors = 0;

            // Pre-fill days
            for i in 0..days {
                let d = Utc::now() - Duration::days(i);
                let date_str = d.format("%Y-%m-%d").to_string();
                day_map.insert(
                    date_str.clone(),
                    DailyStats {
                        date: date_str,
                        reports: 0,
                        errors: 0,
                        th_queries: 0,
                        total: 0,
                    },
                );
            }

            for row in rows {
                if let Some(day) = row.day {
                    let date_str = day.format("%Y-%m-%d").to_string();
                    let count = row.count.unwrap_or(0);

                    if let Some(stats) = day_map.get_mut(&date_str) {
                        stats.total += count;
                        if row.category.contains("report") {
                            stats.reports += count;
                            total_reports += count;
                        } else if row.category.contains("error") {
                            stats.errors += count;
                            total_errors += count;
                        } else if row.category.contains("threat") {
                            stats.th_queries += count;
                        }
                    }
                }
            }

            let mut daily_stats: Vec<DailyStats> = day_map.into_values().collect();
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
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(StatsResponse {
                success: false,
                period: format!("{}d", days),
                total_reports: 0,
                total_errors: 0,
                daily_stats: vec![],
                message: format!("Database error: {}", e),
            }),
        ),
    }
}
