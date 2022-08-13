use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use datastore::storage_manager::StorageManager;
use filescanner::PathFileEntry;

const CACHE_KEY: &str = "fc_0.1";

struct Filewatcher {
    handle: StorageManager
}

pub type FileWatcherError = Box<dyn Error + Sync + Send + 'static>;
pub type FileWatcherResult = Result<Filewatcher, FileWatcherError>;

impl Filewatcher {
    fn new<S:AsRef<Path>>(cache_path: S) -> FileWatchResult {
        match StorageManager::new(cache_path) {
            Ok(sm) => Filewatcher {
                handle: sm
            },
            Err(e) => e
        }
    }

    fn sync<S:AsRef<Path>>(&mut self, folder: S) -> Option<Vec<PathFileEntry>> {
        let sr = match filescanner::scan(folder) {
            Some(v) => v,
            None => return None
        };

        if Some(cache: Vec<PathFileEntry>) = self.handle.get_kv(CACHE_KEY) {

            None
        } else {
            self.handle.set_kv(CACHE_KEY, sr.clone());
            sr
        }
    }
}