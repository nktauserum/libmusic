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