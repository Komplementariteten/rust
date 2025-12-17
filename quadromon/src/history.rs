use std::collections::{HashMap, HashSet};
use crate::store::StoreItem;

pub struct History{
    pub individual_values: HashMap<String, u32>,
    pub history: HashMap<String, Vec<u64>>
}

impl History {
    pub(crate) fn from_store(stored: Vec<StoreItem>) -> History {
        let mut map = HashMap::new();
        let mut hist_map = HashMap::new();
          
        let keys = stored.iter().map(| s| s.item_name.clone()).collect::<HashSet<String>>();
        for key in keys {
            let max = stored.iter().filter(|s| s.item_name == key).map(| m | m.value).max().unwrap_or(0);
            map.insert(key, max);
        }

        History {
            individual_values: map
        }
    }

    pub(crate) fn get_history(&self) -> HashMap<String, Vec<u64>> {
        let mut map = HashMap::new();
        for (k, v) in &self.individ
        HashMap::from([("ABC".to_string(), vec![1, 2, 3]), ("DEF".to_string(), vec![10,11,12])])
    }
}