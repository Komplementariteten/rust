use crate::{DataOption, StoreSerializationError};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub enum DataError {
    PopError,
    IndexNotFound,
    DeserializationError,
    NotFound,
    SerializationError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
    index: HashMap<String, Vec<u64>>,
    meta: HashMap<String, DataOption>,
    data: HashMap<String, Vec<u8>>,
    know_index: Vec<u64>,
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

    pub fn remove(&mut self, index: String) -> Result<Vec<u8>, DataError> {
        let id = match self.index_to_id(&index) {
            Some(id) => id,
            None => return Err(DataError::IndexNotFound),
        };
        let d = match self.pop(&id) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(d)
    }

    pub fn pop(&mut self, index: &u64) -> Result<Vec<u8>, DataError> {
        let index = self.know_index.iter().position(|n| *n == *index);
        if let Some(found_index) = index {
            let removed = self.know_index.remove(found_index);
            self.meta.remove(format_inx!(removed).as_str());
            let removed_data = match self.data.remove(format_inx!(removed).as_str()) {
                None => Vec::new(),
                Some(v) => v,
            };
            self.index.remove(format_inx!(removed).as_str());
            let mut keys_to_remove = Vec::new();
            for (key, value) in self.index.iter() {
                if *value.first().unwrap() == removed {
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
        match self.data.is_empty() {
            false => self.data.len(),
            true => 0,
        }
    }

    pub fn inx_len(&self) -> usize {
        match self.index.is_empty() {
            false => self.index.len(),
            true => 0,
        }
    }
    fn index_to_id(&self, index: &str) -> Option<u64> {
        let inx_str = index[5..].to_string().clone();
        let id = match inx_str.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return None,
        };
        Some(id)
    }

    pub fn add_with_index<T: Serialize>(
        &mut self,
        item: T,
        indexes: Vec<String>,
        options: Option<DataOption>,
    ) -> String {
        let inx = self.add_single(item);
        let id = self.index_to_id(&inx).unwrap();
        for index in indexes.iter() {
            if let Some(vec) = self.index.get_mut(index) {
                vec.push(id);
            } else {
                self.index.insert(index.clone(), vec![id]);
            }
        }
        if let Some(opt) = options {
            self.meta.insert(inx.clone(), opt);
        }
        inx
    }

    pub fn add_single<T: Serialize>(&mut self, item: T) -> String {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        let data = match item.serialize(&mut s) {
            Ok(_) => s.take_buffer(),
            Err(e) => panic!("Flexbuffer serialization Error: {:?}", e),
        };

        let inx = self.add_raw(data);
        return format_inx!(inx);
    }

    pub fn find<T: DeserializeOwned>(&self, term: &str) -> Option<HashMap<String, T>> {
        let mut result = HashMap::new();
        let indexes = match self.index.get(term) {
            Some(inx) => inx,
            None => return None,
        };
        for index in indexes.iter() {
            let inx = format_inx!(index);
            if let Some(r) = self.get(inx.as_str()) {
                if !result.contains_key(&inx) {
                    result.insert(inx, r);
                }
            }
        }
        Some(result)
    }

    pub fn get<T: DeserializeOwned>(&self, index: &str) -> Option<T> {
        if !self.data.contains_key(index) {
            return None;
        }
        let data = match self.data.get(index) {
            Some(d) => d.clone(),
            None => return None,
        };
        let reader = match flexbuffers::Reader::get_root(data.as_slice()) {
            Ok(r) => r,
            Err(_err) => return None,
        };

        let item = match T::deserialize(reader) {
            Ok(i) => i,
            Err(_err) => return None,
        };

        Some(item)
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
        let inx_str = format_inx!(inx);
        self.index.insert(inx_str.clone(), vec![inx]);
        self.data.insert(inx_str.clone(), data);
        self.know_index.push(inx);
        inx
    }
}
