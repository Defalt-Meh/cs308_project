//! # Database Module
//!
//! Houses connection pool creation and SQL migrations. Migrations
//! are embedded at compile time via `sqlx::migrate!` so the binary
//! is self-contained — no need to ship .sql files alongside it.
//!
//! ## Module layout
//! - `pool` — PgPool construction with connection limits and timeouts
//! - `migrations/` — numbered .sql files run automatically on startup

pub mod pool;