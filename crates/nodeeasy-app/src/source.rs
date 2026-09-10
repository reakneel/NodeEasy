use crate::{repository, source_fetcher::SourceFetcher};
use async_trait::async_trait;
use chrono::Utc;
use nodeeasy_core::source::{deduplicate, parse_subscription, ParsedNode};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait NodeSource: Send + Sync {
    fn kind(&self) -> &'static str;
    async fn collect(&self) -> Result<Vec<ParsedNode>, nodeeasy_core::CoreError>;
}

pub struct HttpSubscriptionSource {
    pub url: String,
    pub fetcher: SourceFetcher,
}

#[async_trait]
impl NodeSource for HttpSubscriptionSource {
    fn kind(&self) -> &'static str { "http_subscription" }
    async fn collect(&self) -> Result<Vec<ParsedNode>, nodeeasy_core::CoreError> {
        let body = self.fetcher.fetch(&self.url).await?;
        Ok(deduplicate(parse_subscription(&body)?))
    }
}

pub struct SourceEngine { pool: SqlitePool, fetcher: SourceFetcher }
impl SourceEngine {
    pub fn new(pool: SqlitePool, fetcher: SourceFetcher) -> Self { Self { pool, fetcher } }
    pub async fn sync_http(&self, source_id: Uuid, url: String) -> Result<SyncStats, Box<dyn std::error::Error + Send + Sync>> {
        let run_id=Uuid::new_v4(); let started=Utc::now();
        sqlx::query("INSERT INTO source_runs(id,source_id,started_at,status) VALUES(?,?,?,'running')").bind(run_id.to_string()).bind(source_id.to_string()).bind(started).execute(&self.pool).await?;
        let source=HttpSubscriptionSource{url,fetcher:self.fetcher.clone()};
        let result=source.collect().await;
        match result {
            Ok(nodes)=>{
                let mut stats=SyncStats{fetched:1,parsed:nodes.len(),added:0,updated:0,failed:0};
                for item in nodes { repository::upsert_node(&self.pool,&item.node).await?; stats.added+=1; sqlx::query("INSERT INTO node_sources(node_id,source_id,raw_name,first_seen_at,last_seen_at) VALUES(?,?,?,?,?) ON CONFLICT(node_id,source_id) DO UPDATE SET raw_name=excluded.raw_name,last_seen_at=excluded.last_seen_at").bind(item.node.id.to_string()).bind(source_id.to_string()).bind(item.node.name.clone()).bind(item.node.first_seen_at).bind(item.node.last_seen_at).execute(&self.pool).await?; }
                sqlx::query("UPDATE source_runs SET finished_at=?,status='success',fetched=?,parsed=?,added=? WHERE id=?").bind(Utc::now()).bind(stats.fetched).bind(stats.parsed as i64).bind(stats.added as i64).bind(run_id.to_string()).execute(&self.pool).await?;
                sqlx::query("UPDATE sources SET last_fetch_at=?,last_success_at=?,last_error=NULL,updated_at=? WHERE id=?").bind(Utc::now()).bind(Utc::now()).bind(Utc::now()).bind(source_id.to_string()).execute(&self.pool).await?;
                Ok(stats)
            }
            Err(error)=>{
                let message=error.to_string();
                sqlx::query("UPDATE source_runs SET finished_at=?,status='failed',failed=1 WHERE id=?").bind(Utc::now()).bind(run_id.to_string()).execute(&self.pool).await?;
                sqlx::query("UPDATE sources SET last_fetch_at=?,last_error=?,updated_at=? WHERE id=?").bind(Utc::now()).bind(&message).bind(Utc::now()).bind(source_id.to_string()).execute(&self.pool).await?;
                Err(message.into())
            }
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct SyncStats { pub fetched: i64, pub parsed: usize, pub added: usize, pub updated: usize, pub failed: usize }

pub type SharedSourceEngine = Arc<SourceEngine>;
