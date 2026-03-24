//! # Role Guard Middleware
//!
//! Enforces role-based access control on admin routes. Runs after
//! the auth middleware, so `AuthUser` is already in request
//! extensions.
//!
//! ## Requirement 10 — Role separation
//! - `customer` — can browse, order, review, wishlist, return.
//! - `sales_manager` — pricing, discounts, invoices, revenue.
//! - `product_manager` — stock, categories, deliveries, comments.
//!
//! ## Requirement 16 — Security privileges
//! > "The various user roles have their own security privileges
//! > and they should not be mixed."
//!
//! This middleware guarantees that a customer cannot access sales
//! manager endpoints even if they forge a request to the URL.

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use crate::AppState;
use crate::middleware::auth::AuthUser;

/// Creates a middleware function that only allows users with the
/// given role to proceed. All others are redirected to home.
///
/// # Usage in router
/// ```ignore
/// use axum::middleware::from_fn_with_state;
///
/// let sales_routes = Router::new()
///     .route("/admin/sales/dashboard", get(dashboard))
///     .layer(from_fn_with_state(
///         state.clone(),
///         |s, r, n| role_guard(s, r, n, "sales_manager"),
///     ));
/// ```
///
/// # Behavior
/// - If the user is not authenticated → redirect to `/auth/login`.
/// - If the user is authenticated but has the wrong role →
///   redirect to `/` with a 303 See Other (prevents POST replays).
/// - If the user has the correct role → continue to the handler.
pub async fn role_guard(
    State(_state): State<Arc<AppState>>,
    request: Request,
    next: Next,
    required_role: &str,
) -> Response {
    // Extract the auth user set by auth_middleware.
    let auth_user = request
        .extensions()
        .get::<Option<AuthUser>>()
        .cloned()
        .flatten();

    match auth_user {
        None => {
            // Not authenticated — redirect to login.
            tracing::warn!(
                path = %request.uri().path(),
                "Unauthenticated access to protected route"
            );
            Redirect::to("/auth/login").into_response()
        }
        Some(ref user) if user.role != required_role => {
            // Authenticated but wrong role.
            tracing::warn!(
                user_id = %user.id,
                user_role = %user.role,
                required_role = %required_role,
                path = %request.uri().path(),
                "Forbidden: role mismatch"
            );
            Redirect::to("/").into_response()
        }
        Some(_) => {
            // Correct role — proceed.
            next.run(request).await
        }
    }
}

// ── Convenience wrappers ─────────────────────────────────────
// These are used in routes/admin/mod.rs to layer the guard on
// each admin sub-router without repeating the closure boilerplate.

/// Middleware that restricts access to sales managers.
pub async fn require_sales_manager(
    state: State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    role_guard(state, request, next, "sales_manager").await
}

/// Middleware that restricts access to product managers.
pub async fn require_product_manager(
    state: State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    role_guard(state, request, next, "product_manager").await
}