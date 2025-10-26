mod library;
mod repository;
mod types;
mod config;
mod handlers;
mod state;
mod responses;

use std::path::Path;
use crate::config::Config;
use crate::handlers::{cover_by_id, create_playlist, delete_playlist, fetch_playlist, index, playlist_list, remove_track_from_pl, track_by_id, track_list, update_playlist};
use axum::{http, routing::get, routing::post, Router};
use tower_http::cors::{CorsLayer, Any};
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let config = Config::load("config.json").await;
    let state = AppState::new(config.db_path.as_str());

    let cloned_state = state.clone();
    tokio::spawn(async move {
        for dir in config.index_dir {
            match cloned_state.lib.index_dir(Path::new(dir.as_str())) {
                Ok(()) => println!("INFO: directory {dir} successfully indexed."),
                Err(err) => {
                    eprintln!("Error: index_dir({dir}): {err}")
                }
            }
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([http::Method::GET, http::Method::POST]);

    let app = Router::new()
        .route("/", get(index))
        .route("/tracks", get(track_list))
        .route("/track/{id}", get(track_by_id))
        .route("/cover/{id}", get(cover_by_id))
        .route("/playlists", get(playlist_list))
        .route("/playlists/create", post(create_playlist))
        .route("/playlists/{id}", get(fetch_playlist))
        .route("/playlists/{id}/add", post(update_playlist))
        .route("/playlists/{id}/remove_track", post(remove_track_from_pl))
        .route("/playlists/{id}/delete", post(delete_playlist))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}