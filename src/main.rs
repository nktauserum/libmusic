mod library;
mod repository;
mod types;
mod config;

use crate::library::Library;
use crate::repository::Repository;
use axum::{response::Json, routing::get, Router, extract::{Path, State}, http::StatusCode, response::IntoResponse, body};
use serde_json::{json, Value};
use std::sync::Arc;
use axum::http::header;
use tokio_util::io::ReaderStream;
use crate::config::Config;

struct AppState {
    lib: Library
}

impl AppState {
    fn new(db_path: &str) -> Arc<Self> {
        let repo = Repository::new(db_path).unwrap();
        let lib = Library::new(repo);

        Arc::new(Self {
            lib
        })
    }
}

#[tokio::main]
async fn main() {
    let config = Config::load("config.json").await;
    let state = AppState::new(config.db_path.as_str());

    let app = Router::new()
        .route("/", get(index))
        .route("/tracks", get(track_list))
        .route("/track/{id}", get(track_by_id))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> StatusCode {
    StatusCode::OK
}

async fn track_by_id(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Result<impl IntoResponse, StatusCode> {
    let track = state.lib.get_track_by_id(id).map_err(|err| {
        eprintln!("Error: track_by_id(): {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let file = match tokio::fs::File::open(&track.path).await {
        Ok(file) => file,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    let stream = ReaderStream::new(file);
    let body = body::Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, "Content-Type: audio/mpeg".to_string()),
    ];

    Ok((headers, body))

}

async fn track_list(State(state): State<Arc<AppState>>) -> Json<Value> {
    let tracks = state.lib.get_all_tracks().unwrap_or_else(|err| {
        eprintln!("Error: get_all_tracks(): {err}");
        vec![]
    });
    Json(json!({"tracks": tracks}))
}
