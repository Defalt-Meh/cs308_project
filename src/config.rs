//! # Application Configuration
//!
//! Reads all settings from environment variables (loaded via `.env`
//! in development). Every config value has a sensible default so the
//! app can start locally without a fully populated `.env` file.

use std::env;

/// Central configuration struct shared across the application
/// through `AppState`.
#[derive(Debug, Clone)]
pub struct AppConfig {
    // ── Server ───────────────────────────────────────────────
    pub host: String,
    pub port: u16,

    // ── Database ─────────────────────────────────────────────
    pub database_url: String,

    // ── Auth ─────────────────────────────────────────────────
    /// Secret key used to sign and verify JWT tokens. Must be a
    /// strong random string in production.
    pub jwt_secret: String,
    /// Token lifetime in hours. Defaults to 24h.
    pub jwt_expiry_hours: i64,

    // ── Email (for invoice delivery) ─────────────────────────
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_from: String,
}

impl AppConfig {
    /// Constructs an `AppConfig` from environment variables.
    /// Panics on missing `DATABASE_URL` or `JWT_SECRET` since the
    /// app cannot operate without them.
    pub fn from_env() -> Self {
        Self {
            // Server
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),

            // Database — required
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),

            // Auth — required
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiry_hours: env::var("JWT_EXPIRY_HOURS")
                .ok()
                .and_then(|h| h.parse().ok())
                .unwrap_or(24),

            // Email — optional in development (will log instead of send)
            smtp_host: env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".into()),
            smtp_port: env::var("SMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(587),
            smtp_user: env::var("SMTP_USER").unwrap_or_default(),
            smtp_pass: env::var("SMTP_PASS").unwrap_or_default(),
            smtp_from: env::var("SMTP_FROM")
                .unwrap_or_else(|_| "noreply@store.local".into()),
        }
    }
}