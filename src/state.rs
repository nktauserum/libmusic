use std::sync::Arc;
use crate::library::Library;
use crate::repository::Repository;

pub struct AppState {
    pub lib: Library
}

impl AppState {
    pub fn new(db_path: &str) -> Arc<Self> {
        let repo = Repository::new(db_path).unwrap();
        let lib = Library::new(repo);

        Arc::new(Self {
            lib
        })
    }
}