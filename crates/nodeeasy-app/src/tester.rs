use chrono::Utc;
use nodeeasy_core::{Node, NodeTest, TestType};
use sqlx::SqlitePool;
use std::time::Instant;
use tokio::net::TcpStream;
use uuid::Uuid;

pub async fn tcp(
    pool: &SqlitePool,
    node: &Node,
    timeout_ms: u64,
) -> Result<NodeTest, sqlx::Error> {
    let started = Instant::now();
    let result = tokio::time::timeout(
        std::time::Duration::from_millis(timeout_ms),
        TcpStream::connect((&*node.endpoint.host, node.endpoint.port)),
    )
    .await;
    let success = result.is_ok() && result.as_ref().is_ok_and(|r| r.is_ok());
    let latency = success.then(|| started.elapsed().as_secs_f64() * 1000.0);
    let test = NodeTest {
        id: Uuid::new_v4(),
        node_id: node.id,
        test_type: TestType::Tcp,
        latency_ms: latency,
        download_bps: None,
        upload_bps: None,
        success,
        error: (!success).then_some("tcp connect failed or timed out".into()),
        tested_at: Utc::now(),
        duration_ms: Some(started.elapsed().as_millis() as u64),
    };
    sqlx::query("INSERT INTO node_tests(id,node_id,test_type,latency_ms,success,error,tested_at,duration_ms) VALUES(?,?,?,?,?,?,?,?)")
        .bind(test.id.to_string())
        .bind(test.node_id.to_string())
        .bind("tcp")
        .bind(test.latency_ms)
        .bind(test.success)
        .bind(&test.error)
        .bind(test.tested_at)
        .bind(test.duration_ms.map(|v| v as i64))
        .execute(pool)
        .await?;
    sqlx::query("UPDATE nodes SET last_tested_at=?,status=?,updated_at=? WHERE id=?")
        .bind(test.tested_at)
        .bind(if success { "healthy" } else { "unavailable" })
        .bind(test.tested_at)
        .bind(node.id.to_string())
        .execute(pool)
        .await?;
    Ok(test)
}
