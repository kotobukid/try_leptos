use axum::Router;
use axum::routing::get;
use std::path::PathBuf;
use std::env;
use tower_http::services::ServeDir;

async fn hello_handler() -> &'static str {
    "Hello from API!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 環境変数からポート番号を取得（デフォルト: 8081）
    let web_port = env::var("API_PORT").unwrap_or_else(|_| "8081".to_string()).parse::<u16>()
        .expect("API_PORT must be a valid port number");

    // 環境変数から静的ファイルのパスを取得（デフォルト: "front/dist"）
    let leptos_dist_path = PathBuf::from(
        env::var("STATIC_FILES_PATH").unwrap_or_else(|_| "front/dist".to_string())
    );

    // APIルーター
    let api_routes = Router::new().route("/hello", get(hello_handler));

    // メインルーター
    let app = Router::new()
        .nest("/api", api_routes)
        .route("/healthz", get(|| async { "OK" }))
        .route("/internal/healthz", get(|| async { "OK" }))
        .fallback_service(ServeDir::new(leptos_dist_path).append_index_html_on_directories(true));

    let web_addr = format!("0.0.0.0:{}", web_port);
    let listener = tokio::net::TcpListener::bind(&web_addr)
        .await
        .expect("Failed to bind port");
    println!("Server is running on http://{}", web_addr);
    println!("Serving Leptos application from {} directory", 
             env::var("STATIC_FILES_PATH").unwrap_or_else(|_| "front/dist".to_string()));
    axum::serve(listener, app).await?;

    Ok(())
}
