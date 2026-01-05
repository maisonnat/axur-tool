//! Import/Export routes for templates
//! Handles PPTX import and export functionality

use crate::injector::{inject_edits, SlideEdit};
use crate::routes::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::Multipart;
use serde::Serialize;
use tracing;
use uuid;

#[derive(Serialize)]
pub struct ImportResponse {
    pub success: bool,
    pub slides: Vec<String>,
    pub message: String,
}

/// Upload PPTX to Google Drive, generate thumbnails via Slides API, and return URLs.
/// Uses strict Rate Limiting (4 req/sec) to stay within Free Tier.
pub async fn import_pptx(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<ImportResponse>, (axum::http::StatusCode, String)> {
    let mut file_data = None;
    let mut file_name = String::from("presentation.pptx");

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            file_name = field.file_name().unwrap_or("presentation.pptx").to_string();
            if let Ok(bytes) = field.bytes().await {
                file_data = Some(bytes.to_vec());
            }
        }
    }

    let Some(data) = file_data else {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "No file uploaded".to_string(),
        ));
    };

    let services = state.google_services.ok_or((
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        "Google Services not configured (missing credentials)".to_string(),
    ))?;

    let uuid = uuid::Uuid::new_v4();
    let temp_name = format!("preview_{}_{}", uuid, file_name);

    // 1. Upload to Drive
    tracing::info!("Uploading to Google Drive: {}", temp_name);
    let file_id = services
        .upload_pptx(&temp_name, data)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // 2. Generate Previews (Rate Limited)
    tracing::info!("Generating previews for file ID: {}", file_id);
    let urls = services.generate_previews(&file_id).await.map_err(|e| {
        // Attempt cleanup even on error
        // tokio::spawn(async move { let _ = services.delete_file(&file_id).await; }); // Move unavailable
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e)
    })?;

    // 3. Cleanup
    tracing::info!("Cleaning up file from Drive: {}", file_id);
    let _ = services.delete_file(&file_id).await;

    Ok(Json(ImportResponse {
        success: true,
        slides: urls,
        message: "Successfully generated previews via Google Slides API".to_string(),
    }))
}

/// Inject placeholders into PPTX and download
pub async fn inject_pptx(
    mut multipart: Multipart,
) -> Result<impl axum::response::IntoResponse, (axum::http::StatusCode, String)> {
    let mut file_data = None;
    let mut file_name = String::from("presentation.pptx");
    let mut edits = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            file_name = field.file_name().unwrap_or("presentation.pptx").to_string();
            if let Ok(bytes) = field.bytes().await {
                file_data = Some(bytes.to_vec());
            }
        } else if name == "edits" {
            if let Ok(text) = field.text().await {
                if let Ok(parsed_edits) = serde_json::from_str::<Vec<SlideEdit>>(&text) {
                    edits = parsed_edits;
                } else {
                    return Err((
                        axum::http::StatusCode::BAD_REQUEST,
                        "Invalid JSON in 'edits' field".to_string(),
                    ));
                }
            }
        }
    }

    let Some(data) = file_data else {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "No file uploaded".to_string(),
        ));
    };

    match inject_edits(&data, edits) {
        Ok(modified_bytes) => {
            let filename_header = format!("attachment; filename=\"injected_{}\"", file_name);

            // We need to return the header string OWNED or use a builder
            let mut res = axum::response::Response::new(axum::body::Body::from(modified_bytes));
            res.headers_mut().insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static(
                    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
                ),
            );
            res.headers_mut().insert(
                axum::http::header::CONTENT_DISPOSITION,
                axum::http::HeaderValue::from_str(&filename_header).unwrap(),
            );

            Ok(res)
        }
        Err(e) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Injection failed: {}", e),
        )),
    }
}
