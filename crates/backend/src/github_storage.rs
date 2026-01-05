//! GitHub Storage Backend
//!
//! Uses a private GitHub repository as storage for user templates and data.
//! Zero SQL cost, automatic versioning via Git history.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// GitHub API configuration
#[derive(Clone)]
pub struct GitHubStorageConfig {
    pub token: String,
    pub owner: String,
    pub repo: String,
}

impl GitHubStorageConfig {
    /// Create from environment variables
    pub fn from_env() -> Option<Self> {
        Some(Self {
            token: std::env::var("GITHUB_TOKEN").ok()?,
            owner: std::env::var("GITHUB_OWNER").unwrap_or_else(|_| "maisonnat".to_string()),
            repo: std::env::var("GITHUB_LOGS_REPO")
                .unwrap_or_else(|_| "axur-logs-private".to_string()),
        })
    }
}

/// Cache entry with TTL
struct CacheEntry {
    data: String,
    expires_at: Instant,
}

/// GitHub Storage Client
pub struct GitHubStorage {
    client: Client,
    config: GitHubStorageConfig,
    cache: RwLock<HashMap<String, CacheEntry>>,
}

impl GitHubStorage {
    /// Create new storage client
    pub fn new(config: GitHubStorageConfig) -> Self {
        Self {
            client: Client::new(),
            config,
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Create from environment
    pub fn from_env() -> Option<Self> {
        GitHubStorageConfig::from_env().map(Self::new)
    }

    /// Hash user ID for privacy (sha256 of email)
    pub fn hash_user_id(user_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(user_id.as_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string() // First 16 chars
    }

    /// Get full path for user storage
    fn user_path(&self, user_hash: &str, key: &str) -> String {
        format!("users/{}/{}", user_hash, key)
    }

    /// Save data to GitHub
    pub async fn save(&self, path: &str, content: &str, message: &str) -> Result<(), String> {
        // First check if file exists to get SHA
        let existing_sha = self.get_file_sha(path).await.ok();

        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.config.owner, self.config.repo, path
        );

        let encoded = BASE64.encode(content.as_bytes());

        let mut body = serde_json::json!({
            "message": message,
            "content": encoded,
            "branch": "main"
        });

        // If file exists, include SHA for update
        if let Some(sha) = existing_sha {
            body["sha"] = serde_json::Value::String(sha);
        }

        let resp = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "axur-backend")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if resp.status().is_success() {
            // Invalidate cache
            if let Ok(mut cache) = self.cache.write() {
                cache.remove(path);
            }
            Ok(())
        } else {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            Err(format!("GitHub save failed ({}): {}", status, text))
        }
    }

    /// Load data from GitHub (with cache)
    pub async fn load(&self, path: &str) -> Result<String, String> {
        // Check cache first
        if let Ok(cache) = self.cache.read() {
            if let Some(entry) = cache.get(path) {
                if entry.expires_at > Instant::now() {
                    return Ok(entry.data.clone());
                }
            }
        }

        // Fetch from GitHub
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.config.owner, self.config.repo, path
        );

        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "axur-backend")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if resp.status().is_success() {
            let file: GitHubFile = resp
                .json()
                .await
                .map_err(|e| format!("Parse failed: {}", e))?;

            let content = BASE64
                .decode(file.content.replace('\n', ""))
                .map_err(|e| format!("Base64 decode failed: {}", e))?;

            let data = String::from_utf8(content).map_err(|e| format!("UTF8 failed: {}", e))?;

            // Cache for 1 hour
            if let Ok(mut cache) = self.cache.write() {
                cache.insert(
                    path.to_string(),
                    CacheEntry {
                        data: data.clone(),
                        expires_at: Instant::now() + Duration::from_secs(3600),
                    },
                );
            }

            Ok(data)
        } else if resp.status() == 404 {
            Err("File not found".to_string())
        } else {
            Err(format!("GitHub load failed: {}", resp.status()))
        }
    }

    /// Delete file from GitHub
    pub async fn delete(&self, path: &str, message: &str) -> Result<(), String> {
        let sha = self.get_file_sha(path).await?;

        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.config.owner, self.config.repo, path
        );

        let body = serde_json::json!({
            "message": message,
            "sha": sha,
            "branch": "main"
        });

        let resp = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "axur-backend")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if resp.status().is_success() {
            // Invalidate cache
            if let Ok(mut cache) = self.cache.write() {
                cache.remove(path);
            }
            Ok(())
        } else {
            Err(format!("GitHub delete failed: {}", resp.status()))
        }
    }

    /// List files in a directory
    pub async fn list(&self, path: &str) -> Result<Vec<String>, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.config.owner, self.config.repo, path
        );

        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "axur-backend")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if resp.status().is_success() {
            let items: Vec<GitHubDirEntry> = resp
                .json()
                .await
                .map_err(|e| format!("Parse failed: {}", e))?;

            Ok(items.into_iter().map(|e| e.name).collect())
        } else if resp.status() == 404 {
            Ok(vec![]) // Directory doesn't exist yet
        } else {
            Err(format!("GitHub list failed: {}", resp.status()))
        }
    }

    /// Get file SHA (needed for updates)
    async fn get_file_sha(&self, path: &str) -> Result<String, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.config.owner, self.config.repo, path
        );

        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "axur-backend")
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if resp.status().is_success() {
            let file: GitHubFile = resp
                .json()
                .await
                .map_err(|e| format!("Parse failed: {}", e))?;
            Ok(file.sha)
        } else {
            Err("File not found".to_string())
        }
    }

    // =========== User Template Operations ===========

    /// Save user template
    pub async fn save_template(
        &self,
        user_id: &str,
        template_name: &str,
        template_json: &str,
    ) -> Result<(), String> {
        let user_hash = Self::hash_user_id(user_id);
        let path = self.user_path(&user_hash, &format!("templates/{}.json", template_name));
        self.save(
            &path,
            template_json,
            &format!("Save template: {}", template_name),
        )
        .await
    }

    /// Load user template
    pub async fn load_template(
        &self,
        user_id: &str,
        template_name: &str,
    ) -> Result<String, String> {
        let user_hash = Self::hash_user_id(user_id);
        let path = self.user_path(&user_hash, &format!("templates/{}.json", template_name));
        self.load(&path).await
    }

    /// List user templates
    pub async fn list_templates(&self, user_id: &str) -> Result<Vec<String>, String> {
        let user_hash = Self::hash_user_id(user_id);
        let path = self.user_path(&user_hash, "templates");
        self.list(&path).await
    }

    /// Delete user template
    pub async fn delete_template(&self, user_id: &str, template_name: &str) -> Result<(), String> {
        let user_hash = Self::hash_user_id(user_id);
        let path = self.user_path(&user_hash, &format!("templates/{}.json", template_name));
        self.delete(&path, &format!("Delete template: {}", template_name))
            .await
    }
}

/// GitHub file response
#[derive(Deserialize)]
struct GitHubFile {
    sha: String,
    content: String,
}

/// GitHub directory entry
#[derive(Deserialize)]
struct GitHubDirEntry {
    name: String,
    #[allow(dead_code)]
    path: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    entry_type: String,
}

// =========== Global Storage Instance ===========

use std::sync::OnceLock;
static GITHUB_STORAGE: OnceLock<Option<GitHubStorage>> = OnceLock::new();

/// Get global GitHub storage instance
pub fn get_github_storage() -> Option<&'static GitHubStorage> {
    GITHUB_STORAGE
        .get_or_init(|| GitHubStorage::from_env())
        .as_ref()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_user_id() {
        let hash = GitHubStorage::hash_user_id("test@example.com");
        assert_eq!(hash.len(), 16);
        // Same input should produce same hash
        assert_eq!(hash, GitHubStorage::hash_user_id("test@example.com"));
    }
}
