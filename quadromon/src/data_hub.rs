use crate::consts::{HIST_SIZE, STORAGE_SIZE};
use crate::sensor_reader::ReadResult;
use crate::store;
use crate::store::StoreItem;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::mpsc::Sender;

pub struct DataHub {
    pub max_values: HashMap<String, u64>,
    history: HashMap<String, Vec<u64>>,
    items: Vec<StoreItem>,
    tx: Sender<StoreItem>,
}

impl DataHub {
    pub(crate) fn new(tx: Sender<StoreItem>) -> DataHub {
        let stored = store::load().expect("Failed to load store");

        let mut map = HashMap::new();
        let mut hist_map = HashMap::new();

        let keys = stored
            .iter()
            .map(|s| s.item_name.clone())
            .collect::<HashSet<String>>();
        for key in keys {
            let values = stored
                .iter()
                .filter(|s| s.item_name == key)
                .map(|m| m.value as u64);
            let max = values.clone().max().unwrap_or(0);
            let total_len = values.clone().count();
            let len = match total_len >= HIST_SIZE {
                true => HIST_SIZE,
                false => total_len,
            };
            let last_thrity = values.collect::<Vec<u64>>()[total_len - len..total_len].to_vec();
            hist_map.insert(key.clone(), last_thrity);
            map.insert(key, max);
        }

        DataHub {
            max_values: map,
            history: hist_map,
            items: stored,
            tx: tx.clone(),
        }
    }

    pub(crate) fn sync(&mut self) {
        let items = self.rotate();
        store::store(&items);
    }

    pub(crate) fn update(&mut self, update: Vec<ReadResult>) {
        let items: Vec<StoreItem> = update.into_iter().map(|r| r.into()).collect();
        for item in items {
            if self.history.contains_key(&item.item_name) {
                let vec = self.history.get_mut(&item.item_name).unwrap();
                vec.push(item.value as u64);
            }
            self.tx.send(item.clone()).unwrap();
            self.items.push(item);
        }
    }
    fn rotate(&self) -> Vec<StoreItem> {
        let mut result = vec![];
        let mut group: HashMap<String, Vec<StoreItem>> = HashMap::new();
        let mut items = self.items.clone();
        items.sort_by(| a, b | b.time_stamp.cmp(&a.time_stamp));
        for item in items {
            group.entry(item.item_name.clone()).or_default().push(item);
        }

        for (_, mut v) in group {
            if v.len() > STORAGE_SIZE {
                let remove = v.len() - STORAGE_SIZE;
                v.drain(..remove);
            }
            for item in v {
                result.push(item);
            }
        }

        result
    }

    pub(crate) fn get_history(&self, items: usize) -> BTreeMap<String, Vec<u64>> {
        let mut r = BTreeMap::new();
        for (k, vec) in self.history.iter() {
            let d = vec.len() - items;
            let mut t = vec.clone();
            t.drain(..d);
            r.insert(k.clone(), t);
        }
        r
    }
}

impl Drop for DataHub {
    fn drop(&mut self) {
        store::store(&self.items);
    }
}
