//! Request Queue with Token Bucket Rate Limiting
//!
//! Manages API request rates to prevent 429 errors and provides
//! real-time queue position updates to users.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Token Bucket Rate Limiter
///
/// Implements the token bucket algorithm for precise rate limiting.
/// Tokens refill at a constant rate up to capacity.
pub struct TokenBucket {
    /// Maximum tokens in the bucket
    capacity: u32,
    /// Current available tokens
    tokens: AtomicU32,
    /// Tokens added per refill
    refill_rate: u32,
    /// Time between refills
    refill_interval: Duration,
    /// Last refill timestamp (ms since epoch)
    last_refill: AtomicU64,
}

impl TokenBucket {
    /// Create a new token bucket
    pub fn new(capacity: u32, refill_rate: u32, refill_interval: Duration) -> Self {
        Self {
            capacity,
            tokens: AtomicU32::new(capacity),
            refill_rate,
            refill_interval,
            last_refill: AtomicU64::new(Self::now_ms()),
        }
    }

    fn now_ms() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis() as u64
    }

    /// Refill tokens based on elapsed time
    fn refill(&self) {
        let now = Self::now_ms();
        let last = self.last_refill.load(Ordering::Relaxed);
        let interval_ms = self.refill_interval.as_millis() as u64;

        if now >= last + interval_ms {
            let intervals = ((now - last) / interval_ms) as u32;
            let tokens_to_add = intervals * self.refill_rate;

            let current = self.tokens.load(Ordering::Relaxed);
            let new_tokens = (current + tokens_to_add).min(self.capacity);
            self.tokens.store(new_tokens, Ordering::Relaxed);

            // Update last refill time
            let new_last = last + (intervals as u64 * interval_ms);
            self.last_refill.store(new_last, Ordering::Relaxed);
        }
    }

    /// Try to acquire a token. Returns true if successful.
    pub fn try_acquire(&self) -> bool {
        self.refill();

        loop {
            let current = self.tokens.load(Ordering::Relaxed);
            if current == 0 {
                return false;
            }

            if self
                .tokens
                .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::Relaxed)
                .is_ok()
            {
                return true;
            }
        }
    }

    /// Get time to wait for next token (if bucket is empty)
    pub fn time_until_available(&self) -> Duration {
        self.refill();

        if self.tokens.load(Ordering::Relaxed) > 0 {
            return Duration::ZERO;
        }

        let now = Self::now_ms();
        let last = self.last_refill.load(Ordering::Relaxed);
        let interval_ms = self.refill_interval.as_millis() as u64;
        let next_refill = last + interval_ms;

        if now >= next_refill {
            Duration::ZERO
        } else {
            Duration::from_millis(next_refill - now)
        }
    }

    /// Current available tokens
    pub fn available(&self) -> u32 {
        self.refill();
        self.tokens.load(Ordering::Relaxed)
    }
}

/// API type for rate limiting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiType {
    Axur,
    GitHub,
    Leapcell,
}

impl ApiType {
    /// Get rate limit configuration for each API
    pub fn rate_limit(&self) -> (u32, u32, Duration) {
        // (capacity, refill_rate, refill_interval)
        match self {
            ApiType::Axur => (60, 60, Duration::from_secs(60)), // 60/min
            ApiType::GitHub => (5000, 5000, Duration::from_secs(3600)), // 5000/hour
            ApiType::Leapcell => (100, 100, Duration::from_secs(60)), // 100/min
        }
    }
}

/// Job type for the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobType {
    GenerateReport { tenant_id: String },
    SaveTemplate { template_name: String },
    LoadTemplate { template_id: String },
    ThreatHuntingSearch { query: String },
}

/// Job status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum JobStatus {
    Queued { position: usize },
    Processing { started_at: u64 },
    Completed { result: serde_json::Value },
    Failed { error: String },
}

/// A job in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueJob {
    pub id: String,
    pub user_id: String,
    pub job_type: JobType,
    pub api_type: ApiType,
    pub created_at: u64,
    pub status: JobStatus,
    pub priority: u8, // Higher = more priority
}

impl QueueJob {
    pub fn new(user_id: String, job_type: JobType, api_type: ApiType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            job_type,
            api_type,
            created_at: TokenBucket::now_ms(),
            status: JobStatus::Queued { position: 0 },
            priority: 0,
        }
    }
}

/// Request Queue Manager
pub struct RequestQueue {
    jobs: RwLock<VecDeque<QueueJob>>,
    rate_limiters: HashMap<ApiType, TokenBucket>,
    completed_jobs: RwLock<HashMap<String, QueueJob>>,
}

impl RequestQueue {
    /// Create a new request queue with rate limiters for each API
    pub fn new() -> Self {
        let mut rate_limiters = HashMap::new();

        for api_type in [ApiType::Axur, ApiType::GitHub, ApiType::Leapcell] {
            let (capacity, refill_rate, interval) = api_type.rate_limit();
            rate_limiters.insert(api_type, TokenBucket::new(capacity, refill_rate, interval));
        }

        Self {
            jobs: RwLock::new(VecDeque::new()),
            rate_limiters,
            completed_jobs: RwLock::new(HashMap::new()),
        }
    }

    /// Submit a job to the queue
    pub async fn submit(&self, mut job: QueueJob) -> String {
        let mut jobs = self.jobs.write().await;
        let position = jobs.len();
        job.status = JobStatus::Queued { position };
        let job_id = job.id.clone();
        jobs.push_back(job);
        job_id
    }

    /// Get job status by ID
    pub async fn get_job(&self, job_id: &str) -> Option<QueueJob> {
        // Check active queue first
        {
            let jobs = self.jobs.read().await;
            for (idx, job) in jobs.iter().enumerate() {
                if job.id == job_id {
                    let mut job = job.clone();
                    job.status = JobStatus::Queued { position: idx };
                    return Some(job);
                }
            }
        }

        // Check completed jobs
        {
            let completed = self.completed_jobs.read().await;
            if let Some(job) = completed.get(job_id) {
                return Some(job.clone());
            }
        }

        None
    }

    /// Get queue length
    pub async fn queue_length(&self) -> usize {
        self.jobs.read().await.len()
    }

    /// Get estimated wait time for a position
    pub fn estimate_wait_time(&self, position: usize, api_type: ApiType) -> Duration {
        if let Some(bucket) = self.rate_limiters.get(&api_type) {
            let available = bucket.available() as usize;
            if position < available {
                return Duration::ZERO;
            }

            let (_, refill_rate, interval) = api_type.rate_limit();
            let jobs_to_wait = position.saturating_sub(available);
            let intervals_needed = (jobs_to_wait as f64 / refill_rate as f64).ceil() as u64;

            interval * intervals_needed as u32
        } else {
            Duration::from_secs(position as u64 * 2) // Fallback: ~2s per job
        }
    }

    /// Try to acquire a rate limit token for an API
    pub fn can_process(&self, api_type: ApiType) -> bool {
        self.rate_limiters
            .get(&api_type)
            .map(|b| b.try_acquire())
            .unwrap_or(true)
    }

    /// Get next job that can be processed
    pub async fn pop_ready(&self) -> Option<QueueJob> {
        let mut jobs = self.jobs.write().await;

        // Find first job whose rate limit allows processing
        for i in 0..jobs.len() {
            if self
                .rate_limiters
                .get(&jobs[i].api_type)
                .map(|b| b.try_acquire())
                .unwrap_or(true)
            {
                return jobs.remove(i);
            }
        }

        None
    }

    /// Mark a job as completed
    pub async fn complete(&self, mut job: QueueJob, result: serde_json::Value) {
        job.status = JobStatus::Completed { result };
        self.completed_jobs
            .write()
            .await
            .insert(job.id.clone(), job);
    }

    /// Mark a job as failed
    pub async fn fail(&self, mut job: QueueJob, error: String) {
        job.status = JobStatus::Failed { error };
        self.completed_jobs
            .write()
            .await
            .insert(job.id.clone(), job);
    }

    /// Get all jobs for a user
    pub async fn user_jobs(&self, user_id: &str) -> Vec<QueueJob> {
        let jobs = self.jobs.read().await;
        let completed = self.completed_jobs.read().await;

        let mut result: Vec<_> = jobs
            .iter()
            .filter(|j| j.user_id == user_id)
            .cloned()
            .collect();

        result.extend(completed.values().filter(|j| j.user_id == user_id).cloned());

        result
    }
}

impl Default for RequestQueue {
    fn default() -> Self {
        Self::new()
    }
}

// Global static queue
use std::sync::OnceLock;
static QUEUE: OnceLock<RequestQueue> = OnceLock::new();

/// Get the global request queue
pub fn get_queue() -> &'static RequestQueue {
    QUEUE.get_or_init(RequestQueue::new)
}

/// Queue status for API response
#[derive(Debug, Clone, Serialize)]
pub struct QueueStatusResponse {
    pub job_id: String,
    pub status: String,
    pub position: Option<usize>,
    pub eta_seconds: Option<u64>,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl From<&QueueJob> for QueueStatusResponse {
    fn from(job: &QueueJob) -> Self {
        match &job.status {
            JobStatus::Queued { position } => Self {
                job_id: job.id.clone(),
                status: "queued".to_string(),
                position: Some(*position),
                eta_seconds: None, // Will be filled by route
                result: None,
                error: None,
            },
            JobStatus::Processing { .. } => Self {
                job_id: job.id.clone(),
                status: "processing".to_string(),
                position: None,
                eta_seconds: None,
                result: None,
                error: None,
            },
            JobStatus::Completed { result } => Self {
                job_id: job.id.clone(),
                status: "completed".to_string(),
                position: None,
                eta_seconds: None,
                result: Some(result.clone()),
                error: None,
            },
            JobStatus::Failed { error } => Self {
                job_id: job.id.clone(),
                status: "failed".to_string(),
                position: None,
                eta_seconds: None,
                result: None,
                error: Some(error.clone()),
            },
        }
    }
}

// =========== Background Worker ===========

use std::sync::atomic::AtomicBool;

static WORKER_RUNNING: AtomicBool = AtomicBool::new(false);

/// Start the background worker for processing queued jobs.
/// Should be called once at server startup.
pub fn start_worker() {
    if WORKER_RUNNING.swap(true, Ordering::SeqCst) {
        tracing::warn!("[Queue Worker] Already running, skipping start");
        return;
    }

    tokio::spawn(async move {
        tracing::info!("[Queue Worker] Started background job processor");

        loop {
            // Check queue every 500ms
            tokio::time::sleep(Duration::from_millis(500)).await;

            let queue = get_queue();

            // Try to pop a ready job
            if let Some(mut job) = queue.pop_ready().await {
                tracing::info!(
                    "[Queue Worker] Processing job {} ({:?})",
                    job.id,
                    job.job_type
                );

                // Mark as processing
                job.status = JobStatus::Processing {
                    started_at: TokenBucket::now_ms(),
                };

                // Process based on job type
                let result = process_job(&job).await;

                // Update job status
                match result {
                    Ok(value) => {
                        job.status = JobStatus::Completed { result: value };
                        tracing::info!("[Queue Worker] Completed job {}", job.id);
                    }
                    Err(e) => {
                        job.status = JobStatus::Failed { error: e };
                        tracing::error!("[Queue Worker] Failed job {}", job.id);
                    }
                }

                // Store in completed jobs
                queue.store_completed(job).await;
            }
        }
    });
}

/// Process a job based on its type
async fn process_job(job: &QueueJob) -> Result<serde_json::Value, String> {
    match &job.job_type {
        JobType::GenerateReport { tenant_id } => {
            // Placeholder - actual report generation would happen here
            // For now, return success with tenant info
            tokio::time::sleep(Duration::from_secs(1)).await; // Simulate work
            Ok(serde_json::json!({
                "status": "report_generation_queued",
                "tenant_id": tenant_id,
                "message": "Use /api/reports/generate directly with the cookie for full report"
            }))
        }
        JobType::SaveTemplate { template_name } => {
            // Template saving via GitHub storage
            tokio::time::sleep(Duration::from_millis(500)).await;
            Ok(serde_json::json!({
                "status": "template_saved",
                "name": template_name
            }))
        }
        JobType::LoadTemplate { template_id } => {
            // Template loading
            tokio::time::sleep(Duration::from_millis(200)).await;
            Ok(serde_json::json!({
                "status": "template_loaded",
                "id": template_id
            }))
        }
        JobType::ThreatHuntingSearch { query } => {
            // TH search placeholder
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok(serde_json::json!({
                "status": "search_complete",
                "query": query,
                "results_count": 0
            }))
        }
    }
}

/// Store a completed job for status lookup
impl RequestQueue {
    pub async fn store_completed(&self, job: QueueJob) {
        let mut completed = self.completed_jobs.write().await;
        completed.insert(job.id.clone(), job);

        // Keep only last 100 completed jobs
        if completed.len() > 100 {
            let oldest: Vec<String> = completed
                .iter()
                .take(completed.len() - 100)
                .map(|(k, _)| k.clone())
                .collect();
            for key in oldest {
                completed.remove(&key);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket_acquire() {
        let bucket = TokenBucket::new(5, 1, Duration::from_secs(1));

        // Should be able to acquire 5 tokens
        for _ in 0..5 {
            assert!(bucket.try_acquire());
        }

        // 6th should fail
        assert!(!bucket.try_acquire());
    }

    #[test]
    fn test_token_bucket_available() {
        let bucket = TokenBucket::new(10, 5, Duration::from_secs(1));
        assert_eq!(bucket.available(), 10);

        bucket.try_acquire();
        bucket.try_acquire();
        assert_eq!(bucket.available(), 8);
    }

    #[tokio::test]
    async fn test_queue_submit() {
        let queue = RequestQueue::new();

        let job = QueueJob::new(
            "user-1".to_string(),
            JobType::GenerateReport {
                tenant_id: "t1".to_string(),
            },
            ApiType::Axur,
        );

        let job_id = queue.submit(job).await;
        assert!(!job_id.is_empty());
        assert_eq!(queue.queue_length().await, 1);
    }
}
