//! Google Services for PPTX preview generation using Google Drive and Slides APIs.

use google_drive3::{hyper_rustls, hyper_util, yup_oauth2, DriveHub};
use governor::{Quota, RateLimiter};
use nonzero_ext::*;
use serde::Deserialize;
use std::sync::Arc;

// Type alias for the connector used by google-drive3
type HttpConnector = hyper_util::client::legacy::connect::HttpConnector;

#[derive(Clone)]
pub struct GoogleServices {
    drive: DriveHub<hyper_rustls::HttpsConnector<HttpConnector>>,
    limiter: Arc<
        RateLimiter<
            governor::state::NotKeyed,
            governor::state::InMemoryState,
            governor::clock::DefaultClock,
        >,
    >,
    auth: yup_oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<HttpConnector>>,
    http_client: reqwest::Client,
}

#[derive(Deserialize)]
struct Presentation {
    slides: Option<Vec<Slide>>,
}

#[derive(Deserialize)]
struct Slide {
    #[serde(rename = "objectId")]
    object_id: Option<String>,
}

#[derive(Deserialize)]
struct ThumbnailResponse {
    #[serde(rename = "contentUrl")]
    content_url: Option<String>,
}

impl GoogleServices {
    /// Create from environment variables (for production)
    /// Required env vars: GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, GOOGLE_REFRESH_TOKEN
    pub async fn from_env() -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let client_id =
            std::env::var("GOOGLE_CLIENT_ID").map_err(|_| "GOOGLE_CLIENT_ID not set")?;
        let client_secret =
            std::env::var("GOOGLE_CLIENT_SECRET").map_err(|_| "GOOGLE_CLIENT_SECRET not set")?;
        let refresh_token =
            std::env::var("GOOGLE_REFRESH_TOKEN").map_err(|_| "GOOGLE_REFRESH_TOKEN not set")?;

        // Write to temp file (yup-oauth2 doesn't expose the struct publicly)
        let json_content = serde_json::json!({
            "type": "authorized_user",
            "client_id": client_id,
            "client_secret": client_secret,
            "refresh_token": refresh_token
        });

        let temp_path = "/tmp/google_token.json";
        std::fs::write(temp_path, json_content.to_string())?;

        Self::new("", temp_path).await
    }

    /// Create from token.json file (for local development)
    pub async fn new(
        _client_secret_path: &str,
        token_path: &str,
    ) -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let secret = yup_oauth2::read_authorized_user_secret(token_path).await?;

        // Build HTTPS connector with rustls using google_drive3's bundled types
        let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()?
            .https_only()
            .enable_http1()
            .build();

        // Build hyper client with hyper_util (hyper v1 style)
        let hyper_client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(https_connector);

        // Build authenticator for Authorized User
        let auth: yup_oauth2::authenticator::Authenticator<
            hyper_rustls::HttpsConnector<HttpConnector>,
        > = yup_oauth2::AuthorizedUserAuthenticator::builder(secret)
            .build()
            .await?;

        // Create Drive hub with scopes
        let drive = DriveHub::new(hyper_client, auth.clone());

        // Rate Limiter: 4 req/sec
        let quota = Quota::per_second(nonzero!(4u32));
        let limiter = Arc::new(RateLimiter::direct(quota));

        // Reqwest client for manual Slides API calls
        // Increase timeout for large PPTX uploads (e.g. 32MB takes >30s)
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()?;

        Ok(GoogleServices {
            drive,
            limiter,
            auth,
            http_client,
        })
    }

    /// Uploads a PPTX file to Google Drive and returns the File ID.
    /// Uses direct reqwest calls instead of google-drive3 to avoid library issues.
    pub async fn upload_pptx(&self, name: &str, data: Vec<u8>) -> Result<String, String> {
        // Shared folder constant removed - uploading to root

        // Get access token from authenticator
        let scopes = &["https://www.googleapis.com/auth/drive.file"];
        let token = self
            .auth
            .token(scopes)
            .await
            .map_err(|e| format!("Token Error: {}", e))?;
        let access_token = token.token().ok_or("No token string")?;

        // Step 1: Initiate resumable upload session
        // Note: No 'parents' specified means upload to user's root "My Drive"
        let metadata = serde_json::json!({
            "name": name,
            "mimeType": "application/vnd.google-apps.presentation"
        });

        let init_url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable";

        let init_resp = self
            .http_client
            .post(init_url)
            .bearer_auth(access_token)
            .header("Content-Type", "application/json; charset=UTF-8")
            .json(&metadata)
            .send()
            .await
            .map_err(|e| format!("Init request failed: {}", e))?;

        if !init_resp.status().is_success() {
            let status = init_resp.status();
            let body = init_resp.text().await.unwrap_or_default();
            tracing::error!("Drive Init FAILED - Status: {}, Body: {}", status, body);
            tracing::error!("Request metadata was: {}", metadata);
            return Err(format!(
                "Drive Upload Error ({} {}): {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or(""),
                body
            ));
        }

        // Get the upload URI from the Location header
        let upload_uri = init_resp
            .headers()
            .get("location")
            .and_then(|h| h.to_str().ok())
            .ok_or("No upload location returned")?
            .to_string();

        tracing::info!("Got upload URI, uploading {} bytes", data.len());

        // Step 2: Upload the actual file content
        let upload_resp = self
            .http_client
            .put(&upload_uri)
            .header(
                "Content-Type",
                "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            )
            .header("Content-Length", data.len().to_string())
            .body(data)
            .send()
            .await
            .map_err(|e| format!("Upload request failed: {}", e))?;

        if !upload_resp.status().is_success() {
            let status = upload_resp.status();
            let body = upload_resp.text().await.unwrap_or_default();
            return Err(format!("Drive Upload Error ({}): {}", status, body));
        }

        // Parse response to get file ID
        let file_info: serde_json::Value = upload_resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse upload response: {}", e))?;

        file_info
            .get("id")
            .and_then(|id| id.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "No file ID in response".to_string())
    }

    /// Deletes a file from Google Drive.
    pub async fn delete_file(&self, file_id: &str) -> Result<(), String> {
        self.drive
            .files()
            .delete(file_id)
            .add_scope("https://www.googleapis.com/auth/drive.file")
            .doit()
            .await
            .map_err(|e| format!("Drive Delete Error: {}", e))?;
        Ok(())
    }

    /// Orchestrates the preview generation: Get Slides -> Loop Thumbnails -> Return URLs
    pub async fn generate_previews(&self, file_id: &str) -> Result<Vec<String>, String> {
        // 1. Get Access Token for Slides API
        let scopes = &["https://www.googleapis.com/auth/presentations.readonly"];
        let token = self
            .auth
            .token(scopes)
            .await
            .map_err(|e| format!("Token Error: {}", e))?;
        let access_token = token.token().ok_or("No token string")?;

        // 2. Get Presentation Structure (Manual API Call via reqwest)
        let url = format!("https://slides.googleapis.com/v1/presentations/{}", file_id);
        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| format!("Slides API Request Error: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Slides API Error ({}): {}", status, body));
        }

        let presentation: Presentation = resp
            .json()
            .await
            .map_err(|e| format!("Slides JSON Parse Error: {}", e))?;

        let slides = presentation
            .slides
            .ok_or("No slides found in presentation")?;
        let mut urls = Vec::new();

        // 3. Loop and get Thumbnails with Rate Limiting
        for slide in slides {
            let page_id = slide.object_id.ok_or("Slide has no ID")?;

            // Wait for rate limiter
            self.limiter.until_ready().await;

            // Fetch thumbnail
            let thumb_url = format!(
                "https://slides.googleapis.com/v1/presentations/{}/pages/{}/thumbnail",
                file_id, page_id
            );

            let thumb_resp = self
                .http_client
                .get(&thumb_url)
                .bearer_auth(access_token)
                .send()
                .await
                .map_err(|e| format!("Thumbnail Request Error: {}", e))?;

            if !thumb_resp.status().is_success() {
                let status = thumb_resp.status();
                return Err(format!(
                    "Thumbnail API Error for slide {}: {}",
                    page_id, status
                ));
            }

            let thumb_json: ThumbnailResponse = thumb_resp
                .json()
                .await
                .map_err(|e| format!("Thumbnail JSON Parse Error for slide {}: {}", page_id, e))?;

            if let Some(content_url) = thumb_json.content_url {
                urls.push(content_url);
            } else {
                return Err(format!(
                    "Thumbnail response missing contentUrl for slide {}",
                    page_id
                ));
            }
        }

        Ok(urls)
    }

    /// Download images from URLs and convert to base64 data URLs.
    /// Handles 429 rate limiting with Retry-After header support.
    pub async fn fetch_images_as_base64(&self, urls: Vec<String>) -> Result<Vec<String>, String> {
        use base64::Engine;
        let mut data_urls = Vec::with_capacity(urls.len());

        for (idx, url) in urls.iter().enumerate() {
            let mut attempt = 0;
            let max_attempts = 5;

            loop {
                // Rate limit first
                self.limiter.until_ready().await;

                let resp = self
                    .http_client
                    .get(url)
                    .send()
                    .await
                    .map_err(|e| format!("Image fetch error: {}", e))?;

                let status = resp.status();

                if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    // Read Retry-After header (seconds)
                    let retry_after = resp
                        .headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(5); // Default 5 seconds if header missing

                    tracing::warn!(
                        "[GoogleServices] 429 on image {}/{}, Retry-After: {}s (attempt {})",
                        idx + 1,
                        urls.len(),
                        retry_after,
                        attempt + 1
                    );

                    tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                    attempt += 1;

                    if attempt >= max_attempts {
                        return Err(format!(
                            "Image {} failed after {} attempts due to rate limiting",
                            idx + 1,
                            max_attempts
                        ));
                    }
                    continue;
                }

                if !status.is_success() {
                    return Err(format!(
                        "Image {} fetch error: {} {}",
                        idx + 1,
                        status.as_u16(),
                        status.canonical_reason().unwrap_or("")
                    ));
                }

                // Get content type for data URL
                let content_type = resp
                    .headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("image/png")
                    .to_string();

                // Download bytes
                let bytes = resp
                    .bytes()
                    .await
                    .map_err(|e| format!("Image {} bytes error: {}", idx + 1, e))?;

                // Encode to base64
                let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
                let data_url = format!("data:{};base64,{}", content_type, b64);

                data_urls.push(data_url);
                tracing::info!(
                    "[GoogleServices] Image {}/{} downloaded ({} KB)",
                    idx + 1,
                    urls.len(),
                    bytes.len() / 1024
                );
                break;
            }
        }

        Ok(data_urls)
    }

    // =================================================================
    // GOOGLE SLIDES EXPORT METHODS
    // =================================================================

    /// Create a new empty Google Slides presentation
    /// Returns the presentation ID
    pub async fn create_presentation(&self, title: &str) -> Result<String, String> {
        // Rate limit
        self.limiter.until_ready().await;

        // Get access token for Slides write scope
        let scopes = &["https://www.googleapis.com/auth/presentations"];
        let token = self
            .auth
            .token(scopes)
            .await
            .map_err(|e| format!("Token Error: {}", e))?;
        let access_token = token.token().ok_or("No token string")?;

        // Create presentation request
        let body = serde_json::json!({
            "title": title
        });

        let url = "https://slides.googleapis.com/v1/presentations";
        let resp = self
            .http_client
            .post(url)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Create Presentation Request Error: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Slides API Error ({}): {}", status, body));
        }

        let result: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Parse Presentation Response Error: {}", e))?;

        result
            .get("presentationId")
            .and_then(|id| id.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "No presentation ID in response".to_string())
    }

    /// Add slides with content to an existing presentation using batchUpdate
    ///
    /// # Arguments
    /// * `presentation_id` - The Google Slides presentation ID
    /// * `slides` - Vector of SlideData with title, body, and layout
    pub async fn add_slides_batch(
        &self,
        presentation_id: &str,
        slides: &[SlideData],
    ) -> Result<(), String> {
        // Rate limit (1 second delay per API rules)
        self.limiter.until_ready().await;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Get access token
        let scopes = &["https://www.googleapis.com/auth/presentations"];
        let token = self
            .auth
            .token(scopes)
            .await
            .map_err(|e| format!("Token Error: {}", e))?;
        let access_token = token.token().ok_or("No token string")?;

        // Build batchUpdate requests
        let mut requests = Vec::new();

        for (index, slide) in slides.iter().enumerate() {
            let slide_id = format!("slide_{}", index);

            // 1. Create slide with layout
            requests.push(serde_json::json!({
                "createSlide": {
                    "objectId": slide_id,
                    "insertionIndex": index + 1,  // After title slide
                    "slideLayoutReference": {
                        "predefinedLayout": slide.layout.as_deref().unwrap_or("BLANK")
                    }
                }
            }));

            // 2. Add title text box
            if !slide.title.is_empty() {
                let title_id = format!("{}_title", slide_id);
                requests.push(serde_json::json!({
                    "createShape": {
                        "objectId": title_id,
                        "shapeType": "TEXT_BOX",
                        "elementProperties": {
                            "pageObjectId": slide_id,
                            "size": {
                                "width": { "magnitude": 600, "unit": "PT" },
                                "height": { "magnitude": 50, "unit": "PT" }
                            },
                            "transform": {
                                "scaleX": 1.0,
                                "scaleY": 1.0,
                                "translateX": 30.0,
                                "translateY": 20.0,
                                "unit": "PT"
                            }
                        }
                    }
                }));
                requests.push(serde_json::json!({
                    "insertText": {
                        "objectId": title_id,
                        "text": &slide.title
                    }
                }));
                // Style title
                requests.push(serde_json::json!({
                    "updateTextStyle": {
                        "objectId": title_id,
                        "style": {
                            "bold": true,
                            "fontSize": { "magnitude": 28, "unit": "PT" },
                            "foregroundColor": {
                                "opaqueColor": {
                                    "rgbColor": { "red": 1.0, "green": 0.294, "blue": 0.0 }  // #FF4B00
                                }
                            }
                        },
                        "fields": "bold,fontSize,foregroundColor"
                    }
                }));
            }

            // 3. Add body text box
            if !slide.body.is_empty() {
                let body_id = format!("{}_body", slide_id);
                let body_text = slide.body.join("\n\n");
                requests.push(serde_json::json!({
                    "createShape": {
                        "objectId": body_id,
                        "shapeType": "TEXT_BOX",
                        "elementProperties": {
                            "pageObjectId": slide_id,
                            "size": {
                                "width": { "magnitude": 600, "unit": "PT" },
                                "height": { "magnitude": 300, "unit": "PT" }
                            },
                            "transform": {
                                "scaleX": 1.0,
                                "scaleY": 1.0,
                                "translateX": 30.0,
                                "translateY": 80.0,
                                "unit": "PT"
                            }
                        }
                    }
                }));
                requests.push(serde_json::json!({
                    "insertText": {
                        "objectId": body_id,
                        "text": body_text
                    }
                }));
                // Style body
                requests.push(serde_json::json!({
                    "updateTextStyle": {
                        "objectId": body_id,
                        "style": {
                            "fontSize": { "magnitude": 14, "unit": "PT" },
                            "foregroundColor": {
                                "opaqueColor": {
                                    "rgbColor": { "red": 0.94, "green": 0.94, "blue": 0.96 }  // Light gray
                                }
                            }
                        },
                        "fields": "fontSize,foregroundColor"
                    }
                }));
            }

            // 4. Set dark background
            requests.push(serde_json::json!({
                "updatePageProperties": {
                    "objectId": slide_id,
                    "pageProperties": {
                        "pageBackgroundFill": {
                            "solidFill": {
                                "color": {
                                    "rgbColor": { "red": 0.039, "green": 0.039, "blue": 0.039 }  // #0A0A0A
                                }
                            }
                        }
                    },
                    "fields": "pageBackgroundFill"
                }
            }));
        }

        if requests.is_empty() {
            return Ok(());
        }

        // Send batchUpdate request
        let url = format!(
            "https://slides.googleapis.com/v1/presentations/{}:batchUpdate",
            presentation_id
        );
        let body = serde_json::json!({ "requests": requests });

        let resp = self
            .http_client
            .post(&url)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("BatchUpdate Request Error: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let error_body = resp.text().await.unwrap_or_default();
            tracing::error!("BatchUpdate failed: {} - {}", status, error_body);
            return Err(format!(
                "Slides BatchUpdate Error ({}): {}",
                status, error_body
            ));
        }

        Ok(())
    }

    /// Get the shareable URL for a Google Slides presentation
    pub fn get_presentation_url(&self, presentation_id: &str) -> String {
        format!(
            "https://docs.google.com/presentation/d/{}/edit",
            presentation_id
        )
    }
}

/// Slide data for Google Slides export
#[derive(Debug, Clone)]
pub struct SlideData {
    pub title: String,
    pub body: Vec<String>,
    pub layout: Option<String>,
}
