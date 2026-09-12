use crate::events::{AppEvent, EventBus};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_job(pool: &SqlitePool, bus: &EventBus, kind: &str) -> Result<Uuid, sqlx::Error> {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO jobs(id,kind,status,created_at,updated_at) VALUES(?,?,'queued',?,?)")
        .bind(id.to_string()).bind(kind).bind(Utc::now()).bind(Utc::now()).execute(pool).await?;
    bus.publish(AppEvent::JobStarted { job_id: id.to_string(), kind: kind.to_owned() });
    Ok(id)
}

pub async fn finish_job(pool: &SqlitePool, bus: &EventBus, id: Uuid, success: bool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE jobs SET status=?,finished_at=?,updated_at=? WHERE id=?")
        .bind(if success { "success" } else { "failed" }).bind(Utc::now()).bind(Utc::now()).bind(id.to_string()).execute(pool).await?;
    bus.publish(AppEvent::JobFinished { job_id: id.to_string(), success });
    Ok(())
}
