//! Admin configuration management
//!
//! Checks admin access against the database.

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
    let email_lower = email.to_lowercase();
    let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");

    // Check Firestore "allowed_users"
    if let Some(firestore) = crate::firebase::get_firestore() {
        match firestore
            .get_doc::<serde_json::Value>("allowed_users", &doc_id)
            .await
        {
            Ok(Some(doc)) => {
                if let Some(role) = doc.get("role").and_then(|v| v.as_str()) {
                    return role == "admin";
                }
            }
            Ok(None) => {}
            Err(e) => {
                tracing::error!("Firestore error checking admin status: {}", e);
            }
        }
    }

    // Fallback? GitHub Storage?
    // admin.rs uses GitHub storage. We should probably use that too if configured.
    if let Some(storage) = crate::github_storage::get_github_storage() {
        match storage.is_admin(email).await {
            Ok(true) => return true,
            _ => {}
        }
    }

    false
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
