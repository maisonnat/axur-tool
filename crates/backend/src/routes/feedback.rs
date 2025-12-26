use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct FeedbackRequest {
    pub message: String,
    pub screenshot: Option<String>, // Base64 data URI
    pub url: String,
    pub user_agent: String,
    pub tenant_id: Option<String>,
    pub user_email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FeedbackResponse {
    pub success: bool,
    pub issue_url: Option<String>,
    pub message: String,
}

pub async fn submit_feedback(Json(payload): Json<FeedbackRequest>) -> impl IntoResponse {
    match process_github_feedback(payload).await {
        Ok(issue_url) => (
            StatusCode::OK,
            Json(FeedbackResponse {
                success: true,
                issue_url: Some(issue_url),
                message: "Feedback submitted successfully".to_string(),
            }),
        ),
        Err(e) => {
            tracing::error!("Failed to submit feedback to GitHub: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(FeedbackResponse {
                    success: false,
                    issue_url: None,
                    message: format!("Failed to submit feedback: {}", e),
                }),
            )
        }
    }
}

async fn process_github_feedback(payload: FeedbackRequest) -> anyhow::Result<String> {
    // Check for both GH_* (production) and GITHUB_* (local dev) naming conventions
    let token = env::var("GH_PAT")
        .or_else(|_| env::var("GITHUB_TOKEN"))
        .map_err(|_| anyhow::anyhow!("GH_PAT or GITHUB_TOKEN not set"))?;
    let owner = env::var("GH_OWNER")
        .or_else(|_| env::var("GITHUB_OWNER"))
        .map_err(|_| anyhow::anyhow!("GH_OWNER or GITHUB_OWNER not set"))?;
    let repo = env::var("GH_REPO")
        .or_else(|_| env::var("GITHUB_REPO"))
        .map_err(|_| anyhow::anyhow!("GH_REPO or GITHUB_REPO not set"))?;

    let client = reqwest::Client::new();
    let headers = {
        let mut h = reqwest::header::HeaderMap::new();
        h.insert(
            "Authorization",
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
        h.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_static("axur-feedback-bot"),
        );
        h.insert(
            "Accept",
            reqwest::header::HeaderValue::from_static("application/vnd.github.v3+json"),
        );
        h
    };

    // 1. Upload Screenshot if present
    let mut image_url = None;
    if let Some(screenshot_data) = payload.screenshot {
        // Simple heuristic to strip data:image/png;base64,...
        if let Some(comma_pos) = screenshot_data.find(',') {
            let base64_content = &screenshot_data[comma_pos + 1..];
            let filename = format!("uploads/{}.png", Uuid::new_v4());

            let upload_url = format!(
                "https://api.github.com/repos/{}/{}/contents/{}",
                owner, repo, filename
            );

            // GitHub API for Create File
            let body = json!({
                "message": "Upload feedback screenshot",
                "content": base64_content, // GitHub expects base64 string
                "committer": {
                    "name": "Axur Feedback Bot",
                    "email": "bot@axur.com"
                }
            });

            let res = client
                .put(&upload_url)
                .headers(headers.clone())
                .json(&body)
                .send()
                .await?;

            if !res.status().is_success() {
                let err_text = res.text().await?;
                tracing::error!("Failed to upload image to GitHub: {}", err_text);
                // Continue without image or fail? Let's continue without image but log error.
            } else {
                // Construct raw URL for the image to display in markdown
                // For private repos, raw tokens are needed, but usually clicking the link in the issue works if authenticated?
                // Actually, for private repos, images in README/Issues need special handling or tokenized URLs.
                // However, linking to the blob UI page is safe.
                // Or linking to raw.githubusercontent?
                // Let's link to the blob page: https://github.com/OWNER/REPO/blob/main/uploads/FILENAME.png
                // We don't know the branch easily? Default is usually main/master.
                // Let's assume 'main' or try to parse response content html_url.
                let resp_json: serde_json::Value = res.json().await?;
                if let Some(content) = resp_json.get("content") {
                    if let Some(html_url) = content.get("html_url").and_then(|v| v.as_str()) {
                        // This is the blob URL (viewer)
                        // For embedding image directly in issue (![alt](url)), we need raw URL.
                        // But raw URL for private repo requires token.
                        // GitHub issues for private repos usually copy assets to user-attachments.
                        // Since we are uploading to repo content:
                        // Users with access to the repo CAN see the image if we link to the Blob UI.
                        // So we will create a link: [View Screenshot](html_url)
                        image_url = Some(html_url.to_string());
                    }
                }
            }
        }
    }

    // 2. Create Issue
    let title = format!(
        "Feedback Beta: {}",
        payload.message.chars().take(50).collect::<String>()
    );

    let mut body = format!(
        "### Feedback Recibido\n\n{}\n\n***\n**Detalles Técnicos:**\n- **Usuario:** {}\n- **Tenant:** {}\n- **URL:** {}\n- **UA:** {}\n",
        payload.message,
        payload.user_email.as_deref().unwrap_or("Anon"),
        payload.tenant_id.as_deref().unwrap_or("N/A"),
        payload.url,
        payload.user_agent
    );

    if let Some(img) = image_url {
        // Convert blob URL to raw URL for display
        // Example: https://github.com/owner/repo/blob/main/path/to/img.png
        // Becomes: https://github.com/owner/repo/raw/main/path/to/img.png
        let raw_img = img.replace("/blob/", "/raw/");

        // Append image link
        body.push_str(&format!(
            "\n\n### Captura\n[![Screenshot]({})]({})",
            raw_img, img
        ));
        body.push_str("\n*(Click en la imagen para ver en tamaño completo)*");
    }

    let issue_url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    let issue_body = json!({
        "title": title,
        "body": body,
        "labels": ["beta-feedback", "automated"]
    });

    let res = client
        .post(&issue_url)
        .headers(headers)
        .json(&issue_body)
        .send()
        .await?;

    if !res.status().is_success() {
        let err = res.text().await?;
        return Err(anyhow::anyhow!("GitHub API Error: {}", err));
    }

    let resp_json: serde_json::Value = res.json().await?;
    let created_issue_url = resp_json["html_url"].as_str().unwrap_or("").to_string();

    Ok(created_issue_url)
}
