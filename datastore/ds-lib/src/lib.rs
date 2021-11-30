extern crate flexbuffers;
extern crate serde;
extern crate serde_derive;

use crate::StoreSerializationError::{DeserializeError, ReaderError, SerializeError};
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::fmt::format;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
    index: HashMap<String, u64>,
    meta: HashMap<String, DataOption>,
    data: HashMap<String, Vec<u8>>,
    know_index: Vec<u64>,
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

pub enum DataError {
    PopError,
    NotFound,
}

impl Store {
    pub fn new() -> Store {
        Store {
            index: HashMap::new(),
            meta: HashMap::new(),
            data: HashMap::new(),
            know_index: Vec::new(),
        }
    }

    pub fn pop(&mut self, index: &u64) -> Result<Vec<u8>, DataError> {
        let index = self.know_index.iter().position(|n| n == index);
        if let Some(found_index) = index {
            let removed = self.know_index.remove(found_index);
            self.meta.remove(format!("_id={}", removed).as_str());
            let removed_data = self
                .data
                .remove(format!("_id={}", removed).as_str())
                .unwrap();
            self.index.remove(format!("_id={}", removed).as_str());
            let (keep, _) = self.index.iter().partition(|&tuple| tuple.1 == removed);
            self.index = keep;
            Ok(removed_data);
        }
        Err(DataError::NotFound)
    }

    fn new_index(&self) -> u64 {
        match self.know_index.max() {
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

    pub fn add(&mut self, data: Vec<u8>) -> u64 {
        let inx = self.new_index();
        let inx_str = format!("_id={}", inx);
        self.index.insert(inx_str.clone(), inx);
        self.data.insert(inx_str.clone(), data);
        self.meta.insert(inx_str, DataOption {});
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
