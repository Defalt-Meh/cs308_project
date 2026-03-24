//! # CS 308 Online Store — Application Entry Point
//!
//! Initializes all subsystems (config, database, templates) and starts
//! the Axum HTTP server. Static assets are served via `tower-http`
//! and all HTML is rendered server-side with Tera + HTMX.

use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod config;
mod db;
mod errors;
mod middleware;
mod models;
mod routes;
mod services;

/// Shared application state passed to every handler via Axum's
/// state extractor. Wrapping in `Arc` makes cloning cheap across
/// async tasks.
pub struct AppState {
    pub db: sqlx::PgPool,
    pub tera: tera::Tera,
    pub config: config::AppConfig,
}

#[tokio::main]
async fn main() {
    // ── Logging ──────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── Configuration ────────────────────────────────────────
    dotenvy::dotenv().ok();
    let config = config::AppConfig::from_env();

    // ── Database ─────────────────────────────────────────────
    let pool = db::pool::create_pool(&config.database_url).await;
    sqlx::migrate!("src/db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    tracing::info!("Database connected and migrations applied");

    // ── Templates ────────────────────────────────────────────
    // Tera watches the templates/ directory. In debug mode it
    // reloads on every request so you can edit HTML live.
    let tera = tera::Tera::new("templates/**/*")
        .expect("Failed to compile Tera templates");

    // ── Shared state ─────────────────────────────────────────
    let state = Arc::new(AppState {
        db: pool,
        tera,
        config: config.clone(),
    });

    // ── Router ───────────────────────────────────────────────
    let app = routes::build_router(state.clone())
        // Serve everything in static/ under the /static path.
        .nest_service("/static", ServeDir::new("static"))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    // ── Start server ─────────────────────────────────────────
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}