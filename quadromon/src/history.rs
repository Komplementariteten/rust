use std::collections::{HashMap, HashSet};
use crate::consts::HIST_SIZE;
use crate::store::StoreItem;

pub struct History{
    pub max_values: HashMap<String, u64>,
    pub history: HashMap<String, Vec<u64>>
}

impl History {
    pub(crate) fn from_store(stored: Vec<StoreItem>) -> History {
        let mut map = HashMap::new();
        let mut hist_map = HashMap::new();
          
        let keys = stored.iter().map(| s| s.item_name.clone()).collect::<HashSet<String>>();
        for key in keys {
            let values = stored.iter().filter(|s| s.item_name == key).map(| m | m.value as u64);
            let max = values.clone().max().unwrap_or(0);
            let total_len = values.clone().count();
            let len = match total_len >= HIST_SIZE {
                true => HIST_SIZE,
                false => total_len
            };
            let last_thrity = values.collect::<Vec<u64>>()[total_len - len..total_len].to_vec();
            hist_map.insert(key.clone(), last_thrity);
            map.insert(key, max);
        }

        History {
            max_values: map,
            history: hist_map
        }
    }

    pub(crate) fn get_history(&self) -> HashMap<String, Vec<u64>> {
        self.history.clone()
    }
}