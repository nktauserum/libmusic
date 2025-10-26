use serde::{Deserialize, Serialize};
use crate::types::Track;

#[derive(Serialize, Deserialize)]
pub struct UpdatePlaylistRequest {
    pub track_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePlaylistRequest {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct FetchPlaylistResponse {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub tracks: Vec<Track>,
}