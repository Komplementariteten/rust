use std::collections::hash_map::HashMap;
use std::path::Path;
use std::path::PathBuf;


pub struct Datastore {
    base_dir: PathBuf,
    known_stores: Vec<String>
}

struct Store {
    index: HashMap<String, u64>,
    data: Vec<u8>
}

impl Datastore {
    pub fn new(path: PathBuf) -> Datastore {
        Datastore{
            base_dir: path,
            known_stores: Vec::new()
        }
    }
    pub fn add_store(&mut self, name: String) {
        self.known_stores.push(name)        
    }
}