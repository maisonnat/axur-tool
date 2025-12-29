//! Admin configuration management
//!
//! Fetches and caches admin configuration from GitHub repository.
//! Config file: config/admins.json in the logs repository.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use super::remote_log::RemoteLogConfig;

/// Cache duration for admin config (5 minutes)
const CACHE_DURATION: Duration = Duration::from_secs(300);

/// Admin configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdminConfig {
    /// List of email addresses with admin access to logs
    #[serde(default)]
    pub allowed_emails: Vec<String>,
}

/// Cached admin config with timestamp
struct CachedConfig {
    config: AdminConfig,
    fetched_at: Instant,
}

/// Global cache for admin config
static ADMIN_CACHE: OnceLock<RwLock<Option<CachedConfig>>> = OnceLock::new();

fn get_cache() -> &'static RwLock<Option<CachedConfig>> {
    ADMIN_CACHE.get_or_init(|| RwLock::new(None))
}

/// Fetch admin config from GitHub, with caching
pub async fn get_admin_config() -> AdminConfig {
    let cache = get_cache();

    // Check if cache is valid
    {
        let cache_read = cache.read().await;
        if let Some(ref cached) = *cache_read {
            if cached.fetched_at.elapsed() < CACHE_DURATION {
                return cached.config.clone();
            }
        }
    }

    // Fetch fresh config
    let config = fetch_admin_config_from_github().await;

    // Update cache
    {
        let mut cache_write = cache.write().await;
        *cache_write = Some(CachedConfig {
            config: config.clone(),
            fetched_at: Instant::now(),
        });
    }

    config
}

/// Fetch admin config directly from GitHub
async fn fetch_admin_config_from_github() -> AdminConfig {
    let Some(gh_config) = RemoteLogConfig::from_env() else {
        tracing::warn!("GitHub config not available, returning empty admin config");
        return AdminConfig::default();
    };

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/contents/config/admins.json",
        gh_config.owner, gh_config.repo
    );

    let res = match client
        .get(&url)
        .header("Authorization", format!("Bearer {}", gh_config.token))
        .header("User-Agent", "axur-admin-config")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("Failed to fetch admin config: {}", e);
            return AdminConfig::default();
        }
    };

    if !res.status().is_success() {
        let status = res.status();
        tracing::warn!("GitHub API returned {} for admin config", status);
        return AdminConfig::default();
    }

    let file_info: serde_json::Value = match res.json().await {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("Failed to parse admin config response: {}", e);
            return AdminConfig::default();
        }
    };

    // Decode base64 content
    let encoded_content = file_info
        .get("content")
        .and_then(|c| c.as_str())
        .unwrap_or("");

    let clean_content = encoded_content.replace('\n', "");

    let json_content = match BASE64.decode(&clean_content) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(e) => {
            tracing::warn!("Failed to decode admin config: {}", e);
            return AdminConfig::default();
        }
    };

    match serde_json::from_str(&json_content) {
        Ok(config) => config,
        Err(e) => {
            tracing::warn!("Failed to parse admin config JSON: {}", e);
            AdminConfig::default()
        }
    }
}

/// Check if an email has admin access to logs
pub async fn has_log_access(email: &str) -> bool {
    let config = get_admin_config().await;
    let email_lower = email.to_lowercase();

    config
        .allowed_emails
        .iter()
        .any(|allowed| allowed.to_lowercase() == email_lower)
}

/// Invalidate the admin config cache (for testing/manual refresh)
#[allow(dead_code)]
pub async fn invalidate_cache() {
    let cache = get_cache();
    let mut cache_write = cache.write().await;
    *cache_write = None;
}
