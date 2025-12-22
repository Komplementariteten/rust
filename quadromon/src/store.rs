use crate::consts::APP_NAME;
use crate::sensor_reader::ReadResult;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{Duration, SystemTime};

const MAX_ITEMS: usize = 1024;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct StoreItem {
    pub item_name: String,
    pub time_stamp: Duration,
    pub value: u32,
}

impl Into<StoreItem> for ReadResult {
    fn into(self) -> StoreItem {
        let v = self.value_as_u32();
        StoreItem {
            item_name: self.get_ui_name(),
            time_stamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            value: v,
        }
    }
}

pub(crate) fn store(data: &Vec<StoreItem>) {
    let store_dir = dirs::data_local_dir()
        .unwrap_or(dirs::home_dir().expect("Failed to determine home directory."))
        .join(APP_NAME);
    if !store_dir.exists() {
        fs::create_dir_all(&store_dir).expect("Failed to create home directory");
    }
    let store_file = store_dir.join("store.json");
    if !store_file.exists() {
        let json_text = serde_json::to_string(&data).expect("Failed to serialize store data");
        fs::write(&store_file, json_text).expect("Failed to write store file");
        return;
    }
    let current = fs::read_to_string(&store_file).expect("Failed to read store file");
    let mut date_vec =
        serde_json::from_str::<Vec<StoreItem>>(&current).expect("Failed to parse store file");
    for i in data {
        let item = i.clone();
        date_vec.push(item);
    }
    let json_text = serde_json::to_string(&date_vec).expect("Failed to serialize store data");
    fs::write(&store_file, json_text).expect("Failed to write store file");
}

pub(crate) fn load() -> Result<Vec<StoreItem>, String> {
    let store_dir = dirs::data_local_dir()
        .unwrap_or(dirs::home_dir().expect("Failed to determine home directory."))
        .join(APP_NAME);
    let store_file = store_dir.join("store.json");
    if !store_file.exists() {
        return Ok(vec![]);
    }

    let stored_text = match fs::read(&store_file) {
        Ok(text) => text,
        Err(e) => return Err(format!("Failed to read store file: {}", e)),
    };

    let mut items: Vec<StoreItem> = match serde_json::from_slice(&stored_text)
        .map_err(|e| format!("Failed to parse store file: {}", e))
    {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    if items.len() > MAX_ITEMS {
        items.sort_by(|a, b| b.time_stamp.cmp(&a.time_stamp));
        items.truncate(MAX_ITEMS);
        // let _  = items.drain(..MAX_ITEMS).collect();
    }

    Ok(items)
}
