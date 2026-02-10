# Files

## File: src/bin/seed_user.rs
```rust
use clap::Parser;
use std::process;
⋮----
struct Args {
⋮----
async fn main() {
⋮----
dotenv::dotenv().ok();
⋮----
let email = args.email.to_lowercase();
let role = args.role.to_lowercase();
⋮----
eprintln!("Error: role must be 'admin' or 'beta_tester'");
⋮----
println!("Initializing Firebase...");
⋮----
eprintln!(
⋮----
let doc_id = email.replace("@", "_at_").replace(".", "_dot_");
⋮----
println!("Adding user {} as {}...", email, role);
⋮----
match firestore.set_doc("allowed_users", &doc_id, &user).await {
Ok(_) => println!("Successfully added user: {}", email),
⋮----
eprintln!("Failed to add user: {}", e);
```

## File: src/middleware/mod.rs
```rust
pub mod security;
```

## File: src/middleware/security.rs
```rust
use axum_extra::extract::CookieJar;
⋮----
pub fn get_token_from_cookies(jar: &CookieJar) -> Option<String> {
jar.get(AUTH_COOKIE_NAME).map(|c| c.value().to_string())
⋮----
pub fn get_user_from_cookies(jar: &CookieJar) -> Option<String> {
jar.get(AUTH_USER_COOKIE_NAME)
.map(|c| c.value().to_string())
⋮----
pub async fn require_auth(
⋮----
if get_token_from_cookies(&jar).is_none() {
return Err(StatusCode::UNAUTHORIZED);
⋮----
if let Some(user_id) = get_user_from_cookies(&jar) {
request.extensions_mut().insert(user_id);
⋮----
Ok(next.run(request).await)
```

## File: src/routes/admin_config.rs
```rust
pub struct AdminConfig {
⋮----
pub async fn has_log_access(email: &str) -> bool {
let email_lower = email.to_lowercase();
let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");
⋮----
if let Some(role) = doc.get("role").and_then(|v| v.as_str()) {
⋮----
match storage.is_admin(email).await {
⋮----
pub async fn invalidate_cache() {
⋮----
pub async fn get_admin_config() -> AdminConfig {
```

## File: src/routes/admin.rs
```rust
use axum_extra::extract::CookieJar;
⋮----
use crate::error::ApiError;
use crate::middleware::AUTH_USER_COOKIE_NAME;
use crate::routes::AppState;
⋮----
pub struct BetaReq {
⋮----
pub struct AllowedUser {
⋮----
pub struct AddUserRequest {
⋮----
fn default_role() -> String {
"beta_tester".to_string()
⋮----
pub fn admin_routes() -> Router<AppState> {
⋮----
.route("/users", get(list_users))
.route("/users", post(add_user))
.route("/users/:email", delete(remove_user))
.route("/beta/requests", get(list_beta_requests))
.route(
⋮----
get(super::beta::get_pending_count),
⋮----
post(handle_beta_request_action),
⋮----
async fn require_admin(jar: &CookieJar) -> Result<String, ApiError> {
⋮----
.get(AUTH_USER_COOKIE_NAME)
.map(|c| c.value().to_string())
.ok_or_else(|| ApiError::Unauthorized("Not logged in".into()))?;
⋮----
let email_lower = user_email.to_lowercase();
⋮----
let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");
⋮----
if let Some(role) = doc.get("role").and_then(|v| v.as_str()) {
⋮----
return Ok(user_email);
⋮----
match storage.is_admin(&user_email).await {
Ok(true) => return Ok(user_email),
⋮----
return Err(ApiError::Forbidden("Admin access required".into()));
⋮----
Err(ApiError::Internal("Admin check unavailable".into()))
⋮----
async fn list_users(
⋮----
require_admin(&jar).await?;
⋮----
Ok(users) => return Ok(Json(users)),
⋮----
Ok(Json(vec![]))
⋮----
async fn add_user(
⋮----
let admin_email = require_admin(&jar).await?;
⋮----
if !payload.email.contains('@') {
return Err(ApiError::BadRequest("Invalid email format".into()));
⋮----
if !valid_roles.contains(&payload.role.as_str()) {
return Err(ApiError::BadRequest(format!(
⋮----
let email_lower = payload.email.to_lowercase();
⋮----
email: email_lower.clone(),
role: payload.role.clone(),
description: payload.description.clone(),
created_at: Some(chrono::Utc::now().to_rfc3339()),
added_by: Some(admin_email.clone()),
⋮----
serde_json::to_value(&user).map_err(|e| ApiError::Internal(e.to_string()))?;
if let Err(e) = firestore.set_doc("allowed_users", &doc_id, &doc_json).await {
⋮----
return Err(ApiError::Internal("Failed to save user".into()));
⋮----
return Err(ApiError::Internal("Storage not available".into()));
⋮----
Ok(Json(serde_json::json!({
⋮----
async fn remove_user(
⋮----
if email.to_lowercase() == admin_email.to_lowercase() {
return Err(ApiError::BadRequest("Cannot remove yourself".into()));
⋮----
let email_lower = email.to_lowercase();
⋮----
if let Err(e) = firestore.delete_doc("allowed_users", &doc_id).await {
⋮----
return Err(ApiError::Internal("Failed to remove user".into()));
⋮----
async fn list_beta_requests(
⋮----
.into_iter()
.map(|d| BetaReq {
id: d.email.clone(),
⋮----
company: d.company.unwrap_or_default(),
⋮----
requested_at: d.requested_at.map(|t| {
⋮----
.unwrap_or_default()
.with_timezone(&chrono::Utc)
⋮----
.collect();
return Ok(Json(requests));
⋮----
struct BetaRequestDoc {
⋮----
pub struct BetaActionRequest {
⋮----
async fn handle_beta_request_action(
⋮----
role: "beta_tester".to_string(),
description: Some("Approved from beta request".to_string()),
⋮----
.set_doc("allowed_users", &doc_id, &doc_json)
⋮----
.map_err(|e| ApiError::Internal(e.to_string()))?;
⋮----
.update_doc("beta_requests", &doc_id, &update)
⋮----
return Err(ApiError::BadRequest(
"Invalid action. Use 'approve' or 'reject'".into(),
```

## File: src/routes/auth.rs
```rust
use axum_extra::extract::CookieJar;
⋮----
use serde_json::json;
⋮----
use crate::error::ApiError;
⋮----
use crate::routes::AppState;
⋮----
pub struct LoginRequest {
⋮----
pub struct TwoFactorRequest {
⋮----
pub struct FinalizeRequest {
⋮----
pub struct LoginResponse {
⋮----
pub struct TwoFactorResponse {
⋮----
pub struct ValidateResponse {
⋮----
struct AxurAuthResponse {
⋮----
pub async fn login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, ApiError> {
⋮----
if payload.email.is_empty() || payload.password.is_empty() {
return Err(ApiError::BadRequest("Email and password required".into()));
⋮----
.timeout(std::time::Duration::from_secs(30))
.build()
.map_err(|e| ApiError::Internal(e.to_string()))?;
⋮----
let url = format!("{}/identity/session", AXUR_API_URL);
⋮----
.post(&url)
.json(&json!({
⋮----
.send()
⋮----
.bytes()
⋮----
.map_err(|e| ApiError::Internal(format!("Failed to parse Axur response: {}", e)))?;
⋮----
if data.correlation.is_none() {
⋮----
let parts: Vec<&str> = token.split('.').collect();
if parts.len() >= 2 {
⋮----
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
⋮----
.decode(parts[1])
.or_else(|_| STANDARD_NO_PAD.decode(parts[1]));
⋮----
if let Some(crl) = claims.get("crl").and_then(|v| v.as_str()) {
⋮----
data.correlation = Some(crl.to_string());
⋮----
Ok(Json(LoginResponse {
⋮----
message: "Credentials validated. Please complete 2FA.".into(),
⋮----
pub async fn verify_2fa(
⋮----
if payload.code.is_empty() || payload.token.is_empty() {
⋮----
return Err(ApiError::BadRequest("Code and token required".into()));
⋮----
.parse()
.map_err(|_| ApiError::BadRequest("2FA code must be numeric".into()))?;
⋮----
let url = format!("{}/identity/session/tfa", AXUR_API_URL);
⋮----
.header("Authorization", format!("Bearer {}", payload.token))
.json(&json!({"code": code}));
⋮----
req = req.header("oxref-token", corr);
⋮----
let resp = req.send().await?;
⋮----
if !resp.status().is_success() {
let status = resp.status();
let body = resp.text().await.unwrap_or_default();
⋮----
return Err(ApiError::Unauthorized("Invalid 2FA code".into()));
⋮----
.json()
⋮----
Ok(Json(TwoFactorResponse {
⋮----
message: "2FA verified. Please finalize login.".into(),
⋮----
pub async fn finalize(
⋮----
if payload.token.is_empty() || payload.device_id.is_empty() {
return Err(ApiError::BadRequest("Token and device_id required".into()));
⋮----
let email_lower = payload.email.to_lowercase();
⋮----
let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");
⋮----
match storage.is_user_allowed(&email_lower).await {
⋮----
return Err(ApiError::Forbidden(
⋮----
.into(),
⋮----
.header("Device-Id", &payload.device_id)
⋮----
return Err(ApiError::Unauthorized("Failed to finalize session".into()));
⋮----
.ok_or_else(|| ApiError::Internal("No master token received".into()))?;
⋮----
.http_only(true)
.secure(true)
.same_site(SameSite::None)
.path("/")
.max_age(cookie::time::Duration::days(7))
.build();
⋮----
let updated_jar = jar.add(cookie).add(user_cookie);
⋮----
Ok((
⋮----
Json(json!({
⋮----
pub async fn validate(
⋮----
let token = match jar.get(AUTH_COOKIE_NAME) {
Some(c) => c.value().to_string(),
⋮----
return Ok(Json(ValidateResponse {
⋮----
message: "No session found".into(),
⋮----
if let Some(user_cookie) = jar.get(AUTH_USER_COOKIE_NAME) {
let email = user_cookie.value();
let email_lower = email.to_lowercase();
⋮----
if let Some(role) = doc.get("role").and_then(|v| v.as_str()) {
⋮----
match storage.is_admin(email).await {
⋮----
.timeout(std::time::Duration::from_secs(10))
⋮----
let url = format!("{}/customers/customers", AXUR_API_URL);
⋮----
.get(&url)
.header("Authorization", format!("Bearer {}", token))
⋮----
let is_valid = resp.status().is_success() || resp.status().as_u16() == 403;
⋮----
Ok(Json(ValidateResponse {
⋮----
"Session valid".into()
⋮----
"Session expired".into()
⋮----
pub async fn logout(jar: CookieJar) -> impl IntoResponse {
⋮----
.max_age(cookie::time::Duration::seconds(0))
```

## File: src/routes/beta.rs
```rust
use crate::error::ApiError;
use crate::routes::AppState;
⋮----
pub struct BetaRequestPayload {
⋮----
pub struct BetaRequestResponse {
⋮----
pub async fn submit_beta_request(
⋮----
if payload.email.trim().is_empty() || payload.company.trim().is_empty() {
return Err(ApiError::BadRequest(
"Email and Company are required".into(),
⋮----
if !payload.email.contains('@') {
return Err(ApiError::BadRequest("Invalid email format".into()));
⋮----
let email_lower = payload.email.to_lowercase();
let doc_id = email_lower.replace("@", "_at_").replace(".", "_dot_");
⋮----
return Ok(Json(BetaRequestResponse {
⋮----
message: "You are already a registered beta user! Please log in.".into(),
⋮----
message: "We already have your request! We'll allow access shortly.".into(),
⋮----
.set_doc("beta_requests", &doc_id, &request_doc)
⋮----
.into(),
⋮----
return Err(ApiError::Internal(format!("Failed to save request: {}", e)));
⋮----
Err(ApiError::Internal("Storage not available".into()))
⋮----
pub async fn check_beta_status(
⋮----
.get("email")
.ok_or(ApiError::BadRequest("Email required".into()))?;
⋮----
let email_lower = email.to_lowercase();
⋮----
Ok(Some(_)) => return Ok("approved".to_string()),
⋮----
Ok(Some(req)) => return Ok(req.status),
⋮----
Ok("unknown".to_string())
⋮----
pub async fn get_pending_count(
⋮----
let pending = requests.iter().filter(|r| r.status == "pending").count();
return Ok(Json(serde_json::json!({
⋮----
Ok(Json(
⋮----
struct BetaRequestDoc {
```

## File: src/routes/feedback.rs
```rust
use serde_json::json;
use std::env;
use uuid::Uuid;
⋮----
pub struct FeedbackRequest {
⋮----
pub struct FeedbackResponse {
⋮----
pub async fn submit_feedback(Json(payload): Json<FeedbackRequest>) -> impl IntoResponse {
match process_github_feedback(payload).await {
⋮----
Json(FeedbackResponse {
⋮----
issue_url: Some(issue_url),
message: "Feedback submitted successfully".to_string(),
⋮----
message: format!("Failed to submit feedback: {}", e),
⋮----
async fn process_github_feedback(payload: FeedbackRequest) -> anyhow::Result<String> {
⋮----
.or_else(|_| env::var("GITHUB_TOKEN"))
.map_err(|_| anyhow::anyhow!("GH_PAT or GITHUB_TOKEN not set"))?;
⋮----
.or_else(|_| env::var("GITHUB_OWNER"))
.map_err(|_| anyhow::anyhow!("GH_OWNER or GITHUB_OWNER not set"))?;
⋮----
.or_else(|_| env::var("GITHUB_REPO"))
.map_err(|_| anyhow::anyhow!("GH_REPO or GITHUB_REPO not set"))?;
⋮----
h.insert(
⋮----
reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))?,
⋮----
if let Some(comma_pos) = screenshot_data.find(',') {
⋮----
let filename = format!("uploads/{}.png", Uuid::new_v4());
⋮----
let upload_url = format!(
⋮----
let body = json!({
⋮----
.put(&upload_url)
.headers(headers.clone())
.json(&body)
.send()
⋮----
if !res.status().is_success() {
let err_text = res.text().await?;
⋮----
let resp_json: serde_json::Value = res.json().await?;
if let Some(content) = resp_json.get("content") {
if let Some(html_url) = content.get("html_url").and_then(|v| v.as_str()) {
⋮----
image_url = Some(html_url.to_string());
⋮----
let title = format!(
⋮----
let mut body = format!(
⋮----
let raw_img = img.replace("/blob/", "/raw/");
⋮----
body.push_str(&format!(
⋮----
body.push_str("\n*(Click en la imagen para ver en tamaño completo)*");
⋮----
let issue_url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
let issue_body = json!({
⋮----
.post(&issue_url)
.headers(headers)
.json(&issue_body)
⋮----
let err = res.text().await?;
return Err(anyhow::anyhow!("GitHub API Error: {}", err));
⋮----
let created_issue_url = resp_json["html_url"].as_str().unwrap_or("").to_string();
⋮----
Ok(created_issue_url)
```

## File: src/routes/import_export.rs
```rust
use crate::routes::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::Multipart;
use serde::Serialize;
use tracing;
use uuid;
⋮----
pub struct ImportResponse {
⋮----
pub async fn import_pptx(
⋮----
while let Ok(Some(field)) = multipart.next_field().await {
let name = field.name().unwrap_or("").to_string();
⋮----
file_name = field.file_name().unwrap_or("presentation.pptx").to_string();
if let Ok(bytes) = field.bytes().await {
file_data = Some(bytes.to_vec());
⋮----
return Err((
⋮----
"No file uploaded".to_string(),
⋮----
let services = state.google_services.ok_or((
⋮----
"Google Services not configured (missing credentials)".to_string(),
⋮----
let temp_name = format!("preview_{}_{}", uuid, file_name);
⋮----
.upload_pptx(&temp_name, data)
⋮----
.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;
⋮----
.generate_previews(&file_id)
⋮----
.fetch_images_as_base64(google_urls)
⋮----
.map_err(|e| {
⋮----
let services_clone = services.clone();
let file_id_clone = file_id.clone();
⋮----
let _ = services_clone.delete_file(&file_id_clone).await;
⋮----
let _ = services.delete_file(&file_id).await;
⋮----
Ok(Json(ImportResponse {
⋮----
message: "Successfully generated previews with embedded images".to_string(),
⋮----
pub async fn inject_pptx(
⋮----
if let Ok(text) = field.text().await {
⋮----
"Invalid JSON in 'edits' field".to_string(),
⋮----
match inject_edits(&data, edits) {
⋮----
let filename_header = format!("attachment; filename=\"injected_{}\"", file_name);
⋮----
res.headers_mut().insert(
⋮----
axum::http::HeaderValue::from_str(&filename_header).unwrap_or_else(|_| {
⋮----
Ok(res)
⋮----
Err(e) => Err((
⋮----
format!("Injection failed: {}", e),
⋮----
use crate::google_services::SlideData;
use serde::Deserialize;
⋮----
pub struct ExportSlidesRequest {
⋮----
pub struct ExportSlideData {
⋮----
pub struct ExportSlidesResponse {
⋮----
pub async fn export_to_slides(
⋮----
.create_presentation(&request.title)
⋮----
.into_iter()
.map(|s| SlideData {
⋮----
.collect();
⋮----
if !slide_data.is_empty() {
⋮----
.add_slides_batch(&presentation_id, &slide_data)
⋮----
let presentation_url = services.get_presentation_url(&presentation_id);
let slides_count = slide_data.len();
⋮----
Ok(Json(ExportSlidesResponse {
⋮----
message: format!(
⋮----
use std::collections::HashMap;
⋮----
pub struct GeneratePptxRequest {
⋮----
pub struct GeneratePptxResponse {
⋮----
pub async fn generate_pptx_report(
⋮----
file_name = field.file_name().unwrap_or("report.pptx").to_string();
⋮----
"No PPTX file uploaded".to_string(),
⋮----
if placeholder_values.is_empty() {
⋮----
"No placeholder_values provided".to_string(),
⋮----
.map(|mut edit| {
⋮----
if let Some(real_value) = placeholder_values.get(key) {
edit.text = real_value.clone();
⋮----
match inject_edits(&data, resolved_edits) {
⋮----
Ok(Json(GeneratePptxResponse {
⋮----
message: format!("Generated PPTX report: {}", file_name),
pptx_base64: Some(base64_pptx),
⋮----
format!("PPTX generation failed: {}", e),
```

## File: src/routes/logs_api.rs
```rust
use super::admin_config;
use super::remote_log::RemoteLogConfig;
⋮----
pub struct ListLogsQuery {
⋮----
pub struct LogEntry {
⋮----
pub struct ListLogsResponse {
⋮----
pub struct LogEntryInternal {
⋮----
pub struct LogContentResponse {
⋮----
pub struct DailyStats {
⋮----
pub struct StatsResponse {
⋮----
pub async fn list_logs(Query(params): Query<ListLogsQuery>) -> impl IntoResponse {
⋮----
Json(ListLogsResponse {
⋮----
files: vec![],
⋮----
message: "Firestore not available".to_string(),
⋮----
let limit = params.limit.unwrap_or(50) as usize;
let offset = params.offset.unwrap_or(0) as usize;
⋮----
.unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());
⋮----
let path = format!("system_logs/{}/entries", date_str);
⋮----
logs.retain(|l| &l.category == cat);
⋮----
logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
⋮----
let total = logs.len() as i64;
⋮----
let logs_page: Vec<LogEntry> = logs.into_iter().skip(offset).take(limit).collect();
⋮----
.into_iter()
.map(|row| {
⋮----
.map(|dt| dt.format("%H:%M:%S").to_string())
.unwrap_or_else(|_| row.timestamp.clone());
⋮----
name: format!("{} - {}", ts, row.message),
path: row.id.clone(),
⋮----
sha: row.id.clone(),
⋮----
.collect();
⋮----
message: "OK".to_string(),
⋮----
message: format!("Firestore error: {}", e),
⋮----
async fn fetch_from_github(path: &str) -> Option<String> {
⋮----
let url = format!(
⋮----
.get(&url)
.header("Authorization", format!("Bearer {}", config.token))
.header("User-Agent", "axur-log-viewer")
.header("Accept", "application/vnd.github.v3+json")
.send()
⋮----
.ok()?;
⋮----
if !res.status().is_success() {
⋮----
let file_info: serde_json::Value = res.json().await.ok()?;
⋮----
let encoded = file_info.get("content").and_then(|c| c.as_str())?;
let clean = encoded.replace('\n', "");
⋮----
match BASE64.decode(&clean) {
Ok(bytes) => Some(String::from_utf8_lossy(&bytes).to_string()),
⋮----
/// Get specific log file content
/// GET /api/logs/content/*path
⋮----
/// GET /api/logs/content/*path
pub async fn get_log_content(Path(id_str): Path<String>) -> impl IntoResponse {
⋮----
pub async fn get_log_content(Path(id_str): Path<String>) -> impl IntoResponse {
⋮----
Json(LogContentResponse {
⋮----
content: "Firestore not available".to_string(),
⋮----
let parts: Vec<&str> = id_str.splitn(2, '_').collect();
if parts.len() != 2 {
⋮----
content: "Invalid ID format (missing date prefix)".to_string(),
⋮----
let is_truncated = row.content.contains("... (truncated");
⋮----
if (row.content.is_empty() || is_truncated) && row.github_path.is_some() {
⋮----
if let Some(full_content) = fetch_from_github(&gh_path).await {
⋮----
content: row.content.clone(),
size: row.content.len() as u64,
⋮----
content: "Log not found".to_string(),
⋮----
content: format!("Firestore error: {}", e),
⋮----
pub async fn list_log_dates() -> impl IntoResponse {
⋮----
let years = vec!["2024".to_string(), "2025".to_string()];
⋮----
Json(serde_json::json!({
⋮----
pub async fn list_log_categories(Query(params): Query<ListLogsQuery>) -> impl IntoResponse {
⋮----
let mut categories: Vec<String> = logs.into_iter().map(|l| l.category).collect();
categories.sort();
categories.dedup();
⋮----
pub struct AccessCheckQuery {
⋮----
pub struct AccessCheckResponse {
⋮----
pub async fn check_log_access(Query(params): Query<AccessCheckQuery>) -> impl IntoResponse {
⋮----
Json(AccessCheckResponse {
⋮----
"Access granted".to_string()
⋮----
"Access denied - not in admin list".to_string()
⋮----
pub struct StatsQuery {
⋮----
pub async fn get_log_stats(Query(params): Query<StatsQuery>) -> impl IntoResponse {
let days = params.days.unwrap_or(7);
⋮----
Json(StatsResponse {
⋮----
period: format!("{}d", days),
⋮----
daily_stats: vec![],
⋮----
for i in (0..days).rev() {
⋮----
let date_str = d.format("%Y-%m-%d").to_string();
⋮----
date: date_str.clone(),
⋮----
stats.total = logs.len() as i64;
⋮----
if log.category.contains("report") {
⋮----
} else if log.category.contains("error") {
⋮----
} else if log.category.contains("threat") {
⋮----
daily_stats.push(stats);
```

## File: src/routes/marketplace.rs
```rust
use crate::routes::AppState;
⋮----
pub struct MarketplaceTemplate {
⋮----
pub struct MarketplaceQuery {
⋮----
pub struct RateTemplateRequest {
⋮----
pub struct MarketplaceResponse {
⋮----
pub async fn list_marketplace(
⋮----
let limit = params.limit.unwrap_or(20).min(50) as usize;
let offset = params.offset.unwrap_or(0) as usize;
let featured_only = params.featured.unwrap_or(false);
⋮----
return fallback_mock_response();
⋮----
.into_iter()
.filter(|t| t.approved && (!featured_only || t.featured))
.collect();
⋮----
filtered.sort_by(|a, b| b.downloads.cmp(&a.downloads));
⋮----
let total = filtered.len();
⋮----
filtered.into_iter().skip(offset).take(limit).collect();
⋮----
Json(serde_json::json!({
⋮----
Err(_) => fallback_mock_response(),
⋮----
fn fallback_mock_response() -> (StatusCode, Json<serde_json::Value>) {
let mock_templates = vec![
⋮----
pub async fn publish_template(
⋮----
Json(MarketplaceResponse {
⋮----
message: "Storage not available".to_string(),
⋮----
.get_doc(&format!("user_templates/{}/items", user_id), &template_id)
⋮----
message: "Template not found".to_string(),
⋮----
message: e.to_string(),
⋮----
let marketplace_id = format!("pub_{}", template_id);
⋮----
message: "Already published".to_string(),
template_id: Some(template_id),
⋮----
id: marketplace_id.clone(),
template_id: template_id.clone(),
⋮----
.get("name")
.and_then(|v| v.as_str())
.unwrap_or("Untitled")
.to_string(),
⋮----
.get("description")
⋮----
.map(|s| s.to_string()),
author_name: Some("User".to_string()),
⋮----
published_at: chrono::Utc::now().to_rfc3339(),
⋮----
.set_doc("marketplace_templates", &marketplace_id, &entry)
⋮----
message: "Submitted for review".to_string(),
⋮----
pub async fn download_template(
⋮----
let mut doc: MarketplaceTemplate = match firestore.get_doc("marketplace_templates", &id).await {
⋮----
message: "Not found".to_string(),
⋮----
match firestore.set_doc("marketplace_templates", &id, &doc).await {
⋮----
message: "Template downloaded".to_string(),
template_id: Some(doc.template_id),
⋮----
pub async fn rate_template(
⋮----
message: "Rating 1-5".to_string(),
⋮----
message: "Rated".to_string(),
template_id: Some(id),
⋮----
pub async fn list_pending_templates(
⋮----
Json(serde_json::json!({"error": "No storage"})),
⋮----
.into_response()
⋮----
all.into_iter().filter(|t| !t.approved).collect();
⋮----
Json(serde_json::json!({ "success": true, "pending": pending })),
⋮----
Json(serde_json::json!({ "success": false, "error": e.to_string() })),
⋮----
.into_response(),
⋮----
pub async fn approve_template(
⋮----
update_approval(&id, true).await
⋮----
pub async fn reject_template(
⋮----
message: "No storage".into(),
⋮----
match firestore.delete_doc("marketplace_templates", &id).await {
⋮----
message: "Rejected/Deleted".to_string(),
⋮----
async fn update_approval(id: &str, approved: bool) -> axum::response::Response {
⋮----
let mut doc: MarketplaceTemplate = match firestore.get_doc("marketplace_templates", id).await {
⋮----
message: "Not found".into(),
⋮----
match firestore.set_doc("marketplace_templates", id, &doc).await {
⋮----
"Approved".to_string()
⋮----
"Updated".to_string()
⋮----
template_id: Some(id.to_string()),
```

## File: src/routes/mod.rs
```rust
pub mod admin;
pub mod admin_config;
pub mod auth;
pub mod beta;
pub mod feedback;
pub mod import_export;
pub mod logs_api;
pub mod marketplace;
pub mod queue;
pub mod remote_log;
pub mod report;
pub mod status;
pub mod storage;
pub mod templates;
⋮----
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
⋮----
pub struct AppState {
⋮----
pub fn create_router(state: AppState) -> Router {
⋮----
.allow_origin([
⋮----
.expect("Static header valid"),
⋮----
.allow_methods([
⋮----
.allow_headers([
⋮----
.expose_headers([header::SET_COOKIE, header::CONTENT_TYPE])
.allow_credentials(true)
.max_age(std::time::Duration::from_secs(3600));
⋮----
.route("/health", get(health_check))
.route("/api/health", get(status::health))
.route("/api/status", get(status::full_status))
.route("/api/public/beta-request", post(beta::submit_beta_request))
.route("/api/public/beta-status", get(beta::check_beta_status))
.route("/api/auth/login", post(auth::login))
.route("/api/auth/2fa", post(auth::verify_2fa))
.route("/api/auth/finalize", post(auth::finalize))
.route("/api/auth/validate", get(auth::validate))
.route("/api/auth/logout", post(auth::logout))
⋮----
.route("/api/marketplace", get(marketplace::list_marketplace))
⋮----
.route("/api/templates/:id", get(templates::get_template));
⋮----
.route("/api/tenants", get(report::list_tenants))
.route("/api/report/generate", post(report::generate_report))
.route("/api/export/inject", post(import_export::inject_pptx))
.route(
⋮----
post(import_export::generate_pptx_report),
⋮----
.route("/api/import/pptx", post(import_export::import_pptx))
.route("/api/export/slides", post(import_export::export_to_slides))
⋮----
post(report::threat_hunting_preview),
⋮----
get(report::threat_hunting_preview_stream),
⋮----
get(report::generate_report_stream),
⋮----
.route("/api/feedback", post(feedback::submit_feedback))
.route("/api/logs/sync", post(remote_log::sync_logs))
⋮----
.route("/api/logs", get(logs_api::list_logs))
.route("/api/logs/dates", get(logs_api::list_log_dates))
.route("/api/logs/categories", get(logs_api::list_log_categories))
.route("/api/logs/content/*path", get(logs_api::get_log_content))
.route("/api/logs/access", get(logs_api::check_log_access))
.route("/api/logs/stats", get(logs_api::get_log_stats))
⋮----
.route("/api/templates", get(templates::list_templates))
.route("/api/templates", post(templates::create_template))
⋮----
.route("/api/templates/:id", put(templates::update_template))
.route("/api/templates/:id", delete(templates::delete_template))
.route("/api/templates/:id/pptx", get(templates::get_template_pptx))
⋮----
post(templates::quick_save_template),
⋮----
get(templates::quick_load_template),
⋮----
post(marketplace::publish_template),
⋮----
post(marketplace::download_template),
⋮----
post(marketplace::rate_template),
⋮----
get(marketplace::list_pending_templates),
⋮----
post(marketplace::approve_template),
⋮----
post(marketplace::reject_template),
⋮----
.nest("/api/admin", admin::admin_routes())
.route_layer(axum::middleware::from_fn(crate::middleware::require_auth));
⋮----
let queue_routes: Router<AppState> = queue::queue_routes().with_state(());
⋮----
let storage_routes: Router<AppState> = storage::storage_routes().with_state(());
⋮----
.merge(public_routes)
.merge(protected_routes)
.nest("/api/queue", queue_routes)
.nest("/api/storage", storage_routes)
.layer(DefaultBodyLimit::max(50 * 1024 * 1024))
.layer(TraceLayer::new_for_http())
.layer(cors);
⋮----
let app_with_state = app.with_state(state);
⋮----
async fn health_check() -> axum::Json<serde_json::Value> {
```

## File: src/routes/queue.rs
```rust
use serde::Deserialize;
⋮----
pub fn queue_routes() -> Router {
⋮----
.route("/submit", post(submit_job))
.route("/status/:job_id", get(get_job_status))
.route("/stream/:job_id", get(stream_job_status))
.route("/length", get(get_queue_length))
⋮----
pub struct SubmitJobRequest {
⋮----
async fn submit_job(Json(req): Json<SubmitJobRequest>) -> impl IntoResponse {
let queue = get_queue();
⋮----
let (job_type, api_type) = match req.job_type.as_str() {
⋮----
.get("tenant_id")
.and_then(|v| v.as_str())
.unwrap_or("default")
.to_string();
⋮----
.get("template_name")
⋮----
.unwrap_or("unnamed")
⋮----
.get("template_id")
⋮----
return Json(serde_json::json!({
⋮----
let job_id = queue.submit(job).await;
let position = queue.queue_length().await;
let eta = queue.estimate_wait_time(position, api_type);
⋮----
Json(serde_json::json!({
⋮----
async fn get_job_status(Path(job_id): Path<String>) -> impl IntoResponse {
⋮----
match queue.get_job(&job_id).await {
⋮----
let eta = queue.estimate_wait_time(position, job.api_type);
response.eta_seconds = Some(eta.as_secs());
⋮----
Json(serde_json::json!(response))
⋮----
None => Json(serde_json::json!({
⋮----
async fn get_queue_length() -> impl IntoResponse {
⋮----
let length = queue.queue_length().await;
⋮----
async fn stream_job_status(
⋮----
let event = match queue.get_job(&job_id).await {
⋮----
let data = serde_json::to_string(&response).unwrap_or_default();
⋮----
let is_final = matches!(
⋮----
return Some((
Ok(Event::default().data(data).event("complete")),
⋮----
Event::default().data(data).event("update")
⋮----
.data(r#"{"error":"Job not found"}"#)
.event("error"),
⋮----
Some((Ok(event), (job_id, tick + 1)))
⋮----
Sse::new(stream).keep_alive(
⋮----
.interval(Duration::from_secs(15))
.text("ping"),
```

## File: src/routes/remote_log.rs
```rust
use axum::http::StatusCode;
⋮----
use serde_json::json;
use uuid::Uuid;
⋮----
pub struct RemoteLogConfig {
⋮----
impl RemoteLogConfig {
⋮----
pub fn from_env() -> Option<Self> {
⋮----
.or_else(|_| std::env::var("GITHUB_TOKEN"))
.ok()?;
⋮----
.or_else(|_| std::env::var("GITHUB_OWNER"))
⋮----
.or_else(|_| std::env::var("GITHUB_LOGS_REPO"))
.unwrap_or_else(|_| "axur-logs-private".to_string());
⋮----
Some(Self { token, owner, repo })
⋮----
async fn get_file_sha(config: &RemoteLogConfig, path: &str) -> Option<String> {
⋮----
let url = format!(
⋮----
.get(&url)
.header("Authorization", format!("Bearer {}", config.token))
.header("User-Agent", "axur-bot")
.header("Accept", "application/vnd.github.v3+json")
.send()
⋮----
if res.status().is_success() {
let json: serde_json::Value = res.json().await.ok()?;
json.get("sha")
.and_then(|s| s.as_str())
.map(|s| s.to_string())
⋮----
async fn upload_to_github(
⋮----
let upload_url = format!(
⋮----
let encoded_content = BASE64.encode(content.as_bytes());
⋮----
body_map.insert("message".to_string(), json!(message));
body_map.insert("content".to_string(), json!(encoded_content));
⋮----
body_map.insert(
"committer".to_string(),
json!({
⋮----
body_map.insert("sha".to_string(), json!(s));
⋮----
.put(&upload_url)
⋮----
.json(&body)
⋮----
let res = perform_upload(None).await.map_err(|e| e.to_string())?;
⋮----
let resp_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
return Ok(resp_json
.get("content")
.and_then(|c| c.get("html_url"))
.and_then(|u| u.as_str())
.unwrap_or("")
.to_string());
⋮----
// Handle conflict (update)
if res.status() == reqwest::StatusCode::CONFLICT
|| res.status() == reqwest::StatusCode::UNPROCESSABLE_ENTITY
⋮----
if let Some(sha) = get_file_sha(config, path).await {
let retry_res = perform_upload(Some(sha)).await.map_err(|e| e.to_string())?;
if retry_res.status().is_success() {
⋮----
retry_res.json().await.map_err(|e| e.to_string())?;
⋮----
Err(format!("GitHub upload failed: {}", res.status()))
⋮----
/// Upload a log entry (Hybrid: GitHub + Firestore)
pub async fn upload_log(category: &str, filename: &str, content: &str) -> Result<String, String> {
⋮----
pub async fn upload_log(category: &str, filename: &str, content: &str) -> Result<String, String> {
⋮----
let message = format!("Log: {} - {}", category, filename);
⋮----
// 1. Upload to GitHub
⋮----
let date_folder = now.format("%Y/%m/%d").to_string();
github_path = format!("logs/{}/{}/{}", date_folder, category, filename);
⋮----
match upload_to_github(&config, &github_path, content, &message).await {
⋮----
None => return Err("Firestore not available".to_string()),
⋮----
let level = if category.contains("error") {
⋮----
} else if category.contains("warn") {
⋮----
let db_content = if content.len() > max_db_content_size {
format!(
⋮----
content.to_string()
⋮----
let metadata = serde_json::from_str::<serde_json::Value>(content).unwrap_or(json!({}));
⋮----
let date_key = now.format("%Y-%m-%d").to_string();
⋮----
let id = format!("{}_{}", date_key, uuid_val);
⋮----
let log_entry = json!({
⋮----
let path = format!("system_logs/{}/entries", date_key);
match firestore.set_doc(&path, &id, &log_entry).await {
⋮----
Ok(id)
⋮----
Err(e) => Err(format!("Firestore error: {}", e)),
⋮----
pub fn upload_log_async(category: &str, filename: &str, content: String) {
let category = category.to_string();
let filename = filename.to_string();
⋮----
match upload_log(&category, &filename, &content).await {
⋮----
pub fn upload_report_async(tenant: &str, filename: &str, content: String) {
let _tenant = tenant.to_string();
⋮----
let category = "reports".to_string();
⋮----
if let Err(e) = upload_log(&category, &filename, &content).await {
⋮----
pub fn upload_debug_json(category: &str, data: &serde_json::Value) {
let filename = format!(
⋮----
let content = serde_json::to_string_pretty(data).unwrap_or_default();
upload_log_async(category, &filename, content);
⋮----
pub fn log_request<T: serde::Serialize>(operation: &str, payload: &T, tenant_id: Option<&str>) {
⋮----
let op = operation.to_string();
let tid = tenant_id.map(|s| s.to_string());
let props = serde_json::to_value(payload).unwrap_or(json!({}));
⋮----
let id = Uuid::new_v4().to_string();
⋮----
let event = json!({
⋮----
.set_doc(
&format!("analytics_events/{}/events", date_key),
⋮----
upload_debug_json(&format!("{}_requests", operation), &log_data);
⋮----
pub fn log_response<T: serde::Serialize>(
⋮----
let props = json!({
⋮----
upload_debug_json(&format!("{}_responses", operation), &log_data);
⋮----
pub fn log_error(
⋮----
let code = error_code.to_string();
let msg = error_message.to_string();
⋮----
let ctx = context.clone();
⋮----
upload_debug_json("errors", &log_data);
⋮----
pub fn log_performance(
⋮----
upload_debug_json("performance_metrics", &log_data);
⋮----
pub fn log_feature_usage(
⋮----
let feat = feature.to_string();
⋮----
let meta = metadata.clone().unwrap_or(json!({}));
⋮----
upload_debug_json("feature_usage", &log_data);
⋮----
use serde::Serialize;
⋮----
pub struct SyncLogsResponse {
⋮----
pub async fn sync_logs() -> impl IntoResponse {
⋮----
if !debug_dir.exists() {
⋮----
Json(SyncLogsResponse {
⋮----
message: "No debug_logs directory found".to_string(),
⋮----
for entry in entries.flatten() {
let path = entry.path();
if path.is_file() {
⋮----
.file_name()
.and_then(|n| n.to_str())
.unwrap_or("unknown")
.to_string();
⋮----
let category = if filename.starts_with("th_") {
⋮----
} else if filename.starts_with("exposure_") {
⋮----
} else if filename.starts_with("fetch_") {
⋮----
match upload_log(category, &filename, &content).await {
⋮----
message: format!("Synced {} files to DB, {} failed", uploaded, failed),
```

## File: src/routes/report.rs
```rust
use axum::extract::State;
⋮----
use axum_extra::extract::CookieJar;
use futures::stream::Stream;
⋮----
use std::convert::Infallible;
use uuid::Uuid;
⋮----
use crate::error::ApiError;
use crate::middleware::AUTH_COOKIE_NAME;
⋮----
use crate::routes::AppState;
⋮----
use axur_core::report::OfflineAssets;
use std::time::Instant;
⋮----
fn default_language() -> String {
"es".to_string()
⋮----
pub struct ThreatHuntingPreviewRequest {
⋮----
pub struct ThreatHuntingPreviewResponse {
⋮----
pub async fn list_tenants(jar: CookieJar) -> Result<Json<Vec<TenantResponse>>, ApiError> {
⋮----
.get(AUTH_COOKIE_NAME)
.map(|c| c.value().to_string())
.ok_or_else(|| ApiError::Unauthorized("No session found".into()))?;
⋮----
let tenants = fetch_available_tenants(&token)
⋮----
.map_err(|e| ApiError::ExternalApi(format!("Failed to fetch tenants: {}", e)))?;
⋮----
.into_iter()
.map(|t| TenantResponse {
⋮----
.collect();
⋮----
Ok(Json(response))
⋮----
pub async fn generate_report(
⋮----
if payload.tenant_id.is_empty() {
⋮----
return Ok(Json(GenerateReportResponse {
⋮----
message: "Tenant ID is required".into(),
error_code: Some(code.code()),
error_message: Some(get_user_friendly_message(&code)),
⋮----
if payload.from_date.is_empty() || payload.to_date.is_empty() {
⋮----
message: "Date range is required".into(),
⋮----
crate::routes::remote_log::log_request("report_generate", &payload, Some(&payload.tenant_id));
⋮----
start_time.elapsed().as_millis(),
Some(&payload.tenant_id),
⋮----
Some(serde_json::json!({
⋮----
let filename = format!(
⋮----
crate::routes::remote_log::upload_report_async(company_name, &filename, html.clone());
⋮----
pub async fn threat_hunting_preview(
⋮----
crate::routes::remote_log::log_request("th_preview", &payload, Some(&payload.tenant_id));
⋮----
vec![]
⋮----
if tickets.is_empty() {
return Ok(Json(ThreatHuntingPreviewResponse {
⋮----
message: format!(
⋮----
match preview_threat_hunting(
⋮----
let duration_ms = start_time.elapsed().as_millis();
⋮----
Ok(Json(ThreatHuntingPreviewResponse {
⋮----
preview: Some(preview),
message: format!("Preview ready. Found {} tickets with tag.", tickets.len()),
⋮----
&e.to_string(),
⋮----
message: format!("Preview failed: {}", e),
⋮----
pub enum ThreatHuntingStreamEvent {
⋮----
pub enum ReportStreamEvent {
⋮----
pub struct GenerateReportStreamParams {
⋮----
pub async fn threat_hunting_preview_stream(
⋮----
let tenant_id = params.tenant_id.clone();
let story_tag = params.story_tag.clone();
⋮----
Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
⋮----
pub async fn generate_report_stream(
⋮----
let from_date = params.from_date.clone();
let to_date = params.to_date.clone();
let language_str = params.language.clone();
⋮----
let template_id = params.template_id.clone();
⋮----
let _plugin_theme = params.plugin_theme.clone();
⋮----
.as_ref()
.map(|s| s.split(',').map(|x| x.trim().to_string()).collect());
```

## File: src/routes/status.rs
```rust
use serde::Serialize;
use std::env;
⋮----
pub async fn health() -> impl IntoResponse {
Json(serde_json::json!({
⋮----
pub struct ServiceCheck {
⋮----
pub enum ServiceStatus {
⋮----
pub struct StatusResponse {
⋮----
pub struct BackendInfo {
⋮----
pub struct EnvironmentInfo {
⋮----
pub async fn full_status() -> impl IntoResponse {
⋮----
let axur_check = check_axur_api().await;
if matches!(axur_check.status, ServiceStatus::Error) {
⋮----
services.push(axur_check);
⋮----
let github_logs_check = check_github_logs();
if matches!(github_logs_check.status, ServiceStatus::Unconfigured) {
⋮----
services.push(github_logs_check);
⋮----
let github_feedback_check = check_github_feedback();
if matches!(github_feedback_check.status, ServiceStatus::Unconfigured) {
⋮----
services.push(github_feedback_check);
⋮----
let db_check = check_firestore().await;
if matches!(db_check.status, ServiceStatus::Error) {
⋮----
services.push(db_check);
⋮----
timestamp: chrono::Utc::now().to_rfc3339(),
⋮----
version: env!("CARGO_PKG_VERSION").to_string(),
rust_version: "1.74".to_string(),
build_profile: if cfg!(debug_assertions) {
"debug".to_string()
⋮----
"release".to_string()
⋮----
git_hash: env::var("GIT_HASH").unwrap_or_else(|_| "unknown".to_string()),
⋮----
axur_api_configured: env::var("AXUR_TOKEN").is_ok(),
github_logs_configured: env::var("GH_LOGS_REPO").is_ok(),
github_feedback_configured: env::var("GITHUB_TOKEN").is_ok(),
⋮----
(status_code, Json(response))
⋮----
async fn check_axur_api() -> ServiceCheck {
⋮----
.timeout(std::time::Duration::from_secs(5))
.build();
⋮----
name: "Axur API".into(),
⋮----
message: Some(format!("Failed to create HTTP client: {}", e)),
⋮----
.get("https://api.axur.com/gateway/1.0/api/customers/customers")
.header("Accept", "application/json")
.send()
⋮----
let latency = start.elapsed().as_millis() as u64;
⋮----
let status_code = resp.status();
⋮----
if status_code.as_u16() == 401
|| status_code.as_u16() == 403
|| status_code.is_success()
⋮----
latency_ms: Some(latency),
message: Some("API reachable".into()),
⋮----
message: Some(format!("Unexpected status: {}", status_code)),
⋮----
message: Some(format!("Connection failed: {}", e)),
⋮----
fn check_github_logs() -> ServiceCheck {
let token_ok = env::var("GH_PAT").is_ok() || env::var("GITHUB_TOKEN").is_ok();
let owner_ok = env::var("GH_OWNER").is_ok() || env::var("GITHUB_OWNER").is_ok();
let repo_ok = env::var("GH_LOGS_REPO").is_ok() || env::var("GITHUB_LOGS_REPO").is_ok();
⋮----
name: "GitHub Logs".into(),
⋮----
message: Some("Configured".into()),
⋮----
message: Some("Configured (using default repo)".into()),
⋮----
Some("GH_PAT/GITHUB_TOKEN")
⋮----
Some("GH_OWNER/GITHUB_OWNER")
⋮----
.into_iter()
.flatten()
.collect();
⋮----
message: Some(format!("Missing: {}", missing.join(", "))),
⋮----
fn check_github_feedback() -> ServiceCheck {
⋮----
let repo_ok = env::var("GH_REPO").is_ok() || env::var("GITHUB_REPO").is_ok();
⋮----
name: "GitHub Feedback".into(),
⋮----
Some("GH_REPO/GITHUB_REPO")
⋮----
async fn check_firestore() -> ServiceCheck {
⋮----
let duration = start.elapsed().as_millis() as u64;
⋮----
name: "Firestore".to_string(),
⋮----
latency_ms: Some(duration),
message: Some("Connected".to_string()),
⋮----
message: Some(format!("Connection success but query failed: {}", e)),
⋮----
message: Some("Firestore client not initialized".to_string()),
```

## File: src/routes/storage.rs
```rust
use serde::Deserialize;
⋮----
use crate::github_storage::get_github_storage;
⋮----
pub fn storage_routes() -> Router {
⋮----
.route("/templates", get(list_templates))
.route("/templates", post(save_template))
.route("/templates/:name", get(load_template))
.route("/templates/:name", delete(delete_template))
⋮----
pub struct SaveTemplateRequest {
⋮----
async fn list_templates(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
⋮----
.get("user_id")
.and_then(|v| v.as_str())
.unwrap_or("");
⋮----
if user_id.is_empty() {
return Json(serde_json::json!({
⋮----
match get_github_storage() {
Some(storage) => match storage.list_templates(user_id).await {
Ok(templates) => Json(serde_json::json!({
⋮----
Err(e) => Json(serde_json::json!({
⋮----
None => Json(serde_json::json!({
⋮----
async fn save_template(Json(req): Json<SaveTemplateRequest>) -> impl IntoResponse {
if req.user_id.is_empty() || req.name.is_empty() {
⋮----
.save_template(&req.user_id, &req.name, &req.content)
⋮----
Ok(()) => Json(serde_json::json!({
⋮----
async fn load_template(
⋮----
Some(storage) => match storage.load_template(user_id, &name).await {
Ok(content) => Json(serde_json::json!({
⋮----
async fn delete_template(
⋮----
Some(storage) => match storage.delete_template(user_id, &name).await {
```

## File: src/routes/templates.rs
```rust
use uuid::Uuid;
⋮----
use crate::routes::AppState;
use axur_core::editor::PresentationTemplate;
⋮----
pub struct TemplateListItem {
⋮----
pub struct RawSlide {
⋮----
pub struct CreateTemplateRequest {
⋮----
pub struct UpdateTemplateRequest {
⋮----
pub struct TemplateDetail {
⋮----
impl TemplateDetail {
⋮----
pub fn from_template(id: &str, template: &PresentationTemplate) -> Self {
⋮----
.iter()
.enumerate()
.map(|(_i, slide)| {
⋮----
.collect();
⋮----
id: id.to_string(),
name: template.name.clone(),
description: template.description.clone(),
⋮----
pub struct TemplateResponse {
⋮----
pub struct ListTemplatesQuery {
⋮----
pub struct GitHubConfig {
⋮----
impl GitHubConfig {
pub fn from_env() -> Option<Self> {
Some(Self {
token: std::env::var("GITHUB_TOKEN").ok()?,
owner: std::env::var("GITHUB_OWNER").unwrap_or_else(|_| "maisonnat".to_string()),
⋮----
.unwrap_or_else(|_| "axur-logs-private".to_string()),
⋮----
async fn get_file_sha(config: &GitHubConfig, path: &str) -> Option<String> {
let url = format!(
⋮----
.get(&url)
.header("Authorization", format!("Bearer {}", config.token))
.header("User-Agent", "axur-bot")
.send()
⋮----
.ok()?;
⋮----
if res.status().is_success() {
let json: serde_json::Value = res.json().await.ok()?;
json.get("sha")?.as_str().map(|s| s.to_string())
⋮----
async fn upload_template_to_github(
⋮----
.map_err(|e| format!("Serialization error: {}", e))?;
⋮----
let encoded = BASE64.encode(content.as_bytes());
let sha = get_file_sha(config, path).await;
⋮----
.put(&url)
⋮----
.header("Accept", "application/vnd.github.v3+json")
.json(&body)
⋮----
.map_err(|e| e.to_string())?;
⋮----
let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
Ok(json
.get("content")
.and_then(|c| c.get("html_url"))
.and_then(|u| u.as_str())
.unwrap_or("")
.to_string())
⋮----
Err(format!("GitHub upload failed: {}", res.status()))
⋮----
async fn upload_file_to_github(
⋮----
let encoded = BASE64.encode(content);
⋮----
/// Fetch raw file content (binary) from GitHub
pub async fn fetch_raw_file_from_github(
⋮----
pub async fn fetch_raw_file_from_github(
⋮----
"https://api.github.com/repos/{}/{}/contents/{}",
⋮----
if !res.status().is_success() {
return Err(format!("Template not found: {}", res.status()));
⋮----
.and_then(|c| c.as_str())
.ok_or("Missing content")?;
⋮----
let clean: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
let decoded = BASE64.decode(&clean).map_err(|e| e.to_string())?;
let content = String::from_utf8(decoded).map_err(|e| e.to_string())?;
⋮----
serde_json::from_str(&content).map_err(|e| e.to_string())
⋮----
pub async fn list_templates(
⋮----
let collection = format!("user_templates/{}/items", user_id);
⋮----
Json(serde_json::json!({
⋮----
pub async fn create_template(
⋮----
while let Ok(Some(field)) = multipart.next_field().await {
let name = field.name().unwrap_or("").to_string();
⋮----
let data = field.text().await.unwrap_or_default();
⋮----
req = Some(parsed);
⋮----
if let Ok(bytes) = field.bytes().await {
file_data = Some(bytes.to_vec());
⋮----
Json(TemplateResponse {
⋮----
message: "Metadata field required".to_string(),
⋮----
if req.name.is_empty() {
⋮----
message: "Name is required".to_string(),
⋮----
message: "GitHub not configured".to_string(),
⋮----
message: "Storage not available".to_string(),
⋮----
let metadata_path = format!("templates/{}/{}/metadata.json", user_id, template_id);
let pptx_path = format!("templates/{}/{}/base.pptx", user_id, template_id);
⋮----
.map(|(i, s)| axur_core::editor::SlideDefinition {
⋮----
.as_ref()
.and_then(|i| Uuid::parse_str(i).ok())
.unwrap_or_else(Uuid::new_v4),
name: s.name.clone(),
canvas_json: s.canvas_json.as_ref().map(|v| v.to_string()),
⋮----
name: req.name.clone(),
description: req.description.clone(),
⋮----
version: "1.0.0".to_string(),
⋮----
if let Err(e) = upload_template_to_github(&config, &metadata_path, &template_obj).await {
⋮----
message: format!("Metadata upload failed: {}", e),
⋮----
if let Err(e) = upload_file_to_github(
⋮----
&format!("Add base PPTX for {}", req.name),
⋮----
message: format!("PPTX upload failed: {}", e),
⋮----
let created_at = chrono::Utc::now().to_rfc3339();
⋮----
.set_doc(
&format!("user_templates/{}/items", user_id),
&template_id.to_string(),
⋮----
id: Some(template_id.to_string()),
message: "Template created".to_string(),
⋮----
message: format!("Failed to save metadata: {}", e),
⋮----
pub fn get_mock_template(id: &str) -> Option<PresentationTemplate> {
⋮----
// Template 2: Executive Summary - KPI Dashboard
⋮----
// Template 3: Risk Assessment - Focus on Risk Score
⋮----
// Template 4: Takedowns Performance
⋮----
// Template 5: Closing / Thank You Slide
⋮----
"1" | "axur_official" => Some(PresentationTemplate {
name: "Axur Official".to_string(),
slides: vec![axur_core::editor::SlideDefinition {
⋮----
"2" | "executive" => Some(PresentationTemplate {
name: "Executive Summary".to_string(),
⋮----
"3" | "risk" => Some(PresentationTemplate {
name: "Risk Focus".to_string(),
⋮----
"4" | "technical" => Some(PresentationTemplate {
name: "Technical Deep Dive".to_string(),
⋮----
"5" | "compliance" => Some(PresentationTemplate {
name: "Compliance Report".to_string(),
⋮----
pub async fn get_template(Path(template_id): Path<String>) -> impl IntoResponse {
⋮----
if let Some(mock) = get_mock_template(&template_id) {
⋮----
id: Some(template_id.clone()),
message: "Template loaded (Mock)".to_string(),
template: Some(TemplateDetail::from_template(&template_id, &mock)),
⋮----
message: format!("Template '{}' not found", template_id),
⋮----
pub async fn update_template(
⋮----
.get_doc(&format!("user_templates/{}/items", user_id), &template_id)
⋮----
message: "Not found".to_string(),
⋮----
message: e.to_string(),
⋮----
.get("github_path")
.and_then(|v| v.as_str())
.unwrap_or_default()
.to_string();
⋮----
if !github_path.is_empty() {
if let Err(e) = upload_template_to_github(&config, &github_path, template).await {
⋮----
.get("name")
⋮----
.unwrap_or("");
let current_desc = current_meta.get("description").and_then(|v| v.as_str());
⋮----
.map(|s| s.as_str())
.unwrap_or(current_name);
⋮----
.or(current_desc);
⋮----
let mut update = current_meta.clone();
if let Some(obj) = update.as_object_mut() {
obj.insert("name".to_string(), serde_json::json!(new_name));
obj.insert("description".to_string(), serde_json::json!(new_desc));
obj.insert(
"updated_at".to_string(),
⋮----
.update_doc(
⋮----
id: Some(template_id),
message: "Updated".to_string(),
⋮----
pub async fn delete_template(
⋮----
.delete_doc(&format!("user_templates/{}/items", user_id), &template_id)
⋮----
message: "Deleted".to_string(),
⋮----
pub async fn get_template_pptx(
⋮----
Json(serde_json::json!({ "success": false, "error": "Storage not available" })),
⋮----
Json(serde_json::json!({ "success": false, "error": "GitHub not configured" })),
⋮----
Json(serde_json::json!({ "success": false, "error": "Template not found" })),
⋮----
Json(serde_json::json!({ "success": false, "error": e.to_string() })),
⋮----
if github_path.is_empty() {
⋮----
Json(serde_json::json!({ "success": false, "error": "No GitHub path for template" })),
⋮----
let pptx_path = github_path.replace("metadata.json", "base.pptx");
⋮----
match fetch_raw_file_from_github(&config, &pptx_path).await {
⋮----
let base64_pptx = BASE64.encode(&bytes);
⋮----
pub struct QuickSaveRequest {
⋮----
pub struct QuickSaveResponse {
⋮----
pub async fn quick_save_template(
⋮----
Json(QuickSaveResponse {
⋮----
let path = format!("user_templates/{}/items", user_id);
let mut template_id = Uuid::new_v4().to_string();
⋮----
if let Some(name) = doc.get("name").and_then(|n| n.as_str()) {
⋮----
if let Some(id) = doc.get("id").and_then(|i| i.as_str()) {
template_id = id.to_string();
⋮----
let saved_at = chrono::Utc::now().to_rfc3339();
⋮----
firestore.update_doc(&path, &template_id, &final_doc).await
⋮----
if let Some(obj) = full_doc.as_object_mut() {
⋮----
"created_at".to_string(),
⋮----
obj.insert("github_path".to_string(), serde_json::json!("")); // Explicit empty
⋮----
firestore.set_doc(&path, &template_id, &full_doc).await
⋮----
.to_string(),
⋮----
pub async fn quick_load_template(
⋮----
.get_doc::<serde_json::Value>(&format!("user_templates/{}/items", user_id), &template_id)
⋮----
let content = doc.get("content").cloned();
⋮----
Json(serde_json::json!({ "success": false, "error": "Not found" })),
```

## File: src/services/mod.rs
```rust
pub mod report_service;
```

## File: src/services/report_service.rs
```rust
use crate::error::ApiError;
⋮----
use axur_core::api::report::fetch_full_report;
⋮----
use axur_core::report::OfflineAssets;
⋮----
use std::time::Instant;
use uuid::Uuid;
⋮----
pub struct TenantResponse {
⋮----
pub struct GenerateReportRequest {
⋮----
fn default_language() -> String {
"es".to_string()
⋮----
pub struct GenerateReportResponse {
⋮----
pub struct ReportService;
⋮----
impl ReportService {
⋮----
pub async fn generate_report(
⋮----
let report_data = match fetch_full_report(
⋮----
payload.story_tag.clone(),
⋮----
let error_code = classify_error(&e.to_string());
⋮----
return Ok(GenerateReportResponse {
⋮----
message: e.to_string(),
error_code: Some(error_code.code()),
error_message: Some(get_user_friendly_message(&error_code)),
⋮----
let language = match payload.language.to_lowercase().as_str() {
⋮----
.or_else(|_| Translations::load("en"))
.map_err(|e| ApiError::Internal(format!("Failed to load translations: {}", e)))?;
⋮----
let dict = get_dictionary(language);
⋮----
.iter()
.filter_map(|s| s.canvas_json.clone())
.collect();
if !slides.is_empty() {
⋮----
custom_template_slides = Some(slides);
⋮----
if custom_template_slides.is_none() {
⋮----
let doc_path = format!("user_templates/{}/items", user_id);
let doc_id = uuid.to_string();
⋮----
if let Some(path) = doc.get("github_path").and_then(|s| s.as_str()) {
⋮----
let html = if payload.use_plugins && custom_template_slides.is_none() {
let theme_mode = match payload.theme.as_deref() {
⋮----
.with_theme(theme_mode)
.disable_plugins(payload.disabled_plugins.clone().unwrap_or_default());
⋮----
generate_report_with_plugins(
⋮----
Some(&offline_assets),
Some(config),
⋮----
generate_full_report_html(
⋮----
Ok(GenerateReportResponse {
⋮----
html: Some(html),
company_name: Some(report_data.company_name),
message: "Report generated successfully".into(),
⋮----
pub fn classify_error(error: &str) -> ErrorCode {
let lower = error.to_lowercase();
if lower.contains("timeout") {
if lower.contains("dark") || lower.contains("threat") {
⋮----
} else if lower.contains("cors") {
⋮----
} else if lower.contains("token") || lower.contains("expired") {
⋮----
} else if lower.contains("tenant") || lower.contains("not found") {
⋮----
} else if lower.contains("rate") || lower.contains("limit") {
⋮----
} else if lower.contains("dns") {
⋮----
} else if lower.contains("ssl") || lower.contains("tls") || lower.contains("certificate") {
⋮----
error_codes::system::internal_error().with_context(error.to_string())
⋮----
pub fn get_user_friendly_message(code: &ErrorCode) -> String {
match code.code().as_str() {
"AUTH-001" => "Credenciales inválidas. Verifica tu email y contraseña.".into(),
"AUTH-002" => "Código 2FA incorrecto. Intenta de nuevo.".into(),
"AUTH-003" => "Tu sesión ha expirado. Por favor, inicia sesión nuevamente.".into(),
"AUTH-004" => "No hay sesión activa. Por favor, inicia sesión.".into(),
"API-001" => "El token de Axur ha expirado. Reconecta tu cuenta.".into(),
"API-002" => "El tenant seleccionado no fue encontrado.".into(),
"API-003" => "Demasiadas solicitudes. Espera unos minutos e intenta de nuevo.".into(),
"RPT-001" => "No hay datos en el período seleccionado.".into(),
"RPT-004" => "Rango de fechas inválido o tenant no especificado.".into(),
⋮----
"La búsqueda en Dark Web excedió el tiempo de espera. Intenta nuevamente.".into()
⋮----
"TI-002" => "El servicio de Threat Intelligence no está disponible temporalmente.".into(),
"NET-001" => "Error de CORS. Contacta al administrador.".into(),
"NET-002" => "Timeout de conexión. Verifica tu conexión a internet.".into(),
"SYS-001" => "Error interno del servidor. Si persiste, contacta soporte.".into(),
_ => "Ha ocurrido un error inesperado.".into(),
```

## File: src/utils/coords.rs
```rust
pub fn px_to_emu(px: f64) -> i64 {
(px * 9525.0).round() as i64
⋮----
pub fn emu_to_px(emu: i64) -> f64 {
```

## File: src/utils/mod.rs
```rust
pub mod coords;
```

## File: src/error.rs
```rust
use serde_json::json;
⋮----
pub enum ApiError {
⋮----
impl IntoResponse for ApiError {
fn into_response(self) -> Response {
⋮----
let body = Json(json!({
⋮----
(status, body).into_response()
⋮----
fn from(err: anyhow::Error) -> Self {
ApiError::Internal(err.to_string())
⋮----
fn from(err: reqwest::Error) -> Self {
ApiError::ExternalApi(format!("API request failed: {}", err))
```

## File: src/firebase.rs
```rust
use once_cell::sync::Lazy;
⋮----
use std::collections::HashMap;
⋮----
use tokio::sync::OnceCell;
⋮----
use base64::Engine;
⋮----
type HttpsConnector =
⋮----
type Authenticator = yup_oauth2::authenticator::Authenticator<HttpsConnector>;
⋮----
pub struct FirebaseConfig {
⋮----
impl FirebaseConfig {
pub fn from_env() -> Option<Self> {
let project_id = std::env::var("FIREBASE_PROJECT_ID").ok()?;
Some(Self {
⋮----
api_key: std::env::var("FIREBASE_API_KEY").ok(),
service_account_json: std::env::var("FIREBASE_SERVICE_ACCOUNT_B64").ok(),
⋮----
struct RateLimiter {
⋮----
impl RateLimiter {
fn new() -> Self {
⋮----
fn reset_if_needed(&mut self) {
if self.hour_start.elapsed() > Duration::from_secs(3600) {
⋮----
fn can_read(&mut self) -> bool {
self.reset_if_needed();
⋮----
fn can_write(&mut self) -> bool {
⋮----
struct CacheEntry {
⋮----
struct Cache {
⋮----
impl Cache {
⋮----
fn get(&self, key: &str) -> Option<String> {
self.entries.get(key).and_then(|entry| {
⋮----
Some(entry.data.clone())
⋮----
fn set(&mut self, key: String, data: String, ttl_secs: u64) {
self.entries.insert(
⋮----
fn invalidate(&mut self, key: &str) {
self.entries.remove(key);
⋮----
fn invalidate_prefix(&mut self, prefix: &str) {
self.entries.retain(|k, _| !k.starts_with(prefix));
⋮----
pub struct FirestoreClient {
⋮----
impl FirestoreClient {
pub fn new(config: FirebaseConfig, auth: Option<Authenticator>) -> Self {
⋮----
async fn get_token(&self) -> Result<Option<String>, FirestoreError> {
⋮----
.token(scopes)
⋮----
.map_err(|e| FirestoreError::NetworkError(format!("Auth error: {}", e)))?;
Ok(token.token().map(|s| s.to_string()))
⋮----
Ok(None)
⋮----
fn base_url(&self) -> String {
format!(
⋮----
pub async fn get_doc<T: DeserializeOwned>(
⋮----
let cache_key = format!("{}/{}", collection, doc_id);
⋮----
if let Ok(cache) = CACHE.read() {
if let Some(cached) = cache.get(&cache_key) {
⋮----
.map(Some)
.map_err(|e| FirestoreError::ParseError(format!("Cache parse error: {}", e)));
⋮----
.write()
.map_err(|e| FirestoreError::LockError(e.to_string()))?;
if !limiter.can_read() {
return Err(FirestoreError::RateLimited);
⋮----
let token = self.get_token().await?;
⋮----
let url = format!("{}/{}/{}", self.base_url(), collection, doc_id);
let mut req = self.http.get(&url);
⋮----
req = req.bearer_auth(t);
⋮----
.send()
⋮----
.map_err(|e| FirestoreError::NetworkError(e.to_string()))?;
⋮----
if res.status() == 404 {
return Ok(None);
⋮----
if !res.status().is_success() {
return Err(FirestoreError::ApiError(format!(
⋮----
.json()
⋮----
.map_err(|e| FirestoreError::ParseError(e.to_string()))?;
⋮----
let value = firestore_to_value(&doc.fields)?;
⋮----
serde_json::to_string(&value).map_err(|e| FirestoreError::ParseError(e.to_string()))?;
⋮----
if let Ok(mut cache) = CACHE.write() {
cache.set(cache_key, json.clone(), CACHE_TTL);
⋮----
.map_err(|e| FirestoreError::ParseError(e.to_string()))
⋮----
pub async fn list_docs<T: DeserializeOwned>(
⋮----
let cache_key = format!("list:{}", collection);
⋮----
let url = format!("{}/{}", self.base_url(), collection);
⋮----
.unwrap_or_default()
.into_iter()
.filter_map(|doc| firestore_to_value(&doc.fields).ok())
.collect();
⋮----
cache.set(cache_key, json, CACHE_TTL);
⋮----
.filter_map(|v| serde_json::from_value(v).ok())
⋮----
Ok(docs)
⋮----
pub async fn set_doc<T: Serialize>(
⋮----
if !limiter.can_write() {
⋮----
let fields = value_to_firestore(
&serde_json::to_value(data).map_err(|e| FirestoreError::ParseError(e.to_string()))?,
⋮----
let mut req = self.http.patch(&url);
⋮----
.json(&body)
⋮----
cache.invalidate(&format!("{}/{}", collection, doc_id));
cache.invalidate_prefix(&format!("list:{}", collection));
⋮----
Ok(())
⋮----
pub async fn delete_doc(&self, collection: &str, doc_id: &str) -> Result<(), FirestoreError> {
⋮----
let mut req = self.http.delete(&url);
⋮----
if !res.status().is_success() && res.status() != 404 {
⋮----
pub async fn update_doc<T: Serialize>(
⋮----
let base_url = format!("{}/{}/{}", self.base_url(), collection, doc_id);
let fields_map = value_to_firestore(
⋮----
if !fields_map.is_empty() {
⋮----
for key in fields_map.keys() {
⋮----
url.push_str("?");
⋮----
url.push_str("&");
⋮----
url.push_str(&format!("updateMask.fieldPaths={}", key));
⋮----
pub type Firestore = FirestoreClient;
⋮----
struct FirestoreDocument {
⋮----
struct FirestoreListResponse {
⋮----
enum FirestoreValue {
⋮----
fn firestore_to_value(
⋮----
map.insert(key.clone(), firestore_value_to_json(val)?);
⋮----
Ok(serde_json::Value::Object(map))
⋮----
fn firestore_value_to_json(val: &FirestoreValue) -> Result<serde_json::Value, FirestoreError> {
⋮----
FirestoreValue::StringValue(s) => Ok(serde_json::Value::String(s.clone())),
⋮----
let n: i64 = s.parse().unwrap_or(0);
Ok(serde_json::Value::Number(n.into()))
⋮----
FirestoreValue::DoubleValue(d) => Ok(serde_json::json!(*d)),
FirestoreValue::BooleanValue(b) => Ok(serde_json::Value::Bool(*b)),
FirestoreValue::NullValue(_) => Ok(serde_json::Value::Null),
FirestoreValue::MapValue { fields } => firestore_to_value(fields),
⋮----
let arr: Result<Vec<_>, _> = values.iter().map(firestore_value_to_json).collect();
Ok(serde_json::Value::Array(arr?))
⋮----
FirestoreValue::TimestampValue(s) => Ok(serde_json::Value::String(s.clone())),
⋮----
fn value_to_firestore(
⋮----
fields.insert(k.clone(), json_to_firestore_value(v)?);
⋮----
Ok(fields)
⋮----
_ => Err(FirestoreError::ParseError("Expected object".to_string())),
⋮----
fn json_to_firestore_value(val: &serde_json::Value) -> Result<FirestoreValue, FirestoreError> {
⋮----
serde_json::Value::Null => Ok(FirestoreValue::NullValue(())),
serde_json::Value::Bool(b) => Ok(FirestoreValue::BooleanValue(*b)),
⋮----
if let Some(i) = n.as_i64() {
Ok(FirestoreValue::IntegerValue(i.to_string()))
} else if let Some(f) = n.as_f64() {
Ok(FirestoreValue::DoubleValue(f))
⋮----
Ok(FirestoreValue::IntegerValue("0".to_string()))
⋮----
serde_json::Value::String(s) => Ok(FirestoreValue::StringValue(s.clone())),
⋮----
let values: Result<Vec<_>, _> = arr.iter().map(json_to_firestore_value).collect();
Ok(FirestoreValue::ArrayValue { values: values? })
⋮----
Ok(FirestoreValue::MapValue { fields })
⋮----
pub enum FirestoreError {
⋮----
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
⋮----
Self::NetworkError(e) => write!(f, "Network error: {}", e),
Self::ApiError(e) => write!(f, "API error: {}", e),
Self::ParseError(e) => write!(f, "Parse error: {}", e),
Self::RateLimited => write!(f, "Rate limited - quota exceeded"),
Self::NotConfigured => write!(f, "Firebase not configured"),
Self::LockError(e) => write!(f, "Lock error: {}", e),
⋮----
// ========================
// GLOBAL CLIENT
⋮----
/// Initialize the global Firestore client (async)
pub async fn init_global() {
⋮----
pub async fn init_global() {
⋮----
// Try to initialize auth if service account is present
⋮----
match base64::engine::general_purpose::STANDARD.decode(b64) {
⋮----
// Similar implementation to google_services.rs
⋮----
.build()
⋮----
auth = Some(authenticator);
⋮----
Some(FirestoreClient::new(config, auth))
⋮----
if FIRESTORE_CLIENT.set(client).is_err() {
⋮----
/// Get the global Firestore client
pub fn get_firestore() -> Option<&'static FirestoreClient> {
⋮----
pub fn get_firestore() -> Option<&'static FirestoreClient> {
FIRESTORE_CLIENT.get().and_then(|opt| opt.as_ref())
```

## File: src/github_storage.rs
```rust
use reqwest::Client;
use serde::Deserialize;
⋮----
use std::collections::HashMap;
use std::sync::RwLock;
⋮----
pub struct GitHubStorageConfig {
⋮----
impl GitHubStorageConfig {
⋮----
pub fn from_env() -> Option<Self> {
Some(Self {
token: std::env::var("GITHUB_TOKEN").ok()?,
owner: std::env::var("GITHUB_OWNER").unwrap_or_else(|_| "maisonnat".to_string()),
⋮----
.unwrap_or_else(|_| "axur-logs-private".to_string()),
⋮----
struct CacheEntry {
⋮----
pub struct GitHubStorage {
⋮----
impl GitHubStorage {
⋮----
pub fn new(config: GitHubStorageConfig) -> Self {
⋮----
GitHubStorageConfig::from_env().map(Self::new)
⋮----
pub fn hash_user_id(user_id: &str) -> String {
⋮----
hasher.update(user_id.as_bytes());
format!("{:x}", hasher.finalize())[..16].to_string()
⋮----
fn user_path(&self, user_hash: &str, key: &str) -> String {
format!("users/{}/{}", user_hash, key)
⋮----
pub async fn save(&self, path: &str, content: &str, message: &str) -> Result<(), String> {
⋮----
let existing_sha = self.get_file_sha(path).await.ok();
⋮----
let url = format!(
⋮----
let encoded = BASE64.encode(content.as_bytes());
⋮----
.put(&url)
.header("Authorization", format!("Bearer {}", self.config.token))
.header("Accept", "application/vnd.github.v3+json")
.header("User-Agent", "axur-backend")
.json(&body)
.send()
⋮----
.map_err(|e| format!("Request failed: {}", e))?;
⋮----
if resp.status().is_success() {
⋮----
if let Ok(mut cache) = self.cache.write() {
cache.remove(path);
⋮----
Ok(())
⋮----
let status = resp.status();
let text = resp.text().await.unwrap_or_default();
Err(format!("GitHub save failed ({}): {}", status, text))
⋮----
pub async fn load(&self, path: &str) -> Result<String, String> {
self.load_with_ttl(path, Duration::from_secs(3600)).await
⋮----
pub async fn load_with_ttl(&self, path: &str, ttl: Duration) -> Result<String, String> {
⋮----
if let Ok(cache) = self.cache.read() {
⋮----
.get(path)
.map(|e| (e.data.clone(), e.etag.clone(), e.expires_at))
⋮----
if *expires_at > Instant::now() && ttl.as_secs() > 0 {
return Ok(data.clone());
⋮----
.get(&url)
⋮----
.header("User-Agent", "axur-backend");
⋮----
req = req.header("If-None-Match", etag);
⋮----
if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
⋮----
return Ok(data);
⋮----
.headers()
.get("etag")
.and_then(|v| v.to_str().ok())
.map(|s| s.to_string());
⋮----
.json()
⋮----
.map_err(|e| format!("Parse failed: {}", e))?;
⋮----
.decode(file.content.replace('\n', ""))
.map_err(|e| format!("Base64 decode failed: {}", e))?;
⋮----
let data = String::from_utf8(content).map_err(|e| format!("UTF8 failed: {}", e))?;
⋮----
// Update cache with new data and ETag
⋮----
cache.insert(
path.to_string(),
⋮----
data: data.clone(),
⋮----
Ok(data)
} else if resp.status() == reqwest::StatusCode::NOT_FOUND {
Err("File not found".to_string())
⋮----
Err(format!("GitHub load failed: {}", resp.status()))
⋮----
pub async fn delete(&self, path: &str, message: &str) -> Result<(), String> {
let sha = self.get_file_sha(path).await?;
⋮----
.delete(&url)
⋮----
Err(format!("GitHub delete failed: {}", resp.status()))
⋮----
pub async fn list(&self, path: &str) -> Result<Vec<String>, String> {
⋮----
Ok(items.into_iter().map(|e| e.name).collect())
} else if resp.status() == 404 {
Ok(vec![])
⋮----
Err(format!("GitHub list failed: {}", resp.status()))
⋮----
async fn get_file_sha(&self, path: &str) -> Result<String, String> {
⋮----
Ok(file.sha)
⋮----
pub async fn save_template(
⋮----
let path = self.user_path(&user_hash, &format!("templates/{}.json", template_name));
self.save(
⋮----
&format!("Save template: {}", template_name),
⋮----
pub async fn load_template(
⋮----
self.load(&path).await
⋮----
pub async fn list_templates(&self, user_id: &str) -> Result<Vec<String>, String> {
⋮----
let path = self.user_path(&user_hash, "templates");
self.list(&path).await
⋮----
pub async fn delete_template(&self, user_id: &str, template_name: &str) -> Result<(), String> {
⋮----
self.delete(&path, &format!("Delete template: {}", template_name))
⋮----
pub async fn is_user_allowed(&self, email: &str) -> Result<bool, String> {
⋮----
.load_with_ttl("system/allowed_users.json", Duration::ZERO)
⋮----
serde_json::from_str(&data).map_err(|e| format!("Parse failed: {}", e))?;
Ok(users.iter().any(|u| u.email.eq_ignore_ascii_case(email)))
⋮----
pub async fn get_user_role(&self, email: &str) -> Result<Option<String>, String> {
⋮----
Ok(users
.iter()
.find(|u| u.email.eq_ignore_ascii_case(email))
.map(|u| u.role.clone()))
⋮----
pub async fn is_admin(&self, email: &str) -> Result<bool, String> {
match self.get_user_role(email).await? {
Some(role) => Ok(role == "admin"),
None => Ok(false),
⋮----
pub struct AllowedUser {
⋮----
struct GitHubFile {
⋮----
struct GitHubDirEntry {
⋮----
use std::sync::OnceLock;
⋮----
pub fn get_github_storage() -> Option<&'static GitHubStorage> {
⋮----
.get_or_init(|| GitHubStorage::from_env())
.as_ref()
⋮----
mod tests {
⋮----
fn test_hash_user_id() {
⋮----
assert_eq!(hash.len(), 16);
⋮----
assert_eq!(hash, GitHubStorage::hash_user_id("test@example.com"));
```

## File: src/google_services.rs
```rust
use serde::Deserialize;
use std::sync::Arc;
⋮----
type HttpConnector = hyper_util::client::legacy::connect::HttpConnector;
⋮----
pub struct GoogleServices {
⋮----
struct Presentation {
⋮----
struct Slide {
⋮----
struct ThumbnailResponse {
⋮----
impl GoogleServices {
⋮----
pub async fn from_env() -> std::result::Result<Self, Box<dyn std::error::Error + Send + Sync>> {
⋮----
std::env::var("GOOGLE_CLIENT_ID").map_err(|_| "GOOGLE_CLIENT_ID not set")?;
⋮----
std::env::var("GOOGLE_CLIENT_SECRET").map_err(|_| "GOOGLE_CLIENT_SECRET not set")?;
⋮----
std::env::var("GOOGLE_REFRESH_TOKEN").map_err(|_| "GOOGLE_REFRESH_TOKEN not set")?;
⋮----
std::fs::write(temp_path, json_content.to_string())?;
⋮----
/// Create from token.json file (for local development)
    pub async fn new(
⋮----
pub async fn new(
⋮----
// Build HTTPS connector with rustls using google_drive3's bundled types
⋮----
.with_native_roots()?
.https_only()
.enable_http1()
.build();
⋮----
// Build hyper client with hyper_util (hyper v1 style)
⋮----
.build(https_connector);
⋮----
// Build authenticator for Authorized User
⋮----
.build()
⋮----
// Create Drive hub with scopes
let drive = DriveHub::new(hyper_client, auth.clone());
⋮----
// Rate Limiter: 4 req/sec
let quota = Quota::per_second(nonzero!(4u32));
⋮----
// Reqwest client for manual Slides API calls
// Increase timeout for large PPTX uploads (e.g. 32MB takes >30s)
⋮----
.timeout(std::time::Duration::from_secs(300))
.build()?;
⋮----
Ok(GoogleServices {
⋮----
/// Uploads a PPTX file to Google Drive and returns the File ID.
    /// Uses direct reqwest calls instead of google-drive3 to avoid library issues.
⋮----
/// Uses direct reqwest calls instead of google-drive3 to avoid library issues.
    pub async fn upload_pptx(&self, name: &str, data: Vec<u8>) -> Result<String, String> {
⋮----
pub async fn upload_pptx(&self, name: &str, data: Vec<u8>) -> Result<String, String> {
// Shared folder constant removed - uploading to root
⋮----
// Get access token from authenticator
⋮----
let access_token = token.token().ok_or("No token string")?;
⋮----
.post(init_url)
.bearer_auth(access_token)
.header("Content-Type", "application/json; charset=UTF-8")
.json(&metadata)
.send()
⋮----
.map_err(|e| format!("Init request failed: {}", e))?;
⋮----
if !init_resp.status().is_success() {
let status = init_resp.status();
let body = init_resp.text().await.unwrap_or_default();
⋮----
return Err(format!(
⋮----
// Get the upload URI from the Location header
⋮----
.headers()
.get("location")
.and_then(|h| h.to_str().ok())
.ok_or("No upload location returned")?
.to_string();
⋮----
.put(&upload_uri)
.header(
⋮----
.header("Content-Length", data.len().to_string())
.body(data)
⋮----
.map_err(|e| format!("Upload request failed: {}", e))?;
⋮----
if !upload_resp.status().is_success() {
let status = upload_resp.status();
let body = upload_resp.text().await.unwrap_or_default();
return Err(format!("Drive Upload Error ({}): {}", status, body));
⋮----
.json()
⋮----
.map_err(|e| format!("Failed to parse upload response: {}", e))?;
⋮----
.get("id")
.and_then(|id| id.as_str())
.map(|s| s.to_string())
.ok_or_else(|| "No file ID in response".to_string())
⋮----
pub async fn delete_file(&self, file_id: &str) -> Result<(), String> {
⋮----
.files()
.delete(file_id)
.add_scope("https://www.googleapis.com/auth/drive.file")
.doit()
⋮----
.map_err(|e| format!("Drive Delete Error: {}", e))?;
Ok(())
⋮----
pub async fn generate_previews(&self, file_id: &str) -> Result<Vec<String>, String> {
⋮----
.token(scopes)
⋮----
.map_err(|e| format!("Token Error: {}", e))?;
⋮----
let url = format!("https://slides.googleapis.com/v1/presentations/{}", file_id);
⋮----
.get(&url)
⋮----
.map_err(|e| format!("Slides API Request Error: {}", e))?;
⋮----
if !resp.status().is_success() {
let status = resp.status();
let body = resp.text().await.unwrap_or_default();
return Err(format!("Slides API Error ({}): {}", status, body));
⋮----
.map_err(|e| format!("Slides JSON Parse Error: {}", e))?;
⋮----
.ok_or("No slides found in presentation")?;
⋮----
let page_id = slide.object_id.ok_or("Slide has no ID")?;
⋮----
self.limiter.until_ready().await;
⋮----
let thumb_url = format!(
⋮----
.get(&thumb_url)
⋮----
.map_err(|e| format!("Thumbnail Request Error: {}", e))?;
⋮----
if !thumb_resp.status().is_success() {
let status = thumb_resp.status();
⋮----
.map_err(|e| format!("Thumbnail JSON Parse Error for slide {}: {}", page_id, e))?;
⋮----
urls.push(content_url);
⋮----
Ok(urls)
⋮----
pub async fn fetch_images_as_base64(&self, urls: Vec<String>) -> Result<Vec<String>, String> {
use base64::Engine;
let mut data_urls = Vec::with_capacity(urls.len());
⋮----
for (idx, url) in urls.iter().enumerate() {
⋮----
.get(url)
⋮----
.map_err(|e| format!("Image fetch error: {}", e))?;
⋮----
.get("Retry-After")
.and_then(|v| v.to_str().ok())
.and_then(|s| s.parse::<u64>().ok())
.unwrap_or(5);
⋮----
if !status.is_success() {
⋮----
// Get content type for data URL
⋮----
.get("content-type")
⋮----
.unwrap_or("image/png")
⋮----
.bytes()
⋮----
.map_err(|e| format!("Image {} bytes error: {}", idx + 1, e))?;
⋮----
let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
let data_url = format!("data:{};base64,{}", content_type, b64);
⋮----
data_urls.push(data_url);
⋮----
Ok(data_urls)
⋮----
pub async fn create_presentation(&self, title: &str) -> Result<String, String> {
⋮----
.post(url)
⋮----
.json(&body)
⋮----
.map_err(|e| format!("Create Presentation Request Error: {}", e))?;
⋮----
.map_err(|e| format!("Parse Presentation Response Error: {}", e))?;
⋮----
.get("presentationId")
⋮----
.ok_or_else(|| "No presentation ID in response".to_string())
⋮----
pub async fn add_slides_batch(
⋮----
for (index, slide) in slides.iter().enumerate() {
let slide_id = format!("slide_{}", index);
⋮----
requests.push(serde_json::json!({
⋮----
if !slide.title.is_empty() {
let title_id = format!("{}_title", slide_id);
⋮----
if !slide.body.is_empty() {
let body_id = format!("{}_body", slide_id);
let body_text = slide.body.join("\n\n");
⋮----
if requests.is_empty() {
return Ok(());
⋮----
let url = format!(
⋮----
.post(&url)
⋮----
.map_err(|e| format!("BatchUpdate Request Error: {}", e))?;
⋮----
let error_body = resp.text().await.unwrap_or_default();
⋮----
pub fn get_presentation_url(&self, presentation_id: &str) -> String {
format!(
⋮----
pub struct SlideData {
```

## File: src/injector.rs
```rust
use crate::utils::coords::px_to_emu;
⋮----
pub struct SlideEdit {
⋮----
pub struct InjectionRequest {
⋮----
pub fn inject_edits(original_pptx: &[u8], edits: Vec<SlideEdit>) -> Result<Vec<u8>, anyhow::Error> {
⋮----
.entry(edit.slide_index)
.or_default()
.push(edit);
⋮----
for i in 0..archive.len() {
let mut file = archive.by_index(i)?;
let name = file.name().to_string();
⋮----
.compression_method(file.compression())
.unix_permissions(file.unix_mode().unwrap_or(0o644));
⋮----
file.read_to_end(&mut content)?;
⋮----
if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
⋮----
.trim_start_matches("ppt/slides/slide")
.trim_end_matches(".xml");
⋮----
if let Some(slide_edits) = edits_by_slide.get(&idx) {
⋮----
let content_str = String::from_utf8(content.clone())?;
if content_str.contains("</p:spTree>") {
⋮----
new_shapes_xml.push_str(&create_textbox_xml(edit));
⋮----
let modified_xml = content_str.replace(
⋮----
&format!("{}{}", new_shapes_xml, "</p:spTree>"),
⋮----
content = modified_xml.into_bytes();
⋮----
zip_writer.start_file(name, options)?;
zip_writer.write_all(&content)?;
⋮----
let cursor = zip_writer.finish()?;
Ok(cursor.into_inner().to_vec())
⋮----
fn create_textbox_xml(edit: &SlideEdit) -> String {
let x_emu = px_to_emu(edit.x);
let y_emu = px_to_emu(edit.y);
let cx_emu = px_to_emu(edit.width);
let cy_emu = px_to_emu(edit.height);
⋮----
.clone()
.map(|k| format!("Placeholder_{}", k))
.unwrap_or(format!("Placeholder_{}", shape_id));
⋮----
format!(
```

## File: src/lib.rs
```rust
pub mod error;
pub mod firebase;
pub mod github_storage;
pub mod google_services;
pub mod injector;
pub mod middleware;
pub mod queue;
pub mod routes;
pub mod services;
pub mod utils;
⋮----
pub use crate::routes::create_router;
```

## File: src/main.rs
```rust
use std::net::SocketAddr;
⋮----
use axur_backend::create_router;
⋮----
async fn main() -> Result<(), Box<dyn std::error::Error>> {
⋮----
dotenv::dotenv().ok();
⋮----
.with_max_level(tracing::Level::DEBUG)
.with_target(false)
.init();
⋮----
let google_services = if std::env::var("GOOGLE_CLIENT_ID").is_ok() {
⋮----
Some(std::sync::Arc::new(service))
⋮----
if std::path::Path::new(token_path).exists() {
⋮----
let app = create_router(app_state);
⋮----
.unwrap_or_else(|_| "3001".to_string())
⋮----
.unwrap_or(3001);
⋮----
Ok(())
```

## File: src/pptx_injector.rs
```rust

```

## File: src/queue.rs
```rust
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;
⋮----
pub struct TokenBucket {
⋮----
impl TokenBucket {
⋮----
pub fn new(capacity: u32, refill_rate: u32, refill_interval: Duration) -> Self {
⋮----
fn now_ms() -> u64 {
⋮----
.duration_since(std::time::UNIX_EPOCH)
.unwrap_or(Duration::ZERO)
.as_millis() as u64
⋮----
fn refill(&self) {
⋮----
let last = self.last_refill.load(Ordering::Relaxed);
let interval_ms = self.refill_interval.as_millis() as u64;
⋮----
let current = self.tokens.load(Ordering::Relaxed);
let new_tokens = (current + tokens_to_add).min(self.capacity);
self.tokens.store(new_tokens, Ordering::Relaxed);
⋮----
self.last_refill.store(new_last, Ordering::Relaxed);
⋮----
pub fn try_acquire(&self) -> bool {
self.refill();
⋮----
.compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::Relaxed)
.is_ok()
⋮----
pub fn time_until_available(&self) -> Duration {
⋮----
if self.tokens.load(Ordering::Relaxed) > 0 {
⋮----
pub fn available(&self) -> u32 {
⋮----
self.tokens.load(Ordering::Relaxed)
⋮----
pub enum ApiType {
⋮----
impl ApiType {
⋮----
pub fn rate_limit(&self) -> (u32, u32, Duration) {
⋮----
pub enum JobType {
⋮----
pub enum JobStatus {
⋮----
pub struct QueueJob {
⋮----
impl QueueJob {
pub fn new(user_id: String, job_type: JobType, api_type: ApiType) -> Self {
⋮----
id: Uuid::new_v4().to_string(),
⋮----
pub struct RequestQueue {
⋮----
impl RequestQueue {
⋮----
pub fn new() -> Self {
⋮----
let (capacity, refill_rate, interval) = api_type.rate_limit();
rate_limiters.insert(api_type, TokenBucket::new(capacity, refill_rate, interval));
⋮----
pub async fn submit(&self, mut job: QueueJob) -> String {
let mut jobs = self.jobs.write().await;
let position = jobs.len();
⋮----
let job_id = job.id.clone();
jobs.push_back(job);
⋮----
pub async fn get_job(&self, job_id: &str) -> Option<QueueJob> {
⋮----
let jobs = self.jobs.read().await;
for (idx, job) in jobs.iter().enumerate() {
⋮----
let mut job = job.clone();
⋮----
return Some(job);
⋮----
let completed = self.completed_jobs.read().await;
if let Some(job) = completed.get(job_id) {
return Some(job.clone());
⋮----
pub async fn queue_length(&self) -> usize {
self.jobs.read().await.len()
⋮----
pub fn estimate_wait_time(&self, position: usize, api_type: ApiType) -> Duration {
if let Some(bucket) = self.rate_limiters.get(&api_type) {
let available = bucket.available() as usize;
⋮----
let (_, refill_rate, interval) = api_type.rate_limit();
let jobs_to_wait = position.saturating_sub(available);
let intervals_needed = (jobs_to_wait as f64 / refill_rate as f64).ceil() as u64;
⋮----
pub fn can_process(&self, api_type: ApiType) -> bool {
⋮----
.get(&api_type)
.map(|b| b.try_acquire())
.unwrap_or(true)
⋮----
pub async fn pop_ready(&self) -> Option<QueueJob> {
⋮----
for i in 0..jobs.len() {
⋮----
.get(&jobs[i].api_type)
⋮----
return jobs.remove(i);
⋮----
pub async fn complete(&self, mut job: QueueJob, result: serde_json::Value) {
⋮----
.write()
⋮----
.insert(job.id.clone(), job);
⋮----
pub async fn fail(&self, mut job: QueueJob, error: String) {
⋮----
pub async fn user_jobs(&self, user_id: &str) -> Vec<QueueJob> {
⋮----
.iter()
.filter(|j| j.user_id == user_id)
.cloned()
.collect();
⋮----
result.extend(completed.values().filter(|j| j.user_id == user_id).cloned());
⋮----
impl Default for RequestQueue {
fn default() -> Self {
⋮----
use std::sync::OnceLock;
⋮----
pub fn get_queue() -> &'static RequestQueue {
QUEUE.get_or_init(RequestQueue::new)
⋮----
pub struct QueueStatusResponse {
⋮----
fn from(job: &QueueJob) -> Self {
⋮----
job_id: job.id.clone(),
status: "queued".to_string(),
position: Some(*position),
⋮----
status: "processing".to_string(),
⋮----
status: "completed".to_string(),
⋮----
result: Some(result.clone()),
⋮----
status: "failed".to_string(),
⋮----
error: Some(error.clone()),
⋮----
use std::sync::atomic::AtomicBool;
⋮----
pub fn start_worker() {
if WORKER_RUNNING.swap(true, Ordering::SeqCst) {
⋮----
let queue = get_queue();
⋮----
if let Some(mut job) = queue.pop_ready().await {
⋮----
let result = process_job(&job).await;
⋮----
queue.store_completed(job).await;
⋮----
async fn process_job(job: &QueueJob) -> Result<serde_json::Value, String> {
⋮----
Ok(serde_json::json!({
⋮----
pub async fn store_completed(&self, job: QueueJob) {
let mut completed = self.completed_jobs.write().await;
completed.insert(job.id.clone(), job);
⋮----
if completed.len() > 100 {
⋮----
.take(completed.len() - 100)
.map(|(k, _)| k.clone())
⋮----
completed.remove(&key);
⋮----
mod tests {
⋮----
fn test_token_bucket_acquire() {
⋮----
assert!(bucket.try_acquire());
⋮----
assert!(!bucket.try_acquire());
⋮----
fn test_token_bucket_available() {
⋮----
assert_eq!(bucket.available(), 10);
⋮----
bucket.try_acquire();
⋮----
assert_eq!(bucket.available(), 8);
⋮----
async fn test_queue_submit() {
⋮----
"user-1".to_string(),
⋮----
tenant_id: "t1".to_string(),
⋮----
let job_id = queue.submit(job).await;
assert!(!job_id.is_empty());
assert_eq!(queue.queue_length().await, 1);
```

## File: Cargo.toml
```toml
[package]
name = "axur-backend"
version.workspace = true
edition.workspace = true

[[bin]]
name = "axur-backend"
path = "src/main.rs"

[lib]
path = "src/lib.rs"

[dependencies]
axur-core = { path = "../core" }

# Web framework
axum = { version = "0.7", features = ["macros", "multipart"] }
tokio.workspace = true

# Serialization
serde.workspace = true
serde_json.workspace = true

# HTTP client (for proxying to Axur API)
reqwest = { version = "0.11", default-features = false, features = ["json", "multipart", "rustls-tls"] }

# Error handling
anyhow.workspace = true
thiserror = "1.0"

# Middleware and security
tower-http = { version = "0.5", features = ["cors", "trace", "fs"] }
tower = { version = "0.4", features = ["util"] }

# Cookie handling for httpOnly sessions
cookie = { version = "0.18", features = ["secure"] }
axum-extra = { version = "0.9", features = ["typed-header", "multipart", "cookie"] }

# SSE streaming
futures = "0.3"
tokio-stream = "0.1"
async-stream = "0.3"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
dotenv = "0.15"

# Remote logging
base64 = "0.21"
chrono = { version = "0.4", features = ["serde"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "sqlite", "macros", "chrono", "uuid"] }

# PPTX parsing
zip = "0.6"
sha2 = { version = "0.10.9", default-features = false }

# New dependencies
rand = "0.8"
jsonwebtoken = "9.2"
bcrypt = "0.15"
tempfile = "3.10"

# Google APIs (google-drive3 bundles hyper, hyper-rustls)
google-drive3 = "*"
# Need service-account feature for ServiceAccountAuthenticator
yup-oauth2 = { version = "12", features = ["service-account", "hyper-rustls"] }
governor = "0.6"
nonzero_ext = "0.3" # Helper for governor
once_cell = "1.19" # Lazy statics for Firebase client
clap = { version = "4.4", features = ["derive"] }
```