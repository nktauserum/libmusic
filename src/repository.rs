use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection, Result};
use crate::types::Track;

#[derive(Clone)]
pub struct Repository {
    connection: Arc<Mutex<Connection>>,
}

impl Repository {
    pub fn new(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS tracks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                artists TEXT NOT NULL,
                album TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                path TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>> {
        let conn_guard = self.connection.lock().unwrap();
        let mut stmt =
            conn_guard.prepare("SELECT id, title, artists, album, created_at, path FROM tracks")?;
        let rows = stmt.query_map([], |row| {
            let artists_str: String = row.get(2)?;
            Ok(Track {
                id: row.get(0)?,
                title: row.get(1)?,
                artists: artists_str
                    .split(',')
                    .map(|p| p.trim().to_string())
                    .filter(|p| !p.is_empty())
                    .collect(),
                album: row.get(3)?,
                created_at: row.get(4)?,
                path: row.get(5)?,
            })
        })?;

        let mut tracks = Vec::new();
        for r in rows {
            tracks.push(r?);
        }

        Ok(tracks)
    }

    pub fn get_track_by_id(&self, id: i64) -> Result<Track> {
        let conn_guard = self.connection.lock().unwrap();
        let mut stmt = conn_guard.prepare(
            "SELECT id, title, artists, album, created_at, path FROM tracks WHERE id = ?1",
        )?;

        Ok(stmt.query_row(params![id], |row| {
            let artists_csv: String = row.get(2)?;
            Ok(Track {
                id: row.get(0)?,
                title: row.get(1)?,
                artists: if artists_csv.is_empty() {
                    Vec::new()
                } else {
                    artists_csv
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect()
                },
                album: row.get(3)?,
                created_at: row.get(4)?,
                path: row.get(5)?,
            })
        })?)
    }

    pub fn add_track(&self, track: &Track) -> Result<()> {
        let conn_guard = self.connection.lock().unwrap();

        conn_guard.execute(
            "INSERT INTO tracks (title, artists, album, path) VALUES (?1, ?2, ?3, ?4)",
            params![
                track.title,
                track.artists.join(", "),
                track.album,
                track.path,
            ],
        )?;

        Ok(())
    }
}