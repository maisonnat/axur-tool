//! Production status and health check module
//!
//! Provides comprehensive health checks for all services without consuming resources.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::env;

/// Simple health endpoint for cold start detection
/// GET /api/health - Returns immediately with minimal response
pub async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Service check result
#[derive(Debug, Serialize)]
pub struct ServiceCheck {
    pub name: String,
    pub status: ServiceStatus,
    pub latency_ms: Option<u64>,
    pub message: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Ok,
    Degraded,
    Error,
    Unconfigured,
}

/// Full status response
#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub overall_status: ServiceStatus,
    pub timestamp: String,
    pub backend: BackendInfo,
    pub services: Vec<ServiceCheck>,
    pub environment: EnvironmentInfo,
}

#[derive(Debug, Serialize)]
pub struct BackendInfo {
    pub version: String,
    pub rust_version: String,
    pub build_profile: String,
    pub git_hash: String,
}

#[derive(Debug, Serialize)]
pub struct EnvironmentInfo {
    pub axur_api_configured: bool,
    pub github_logs_configured: bool,
    pub github_feedback_configured: bool,
}

/// Comprehensive status endpoint - checks all services
/// GET /api/status
pub async fn full_status() -> impl IntoResponse {
    let mut services = Vec::new();
    let mut has_errors = false;
    let mut has_degraded = false;

    // 1. Check Axur API connectivity
    let axur_check = check_axur_api().await;
    if matches!(axur_check.status, ServiceStatus::Error) {
        has_errors = true;
    }
    services.push(axur_check);

    // 2. Check GitHub Logs configuration
    let github_logs_check = check_github_logs();
    if matches!(github_logs_check.status, ServiceStatus::Unconfigured) {
        has_degraded = true;
    }
    services.push(github_logs_check);

    // 3. Check GitHub Feedback configuration
    let github_feedback_check = check_github_feedback();
    if matches!(github_feedback_check.status, ServiceStatus::Unconfigured) {
        has_degraded = true;
    }
    services.push(github_feedback_check);

    // 4. Check Firestore connectivity
    let db_check = check_firestore().await;
    if matches!(db_check.status, ServiceStatus::Error) {
        has_errors = true;
    }
    services.push(db_check);

    // Overall status
    let overall_status = if has_errors {
        ServiceStatus::Error
    } else if has_degraded {
        ServiceStatus::Degraded
    } else {
        ServiceStatus::Ok
    };

    let response = StatusResponse {
        overall_status,
        timestamp: chrono::Utc::now().to_rfc3339(),
        backend: BackendInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            rust_version: "1.74".to_string(), // Approximate
            build_profile: if cfg!(debug_assertions) {
                "debug".to_string()
            } else {
                "release".to_string()
            },
            git_hash: env::var("GIT_HASH").unwrap_or_else(|_| "unknown".to_string()),
        },
        services,
        environment: EnvironmentInfo {
            axur_api_configured: env::var("AXUR_TOKEN").is_ok(),
            github_logs_configured: env::var("GH_LOGS_REPO").is_ok(),
            github_feedback_configured: env::var("GITHUB_TOKEN").is_ok(),
        },
    };

    let status_code = match response.overall_status {
        ServiceStatus::Ok => StatusCode::OK,
        ServiceStatus::Degraded => StatusCode::OK, // Still operational
        ServiceStatus::Error => StatusCode::SERVICE_UNAVAILABLE,
        ServiceStatus::Unconfigured => StatusCode::OK,
    };

    (status_code, Json(response))
}

/// Check Axur API connectivity (public endpoint)
async fn check_axur_api() -> ServiceCheck {
    let start = std::time::Instant::now();
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build();

    let client = match client {
        Ok(c) => c,
        Err(e) => {
            return ServiceCheck {
                name: "Axur API".into(),
                status: ServiceStatus::Error,
                latency_ms: None,
                message: Some(format!("Failed to create HTTP client: {}", e)),
                version: None,
            };
        }
    };

    // Try to reach Axur's public API endpoint
    let result = client
        .get("https://api.axur.com/gateway/1.0/api/customers/customers")
        .header("Accept", "application/json")
        .send()
        .await;

    let latency = start.elapsed().as_millis() as u64;

    match result {
        Ok(resp) => {
            let status_code = resp.status();
            // 401/403 are expected (no auth), but means API is reachable
            if status_code.as_u16() == 401
                || status_code.as_u16() == 403
                || status_code.is_success()
            {
                ServiceCheck {
                    name: "Axur API".into(),
                    status: ServiceStatus::Ok,
                    latency_ms: Some(latency),
                    message: Some("API reachable".into()),
                    version: None,
                }
            } else {
                ServiceCheck {
                    name: "Axur API".into(),
                    status: ServiceStatus::Degraded,
                    latency_ms: Some(latency),
                    message: Some(format!("Unexpected status: {}", status_code)),
                    version: None,
                }
            }
        }
        Err(e) => ServiceCheck {
            name: "Axur API".into(),
            status: ServiceStatus::Error,
            latency_ms: Some(latency),
            message: Some(format!("Connection failed: {}", e)),
            version: None,
        },
    }
}

/// Check GitHub Logs configuration
fn check_github_logs() -> ServiceCheck {
    let token_ok = env::var("GH_PAT").is_ok() || env::var("GITHUB_TOKEN").is_ok();
    let owner_ok = env::var("GH_OWNER").is_ok() || env::var("GITHUB_OWNER").is_ok();
    let repo_ok = env::var("GH_LOGS_REPO").is_ok() || env::var("GITHUB_LOGS_REPO").is_ok();

    if token_ok && owner_ok && repo_ok {
        ServiceCheck {
            name: "GitHub Logs".into(),
            status: ServiceStatus::Ok,
            latency_ms: None,
            message: Some("Configured".into()),
            version: None,
        }
    } else if token_ok && owner_ok {
        ServiceCheck {
            name: "GitHub Logs".into(),
            status: ServiceStatus::Ok,
            latency_ms: None,
            message: Some("Configured (using default repo)".into()),
            version: None,
        }
    } else {
        let missing: Vec<&str> = [
            if !token_ok {
                Some("GH_PAT/GITHUB_TOKEN")
            } else {
                None
            },
            if !owner_ok {
                Some("GH_OWNER/GITHUB_OWNER")
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect();

        ServiceCheck {
            name: "GitHub Logs".into(),
            status: ServiceStatus::Unconfigured,
            latency_ms: None,
            message: Some(format!("Missing: {}", missing.join(", "))),
            version: None,
        }
    }
}

/// Check GitHub Feedback configuration
fn check_github_feedback() -> ServiceCheck {
    let token_ok = env::var("GH_PAT").is_ok() || env::var("GITHUB_TOKEN").is_ok();
    let owner_ok = env::var("GH_OWNER").is_ok() || env::var("GITHUB_OWNER").is_ok();
    let repo_ok = env::var("GH_REPO").is_ok() || env::var("GITHUB_REPO").is_ok();

    if token_ok && owner_ok && repo_ok {
        ServiceCheck {
            name: "GitHub Feedback".into(),
            status: ServiceStatus::Ok,
            latency_ms: None,
            message: Some("Configured".into()),
            version: None,
        }
    } else {
        let missing: Vec<&str> = [
            if !token_ok {
                Some("GH_PAT/GITHUB_TOKEN")
            } else {
                None
            },
            if !owner_ok {
                Some("GH_OWNER/GITHUB_OWNER")
            } else {
                None
            },
            if !repo_ok {
                Some("GH_REPO/GITHUB_REPO")
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect();

        ServiceCheck {
            name: "GitHub Feedback".into(),
            status: ServiceStatus::Unconfigured,
            latency_ms: None,
            message: Some(format!("Missing: {}", missing.join(", "))),
            version: None,
        }
    }
}

/// Check Firestore connectivity
async fn check_firestore() -> ServiceCheck {
    let start = std::time::Instant::now();

    if let Some(firestore) = crate::firebase::get_firestore() {
        // Perform a lightweight list operation.
        // We can list `users` with limit 1.
        match firestore.list_docs::<serde_json::Value>("users").await {
            Ok(_) => {
                let duration = start.elapsed().as_millis() as u64;
                ServiceCheck {
                    name: "Firestore".to_string(),
                    status: ServiceStatus::Ok,
                    latency_ms: Some(duration),
                    message: Some("Connected".to_string()),
                    version: None,
                }
            }
            Err(e) => ServiceCheck {
                name: "Firestore".to_string(),
                status: ServiceStatus::Error,
                latency_ms: None,
                message: Some(format!("Connection success but query failed: {}", e)),
                version: None,
            },
        }
    } else {
        ServiceCheck {
            name: "Firestore".to_string(),
            status: ServiceStatus::Error,
            latency_ms: None,
            message: Some("Firestore client not initialized".to_string()),
            version: None,
        }
    }
}
