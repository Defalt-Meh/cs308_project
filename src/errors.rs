//! # Application Error Types
//!
//! Defines a single `AppError` enum that every handler and service
//! can return. Axum's `IntoResponse` implementation converts each
//! variant into the appropriate HTTP status code and, for HTMX
//! requests, returns an error partial that can be swapped into the
//! page. For non-HTMX requests it renders a full error page.

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

/// Unified error type for the entire application. Use `thiserror`
/// to derive `Display` automatically from the `#[error(...)]`
/// attributes.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // ── Client errors ────────────────────────────────────────
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized — please log in")]
    Unauthorized,

    #[error("Forbidden — insufficient permissions")]
    Forbidden,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    // ── Server errors ────────────────────────────────────────
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Template rendering error: {0}")]
    Template(#[from] tera::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl AppError {
    /// Maps each variant to its HTTP status code.
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::Unauthorized       => StatusCode::UNAUTHORIZED,
            Self::Forbidden          => StatusCode::FORBIDDEN,
            Self::NotFound(_)        => StatusCode::NOT_FOUND,
            Self::Validation(_)      => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Conflict(_)        => StatusCode::CONFLICT,
            Self::Database(_)        => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Template(_)        => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Internal(_)        => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// Axum calls this automatically when a handler returns `Err(AppError)`.
/// We return a minimal HTML snippet so HTMX can swap it into a toast
/// or error container.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let message = self.to_string();

        // Log server-side errors at error level, client errors at warn.
        if status.is_server_error() {
            tracing::error!(%status, %message, "Server error");
        } else {
            tracing::warn!(%status, %message, "Client error");
        }

        // Return an HTML fragment that the toast component can display.
        let html = format!(
            r#"<div class="toast toast--error" role="alert">{}</div>"#,
            message
        );

        (status, Html(html)).into_response()
    }
}

/// Convenience type alias so handlers can write:
///   `async fn handler() -> Result<Html<String>>` 
pub type Result<T> = std::result::Result<T, AppError>;