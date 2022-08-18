use crate::batch::Batch;
use crate::store::Store;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::fmt::Display;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::string::String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataOption {}

pub trait Schema {
    fn indexes(&self) -> Vec<String>;
    fn desc(&self) -> Option<DataOption>;
    fn name(&self) -> &str;
}

pub trait StoreableWithSchema: Serialize + Schema + DeserializeOwned {}
pub trait KeyValueElement:
    PartialEq + DeserializeOwned + Serialize + Clone + PartialEq + Display
{
}

impl KeyValueElement for String {}
impl KeyValueElement for i32 {}
impl KeyValueElement for f32 {}

pub const STORE_FILE_EXTENSION: &str = "sdbf";

#[derive(Debug)]
pub struct Datastore {
    base_dir: PathBuf,
    stores: HashMap<String, Store>,
}

#[derive(Debug)]
pub enum StoreSerializationError {
    DeserializeError,
    SerializeError,
    ReaderError,
}

#[derive(Debug)]
pub enum DataStoreError {
    StoreNotFound,
    DataError,
}

impl Datastore {
    pub fn new(path: PathBuf) -> Datastore {
        if !path.exists() {
            create_dir_all(&path).expect("Can't create Path");
        }
        Datastore {
            base_dir: path,
            stores: HashMap::new(),
        }
    }

    pub fn get_hm<T: DeserializeOwned>(
        &self,
        store_name: &str,
        index: &str,
    ) -> Option<HashMap<String, T>> {
        if let Some(store) = self.stores.get(store_name) {
            let r: Option<HashMap<String, T>> = store.get(index);
            return r;
        }
        return None;
    }

    fn add_raw<T: Serialize>(&mut self, store_name: &str, item: T) -> String {
        if let Some(store) = self.stores.get_mut(store_name) {
            store.add_single(item)
        } else {
            let mut vs = Store::new();
            let inx = vs.add_single(item);
            self.stores.insert(store_name.to_string(), vs);
            inx
        }
    }

    pub fn add_hm<T: Serialize>(&mut self, store_name: &str, item: HashMap<String, T>) -> String {
        self.add_raw(store_name, item)
    }

    pub fn add_vec<T: Serialize>(&mut self, store_name: &str, item: Vec<T>) -> String {
        self.add_raw(store_name, item)
    }

    pub fn get_vec<T: DeserializeOwned>(&self, store_name: &str, index: &str) -> Option<Vec<T>> {
        if let Some(store) = self.stores.get(store_name) {
            return store.get(index);
        }
        return None;
    }

    pub fn lookup<T: DeserializeOwned + PartialEq>(
        &self,
        store_name: &str,
        index: &str,
    ) -> Option<Vec<(String, T)>> {
        if let Some(store) = self.stores.get(store_name) {
            return store.lookup_as_vec::<T>(index);
        }
        return None;
    }

    pub fn get_data_by_index<T: DeserializeOwned>(&self, store_name: &str, key: &str) -> Option<T> {
        if let Some(s) = self.stores.get(store_name) {
            return s.get_object(key);
        }
        None
    }

    pub fn set_data_by_index<T: Serialize>(&mut self, store_name: &str, key: &str, value: T) {
        if let Some(s) = self.stores.get_mut(store_name) {
            s.set_object(key, value);
        } else {
            let mut s = Store::new();
            s.set_object(key, value);
            self.stores.insert(store_name.to_string(), s);
        }
    }

    pub fn get_kvp_value<T: KeyValueElement>(&mut self, store_name: &str, key: &str) -> Option<T> {
        if let Some(s) = self.stores.get_mut(store_name) {
            let obj_lookup: Option<T> = s.get_object(key);
            return obj_lookup;
        }
        return None;
    }

    pub fn set_kvp<T: KeyValueElement>(&mut self, store_name: &str, key: &str, value: T) {
        if let Some(kvp_store) = self.stores.get_mut(store_name) {
            println!("KVP-Store . key: {} => {}", key, value);
            // kvp_store.replace()
            kvp_store.set_object(key, value);
        } else {
            let mut s = Store::new();
            println!("new KVP-Store - key: {} => {}", &key, &value);
            s.set_object(key, value);
            self.stores.insert(store_name.to_string(), s);
        }
    }

    pub fn get<T: StoreableWithSchema>(&mut self, store_name: &str, index: &str) -> Option<T> {
        if self.stores.contains_key(store_name) {
            self.stores[store_name].get(index)
        } else {
            let item: Option<T> = match self.open(store_name) {
                Some(s) => s.get(index),
                None => None,
            };
            item
        }
    }
    pub fn remove(&mut self, store_name: &str, index: &str) -> Result<usize, DataStoreError> {
        if let Some(store) = self.stores.get_mut(store_name) {
            let rs = match store.remove(index) {
                Ok(s) => s.len(),
                Err(_) => return Err(DataStoreError::DataError),
            };
            return Ok(rs);
        }
        Err(DataStoreError::StoreNotFound)
    }
    pub fn open(&mut self, name: &str) -> Option<&Store> {
        let mut file_name = self.base_dir.join(name);
        file_name.set_extension(STORE_FILE_EXTENSION);
        let f = OpenOptions::new()
            .append(false)
            .create(false)
            .read(true)
            .open(file_name.as_path())
            .expect("Unable to open file");
        let mut r = BufReader::new(f);
        let mut data: Vec<u8> = Vec::new();
        let _ = r.read_to_end(&mut data);

        let reader = match flexbuffers::Reader::get_root(data.as_slice()) {
            Ok(reader) => reader,
            Err(_err) => return None,
        };

        let store = match Store::deserialize(reader) {
            Ok(s) => s,
            Err(_err) => return None,
        };
        self.stores.insert(name.to_string(), store);
        Some(&self.stores[&name.to_string()])
    }
    pub fn close(&mut self) {
        self.save_all();
        self.stores = HashMap::new()
    }

    pub fn execute<T: StoreableWithSchema>(
        &mut self,
        store_name: &str,
        mut b: Batch<T>,
    ) -> Vec<String> {
        let mut indexes = Vec::new();
        if let Some(store) = self.stores.get_mut(store_name) {
            for item in b.clear() {
                let inx = store.add_with_index(&item, item.indexes(), item.desc());
                indexes.push(inx);
            }
            self.save(store_name)
        } else {
            let mut store = Store::new();
            for item in b.clear() {
                let inx = store.add_with_index(&item, item.indexes(), item.desc());
                indexes.push(inx);
            }
            self.stores.insert(store_name.to_string(), store);
            self.save(store_name);
        }
        indexes
    }

    pub fn insert_and_save<T: StoreableWithSchema>(
        &mut self,
        item: T,
    ) -> Result<String, DataStoreError> {
        let name = item.name().to_string();
        if let Some(store) = self.stores.get_mut(&name) {
            let inx = store.add_with_index(&item, item.indexes(), item.desc());
            self.save(&name);
            return Ok(inx);
        }
        let mut s = Store::new();
        let inx = s.add_with_index(&item, item.indexes(), item.desc());
        self.stores.insert(name.to_string(), s);
        self.save(&name);
        Ok(inx)
    }
    pub fn save_all(&mut self) -> Option<()> {
        for kvp in &self.stores {
            self.save(kvp.0.as_str())
        }
        Some(())
    }
    fn save(&self, name: &str) {
        let mut fb = flexbuffers::FlexbufferSerializer::new();
        let data = match self.stores.get(name).serialize(&mut fb) {
            Ok(_) => fb.take_buffer(),
            Err(e) => panic!("Flexbuffer serialization Error: {:?}", e),
        };
        let mut file_name = self.base_dir.join(name);
        file_name.set_extension(STORE_FILE_EXTENSION);
        let p = file_name.as_path();
        let f = OpenOptions::new()
            .append(false)
            .truncate(true)
            .read(false)
            .write(true)
            .create(true)
            .open(p)
            .expect("Unable to open file");
        let mut w = BufWriter::new(f);
        w.write_all(data.as_slice()).expect("Unable to write data");
    }
}
