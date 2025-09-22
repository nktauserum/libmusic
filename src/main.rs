mod library;
mod repository;
mod types;

use crate::library::Library;
use crate::repository::Repository;
use axum::{response::Json, routing::get, Router, extract::{Path, State}, http::StatusCode, response::IntoResponse, body};
use serde_json::{json, Value};
use std::sync::Arc;
use axum::http::header;
use axum::serve::IncomingStream;
use tokio_util::io::ReaderStream;

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
        .route("/track/{id}", get(track_by_id))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6432").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello, World!"
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
