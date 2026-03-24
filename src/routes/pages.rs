//! # Page Routes
//!
//! Serves full server-rendered HTML pages. These are the "top-level"
//! pages that a user navigates to — each extends `base.html` and
//! renders a complete document with `<head>`, nav, content, footer.
//!
//! HTMX partial routes (fragments swapped into existing pages) live
//! in their respective domain modules (e.g. `products::routes()`
//! serves both the full product list page and the search results
//! partial).
//!
//! ## Auth-aware rendering
//! Every page handler receives `Option<AuthUser>` from the auth
//! middleware. Templates can check `{% if user %}` to show
//! logged-in UI (username, logout button, cart count) or anonymous
//! UI (login/register links).

use std::sync::Arc;

use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};

use crate::AppState;
use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::user::UserResponse;
use crate::services::auth_service;

/// Mounts page routes.
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(home_page))
        // Fallback for unmatched routes → 404 page.
        .fallback(not_found_page)
}

// ── Home Page ────────────────────────────────────────────────

/// GET / — landing page showing featured products, categories,
/// and a search bar. Auth-aware: shows user greeting if logged in.
///
/// TODO: Once product_service is implemented, fetch featured
/// products and pass them into the template context.
async fn home_page(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<Option<AuthUser>>,
) -> Result<Html<String>> {
    let mut ctx = tera::Context::new();

    // If the user is logged in, fetch their full profile for the
    // template (name, role, etc.).
    if let Some(ref au) = auth_user {
        if let Ok(user) = auth_service::get_user_by_id(&state.db, au.id).await {
            let user_resp: UserResponse = user.into();
            ctx.insert("user", &user_resp);
        }
    }

    // Placeholder: featured products will be fetched here once
    // the product service is implemented.
    // let featured = product_service::get_featured(&state.db).await?;
    // ctx.insert("featured_products", &featured);

    let html = state.tera.render("pages/home.html", &ctx)?;
    Ok(Html(html))
}

// ── 404 Page ─────────────────────────────────────────────────

/// Fallback handler for any route that doesn't match. Returns a
/// styled 404 page instead of a raw "Not Found" text response.
async fn not_found_page(
    State(state): State<Arc<AppState>>,
    axum::Extension(auth_user): axum::Extension<Option<AuthUser>>,
) -> Result<Html<String>> {
    let mut ctx = tera::Context::new();

    if let Some(ref au) = auth_user {
        if let Ok(user) = auth_service::get_user_by_id(&state.db, au.id).await {
            let user_resp: UserResponse = user.into();
            ctx.insert("user", &user_resp);
        }
    }

    let html = state.tera.render("pages/not_found.html", &ctx)?;
    Ok(Html(html))
}