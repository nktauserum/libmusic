use std::sync::Arc;
use lofty::file::TaggedFileExt;
use lofty::read_from_path;
use lofty::tag::ItemKey;
use std::fs;
use std::path::Path;
use crate::repository::Repository;
use crate::types::Track;

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
                    // TODO: Option
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

        Ok(self.repo.add_track(&metadata)?)
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
        let tracks = self.repo.get_all_tracks()?;

        Ok(tracks)
    }

    pub fn get_track_by_id(&self, id: i64) -> Result<Track, Box<dyn std::error::Error>> {
        let track = self.repo.get_track_by_id(id)?;

        Ok(track)
    }
}
