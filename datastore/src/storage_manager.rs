extern crate serde_traitobject;

use datastorelib::batch::Batch;
use datastorelib::datastore::{
    Datastore, KeyValueElement, StoreableWithSchema, STORE_FILE_EXTENSION,
};
use datastorelib::datastore_cfg::DatastoreConfig;
use serde::de::DeserializeOwned;
use serde_traitobject as st;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error;
use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, read_dir, remove_file, File};
use std::ops::Drop;
use std::path::Path;

const MANAGER_LOCK: &str = ".lock";
// const VEC_STORE: &str = "vec";
const HM_STORE: &str = "hm";

#[derive(Debug, PartialEq)]
pub enum StorageManagerError {
    ManagerAlreadyExists,
    FailedToCreateLock,
}

impl Display for StorageManagerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Storage manager error")
    }
}

impl error::Error for StorageManagerError {}
#[derive(Debug)]
pub struct StorageManager {
    store: Datastore,
    cfg: DatastoreConfig,
    save_on_op: bool,
}

impl Drop for StorageManager {
    fn drop(&mut self) {
        self.store.close();
        self.cfg.save();
        let lck_p = self.cfg.cfg_path.join(MANAGER_LOCK);
        remove_file(lck_p).expect("Failed to remove Lock file");
    }
}

pub struct BatchManager<'a, T: StoreableWithSchema> {
    b: Batch<T>,
    save_on_commit: bool,
    s: &'a mut Datastore,
}

impl<'a, T: StoreableWithSchema> BatchManager<'a, T> {
    pub fn new(d: &'a mut Datastore, save_on_commit: bool) -> BatchManager<'a, T> {
        BatchManager {
            b: Batch::new(),
            s: d,
            save_on_commit: save_on_commit,
        }
    }
    pub fn add(&'a mut self, item: T) -> &mut Self {
        self.b.add(item);
        return self;
    }

    pub fn commit(&'a mut self, table_name: &str) -> Vec<String> {
        let b = std::mem::replace(&mut self.b, Batch::new());
        let indexes = self.s.execute(table_name, b);
        if self.save_on_commit {
            self.s.save_all();
        }
        indexes
    }
}

impl StorageManager {
    pub fn sync(&mut self) -> Option<()> {
        self.store.save_all()
    }
    pub fn is_locked(&self) -> bool {
        let lock_path = self.cfg.cfg_path.join(MANAGER_LOCK);
        lock_path.exists()
    }
    pub fn add_any(&mut self, item: HashMap<String, st::Box<dyn st::Any>>) -> String {
        let inx = self.store.add_hm(HM_STORE, item);
        if self.save_on_op {
            self.store.save_all();
        }
        inx
    }
    pub fn get_kv<T: KeyValueElement>(&mut self, key: &str) -> Option<T> {
        self.store.get_kvp_value(self.cfg.kv_store.as_str(), key)
    }
    pub fn set_kv<T: KeyValueElement>(&mut self, key: &str, value: T) {
        self.store.set_kvp(self.cfg.kv_store.as_str(), key, value);
        if self.save_on_op {
            self.store.save_all();
        }
    }

    pub fn get_kv_vector<T: DeserializeOwned>(&mut self, key: &str) -> Option<Vec<T>> {
        self.store
            .get_data_by_index(self.cfg.kv_store.as_str(), key)
    }

    pub fn set_kv_vector<T: serde::Serialize>(&mut self, key: &str, value: Vec<T>) {
        self.store
            .set_data_by_index(self.cfg.kv_store.as_str(), key, value);
        if self.save_on_op {
            self.store.save_all();
        }
    }

    pub fn batch<T: StoreableWithSchema>(&mut self) -> BatchManager<T> {
        BatchManager::new(self.store.borrow_mut(), self.save_on_op)
    }
    pub fn default<P: AsRef<Path>>(base_dir: P) -> Result<StorageManager, StorageManagerError> {
        StorageManager::new(base_dir, true)
    }

    pub fn new<P: AsRef<Path>>(
        base_dir: P,
        save_after_op: bool,
    ) -> Result<StorageManager, StorageManagerError> {
        let cfg = DatastoreConfig::new(base_dir);
        let p = Path::new(&cfg.base_path);
        let lock_path = cfg.cfg_path.join(MANAGER_LOCK);
        let mut reopen_ds = true;
        if !&cfg.cfg_path.exists() {
            reopen_ds = true;
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
        let mut ds = Datastore::new(p.to_path_buf());
        if reopen_ds {
            // Path is garanteed to work
            let rd = read_dir(p)
                .expect("this should not happen")
                .filter_map(|de| de.ok());
            for de in rd {
                if de.path().is_file()
                    && de.path().to_str().unwrap().ends_with(STORE_FILE_EXTENSION)
                {
                    let path = de.path();
                    let name = path.file_stem().unwrap().to_str().unwrap();
                    ds.open(name);
                }
            }
        }

        Ok(StorageManager {
            save_on_op: save_after_op,
            store: ds,
            cfg: cfg,
        })
    }
}
