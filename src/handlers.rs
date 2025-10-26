use std::sync::Arc;
use axum::{body, Json};
use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio_util::io::ReaderStream;
use crate::AppState;
use crate::responses::{CreatePlaylistRequest, UpdatePlaylistRequest};

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

pub async fn create_playlist(State(state): State<Arc<AppState>>, Json(req): Json<CreatePlaylistRequest>) -> impl IntoResponse {
    match state.lib.create_playlist(req.name) {
        Ok(id) => (StatusCode::CREATED, Json(json!({"id": id}))),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()})))
    }
}

pub async fn playlist_list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.lib.get_all_playlists() {
        Ok(pls) => (StatusCode::OK, Json(json!({"playlists": pls}))),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()})))
    }
}

pub async fn update_playlist(State(state): State<Arc<AppState>>, Path(id): Path<i64>, Json(req): Json<UpdatePlaylistRequest>) -> impl IntoResponse {
    match state.lib.add_track_to_playlist(id, req.track_id) {
        Ok(_) => (StatusCode::OK, Json(json!({}))),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()})))
    }
}

pub async fn fetch_playlist(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Result<impl IntoResponse, StatusCode> {
    let pl = state.lib.fetch_playlist(id).map_err(|err| {
        eprintln!("Error: cover_by_id(): {err}");
        StatusCode::NOT_FOUND
    })?;

    Ok(Json(pl))
}

pub async fn remove_track_from_pl(State(state): State<Arc<AppState>>, Path(id): Path<i64>, Json(req): Json<UpdatePlaylistRequest>) -> Result<impl IntoResponse, StatusCode>  {
    Ok(state.lib.remove_track_from_playlist(id, req.track_id).map_err(|err| {
        eprintln!("Error: track_by_id(): {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?)
}

pub async fn delete_playlist(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Result<impl IntoResponse, StatusCode>  {
    Ok(state.lib.delete_playlist(id).map_err(|err| {
        eprintln!("Error: track_by_id(): {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?)
}
