pub mod db;
pub mod events;
pub mod export;
pub mod jobs;
pub mod repository;
pub mod source;
pub mod source_fetcher;
pub mod tester;
pub mod ws;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use nodeeasy_core::Source;
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;
use std::{env, sync::Arc};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub sources: source::SharedSourceEngine,
    pub bus: Arc<events::EventBus>,
}

async fn health() -> impl IntoResponse {
    Json(json!({"status":"ok","version":"3.0.0","api":"v1"}))
}

async fn list_nodes(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    match repository::list_nodes(&s.pool, 100).await {
        Ok(v) => (StatusCode::OK, Json(json!({"items":v}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
    }
}

async fn list_sources(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    match repository::list_sources(&s.pool).await {
        Ok(v) => (StatusCode::OK, Json(json!({"items":v}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
    }
}

async fn export_nodes(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    match repository::list_nodes(&s.pool, 1000).await {
        Ok(v) => (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, "application/json")],
            export::json(&v),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
    }
}

async fn qr_node(State(s): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repository::list_nodes(&s.pool, 1000).await {
        Ok(v) => match v.into_iter().find(|n| n.id == id) {
            Some(n) => match export::qr_ascii(&n) {
                Ok(q) => (
                    StatusCode::OK,
                    [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
                    q,
                )
                    .into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error":e.to_string()})),
                )
                    .into_response(),
            },
            None => (
                StatusCode::NOT_FOUND,
                Json(json!({"error":"node not found"})),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
struct CreateSource {
    name: String,
    kind: Option<String>,
    url: String,
    interval_seconds: Option<u64>,
}

async fn create_source(
    State(s): State<Arc<AppState>>,
    Json(i): Json<CreateSource>,
) -> impl IntoResponse {
    let source = Source {
        id: Uuid::new_v4(),
        name: i.name,
        kind: i.kind.unwrap_or_else(|| "http_subscription".into()),
        url: i.url,
        enabled: true,
        interval_seconds: i.interval_seconds.unwrap_or(3600),
        last_fetch_at: None,
        last_success_at: None,
        last_error: None,
    };
    match repository::upsert_source(&s.pool, &source).await {
        Ok(()) => (StatusCode::CREATED, Json(source)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
    }
}

async fn sync_source(State(s): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let row = sqlx::query_as::<_, (String, String)>("SELECT kind,url FROM sources WHERE id=?")
        .bind(id.to_string())
        .fetch_optional(&s.pool)
        .await;

    let Some((kind, url)) = (match row {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error":e.to_string()})),
            )
                .into_response()
        }
    }) else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error":"source not found"})),
        )
            .into_response();
    };

    if kind != "http_subscription" {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"source kind is not supported"})),
        )
            .into_response();
    }

    match s.sources.sync_http(id, url).await {
        Ok(stats) => {
            s.bus.publish(events::AppEvent::SourceSynced {
                source_id: id.to_string(),
                parsed: stats.parsed,
            });
            (
                StatusCode::OK,
                Json(json!({"source_id":id,"synced_at":Utc::now(),"stats":stats})),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
    }
}

async fn test_node(State(s): State<Arc<AppState>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repository::list_nodes(&s.pool, 1000).await {
        Ok(v) => match v.into_iter().find(|n| n.id == id) {
            Some(n) => match tester::tcp(&s.pool, &n, 5000).await {
                Ok(t) => {
                    s.bus.publish(events::AppEvent::NodeTested {
                        node_id: id.to_string(),
                        success: t.success,
                        latency_ms: t.latency_ms,
                    });
                    (StatusCode::OK, Json(t)).into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error":e.to_string()})),
                )
                    .into_response(),
            },
            None => (
                StatusCode::NOT_FOUND,
                Json(json!({"error":"node not found"})),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
    }
}

async fn create_job(State(s): State<Arc<AppState>>) -> impl IntoResponse {
    match jobs::create_job(&s.pool, &s.bus, "manual").await {
        Ok(id) => (
            StatusCode::ACCEPTED,
            Json(json!({"job_id":id,"status":"queued"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn router(database_url: &str) -> anyhow::Result<Router> {
    let pool = db::connect(database_url).await?;
    let fetcher = source_fetcher::SourceFetcher::new(
        env::var("NODEEASY_ALLOW_PRIVATE_SOURCES").ok().as_deref() == Some("true"),
    )?;
    let engine = Arc::new(source::SourceEngine::new(pool.clone(), fetcher));
    let bus = Arc::new(events::EventBus::new(512));
    let state = Arc::new(AppState {
        pool,
        sources: engine,
        bus,
    });

    Ok(Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/nodes", get(list_nodes))
        .route("/api/v1/nodes/{id}/test", post(test_node))
        .route("/api/v1/sources", get(list_sources).post(create_source))
        .route("/api/v1/sources/{id}/sync", post(sync_source))
        .route("/api/v1/jobs", post(create_job))
        .route("/api/v1/export/nodes.json", get(export_nodes))
        .route("/api/v1/nodes/{id}/qr", get(qr_node))
        .route("/api/v1/ws", get(ws::handler))
        .with_state(state))
}
