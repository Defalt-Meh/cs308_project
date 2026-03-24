//! # Middleware Layer
//!
//! Axum middleware that runs before route handlers to enforce
//! cross-cutting concerns:
//!
//! - **auth** — Extracts and validates the JWT from cookies.
//!   Populates `AuthUser` in request extensions so handlers can
//!   access the current user without re-parsing the token.
//!
//! - **role_guard** — Checks that the authenticated user has the
//!   required role for a given route group (e.g. only
//!   `product_manager` can access delivery management).
//!
//! - **cors** — Configures Cross-Origin Resource Sharing headers.
//!   Mostly relevant if a future mobile app calls the API directly.

pub mod auth;
pub mod role_guard;
pub mod cors;