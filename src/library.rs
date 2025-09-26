use crate::repository::Repository;
use crate::types::Track;
use lofty::file::TaggedFileExt;
use lofty::picture::PictureType;
use lofty::read_from_path;
use lofty::tag::ItemKey;
use rusqlite::Error;
use std::fmt::{Debug, Display};
use std::fs;
use std::path::Path;
use std::sync::Arc;

pub struct Library {
    repo: Arc<Repository>,
}

impl Library {
    pub fn new(repository: Repository) -> Self {
        Self {
            repo: Arc::new(repository),
        }
    }

    pub fn index_dir(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    self.index_dir(&entry_path)?;
                } else {
                    self.add_track(entry_path.to_str().unwrap())?
                }
            }
        }

        Ok(())
    }

    pub fn add_track(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = match read_from_path(path) {
            Ok(f) => f,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        let mut metadata: Track = Track::default();
        metadata.path = path.to_string();

        if let Some(tag) = file.primary_tag() {
            for item in tag.items() {
                match item.key() {
                    ItemKey::TrackTitle => {
                        if let Some(v) = item.value().text() {
                            metadata.title = v.to_string();
                        }
                    }

                    ItemKey::TrackArtist => {
                        let val = item
                            .value()
                            .text()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                let Some(text) = item.value().text() else {
                                    todo!()
                                };

                                text.to_string()
                            });

                        metadata.artists = val
                            .split(',')
                            .map(|p| p.trim().to_string())
                            .filter(|p| !p.is_empty())
                            .collect();
                    }

                    ItemKey::AlbumTitle => {
                        if let Some(v) = item.value().text() {
                            metadata.album = v.to_string();
                        }
                    }

                    _ => {}
                }
            }
        }

        match self.repo.add_track(&metadata) {
            Ok(()) => Ok(()),
            Err(err) => {
                match &err {
                    Error::SqliteFailure(err_code, _) => {
                        // code of the UNIQUE constraint error
                        if err_code.extended_code == 2067 {
                            Ok(())
                        } else {
                            Err(err.into())
                        }
                    }
                    _ => Err(err.into()),
                }
            }
        }
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
        let tracks = self.repo.get_all_tracks()?;

        Ok(tracks)
    }

    pub fn get_track_by_id(&self, id: i64) -> Result<Track, Box<dyn std::error::Error>> {
        let track = self.repo.get_track_by_id(id)?;

        Ok(track)
    }

    pub fn get_cover_by_track_id(&self, id: i64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let track = self.get_track_by_id(id)?;

        let file = read_from_path(&track.path)?;

        let tag = file.primary_tag().ok_or("Нет основного тега")?;

        if let Some(cover) = tag.get_picture_type(PictureType::CoverFront) {
            Ok(cover.data().to_vec())
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Обложка не найдена")))
        }
    }
}
