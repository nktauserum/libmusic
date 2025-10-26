use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Track {
    pub id: i64,
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub created_at: String,
    #[serde(skip_serializing)]
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PlaylistTrack {
    pub playlist_id: i64,
    pub track_id: i64,
    pub position: i32,
}