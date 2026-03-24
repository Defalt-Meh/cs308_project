//! # Connection Pool
//!
//! Creates a `sqlx::PgPool` with tuned settings for concurrent access.
//! The pool is shared across all Axum handlers via `AppState` (wrapped
//! in `Arc`), so every async task can check out a connection without
//! blocking others.
//!
//! ## Concurrency notes (Requirement 17)
//! - `max_connections` caps the total open connections to Postgres.
//!   Set this to match your Postgres `max_connections` minus headroom
//!   for admin tools.
//! - `min_connections` keeps a warm pool so the first requests after
//!   idle periods don't pay the TCP + TLS handshake cost.
//! - `acquire_timeout` prevents handlers from hanging indefinitely
//!   when the pool is saturated under load.

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

/// Constructs a new `PgPool` connected to the given `database_url`.
///
/// # Panics
/// Panics if the initial connection cannot be established — this is
/// intentional: the app should fail fast on startup if the database
/// is unreachable rather than serving 500s to every request.
pub async fn create_pool(database_url: &str) -> sqlx::PgPool {
    PgPoolOptions::new()
        // Maximum simultaneous connections. Adjust based on expected
        // load and Postgres server capacity.
        .max_connections(20)
        // Keep at least 5 connections warm to reduce latency spikes.
        .min_connections(5)
        // If all 20 connections are checked out, a handler will wait
        // up to 5 seconds before receiving a timeout error.
        .acquire_timeout(Duration::from_secs(5))
        // Close connections that have been idle for 10 minutes to
        // free resources on the Postgres side.
        .idle_timeout(Duration::from_secs(600))
        .connect(database_url)
        .await
        .expect("Failed to connect to database — check DATABASE_URL")
}