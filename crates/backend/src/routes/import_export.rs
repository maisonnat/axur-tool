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

/// Upload PPTX to Google Drive, generate thumbnails via Slides API, and return data URLs.
/// Uses strict Rate Limiting (4 req/sec) to stay within Free Tier.
/// Images are downloaded to base64 data URLs to avoid 429 errors on frontend.
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

    // 2. Generate Previews (Rate Limited) - get Google content URLs
    tracing::info!("Generating previews for file ID: {}", file_id);
    let google_urls = services
        .generate_previews(&file_id)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    // 3. Download images as base64 to avoid 429 on frontend
    // Uses Retry-After header handling for proper rate limiting
    tracing::info!("Downloading {} images as base64...", google_urls.len());
    let data_urls = services
        .fetch_images_as_base64(google_urls)
        .await
        .map_err(|e| {
            // Attempt cleanup on error
            let services_clone = services.clone();
            let file_id_clone = file_id.clone();
            tokio::spawn(async move {
                let _ = services_clone.delete_file(&file_id_clone).await;
            });
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e)
        })?;

    // 4. Cleanup
    tracing::info!("Cleaning up file from Drive: {}", file_id);
    let _ = services.delete_file(&file_id).await;

    Ok(Json(ImportResponse {
        success: true,
        slides: data_urls, // Now returns base64 data URLs instead of Google URLs
        message: "Successfully generated previews with embedded images".to_string(),
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
                axum::http::HeaderValue::from_str(&filename_header).unwrap_or_else(|_| {
                    axum::http::HeaderValue::from_static(
                        "attachment; filename=\"presentation.pptx\"",
                    )
                }),
            );

            Ok(res)
        }
        Err(e) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Injection failed: {}", e),
        )),
    }
}

// =================================================================
// GOOGLE SLIDES EXPORT
// =================================================================

use crate::google_services::SlideData;
use serde::Deserialize;

/// Request to export slides to Google Slides
#[derive(Deserialize)]
pub struct ExportSlidesRequest {
    /// Title for the new presentation
    pub title: String,
    /// Array of slide data (title + body content)
    pub slides: Vec<ExportSlideData>,
}

#[derive(Deserialize)]
pub struct ExportSlideData {
    pub title: String,
    pub body: Vec<String>,
    pub layout: Option<String>,
}

/// Response from export_to_slides
#[derive(Serialize)]
pub struct ExportSlidesResponse {
    pub success: bool,
    pub presentation_id: String,
    pub presentation_url: String,
    pub slides_count: usize,
    pub message: String,
}

/// Export slides to Google Slides presentation
///
/// Creates a new Google Slides presentation and populates it with the provided slides.
/// Uses rate limiting (1s delay between API calls) to stay within free tier.
///
/// POST /api/export/slides
pub async fn export_to_slides(
    State(state): State<AppState>,
    Json(request): Json<ExportSlidesRequest>,
) -> Result<Json<ExportSlidesResponse>, (axum::http::StatusCode, String)> {
    let services = state.google_services.ok_or((
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        "Google Services not configured (missing credentials)".to_string(),
    ))?;

    tracing::info!(
        "Creating Google Slides presentation: {} ({} slides)",
        request.title,
        request.slides.len()
    );

    // 1. Create new presentation
    let presentation_id = services
        .create_presentation(&request.title)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    tracing::info!("Created presentation: {}", presentation_id);

    // 2. Convert to SlideData format
    let slide_data: Vec<SlideData> = request
        .slides
        .into_iter()
        .map(|s| SlideData {
            title: s.title,
            body: s.body,
            layout: s.layout,
        })
        .collect();

    // 3. Add slides to presentation
    if !slide_data.is_empty() {
        services
            .add_slides_batch(&presentation_id, &slide_data)
            .await
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;
    }

    // 4. Get presentation URL
    let presentation_url = services.get_presentation_url(&presentation_id);
    let slides_count = slide_data.len();

    tracing::info!("Export complete: {}", presentation_url);

    Ok(Json(ExportSlidesResponse {
        success: true,
        presentation_id,
        presentation_url,
        slides_count,
        message: format!(
            "Successfully exported {} slides to Google Slides",
            slides_count
        ),
    }))
}

// =================================================================
// PPTX GENERATION WITH REAL DATA
// =================================================================

use std::collections::HashMap;

/// Request to generate PPTX with real report data
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct GeneratePptxRequest {
    /// Placeholder edits from the template (with placeholder_key)
    pub edits: Vec<SlideEdit>,
    /// Pre-mapped placeholder values (key -> value)
    pub placeholder_values: HashMap<String, String>,
}

/// Response for PPTX generation
#[derive(Serialize)]
pub struct GeneratePptxResponse {
    pub success: bool,
    pub message: String,
    /// Base64 encoded PPTX file
    pub pptx_base64: Option<String>,
}

/// Generate PPTX with real data from report
///
/// Takes: PPTX file + template edits + placeholder_values (already mapped)
/// Returns: Modified PPTX with placeholder values replaced by real data
pub async fn generate_pptx_report(
    mut multipart: Multipart,
) -> Result<impl axum::response::IntoResponse, (axum::http::StatusCode, String)> {
    let mut file_data = None;
    let mut file_name = String::from("report.pptx");
    let mut edits: Vec<SlideEdit> = Vec::new();
    let mut placeholder_values: HashMap<String, String> = HashMap::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            file_name = field.file_name().unwrap_or("report.pptx").to_string();
            if let Ok(bytes) = field.bytes().await {
                file_data = Some(bytes.to_vec());
            }
        } else if name == "edits" {
            if let Ok(text) = field.text().await {
                if let Ok(parsed_edits) = serde_json::from_str::<Vec<SlideEdit>>(&text) {
                    edits = parsed_edits;
                }
            }
        } else if name == "placeholder_values" {
            if let Ok(text) = field.text().await {
                if let Ok(parsed_values) = serde_json::from_str::<HashMap<String, String>>(&text) {
                    placeholder_values = parsed_values;
                }
            }
        }
    }

    let Some(data) = file_data else {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "No PPTX file uploaded".to_string(),
        ));
    };

    if placeholder_values.is_empty() {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "No placeholder_values provided".to_string(),
        ));
    }

    // Replace placeholder text with real values
    let resolved_edits: Vec<SlideEdit> = edits
        .into_iter()
        .map(|mut edit| {
            if let Some(key) = &edit.placeholder_key {
                if let Some(real_value) = placeholder_values.get(key) {
                    edit.text = real_value.clone();
                }
            }
            edit
        })
        .collect();

    tracing::info!(
        "Generating PPTX report: {} edits mapped from {} placeholder values",
        resolved_edits.len(),
        placeholder_values.len()
    );

    match inject_edits(&data, resolved_edits) {
        Ok(modified_bytes) => {
            // Return as base64 for easier handling
            let base64_pptx =
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &modified_bytes);

            Ok(Json(GeneratePptxResponse {
                success: true,
                message: format!("Generated PPTX report: {}", file_name),
                pptx_base64: Some(base64_pptx),
            }))
        }
        Err(e) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("PPTX generation failed: {}", e),
        )),
    }
}
