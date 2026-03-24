//! # Router Construction
//!
//! Assembles the complete Axum router by mounting every route group
//! under its prefix and layering the appropriate middleware.
//!
//! ## URL structure
//!
//! ```text
//! /                     → pages (full HTML, server-rendered)
//! /auth/login           → auth forms + handlers
//! /auth/register
//! /auth/logout
//! /products             → product browsing (pages + HTMX partials)
//! /cart                 → cart operations
//! /orders               → order management
//! /reviews              → comment & rating submission
//! /wishlist             → wishlist management
//! /payments             → mock payment processing
//! /refunds              → return / refund requests
//! /admin/sales/...      → sales manager panel
//! /admin/products/...   → product manager panel
//! /static/...           → CSS, JS, images (served by tower-http)
//! ```
//!
//! All routes except `/auth/*` and public browsing pages pass through
//! the auth middleware. Admin routes additionally pass through the
//! role guard.

use std::sync::Arc;

use axum::{
    middleware as axum_mw,
    Router,
};

use crate::AppState;
use crate::middleware::auth::auth_middleware;

pub mod auth;
pub mod pages;
pub mod products;
pub mod categories;
pub mod cart;
pub mod orders;
pub mod reviews;
pub mod wishlist;
pub mod payments;
pub mod refunds;
pub mod admin;

/// Builds and returns the top-level router with all routes mounted.
///
/// The `state` is cloned into each sub-router via `.with_state()`.
/// Middleware is layered from bottom to top — auth runs first, then
/// role guards on admin routes.
pub fn build_router(state: Arc<AppState>) -> Router {
    // ── Public routes (no auth required) ─────────────────────
    let public_routes = Router::new()
        .merge(pages::routes())
        .merge(auth::routes());

    // ── Authenticated routes (auth middleware applied) ────────
    let protected_routes = Router::new()
        .merge(cart::routes())
        .merge(orders::routes())
        .merge(reviews::routes())
        .merge(wishlist::routes())
        .merge(payments::routes())
        .merge(refunds::routes());

    // ── Product browsing (public, but auth-aware for wishlist
    //    buttons and "add to cart" state) ──────────────────────
    let product_routes = Router::new()
        .merge(products::routes())
        .merge(categories::routes());

    // ── Admin routes (auth + role guard) ─────────────────────
    let admin_routes = admin::routes();

    // ── Assemble ─────────────────────────────────────────────
    Router::new()
        .merge(public_routes)
        .merge(product_routes)
        .merge(protected_routes)
        .merge(admin_routes)
        // Auth middleware runs on ALL routes so that even public
        // pages can show "Welcome, <name>" if the user is logged in.
        .layer(axum_mw::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}