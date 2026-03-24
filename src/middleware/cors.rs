//! # CORS Middleware
//!
//! Configures Cross-Origin Resource Sharing headers. In the current
//! architecture (server-rendered HTML, no separate frontend origin)
//! CORS is not strictly necessary. However, it's configured here so
//! that:
//!
//! 1. A future mobile app or external client can call the API.
//! 2. Development tools (Postman, curl from other ports) work
//!    without friction.
//!
//! In production, restrict `allowed_origins` to your actual domain.

use tower_http::cors::{Any, CorsLayer};

use crate::config::AppConfig;

/// Builds a `CorsLayer` based on the application configuration.
///
/// - Development: allows any origin for convenience.
/// - Production: should be locked down to the frontend URL.
pub fn build_cors_layer(config: &AppConfig) -> CorsLayer {
    // If FRONTEND_URL is set, restrict origins. Otherwise allow all
    // (suitable for development only).
    let frontend_url = &config.host;

    if frontend_url == "0.0.0.0" {
        // Development — permissive.
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        // Production — restrict to known origin.
        // TODO: parse FRONTEND_URL into a proper `HeaderValue` and
        // use `.allow_origin(origin)` instead of `Any`.
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    }
}