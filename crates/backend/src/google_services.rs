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
}
