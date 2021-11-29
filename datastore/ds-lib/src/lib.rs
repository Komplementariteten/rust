extern crate serde;
extern crate serde_derive;
use serde_derive::{Serialize, Deserialize};
use std::collections::hash_map::HashMap;
use std::path::Path;
use std::path::PathBuf;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
    pub index: HashMap<String, u64>,
    pub data: Vec<u8>
}

pub struct Datastore {
    base_dir: PathBuf,
    known_stores: Vec<String>
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