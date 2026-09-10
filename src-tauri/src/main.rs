#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tracing_subscriber::fmt().with_env_filter("nodeeasy=info").init();
    tauri::Builder::default().run(tauri::generate_context!()).expect("error while running NodeEasy");
}
