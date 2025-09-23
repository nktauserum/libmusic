mod library;
mod repository;
mod types;
mod config;
mod handlers;
mod state;

use crate::config::Config;
use crate::handlers::{index, track_by_id, track_list};
use axum::{http, routing::get, Router};
use tower_http::cors::{CorsLayer, Any};
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let config = Config::load("config.json").await;
    let state = AppState::new(config.db_path.as_str());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([http::Method::GET, http::Method::POST]);

    let app = Router::new()
        .route("/", get(index))
        .route("/tracks", get(track_list))
        .route("/track/{id}", get(track_by_id))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}