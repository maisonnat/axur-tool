//! Admin configuration management
//!
//! Checks admin access against the database.

use crate::db::get_db;
use serde::{Deserialize, Serialize};

/// Admin configuration structure (kept for compatibility)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdminConfig {
    /// List of email addresses with admin access to logs
    #[serde(default)]
    pub allowed_emails: Vec<String>,
}

/// Check if an email has admin access to logs
pub async fn has_log_access(email: &str) -> bool {
    let pool = match get_db() {
        Some(p) => p,
        None => {
            tracing::warn!("DB not initialized, defaulting to deny access");
            return false;
        }
    };

    let email_lower = email.to_lowercase();

    // Check if email exists in admin_users table
    let res = sqlx::query("SELECT 1 FROM admin_users WHERE LOWER(email) = $1")
        .bind(&email_lower)
        .fetch_optional(pool)
        .await;

    match res {
        Ok(Some(_)) => true,
        Ok(None) => {
            // BACKWARD COMPATIBILITY / FALLBACK
            // If table is empty, maybe allow hardcoded admins or check env var?
            // For now, let's allow a specific env var admin for bootstrapping
            if let Ok(admin_env) = std::env::var("AXUR_ADMIN_EMAIL") {
                if admin_env.to_lowercase() == email_lower {
                    return true;
                }
            }
            false
        }
        Err(e) => {
            tracing::error!("Failed to check admin status: {}", e);
            false
        }
    }
}

/// Invalidate the admin config cache (No-op as we check DB directly now)
#[allow(dead_code)]
pub async fn invalidate_cache() {
    // No-op
}

// Deprecated functions kept to avoid breaking compilation if used elsewhere
#[allow(dead_code)]
pub async fn get_admin_config() -> AdminConfig {
    // Return empty or dummy config
    AdminConfig::default()
}
