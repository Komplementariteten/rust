use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const MANAGER_CFG: &str = "mng.cfg";
const MANAGER_CONFIG_PATH: &str = ".config";
const KV_TAB: &str = "kv";

#[derive(Deserialize, Serialize, Debug)]
pub struct DatastoreConfig {
    pub base_path: String,
    pub cfg_path: PathBuf,
    pub kv_store: String,
}

impl DatastoreConfig {
    pub fn new(p: &str) -> DatastoreConfig {
        let base_path = Path::new(p);
        let cfg_path = base_path.join(MANAGER_CONFIG_PATH);
        DatastoreConfig {
            base_path: p.to_string(),
            cfg_path: cfg_path,
            kv_store: KV_TAB.to_string(),
        }
    }
    pub fn save(&self) {
        let base_p = Path::new(self.base_path.as_str());
        let cfg_p = base_p.join(MANAGER_CONFIG_PATH);
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
