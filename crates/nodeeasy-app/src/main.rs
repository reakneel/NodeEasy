mod db;
mod events;
mod jobs;
mod repository;
mod source;
mod source_fetcher;
mod tester;
mod ws;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use chrono::Utc;
use nodeeasy_core::Source;
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;
use std::{env, sync::Arc};
use uuid::Uuid;

#[derive(Clone)] struct AppState { pool:SqlitePool, sources:source::SharedSourceEngine, bus:Arc<events::EventBus> }
async fn health()->impl IntoResponse{Json(json!({"status":"ok","version":"3.0.0"}))}
async fn list_nodes(State(state):State<Arc<AppState>>)->impl IntoResponse{match repository::list_nodes(&state.pool,100).await{Ok(nodes)=>(StatusCode::OK,Json(json!({"items":nodes}))).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))).into_response()}}
async fn list_sources(State(state):State<Arc<AppState>>)->impl IntoResponse{match repository::list_sources(&state.pool).await{Ok(items)=>(StatusCode::OK,Json(json!({"items":items}))).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))).into_response()}}
#[derive(Deserialize)] struct CreateSource{name:String,kind:Option<String>,url:String,interval_seconds:Option<u64>}
async fn create_source(State(state):State<Arc<AppState>>,Json(input):Json<CreateSource>)->impl IntoResponse{let source=Source{id:Uuid::new_v4(),name:input.name,kind:input.kind.unwrap_or_else(||"http_subscription".into()),url:input.url,enabled:true,interval_seconds:input.interval_seconds.unwrap_or(3600),last_fetch_at:None,last_success_at:None,last_error:None};match repository::upsert_source(&state.pool,&source).await{Ok(())=>(StatusCode::CREATED,Json(source)).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))).into_response()}}
async fn sync_source(State(state):State<Arc<AppState>>,Path(id):Path<Uuid>)->impl IntoResponse{let row=sqlx::query_as::<_,(String,String)>("SELECT kind,url FROM sources WHERE id=?").bind(id.to_string()).fetch_optional(&state.pool).await;let Some((kind,url))=match row{Ok(v)=>v,Err(e)=>return(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))).into_response()}else{return(StatusCode::NOT_FOUND,Json(json!({"error":"source not found"}))).into_response()};if kind!="http_subscription"{return(StatusCode::BAD_REQUEST,Json(json!({"error":"source kind is not supported"}))).into_response();}match state.sources.sync_http(id,url).await{Ok(stats)=>{state.bus.publish(events::AppEvent::SourceSynced{source_id:id.to_string(),parsed:stats.parsed});(StatusCode::OK,Json(json!({"source_id":id,"synced_at":Utc::now(),"stats":stats}))).into_response()},Err(e)=>(StatusCode::BAD_GATEWAY,Json(json!({"error":e.to_string()}))).into_response()}}
async fn test_node(State(state):State<Arc<AppState>>,Path(id):Path<Uuid>)->impl IntoResponse{match repository::list_nodes(&state.pool,1000).await{Ok(nodes)=>match nodes.into_iter().find(|n|n.id==id){Some(node)=>match tester::tcp(&state.pool,&node,5000).await{Ok(test)=>{state.bus.publish(events::AppEvent::NodeTested{node_id:id.to_string(),success:test.success,latency_ms:test.latency_ms});(StatusCode::OK,Json(test)).into_response()},Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))).into_response()},None=>(StatusCode::NOT_FOUND,Json(json!({"error":"node not found"}))).into_response()},Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))).into_response()}}
async fn create_job(State(state):State<Arc<AppState>>)->impl IntoResponse{match jobs::create_job(&state.pool,&state.bus,"manual").await{Ok(id)=>(StatusCode::ACCEPTED,Json(json!({"job_id":id,"status":"queued"}))).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))).into_response()}}
#[tokio::main] async fn main()->anyhow::Result<()>{tracing_subscriber::fmt().with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_|"nodeeasy=info".into())).init();let database_url=env::var("NODEEASY_DATABASE_URL").unwrap_or_else(|_|"sqlite://nodeeasy.db?mode=rwc".into());let pool=db::connect(&database_url).await?;let fetcher=source_fetcher::SourceFetcher::new(env::var("NODEEASY_ALLOW_PRIVATE_SOURCES").ok().as_deref()==Some("true"))?;let engine=Arc::new(source::SourceEngine::new(pool.clone(),fetcher));let bus=Arc::new(events::EventBus::new(512));let state=Arc::new(AppState{pool,sources:engine,bus:bus.clone()});let app=Router::new().route("/api/v1/health",get(health)).route("/api/v1/nodes",get(list_nodes)).route("/api/v1/nodes/{id}/test",post(test_node)).route("/api/v1/sources",get(list_sources).post(create_source)).route("/api/v1/sources/{id}/sync",post(sync_source)).route("/api/v1/jobs",post(create_job)).route("/api/v1/ws",get(ws::handler)).with_state(state);let bind=env::var("NODEEASY_BIND").unwrap_or_else(|_|"127.0.0.1:3000".into());let listener=tokio::net::TcpListener::bind(&bind).await?;tracing::info!(address=?listener.local_addr()?,"NodeEasy API initialized");axum::serve(listener,app).await?;Ok(())}
