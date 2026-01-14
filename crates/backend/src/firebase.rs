//! Firebase Firestore Client (REST API)
//!
//! Lightweight Firestore client using REST API instead of heavy SDK.
//! Includes rate limiting and caching for zero-cost operation.
//!
//! ## Zero-Cost Safeguards
//! - Rate limiting: 2K reads/hour, 800 writes/hour
//! - In-memory cache with 5 minute TTL
//! - Fallback mode when quota exceeded

use once_cell::sync::Lazy;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

// ========================
// CONFIGURATION
// ========================

/// Firebase project configuration
#[derive(Clone)]
pub struct FirebaseConfig {
    pub project_id: String,
    pub api_key: Option<String>,
    /// Service account JSON for server-side auth (base64 encoded)
    pub service_account_json: Option<String>,
}

impl FirebaseConfig {
    pub fn from_env() -> Option<Self> {
        let project_id = std::env::var("FIREBASE_PROJECT_ID").ok()?;
        Some(Self {
            project_id,
            api_key: std::env::var("FIREBASE_API_KEY").ok(),
            service_account_json: std::env::var("FIREBASE_SERVICE_ACCOUNT_B64").ok(),
        })
    }
}

// ========================
// RATE LIMITING
// ========================

/// Rate limiter with hourly quotas
struct RateLimiter {
    reads: u32,
    writes: u32,
    hour_start: Instant,
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            reads: 0,
            writes: 0,
            hour_start: Instant::now(),
        }
    }

    fn reset_if_needed(&mut self) {
        if self.hour_start.elapsed() > Duration::from_secs(3600) {
            self.reads = 0;
            self.writes = 0;
            self.hour_start = Instant::now();
        }
    }

    fn can_read(&mut self) -> bool {
        self.reset_if_needed();
        const MAX_READS_PER_HOUR: u32 = 2000;
        if self.reads < MAX_READS_PER_HOUR {
            self.reads += 1;
            true
        } else {
            tracing::warn!("Firebase rate limit reached: reads");
            false
        }
    }

    fn can_write(&mut self) -> bool {
        self.reset_if_needed();
        const MAX_WRITES_PER_HOUR: u32 = 800;
        if self.writes < MAX_WRITES_PER_HOUR {
            self.writes += 1;
            true
        } else {
            tracing::warn!("Firebase rate limit reached: writes");
            false
        }
    }
}

static RATE_LIMITER: Lazy<Arc<RwLock<RateLimiter>>> =
    Lazy::new(|| Arc::new(RwLock::new(RateLimiter::new())));

// ========================
// CACHING
// ========================

/// Cache entry with TTL
struct CacheEntry {
    data: String,
    expires_at: Instant,
}

/// In-memory cache for Firestore documents
struct Cache {
    entries: HashMap<String, CacheEntry>,
}

impl Cache {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        self.entries.get(key).and_then(|entry| {
            if entry.expires_at > Instant::now() {
                Some(entry.data.clone())
            } else {
                None
            }
        })
    }

    fn set(&mut self, key: String, data: String, ttl_secs: u64) {
        self.entries.insert(
            key,
            CacheEntry {
                data,
                expires_at: Instant::now() + Duration::from_secs(ttl_secs),
            },
        );
    }

    fn invalidate(&mut self, key: &str) {
        self.entries.remove(key);
    }

    fn invalidate_prefix(&mut self, prefix: &str) {
        self.entries.retain(|k, _| !k.starts_with(prefix));
    }
}

static CACHE: Lazy<Arc<RwLock<Cache>>> = Lazy::new(|| Arc::new(RwLock::new(Cache::new())));

/// Cache TTL in seconds (5 minutes)
const CACHE_TTL: u64 = 300;

// ========================
// FIRESTORE CLIENT
// ========================

/// Firestore REST API client
pub struct FirestoreClient {
    config: FirebaseConfig,
    http: reqwest::Client,
    access_token: Option<String>,
}

impl FirestoreClient {
    pub fn new(config: FirebaseConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
            access_token: None,
        }
    }

    /// Initialize from environment
    pub fn from_env() -> Option<Self> {
        FirebaseConfig::from_env().map(Self::new)
    }

    /// Get Firestore base URL
    fn base_url(&self) -> String {
        format!(
            "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents",
            self.config.project_id
        )
    }

    /// Get a single document
    pub async fn get_doc<T: DeserializeOwned>(
        &self,
        collection: &str,
        doc_id: &str,
    ) -> Result<Option<T>, FirestoreError> {
        let cache_key = format!("{}/{}", collection, doc_id);

        // Check cache first
        if let Some(cached) = CACHE.read().unwrap().get(&cache_key) {
            return serde_json::from_str(&cached)
                .map(Some)
                .map_err(|e| FirestoreError::ParseError(format!("Cache parse error: {}", e)));
        }

        // Rate limit check
        if !RATE_LIMITER.write().unwrap().can_read() {
            return Err(FirestoreError::RateLimited);
        }

        let url = format!("{}/{}/{}", self.base_url(), collection, doc_id);
        let res = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| FirestoreError::NetworkError(e.to_string()))?;

        if res.status() == 404 {
            return Ok(None);
        }

        if !res.status().is_success() {
            return Err(FirestoreError::ApiError(format!(
                "Status: {}",
                res.status()
            )));
        }

        let doc: FirestoreDocument = res
            .json()
            .await
            .map_err(|e| FirestoreError::ParseError(e.to_string()))?;

        let value = firestore_to_value(&doc.fields)?;
        let json =
            serde_json::to_string(&value).map_err(|e| FirestoreError::ParseError(e.to_string()))?;

        // Cache the result
        CACHE
            .write()
            .unwrap()
            .set(cache_key, json.clone(), CACHE_TTL);

        serde_json::from_str(&json)
            .map(Some)
            .map_err(|e| FirestoreError::ParseError(e.to_string()))
    }

    /// List documents in a collection
    pub async fn list_docs<T: DeserializeOwned>(
        &self,
        collection: &str,
    ) -> Result<Vec<T>, FirestoreError> {
        let cache_key = format!("list:{}", collection);

        // Check cache
        if let Some(cached) = CACHE.read().unwrap().get(&cache_key) {
            return serde_json::from_str(&cached)
                .map_err(|e| FirestoreError::ParseError(format!("Cache parse error: {}", e)));
        }

        // Rate limit
        if !RATE_LIMITER.write().unwrap().can_read() {
            return Err(FirestoreError::RateLimited);
        }

        let url = format!("{}/{}", self.base_url(), collection);
        let res = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| FirestoreError::NetworkError(e.to_string()))?;

        if !res.status().is_success() {
            return Err(FirestoreError::ApiError(format!(
                "Status: {}",
                res.status()
            )));
        }

        let list_res: FirestoreListResponse = res
            .json()
            .await
            .map_err(|e| FirestoreError::ParseError(e.to_string()))?;

        // Convert to serde_json::Value array for caching
        let values: Vec<serde_json::Value> = list_res
            .documents
            .unwrap_or_default()
            .into_iter()
            .filter_map(|doc| firestore_to_value(&doc.fields).ok())
            .collect();

        // Cache the JSON values
        let json = serde_json::to_string(&values)
            .map_err(|e| FirestoreError::ParseError(e.to_string()))?;
        CACHE.write().unwrap().set(cache_key, json, CACHE_TTL);

        // Deserialize to target type
        let docs: Vec<T> = values
            .into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect();

        Ok(docs)
    }

    /// Create or update a document
    pub async fn set_doc<T: Serialize>(
        &self,
        collection: &str,
        doc_id: &str,
        data: &T,
    ) -> Result<(), FirestoreError> {
        // Rate limit
        if !RATE_LIMITER.write().unwrap().can_write() {
            return Err(FirestoreError::RateLimited);
        }

        let url = format!("{}/{}/{}", self.base_url(), collection, doc_id);
        let fields = value_to_firestore(&serde_json::to_value(data).unwrap())?;
        let body = serde_json::json!({ "fields": fields });

        let res = self
            .http
            .patch(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| FirestoreError::NetworkError(e.to_string()))?;

        if !res.status().is_success() {
            return Err(FirestoreError::ApiError(format!(
                "Status: {}",
                res.status()
            )));
        }

        // Invalidate cache
        CACHE
            .write()
            .unwrap()
            .invalidate(&format!("{}/{}", collection, doc_id));
        CACHE
            .write()
            .unwrap()
            .invalidate_prefix(&format!("list:{}", collection));

        Ok(())
    }

    /// Delete a document
    pub async fn delete_doc(&self, collection: &str, doc_id: &str) -> Result<(), FirestoreError> {
        if !RATE_LIMITER.write().unwrap().can_write() {
            return Err(FirestoreError::RateLimited);
        }

        let url = format!("{}/{}/{}", self.base_url(), collection, doc_id);
        let res = self
            .http
            .delete(&url)
            .send()
            .await
            .map_err(|e| FirestoreError::NetworkError(e.to_string()))?;

        if !res.status().is_success() && res.status() != 404 {
            return Err(FirestoreError::ApiError(format!(
                "Status: {}",
                res.status()
            )));
        }

        CACHE
            .write()
            .unwrap()
            .invalidate(&format!("{}/{}", collection, doc_id));
        CACHE
            .write()
            .unwrap()
            .invalidate_prefix(&format!("list:{}", collection));

        Ok(())
    }
}

// ========================
// FIRESTORE TYPES
// ========================

#[derive(Debug, Deserialize)]
struct FirestoreDocument {
    #[allow(dead_code)]
    name: Option<String>,
    fields: HashMap<String, FirestoreValue>,
}

#[derive(Debug, Deserialize)]
struct FirestoreListResponse {
    documents: Option<Vec<FirestoreDocument>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum FirestoreValue {
    StringValue(String),
    IntegerValue(String),
    DoubleValue(f64),
    BooleanValue(bool),
    NullValue(()),
    MapValue {
        fields: HashMap<String, FirestoreValue>,
    },
    ArrayValue {
        values: Vec<FirestoreValue>,
    },
    TimestampValue(String),
}

/// Convert Firestore fields to serde_json::Value
fn firestore_to_value(
    fields: &HashMap<String, FirestoreValue>,
) -> Result<serde_json::Value, FirestoreError> {
    let mut map = serde_json::Map::new();
    for (key, val) in fields {
        map.insert(key.clone(), firestore_value_to_json(val)?);
    }
    Ok(serde_json::Value::Object(map))
}

fn firestore_value_to_json(val: &FirestoreValue) -> Result<serde_json::Value, FirestoreError> {
    match val {
        FirestoreValue::StringValue(s) => Ok(serde_json::Value::String(s.clone())),
        FirestoreValue::IntegerValue(s) => {
            let n: i64 = s.parse().unwrap_or(0);
            Ok(serde_json::Value::Number(n.into()))
        }
        FirestoreValue::DoubleValue(d) => Ok(serde_json::json!(*d)),
        FirestoreValue::BooleanValue(b) => Ok(serde_json::Value::Bool(*b)),
        FirestoreValue::NullValue(_) => Ok(serde_json::Value::Null),
        FirestoreValue::MapValue { fields } => firestore_to_value(fields),
        FirestoreValue::ArrayValue { values } => {
            let arr: Result<Vec<_>, _> = values.iter().map(firestore_value_to_json).collect();
            Ok(serde_json::Value::Array(arr?))
        }
        FirestoreValue::TimestampValue(s) => Ok(serde_json::Value::String(s.clone())),
    }
}

/// Convert serde_json::Value to Firestore fields
fn value_to_firestore(
    val: &serde_json::Value,
) -> Result<HashMap<String, FirestoreValue>, FirestoreError> {
    match val {
        serde_json::Value::Object(map) => {
            let mut fields = HashMap::new();
            for (k, v) in map {
                fields.insert(k.clone(), json_to_firestore_value(v)?);
            }
            Ok(fields)
        }
        _ => Err(FirestoreError::ParseError("Expected object".to_string())),
    }
}

fn json_to_firestore_value(val: &serde_json::Value) -> Result<FirestoreValue, FirestoreError> {
    match val {
        serde_json::Value::Null => Ok(FirestoreValue::NullValue(())),
        serde_json::Value::Bool(b) => Ok(FirestoreValue::BooleanValue(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(FirestoreValue::IntegerValue(i.to_string()))
            } else if let Some(f) = n.as_f64() {
                Ok(FirestoreValue::DoubleValue(f))
            } else {
                Ok(FirestoreValue::IntegerValue("0".to_string()))
            }
        }
        serde_json::Value::String(s) => Ok(FirestoreValue::StringValue(s.clone())),
        serde_json::Value::Array(arr) => {
            let values: Result<Vec<_>, _> = arr.iter().map(json_to_firestore_value).collect();
            Ok(FirestoreValue::ArrayValue { values: values? })
        }
        serde_json::Value::Object(map) => {
            let mut fields = HashMap::new();
            for (k, v) in map {
                fields.insert(k.clone(), json_to_firestore_value(v)?);
            }
            Ok(FirestoreValue::MapValue { fields })
        }
    }
}

// ========================
// ERRORS
// ========================

#[derive(Debug)]
pub enum FirestoreError {
    NetworkError(String),
    ApiError(String),
    ParseError(String),
    RateLimited,
    NotConfigured,
}

impl std::fmt::Display for FirestoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "Network error: {}", e),
            Self::ApiError(e) => write!(f, "API error: {}", e),
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
            Self::RateLimited => write!(f, "Rate limited - quota exceeded"),
            Self::NotConfigured => write!(f, "Firebase not configured"),
        }
    }
}

impl std::error::Error for FirestoreError {}

// ========================
// GLOBAL CLIENT
// ========================

static FIRESTORE_CLIENT: Lazy<Option<FirestoreClient>> = Lazy::new(|| FirestoreClient::from_env());

/// Get the global Firestore client
pub fn get_firestore() -> Option<&'static FirestoreClient> {
    FIRESTORE_CLIENT.as_ref()
}

// ========================
// USAGE STATS
// ========================

/// Get current rate limit usage stats
pub fn get_usage_stats() -> (u32, u32) {
    let limiter = RATE_LIMITER.read().unwrap();
    (limiter.reads, limiter.writes)
}
