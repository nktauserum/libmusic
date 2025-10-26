use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection, Result};
use crate::types::{Playlist, Track};
use rusqlite_migration::{Migrations, M};

fn migrations<'a>() -> Migrations<'a> {
    Migrations::new(vec![
        M::up("CREATE TABLE IF NOT EXISTS tracks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                artists TEXT NOT NULL,
                album TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                path TEXT NOT NULL UNIQUE
            )"),
        M::up("CREATE TABLE IF NOT EXISTS playlists (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"),
        M::up("CREATE TABLE IF NOT EXISTS playlist_tracks (
                playlist_id INTEGER NOT NULL,
                track_id INTEGER NOT NULL,
                position INTEGER NOT NULL,
                PRIMARY KEY (playlist_id, track_id),
                FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
                FOREIGN KEY (track_id) REFERENCES tracks(id) ON DELETE CASCADE
        )")
    ])
}

#[derive(Clone)]
pub struct Repository {
    connection: Arc<Mutex<Connection>>,
}

impl Repository {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut connection = Connection::open(path)?;

        migrations().to_latest(&mut connection)?;

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

    pub fn update_track(&self, track: Track) -> Result<()> {
        let conn_guard = self.connection.lock().unwrap();

        conn_guard.execute(
            "UPDATE tracks
                SET title = ?1, artists = ?2, album = ?3
                WHERE path = ?4
            ",
            params![
                track.title,
                track.artists.join(", "),
                track.album,
                track.path,
            ],
        )?;

        Ok(())
    }

    pub fn create_playlist(&self, name: String) -> Result<i64> {
        let conn_guard = self.connection.lock().unwrap();
        conn_guard.execute(
            "INSERT INTO playlists (name) VALUES (?1)",
            params![name],
        )?;
        Ok(conn_guard.last_insert_rowid())
    }

    pub fn get_all_playlists(&self) -> Result<Vec<Playlist>> {
        let conn_guard = self.connection.lock().unwrap();
        let mut stmt = conn_guard.prepare("SELECT id, name, created_at FROM playlists")?;
        let rows = stmt.query_map([], |row| {
            Ok(Playlist {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;
        rows.collect()
    }

    pub fn add_track_to_playlist(&self, playlist_id: i64, track_id: i64) -> Result<()> {
        let conn_guard = self.connection.lock().unwrap();
        let max_pos: i32 = conn_guard.query_row(
            "SELECT COALESCE(MAX(position), 0) FROM playlist_tracks WHERE playlist_id = ?1",
            params![playlist_id],
            |row| row.get(0),
        )?;
        conn_guard.execute(
            "INSERT OR IGNORE INTO playlist_tracks (playlist_id, track_id, position) VALUES (?1, ?2, ?3)",
            params![playlist_id, track_id, max_pos + 1],
        )?;
        Ok(())
    }

    pub fn get_playlist_tracks(&self, playlist_id: i64) -> Result<Vec<Track>> {
        let conn_guard = self.connection.lock().unwrap();
        let mut stmt = conn_guard.prepare(
            "SELECT t.id, t.title, t.artists, t.album, t.created_at, t.path
                 FROM tracks t
                 JOIN playlist_tracks pt ON t.id = pt.track_id
                 WHERE pt.playlist_id = ?1
                 ORDER BY pt.position ASC"
        )?;
        let rows = stmt.query_map(params![playlist_id], |row| {
            let artists_str: String = row.get(2)?;
            Ok(Track {
                id: row.get(0)?,
                title: row.get(1)?,
                artists: artists_str.split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect(),
                album: row.get(3)?,
                created_at: row.get(4)?,
                path: row.get(5)?,
            })
        })?;
        rows.collect()
    }

    pub fn fetch_playlist(&self, playlist_id:i64) -> Result<Playlist> {
        let conn_guard = self.connection.lock().unwrap();
        let mut stmt = conn_guard.prepare(
            "SELECT id, name, created_at FROM playlists WHERE id = ?1",
        )?;

        Ok(stmt.query_row(params![playlist_id], |row| {
            Ok(Playlist {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?)
    }

    pub fn remove_track_from_playlist(&self, playlist_id: i64, track_id: i64) -> Result<()> {
        let conn_guard = self.connection.lock().unwrap();
        conn_guard.execute(
            "DELETE FROM playlist_tracks WHERE playlist_id = ?1 AND track_id = ?2",
            params![playlist_id, track_id],
        )?;
        conn_guard.execute(
            "UPDATE playlist_tracks SET position = position - 1 WHERE playlist_id = ?1 AND position > (SELECT position FROM playlist_tracks WHERE playlist_id = ?1 AND track_id = ?2)",
            params![playlist_id, track_id],
        )?;
        Ok(())
    }

    pub fn delete_playlist(&self, playlist_id: i64) -> Result<()> {
        let conn_guard = self.connection.lock().unwrap();
        conn_guard.execute("DELETE FROM playlists WHERE id = ?1", params![playlist_id])?;
        Ok(())
    }
}