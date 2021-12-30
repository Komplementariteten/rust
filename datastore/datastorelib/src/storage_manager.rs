extern crate serde_traitobject;

use crate::batch::Batch;
use crate::datastore::{Datastore, KeyValueElement, StoreableWithSchema};
use crate::datastore_cfg::DatastoreConfig;
use serde_traitobject as st;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs::{create_dir_all, remove_file, File};
use std::ops::Drop;
use std::path::Path;

const MANAGER_LOCK: &str = ".lock";
const VEC_STORE: &str = "vec";
const HM_STORE: &str = "hm";

#[derive(Debug, PartialEq)]
pub enum StorageManagerError {
    ManagerAlreadyExists,
    FailedToCreateLock,
}

#[derive(Debug)]
pub struct StorageManager {
    store: Datastore,
    cfg: DatastoreConfig,
}

impl Drop for StorageManager {
    fn drop(&mut self) {
        self.cfg.save();
        let lck_p = self.cfg.cfg_path.join(MANAGER_LOCK);
        remove_file(lck_p).expect("Failed to remove Lock file");
    }
}

pub struct BatchManager<'a, T: StoreableWithSchema> {
    b: Batch<T>,
    s: &'a mut Datastore,
}

impl<'a, T: StoreableWithSchema> BatchManager<'a, T> {
    pub fn new(d: &'a mut Datastore) -> BatchManager<'a, T> {
        BatchManager {
            b: Batch::new(),
            s: d,
        }
    }
    pub fn add(&'a mut self, item: T) -> &mut Self {
        self.b.add(item);
        return self;
    }

    pub fn commit(&'a mut self, table_name: &str) -> Vec<String> {
        let b = std::mem::replace(&mut self.b, Batch::new());
        let indexes = self.s.execute(table_name, b);
        indexes
    }
}

impl StorageManager {
    pub fn is_locked(&self) -> bool {
        let lock_path = self.cfg.cfg_path.join(MANAGER_LOCK);
        lock_path.exists()
    }
    pub fn add_any(&mut self, item: HashMap<String, st::Box<dyn st::Any>>) -> String {
        self.store.add_hm(HM_STORE, item)
    }
    pub fn get_kv<T: KeyValueElement>(&self, key: &str) -> Option<T> {
        self.store.get_kvp_value(self.cfg.kv_store.as_str(), key)
    }
    pub fn set_kv<T: KeyValueElement>(&mut self, key: &str, value: T) {
        self.store.set_kvp(self.cfg.kv_store.as_str(), key, value);
    }
    pub fn batch<T: StoreableWithSchema>(&mut self) -> BatchManager<T> {
        BatchManager::new(self.store.borrow_mut())
    }
    pub fn new(base_dir: &str) -> Result<StorageManager, StorageManagerError> {
        let cfg = DatastoreConfig::new(base_dir);
        let p = Path::new(&cfg.base_path);
        let lock_path = cfg.cfg_path.join(MANAGER_LOCK);
        if !&cfg.cfg_path.exists() {
            create_dir_all(&cfg.cfg_path).expect("Failed to create config path");
        }
        if lock_path.exists() {
            return Err(StorageManagerError::ManagerAlreadyExists);
        } else {
            let _ = match File::create(lock_path) {
                Ok(f) => f,
                Err(_) => return Err(StorageManagerError::FailedToCreateLock),
            };
        }
        Ok(StorageManager {
            store: Datastore::new(p.to_path_buf()),
            cfg: cfg,
        })
    }
}
