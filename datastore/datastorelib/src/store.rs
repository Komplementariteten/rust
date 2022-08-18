use crate::datastore::{DataOption, StoreSerializationError};
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

    ///
    ///
    /// # Arguments
    ///
    /// * `index`:
    /// * `item`:
    ///
    /// returns: Option<DataError>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn replace<T: Serialize>(
        &mut self,
        index: &str,
        item: T,
        indexes: Vec<String>,
        options: Option<DataOption>,
    ) -> Option<DataError> {
        let result = self.remove(index);
        if result.is_err() {
            return Some(DataError::NotFound);
        }
        let id = self.index_to_id(index).unwrap();
        let mut s = flexbuffers::FlexbufferSerializer::new();
        let data = match item.serialize(&mut s) {
            Ok(_) => s.take_buffer(),
            Err(_) => return Some(DataError::SerializationError),
        };
        self.data.insert(index.to_string(), data);
        self.know_index.push(id);
        self.index.insert(index.to_string(), vec![id]);
        for index in indexes.iter() {
            if let Some(vec) = self.index.get_mut(index) {
                vec.push(id);
            } else {
                self.index.insert(index.clone(), vec![id]);
            }
        }
        if options.is_some() {
            self.meta.insert(index.to_string(), options.unwrap());
        }
        return None;
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `index`:
    ///
    /// returns: Result<Vec<u8, Global>, DataError>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn remove(&mut self, index: &str) -> Result<Vec<u8>, DataError> {
        let id = match self.index_to_id(index) {
            Some(id) => id,
            None => return Err(DataError::IndexNotFound),
        };
        let d = match self.pop_raw(&id) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(d)
    }

    fn pop_raw(&mut self, index: &u64) -> Result<Vec<u8>, DataError> {
        let index = self.know_index.iter().position(|n| *n == *index);
        if let Some(found_index) = index {
            let removed = self.know_index.remove(found_index);
            self.meta.remove(format_inx!(removed).as_str());
            let removed_data = match self.data.remove(format_inx!(removed).as_str()) {
                None => Vec::new(),
                Some(v) => v,
            };
            self.index.remove(format_inx!(removed).as_str());
            for (_key, value) in self.index.iter_mut() {
                if let Some(pos) = value.iter().position(|&i| i == removed) {
                    value.remove(pos);
                }
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

    pub fn has_object(&mut self, index: &str) -> bool {
        self.data.contains_key(index)
    }

    pub fn get_object<T: DeserializeOwned>(&self, index: &str) -> Option<T> {
        let data = match self.data.get(index) {
            Some(v) => v,
            None => return None,
        };
        let reader = match flexbuffers::Reader::get_root(data.as_slice()) {
            Ok(r) => r,
            Err(_err) => return None,
        };
        if let Ok(item) = T::deserialize(reader) {
            return Some(item);
        }
        None
    }

    pub fn set_object<T: Serialize>(&mut self, index: &str, item: T) {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        let data = match item.serialize(&mut s) {
            Ok(_) => s.take_buffer(),
            Err(e) => panic!("Flexbuffer serialization Error: {:?}", e),
        };
        self.data.insert(index.to_string(), data);
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `item`:
    /// * `indexes`:
    /// * `options`:
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
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

    pub fn lookup_as_vec<T: DeserializeOwned + PartialEq>(
        &self,
        term: &str,
    ) -> Option<Vec<(String, T)>> {
        let mut result = Vec::new();
        let indexes = match self.index.get(term) {
            Some(inx) => inx,
            None => return None,
        };
        for id in indexes.iter() {
            let inx = format_inx!(id);
            if let Some(r) = self.get(inx.as_str()) {
                let r_tuple: (String, T) = (inx, r);
                if !result.contains(&r_tuple) {
                    result.push(r_tuple);
                }
            }
        }

        return Some(result);
    }

    pub fn lookup_as_hashmap<T: DeserializeOwned>(&self, term: &str) -> Option<HashMap<String, T>> {
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

        if let Ok(item) = T::deserialize(reader) {
            return Some(item);
        }
        None
    }

    pub fn has_index(&mut self, index: &str) -> bool {
        self.index.contains_key(index)
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
