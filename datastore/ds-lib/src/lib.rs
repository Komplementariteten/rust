extern crate flexbuffers;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use crate::batch::Batch;
use crate::store::Store;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::string::String;
/*
macro_rules! zoom_and_enhance {
    (struct $name:ident { $($fname:ident : $ftype:ty), *}) => {
        struct $name  {
            $($fname : $ftype), *
        }

        impl $name {
            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)), *];
                NAMES
            }
        }
    }
}
*/
#[macro_export]
macro_rules! format_inx {
    ($t: tt) => {
        format!("_$id={}", $t)
    };
}

pub mod batch;
pub mod store;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataOption {}

pub trait Schema {
    fn indexes(&self) -> Vec<String>;
    fn desc(&self) -> Option<DataOption>;
    fn name(&self) -> &str;
}

pub trait StoreableWithSchema: Serialize + Schema + DeserializeOwned {}
pub trait KeyValueElement: PartialEq + DeserializeOwned + Serialize + Clone + PartialEq {}

const STORE_FILE_EXTENSION: &str = "sdbf";
const KEY_VALUE_STORE: &str = "key_value_store";

#[derive(Debug)]
pub struct Datastore {
    is_dirty: bool,
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
            is_dirty: false,
            base_dir: path,
            stores: HashMap::new(),
        }
    }

    pub fn lookup<T: DeserializeOwned + PartialEq>(
        self,
        store_name: &str,
        index: &str,
    ) -> Option<Vec<(String, T)>> {
        if let Some(store) = self.stores.get(store_name) {
            return store.lookup_as_vec::<T>(index);
        }
        return None;
    }

    pub fn get_kvp_value<T: KeyValueElement>(self, key: &str) -> Option<T> {
        if let Some(s) = self.stores.get(KEY_VALUE_STORE) {
            let lookup_opt = s.lookup_as_vec(key);
            if lookup_opt.is_some() {
                let lookup_res = lookup_opt.unwrap();
                let kvp_opt = lookup_res.first();
                if kvp_opt.is_some() {
                    let value: (String, T) = kvp_opt.unwrap().clone();
                    return Some(value.1);
                }
                return None;
            }
            return None;
        }
        return None;
    }

    pub fn set_kvp<T: KeyValueElement>(&mut self, key: &str, value: T) {
        if let Some(kvp_store) = self.stores.get_mut(KEY_VALUE_STORE) {
            kvp_store.add_with_index(&value, vec![key.to_string()], None);
        } else {
            let mut s = Store::new();
            s.add_with_index(&value, vec![key.to_string()], None);
            self.stores.insert(KEY_VALUE_STORE.to_string(), s);
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
    fn open(&mut self, name: &str) -> Option<&Store> {
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
        self.save(&name);
        self.stores.insert(name.to_string(), s);
        self.is_dirty = true;
        Ok(inx)
    }
    pub fn save_all(&mut self) -> Option<()> {
        if !self.is_dirty {
            return None;
        }
        for kvp in &self.stores {
            self.save(kvp.0.as_str())
        }
        self.is_dirty = false;
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
