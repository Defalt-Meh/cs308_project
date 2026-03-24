//! # Auth Routes
//!
//! Handles login, registration, and logout. Each action has two
//! endpoints:
//!
//! - **GET**  — renders the full page (Tera template).
//! - **POST** — processes the form submission. On success, sets
//!   the `token` cookie and redirects. On failure, re-renders the
//!   form with an error message via HTMX partial swap.
//!
//! ## Cookie policy (Requirement 16)
//! - `HttpOnly`: JavaScript cannot read the token → XSS-safe.
//! - `SameSite=Lax`: cookie is sent on same-site navigations and
//!   top-level GET cross-site requests, but not on cross-site
//!   POSTs → CSRF-resistant.
//! - `Secure`: set to true in production (HTTPS only).
//! - `Path=/`: available on all routes.
//! - `Max-Age`: matches JWT expiry from config.

use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Router,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use time::Duration;

use crate::AppState;
use crate::errors::{AppError, Result};
use crate::middleware::auth::AuthUser;
use crate::models::user::{LoginRequest, RegisterRequest};
use crate::services::auth_service;

/// Mounts all auth routes under `/auth`.
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/login", get(login_page).post(login_handler))
        .route("/auth/register", get(register_page).post(register_handler))
        .route("/auth/logout", post(logout_handler))
}

// ── Login ────────────────────────────────────────────────────

/// GET /auth/login — renders the login page.
/// If the user is already authenticated, redirects to home.
async fn login_page(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<Option<AuthUser>>,
) -> Result<Response> {
    // Already logged in → redirect to home.
    if auth_user.is_some() {
        return Ok(Redirect::to("/").into_response());
    }

    let ctx = tera::Context::new();
    let html = state.tera.render("pages/login.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// POST /auth/login — validates credentials, sets cookie, redirects.
///
/// On success: sets `token` cookie and redirects to `/`.
/// On failure: returns an HTMX-compatible error fragment that the
/// login form can swap into its error container.
async fn login_handler(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(req): Form<LoginRequest>,
) -> Result<Response> {
    // Attempt login — this returns a JWT string on success.
    let token = auth_service::login(&state.db, &state.config, &req).await?;

    // Build the auth cookie.
    let cookie = build_auth_cookie(token, state.config.jwt_expiry_hours);

    // Set the cookie and redirect to home.
    // The `HX-Redirect` header tells HTMX to do a full-page
    // navigation instead of a partial swap.
    Ok((
        jar.add(cookie),
        [("HX-Redirect", "/")],
        Redirect::to("/"),
    )
        .into_response())
}

// ── Registration ─────────────────────────────────────────────

/// GET /auth/register — renders the registration page.
async fn register_page(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<Option<AuthUser>>,
) -> Result<Response> {
    if auth_user.is_some() {
        return Ok(Redirect::to("/").into_response());
    }

    let ctx = tera::Context::new();
    let html = state.tera.render("pages/register.html", &ctx)?;
    Ok(Html(html).into_response())
}

/// POST /auth/register — creates account, sets cookie, redirects.
async fn register_handler(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(req): Form<RegisterRequest>,
) -> Result<Response> {
    // Register the user.
    let _user_id = auth_service::register(&state.db, &req).await?;

    // Automatically log them in by creating a token.
    let login_req = LoginRequest {
        email: req.email,
        password: req.password,
    };
    let token = auth_service::login(&state.db, &state.config, &login_req).await?;

    let cookie = build_auth_cookie(token, state.config.jwt_expiry_hours);

    Ok((
        jar.add(cookie),
        [("HX-Redirect", "/")],
        Redirect::to("/"),
    )
        .into_response())
}

// ── Logout ───────────────────────────────────────────────────

/// POST /auth/logout — clears the auth cookie and redirects to
/// the login page.
async fn logout_handler(jar: CookieJar) -> impl IntoResponse {
    // Remove the cookie by setting it with an expired max-age.
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(Duration::seconds(0))
        .http_only(true)
        .build();

    (
        jar.add(cookie),
        [("HX-Redirect", "/auth/login")],
        Redirect::to("/auth/login"),
    )
}

// ── Helpers ──────────────────────────────────────────────────

/// Constructs the `token` cookie with security flags.
fn build_auth_cookie(token: String, expiry_hours: i64) -> Cookie<'static> {
    Cookie::build(("token", token))
        .path("/")
        .max_age(Duration::hours(expiry_hours))
        .http_only(true)
        .same_site(SameSite::Lax)
        // TODO: set `.secure(true)` when deploying behind HTTPS.
        .build()
}