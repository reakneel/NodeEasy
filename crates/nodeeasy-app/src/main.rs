mod db;
mod repository;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use sqlx::SqlitePool;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
struct AppState { pool: SqlitePool }

async fn health() -> &'static str { "ok" }

async fn list_nodes(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match repository::list_nodes(&state.pool, 100).await {
        Ok(nodes) => (StatusCode::OK, Json(json!({"items": nodes}))).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": error.to_string()}))).into_response(),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "nodeeasy=info".into())).init();
    let database_url = env::var("NODEEASY_DATABASE_URL").unwrap_or_else(|_| "sqlite://nodeeasy.db?mode=rwc".into());
    let pool = db::connect(&database_url).await?;
    let state = Arc::new(AppState { pool });
    let app = Router::new().route("/api/v1/health", get(health)).route("/api/v1/nodes", get(list_nodes)).with_state(state);
    let bind = env::var("NODEEASY_BIND").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    tracing::info!(address = ?listener.local_addr()?, "NodeEasy API initialized");
    axum::serve(listener, app).await?;
    Ok(())
}
