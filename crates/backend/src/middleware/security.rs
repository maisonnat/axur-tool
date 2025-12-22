//! Security middleware and utilities

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use axum_extra::extract::CookieJar;

/// Cookie name for the auth token
pub const AUTH_COOKIE_NAME: &str = "axur_session";

/// Extract token from httpOnly cookie
pub fn get_token_from_cookies(jar: &CookieJar) -> Option<String> {
    jar.get(AUTH_COOKIE_NAME).map(|c| c.value().to_string())
}

/// Middleware that requires authentication
pub async fn require_auth(
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if get_token_from_cookies(&jar).is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(request).await)
}
