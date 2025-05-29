use axum::Router;
use axum::routing::get;
use std::path::PathBuf;
use tower_http::services::ServeDir;

async fn hello_handler() -> &'static str {
    "Hello from API!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let web_port = 8081;

    // Leptosのビルド成果物のパス
    let leptos_dist_path = PathBuf::from("front/dist");

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
    println!("Serving Leptos application from front/dist directory");
    axum::serve(listener, app).await?;

    Ok(())
}
