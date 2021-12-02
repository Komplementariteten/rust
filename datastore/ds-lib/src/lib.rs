extern crate flexbuffers;
extern crate serde;
extern crate serde_derive;

use serde::{Deserialize, Serialize};
use serde_derive::*;
use std::borrow::Borrow;
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
    IndexNotFound,
    DeserializationError,
    NotFound,
    SerializationError,
}

macro_rules! format_inx {
    ($t: expr) => {
        format!("{}", stringify!(t));
    };
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
            self.meta.remove(format_inx!(removed).as_str());
            let removed_data = self.data.remove(format_inx!(removed).as_str()).unwrap();
            self.index.remove(format!("_id={}", removed).as_str());
            let mut keys_to_remove = Vec::new();
            for (key, &value) in self.index.iter() {
                if value == removed {
                    keys_to_remove.push(key.clone());
                }
            }

            for key in keys_to_remove.iter() {
                self.index.remove(key);
            }

            return Ok(removed_data);
        }
        Err(DataError::NotFound)
    }

    fn new_index(&mut self) -> u64 {
        self.know_index.sort();
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

    pub fn add<T: Serialize>(&mut self, item: T) -> Result<String, DataError> {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        let data = match item.serialize(&mut s) {
            Ok(_) => s.take_buffer(),
            Err(_) => return Err(DataError::SerializationError),
        };

        let inx = self.add_raw(data);
        return Ok(format_inx!(inx));
    }

    pub fn get<'a, T: Deserialize<'a>>(&self, index: &str) -> Result<T, DataError> {
        if !self.data.contains_key(index) {
            return Err(DataError::IndexNotFound);
        }
        let data = match self.data.get(index) {
            Some(d) => d.clone(),
            None => return Err(DataError::NotFound),
        };
        let reader = match flexbuffers::Reader::get_root(data.as_slice()) {
            Ok(r) => r,
            Err(_err) => return Err(DataError::DeserializationError),
        };

        let item = match T::deserialize(reader) {
            Ok(i) => i,
            Err(_err) => return Err(DataError::DeserializationError),
        };

        Ok(item)
    }

    pub fn load(data: Vec<u8>) -> Result<Store, StoreSerializationError> {
        let reader = match flexbuffers::Reader::get_root(data.as_slice()) {
            Ok(reader) => reader,
            Err(_err) => return Err(StoreSerializationError::ReaderError),
        };

        let store = match Store::deserialize(reader) {
            Ok(s) => s,
            Err(_err) => return Err(StoreSerializationError::DeserializeError),
        };
        Ok(store)
    }

    pub fn as_slice(&self) -> Result<Vec<u8>, StoreSerializationError> {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        match self.serialize(&mut s) {
            Ok(_) => Ok(s.take_buffer()),
            Err(_) => Err(StoreSerializationError::SerializeError),
        }
    }

    pub fn add_raw(&mut self, data: Vec<u8>) -> u64 {
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
