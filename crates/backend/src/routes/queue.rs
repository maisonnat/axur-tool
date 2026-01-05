//! Queue Routes - API endpoints for queue status and job submission

use axum::{
    extract::Path,
    response::{
        sse::{Event, Sse},
        IntoResponse,
    },
    routing::{get, post},
    Json, Router,
};
use futures::stream::{self, Stream};
use serde::Deserialize;
use std::{convert::Infallible, time::Duration};

use crate::queue::{get_queue, ApiType, JobStatus, JobType, QueueJob, QueueStatusResponse};

/// Create queue routes (no state needed - uses global queue)
pub fn queue_routes() -> Router {
    Router::new()
        .route("/submit", post(submit_job))
        .route("/status/:job_id", get(get_job_status))
        .route("/stream/:job_id", get(stream_job_status))
        .route("/length", get(get_queue_length))
}

/// Request body for job submission
#[derive(Debug, Deserialize)]
pub struct SubmitJobRequest {
    pub user_id: String,
    pub job_type: String,
    pub params: serde_json::Value,
}

/// Submit a new job to the queue
async fn submit_job(Json(req): Json<SubmitJobRequest>) -> impl IntoResponse {
    let queue = get_queue();

    // Parse job type
    let (job_type, api_type) = match req.job_type.as_str() {
        "generate_report" => {
            let tenant_id = req
                .params
                .get("tenant_id")
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string();
            (JobType::GenerateReport { tenant_id }, ApiType::Axur)
        }
        "save_template" => {
            let template_name = req
                .params
                .get("template_name")
                .and_then(|v| v.as_str())
                .unwrap_or("unnamed")
                .to_string();
            (JobType::SaveTemplate { template_name }, ApiType::GitHub)
        }
        "load_template" => {
            let template_id = req
                .params
                .get("template_id")
                .and_then(|v| v.as_str())
                .unwrap_or("default")
                .to_string();
            (JobType::LoadTemplate { template_id }, ApiType::GitHub)
        }
        _ => {
            return Json(serde_json::json!({
                "error": "Unknown job type"
            }));
        }
    };

    let job = QueueJob::new(req.user_id, job_type, api_type);
    let job_id = queue.submit(job).await;
    let position = queue.queue_length().await;
    let eta = queue.estimate_wait_time(position, api_type);

    Json(serde_json::json!({
        "job_id": job_id,
        "position": position,
        "eta_seconds": eta.as_secs()
    }))
}

/// Get current job status
async fn get_job_status(Path(job_id): Path<String>) -> impl IntoResponse {
    let queue = get_queue();

    match queue.get_job(&job_id).await {
        Some(job) => {
            let mut response = QueueStatusResponse::from(&job);

            // Add ETA if queued
            if let Some(position) = response.position {
                let eta = queue.estimate_wait_time(position, job.api_type);
                response.eta_seconds = Some(eta.as_secs());
            }

            Json(serde_json::json!(response))
        }
        None => Json(serde_json::json!({
            "error": "Job not found"
        })),
    }
}

/// Get queue length
async fn get_queue_length() -> impl IntoResponse {
    let queue = get_queue();
    let length = queue.queue_length().await;
    Json(serde_json::json!({
        "length": length
    }))
}

/// Server-Sent Events stream for job status
async fn stream_job_status(
    Path(job_id): Path<String>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::unfold((job_id, 0u32), |(job_id, tick)| async move {
        // Poll every 2 seconds, max 300 ticks (10 minutes)
        if tick > 300 {
            return None;
        }

        tokio::time::sleep(Duration::from_secs(2)).await;

        let queue = get_queue();
        let event = match queue.get_job(&job_id).await {
            Some(job) => {
                let response = QueueStatusResponse::from(&job);
                let data = serde_json::to_string(&response).unwrap_or_default();

                // If completed or failed, this will be the last event
                let is_final = matches!(
                    &job.status,
                    JobStatus::Completed { .. } | JobStatus::Failed { .. }
                );

                if is_final {
                    return Some((
                        Ok(Event::default().data(data).event("complete")),
                        (job_id, 301), // Stop after this
                    ));
                }

                Event::default().data(data).event("update")
            }
            None => Event::default()
                .data(r#"{"error":"Job not found"}"#)
                .event("error"),
        };

        Some((Ok(event), (job_id, tick + 1)))
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}
