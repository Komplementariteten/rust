use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::{DataStoreError, Schema, Store};
use crate::store::DataError;

pub enum TransactionError {}

pub struct Transaction {
    store: *Store,
    pub store_name:String,
    pub changes: u64,
}

impl Transaction {
    pub fn new(store: &mut Store, store_name: &str) -> Transaction {
        Transaction {
            changes: 0,
            store: store,
            store_name:store_name.to_string()
        }
    }
    pub fn add<T: Serialize + Schema>(&mut self, item: T) -> String {
        self.changes = self.changes + 1;
        self.store.add_with_index(&item, item.indexes(), item.desc())
    }
    pub fn remove(&mut self, index: &str) -> Option<DataError> {
        let inx_str = index[5..].to_string().clone();
        let id = inx_str.parse::<u64>().unwrap();
        let pop_result = self.store.pop(&id);
        self.changes = self.changes + 1;
        pop_result.err()
    }

    pub fn get<T: DeserializeOwned + Schema>(
        &self,
        index: &str,
    ) -> Option<T> {
        self.store.get(index);
    }
}