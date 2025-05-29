use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let web_port = 8080;

    let app = Router::new()
        // should mount leptos
        .route("/", get(|| async { "Hello, world!" }))
        .route("/healthz", get(|| async { "OK" }))
        .route("/internal/healthz", get(|| async { "OK" }));

    let web_addr = format!("0.0.0.0:{}", web_port);
    let listener = tokio::net::TcpListener::bind(&web_addr)
        .await
        .expect("Failed to bind port");
    println!("Server is running on http://{}", web_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
