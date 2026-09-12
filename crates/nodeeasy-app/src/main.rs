#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "nodeeasy=info".into())).init();
    let database_url=std::env::var("NODEEASY_DATABASE_URL").unwrap_or_else(|_| "sqlite://nodeeasy.db?mode=rwc".into());
    let app=nodeeasy_app::router(&database_url).await?;
    let bind=std::env::var("NODEEASY_BIND").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener=tokio::net::TcpListener::bind(&bind).await?;
    tracing::info!(address=?listener.local_addr()?,"NodeEasy API initialized");
    axum::serve(listener,app).await?;
    Ok(())
}
