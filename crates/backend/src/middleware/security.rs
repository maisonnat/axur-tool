//! Security middleware and utilities

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;

/// Cookie name for the auth token
pub const AUTH_COOKIE_NAME: &str = "axur_session";
/// Cookie name for the user ID (email)
pub const AUTH_USER_COOKIE_NAME: &str = "axur_user";

/// Extract token from httpOnly cookie
pub fn get_token_from_cookies(jar: &CookieJar) -> Option<String> {
    jar.get(AUTH_COOKIE_NAME).map(|c| c.value().to_string())
}

/// Extract user ID from httpOnly cookie
pub fn get_user_from_cookies(jar: &CookieJar) -> Option<String> {
    jar.get(AUTH_USER_COOKIE_NAME)
        .map(|c| c.value().to_string())
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

    // Extract user ID (email) from cookie and insert into request extensions
    let mut request = request;
    if let Some(user_id) = get_user_from_cookies(&jar) {
        request.extensions_mut().insert(user_id);
    } else {
        // If we have a token but no user cookie, we can't identify the user for templates
        // We could error out, or let it slide and fail in handlers.
        // Failing here is safer/clearer.
        tracing::warn!("Auth token present but axur_user cookie missing");
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(request).await)
}
