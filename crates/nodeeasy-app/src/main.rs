use axum::{routing::get, Router};

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/api/v1/health", get(health));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    tracing::info!(address = ?listener.local_addr()?, "NodeEasy API initialized");
    axum::serve(listener, app).await?;
    Ok(())
}
