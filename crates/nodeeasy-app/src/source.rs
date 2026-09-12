use crate::{repository, source_adapters::SourceAdapter};
use chrono::Utc;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub struct SourceEngine { pool: SqlitePool }
impl SourceEngine {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }
    pub async fn sync(&self, source_id: Uuid, adapter: &dyn SourceAdapter) -> Result<SyncStats, Box<dyn std::error::Error + Send + Sync>> {
        let run_id = Uuid::new_v4();
        sqlx::query("INSERT INTO source_runs(id,source_id,started_at,status) VALUES(?,?,?,'running')").bind(run_id.to_string()).bind(source_id.to_string()).bind(Utc::now()).execute(&self.pool).await?;
        match adapter.collect().await {
            Ok(nodes) => {
                let mut stats = SyncStats { fetched: 1, parsed: nodes.len(), added: 0, updated: 0, failed: 0 };
                for item in nodes {
                    let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM nodes WHERE fingerprint=?").bind(&item.node.fingerprint).fetch_one(&self.pool).await? > 0;
                    repository::upsert_node(&self.pool, &item.node).await?;
                    if exists { stats.updated += 1; } else { stats.added += 1; }
                    sqlx::query("INSERT INTO node_sources(node_id,source_id,raw_name,first_seen_at,last_seen_at) VALUES(?,?,?,?,?) ON CONFLICT(node_id,source_id) DO UPDATE SET raw_name=excluded.raw_name,last_seen_at=excluded.last_seen_at").bind(item.node.id.to_string()).bind(source_id.to_string()).bind(item.node.name.clone()).bind(item.node.first_seen_at).bind(item.node.last_seen_at).execute(&self.pool).await?;
                }
                sqlx::query("UPDATE source_runs SET finished_at=?,status='success',fetched=?,parsed=?,added=?,updated=? WHERE id=?").bind(Utc::now()).bind(stats.fetched).bind(stats.parsed as i64).bind(stats.added as i64).bind(stats.updated as i64).bind(run_id.to_string()).execute(&self.pool).await?;
                sqlx::query("UPDATE sources SET last_fetch_at=?,last_success_at=?,last_error=NULL,updated_at=? WHERE id=?").bind(Utc::now()).bind(Utc::now()).bind(Utc::now()).bind(source_id.to_string()).execute(&self.pool).await?;
                Ok(stats)
            }
            Err(error) => {
                let message = error.to_string();
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
