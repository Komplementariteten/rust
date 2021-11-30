extern crate flexbuffers;
extern crate serde;
extern crate serde_derive;

use crate::StoreSerializationError::{DeserializeError, ReaderError, SerializeError};
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
    index: HashMap<String, usize>,
    meta: Vec<DataOption>,
    data: Vec<Vec<u8>>,
    know_index: Vec<usize>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DataOption {}

pub struct Datastore {
    base_dir: PathBuf,
    known_stores: Vec<String>,
}

pub enum StoreSerializationError {
    DeserializeError,
    SerializeError,
    ReaderError,
}

impl Store {
    pub fn new() -> Store {
        Store {
            index: HashMap::new(),
            meta: Vec::new(),
            data: Vec::new(),
            know_index: Vec::new(),
        }
    }
    fn new_index(&self) -> usize {
        match self.know_index.last() {
            Some(last) => return last + 1,
            None => 1,
        }
    }

    pub fn len(&self) -> usize {
        match self.index.is_empty() {
            false => self.data.len(),
            true => 0,
        }
    }

    pub fn load(data: Vec<u8>) -> Result<Store, StoreSerializationError> {
        let reader = match flexbuffers::Reader::get_root(data.as_slice()) {
            Ok(reader) => reader,
            Err(_err) => return Err(ReaderError),
        };

        let store = match Store::deserialize(reader) {
            Ok(s) => s,
            Err(_err) => return Err(DeserializeError),
        };
        Ok(store)
    }

    pub fn as_slice(&self) -> Result<Vec<u8>, StoreSerializationError> {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        match self.serialize(&mut s) {
            Ok(_) => Ok(s.take_buffer()),
            Err(_) => Err(SerializeError),
        }
    }

    pub fn add(&mut self, data: Vec<u8>) -> usize {
        let inx = self.new_index();
        self.index.insert("id=".to_string(), inx);
        self.data.push(data);
        self.meta.push(DataOption {});
        self.know_index.push(inx);
        inx
    }
}

impl Datastore {
    pub fn new(path: PathBuf) -> Datastore {
        Datastore {
            base_dir: path,
            known_stores: Vec::new(),
        }
    }
    pub fn add_store(&mut self, name: String) {
        self.known_stores.push(name)
    }
}
