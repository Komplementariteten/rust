extern crate flexbuffers;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use crate::store::Store;
use crate::transaction::{Transaction, TransactionError};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
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

macro_rules! format_inx {
    ($t: tt) => {
        format!("_$id={}", $t)
    };
}

pub mod store;
pub mod transaction;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DataOption {}

pub trait Schema {
    fn indexes(&self) -> Vec<String>;
    fn desc(&self) -> Option<DataOption>;
    fn name(&self) -> &str;
}

const STORE_FILE_EXTENSION: &str = "sdbf";

#[derive(Debug)]
pub struct Datastore {
    is_dirty: bool,
    base_dir: PathBuf,
    stores: HashMap<String, Store>,
    transactions: Vec<String>,
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
}

impl Datastore {
    pub fn new(path: PathBuf) -> Datastore {
        if !path.exists() {
            create_dir_all(&path).expect("Can't create Path");
        }
        Datastore {
            transactions: Vec::new(),
            is_dirty: false,
            base_dir: path,
            stores: HashMap::new(),
        }
    }
    pub fn start(&mut self, name: &str) -> Option<Transaction> {
        let store = match self.stores.get_mut(name) {
            None => return None,
            Some(s) => s,
        };
        Some(Transaction::new(store, name))
    }
    pub fn commit(&mut self, tx: Transaction) -> Option<u64> {
        if tx.changes == 0 {
            None
        }
        self.save(tx.store_name.as_str());
        Some(tx.changes)
    }
    pub fn get<T: DeserializeOwned + Schema>(
        &mut self,
        store_name: &str,
        index: &str,
    ) -> Option<T> {
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
        let data = r.fill_buf();

        if data.is_err() {
            return None;
        }

        let reader = match flexbuffers::Reader::get_root(data.unwrap()) {
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

    pub fn add<T: Serialize + Schema>(&mut self, item: T) -> Result<String, DataStoreError> {
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
            None
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
