//! # Auth Middleware
//!
//! Reads the `token` cookie from incoming requests, validates the
//! JWT, and injects an `AuthUser` struct into Axum's request
//! extensions. Downstream handlers and middleware can then extract
//! the current user cheaply with `Extension<AuthUser>`.
//!
//! ## Design decisions
//!
//! **Cookie-based auth** (not `Authorization` header):
//! Since we're server-rendering HTML with Tera + HTMX, there's no
//! JavaScript managing tokens. Cookies are sent automatically on
//! every request, including HTMX partials and full page loads.
//! The cookie is `HttpOnly` + `SameSite=Lax` to prevent XSS and
//! CSRF attacks.
//!
//! **Optional vs required auth**:
//! This middleware does NOT reject unauthenticated requests — it
//! just sets `Option<AuthUser>` in extensions. Routes that require
//! auth use the `require_auth` extractor or the `role_guard`
//! middleware on top of this. This way, pages like the home page
//! can optionally show a "Welcome, <name>" without blocking
//! anonymous users.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::AppState;
use crate::services::auth_service;

/// Lightweight struct injected into request extensions after
/// successful JWT validation. Handlers extract this instead of
/// re-parsing the token.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub role: String,
}

/// Middleware function. Axum calls this for every request on
/// routes where it's installed.
///
/// Flow:
/// 1. Read the `token` cookie from the request.
/// 2. If present, validate the JWT and extract claims.
/// 3. If valid, insert `Some(AuthUser)` into extensions.
/// 4. If absent or invalid, insert `None` — do NOT reject.
/// 5. Call `next.run(request)` to continue the chain.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let auth_user = jar
        .get("token")
        .and_then(|cookie| {
            auth_service::validate_token(cookie.value(), &state.config).ok()
        })
        .and_then(|claims| {
            // Parse the UUID from the `sub` claim. If it's malformed,
            // treat the token as invalid and continue as anonymous.
            Uuid::parse_str(&claims.sub).ok().map(|id| AuthUser {
                id,
                role: claims.role,
            })
        });

    // Insert the auth user (or None) into extensions so handlers
    // can access it via `Extension<Option<AuthUser>>`.
    request.extensions_mut().insert(auth_user.clone());

    next.run(request).await
}

// ── Extractor Helpers ────────────────────────────────────────

/// Convenience extractor that requires authentication. Use this
/// in handler signatures to reject unauthenticated requests with
/// a redirect to the login page.
///
/// # Example
/// ```ignore
/// async fn checkout(user: RequireAuth) -> impl IntoResponse {
///     // user.0 is the AuthUser — guaranteed to exist here.
/// }
/// ```
pub struct RequireAuth(pub AuthUser);

#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = crate::errors::AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Option<AuthUser>>()
            .cloned()
            .flatten()
            .map(RequireAuth)
            .ok_or(crate::errors::AppError::Unauthorized)
    }
}