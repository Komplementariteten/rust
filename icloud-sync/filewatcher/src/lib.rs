pub mod filescanner;

use std::error::Error;
use std::path::Path;
use datastore::storage_manager::StorageManager;
use crate::filescanner::PathFileEntry;

const CACHE_KEY: &str = "fc_0.1";
const WATCH_FOLDER_KEY: &str = "watch-folder";

pub struct FileWatcher {
    handle: StorageManager,
    gen_hash: bool
}

pub type FileWatcherError = Box<dyn Error + Sync + Send + 'static>;

impl FileWatcher {
    pub fn new<S:AsRef<Path>>(folder: S, cache_path: S, with_filehash: bool) -> Result<FileWatcher, FileWatcherError> {
        let folder_string = match folder.as_ref().to_str() {
            Some(str_ref) => str_ref.to_string(),
            None => return Err(FileWatcherError::try_from("Folder could not be converted to string").unwrap())
        };

        match StorageManager::default( cache_path) {
            Ok(mut sm) => {
                sm.set_kv(WATCH_FOLDER_KEY, folder_string);
                Ok(FileWatcher {
                    handle: sm,
                    gen_hash: with_filehash
                })
            },
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn list(&mut self) -> Option<Vec<PathFileEntry>> {
        let sync_folder:String = self.handle.get_kv(WATCH_FOLDER_KEY)?;
        filescanner::scan_ordered(sync_folder, self.gen_hash)
    }

    pub fn ack(&mut self, direntry: PathFileEntry) {
        let cache_lookup:Option<Vec<PathFileEntry>> = self.handle.get_kv_vector(CACHE_KEY);
        if cache_lookup.is_none() {
            let new_cache = vec![direntry];
            self.handle.set_kv_vector(CACHE_KEY, new_cache);
        } else {
            // It is guaranteed to have some
            let mut cache = cache_lookup.unwrap();
            cache.push(direntry);
            self.handle.set_kv_vector(CACHE_KEY, cache);
        }
    }

    pub fn sync(&self) -> Option<Vec<PathFileEntry>> {
        let sync_folder:String = self.handle.get_kv(WATCH_FOLDER_KEY)?;
        let sr = match filescanner::scan_ordered(sync_folder, self.gen_hash) {
            Some(v) => v,
            None => return None
        };
        let cache:Option<Vec<PathFileEntry>> = self.handle.get_kv_vector(CACHE_KEY);
        match cache {
            Some(c) => {
                let mut result = Vec::<PathFileEntry>::new();
                for entry in &sr {
                    if entry.modified > c.first().unwrap().modified {
                        result.push(entry.clone());
                    }
                }
                if c.len() < sr.len() {
                    let _ = sr.iter().filter(| b | c.contains(b)).map(| i | result.push(i.clone()));
                }
                if result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            },
            None => Some(sr)
        }
    }
}