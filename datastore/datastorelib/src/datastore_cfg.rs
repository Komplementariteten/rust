// use serde::Deserialize;
use serde::Serialize;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const MANAGER_CFG: &str = "mng.cfg";
const MANAGER_CONFIG_PATH: &str = ".config";
const KV_TAB: &str = "kv";

#[derive(Serialize, Debug)]
pub struct DatastoreConfig {
    pub base_path: PathBuf,
    pub cfg_path: PathBuf,
    pub kv_store: String,
}

impl DatastoreConfig {
    pub fn new<P: AsRef<Path>>(p: P) -> DatastoreConfig {
        let cfg_path = p.as_ref().join(MANAGER_CONFIG_PATH);
        DatastoreConfig {
            base_path: p.as_ref().to_path_buf(),
            cfg_path: cfg_path,
            kv_store: KV_TAB.to_string(),
        }
    }
    pub fn save(&self) {
        let cfg_p = self.base_path.join(MANAGER_CONFIG_PATH);
        if !cfg_p.exists() {
            create_dir_all(&cfg_p).expect("Failed to create config directory");
        }
        let cfg_file = cfg_p.join(MANAGER_CFG);
        let mut fb = flexbuffers::FlexbufferSerializer::new();
        self.serialize(&mut fb)
            .expect("Failed to save configuration");
        let data = fb.take_buffer();
        let f = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(cfg_file)
            .expect("Failed to open config File to save");
        let mut w = BufWriter::new(f);
        w.write_all(data.as_slice())
            .expect("Unable to config File data");
    }
}
