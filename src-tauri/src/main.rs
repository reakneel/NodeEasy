#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tracing_subscriber::fmt().with_env_filter("nodeeasy=info").init();
    tauri::Builder::default()
        .setup(|_app| {
            tauri::async_runtime::spawn(async {
                let database_url=std::env::var("NODEEASY_DATABASE_URL").unwrap_or_else(|_| "sqlite://nodeeasy.db?mode=rwc".into());
                match nodeeasy_app::router(&database_url).await {
                    Ok(router) => match tokio::net::TcpListener::bind("127.0.0.1:3000").await {
                        Ok(listener) => { let _=axum::serve(listener,router).await; }
                        Err(e)=>tracing::error!(error=%e,"failed to bind local API"),
                    },
                    Err(e)=>tracing::error!(error=%e,"failed to initialize API"),
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running NodeEasy");
}
