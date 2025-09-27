use std::sync::Arc;
use axum::{body, Json};
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use serde_json::{json, Value};
use tokio_util::io::ReaderStream;
use crate::AppState;

pub async fn index() -> StatusCode {
    StatusCode::OK
}

pub async fn track_by_id(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Result<impl IntoResponse, StatusCode> {
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

pub async fn track_list(State(state): State<Arc<AppState>>) -> Json<Value> {
    let tracks = state.lib.get_all_tracks().unwrap_or_else(|err| {
        eprintln!("Error: get_all_tracks(): {err}");
        vec![]
    });
    Json(json!({"tracks": tracks}))
}

pub async fn cover_by_id(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Result<impl IntoResponse, StatusCode> {
    let cover = state.lib.get_cover_by_track_id(id).map_err(|err| {
        eprintln!("Error: cover_by_id(): {err}");
        StatusCode::NOT_FOUND
    })?;

    let headers = [
        (header::CONTENT_TYPE, "image/png".to_string()),
        (header::CACHE_CONTROL, "public, max-age=31536000".to_string()),
    ];

    Ok((headers, cover))
}
