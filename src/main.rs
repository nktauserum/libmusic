mod library;
mod repository;
mod types;

use crate::library::Library;
use crate::repository::Repository;
use axum::extract::State;
use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;

struct AppState {
    lib: Library
}

impl AppState {
    fn new() -> Arc<Self> {
        let repo = Repository::new("sqlite.db").unwrap();
        let lib = Library::new(repo);

        Arc::new(Self {
            lib
        })
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/", get(index))
        .route("/tracks", get(track_list))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6432").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello, World!"
}

async fn track_list(State(state): State<Arc<AppState>>) -> Json<Value> {
    let tracks = state.lib.get_all_tracks().unwrap_or_else(|err| {
        eprintln!("Error: get_all_tracks(): {err}");
        vec![]
    });
    Json(json!({"tracks": tracks}))
}
