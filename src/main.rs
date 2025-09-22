use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6432").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello, World!"
}
