use dslib::{DataOption, Schema, StoreableWithSchema};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::remove_dir_all;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TestStruct {
    pub name: String,
    pub id: i32,
    pub content: String,
}

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct KeyValueItem {
    element_ids: Vec<String>,
    pub content: String,
}

impl dslib::KeyValueElement for KeyValueItem {}

impl KeyValueItem {
    #[allow(dead_code)]
    pub fn new() -> KeyValueItem {
        let str = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100)
            .map(char::from)
            .collect();
        KeyValueItem {
            element_ids: Vec::new(),
            content: str,
        }
    }
}
pub const STORE_TAB: &str = "test-data";

impl Schema for TestStruct {
    fn indexes(&self) -> Vec<String> {
        let mut v = Vec::new();
        v.push(format!("name={}", self.name));
        v.push(format!("Ind={}", self.id));
        v.push("test-struct".to_string());
        return v;
    }

    fn desc(&self) -> Option<DataOption> {
        Some(DataOption {})
    }

    fn name(&self) -> &str {
        STORE_TAB
    }
}

impl TestStruct {
    #[allow(dead_code)]
    pub fn new_rng() -> TestStruct {
        let mut rng = rand::thread_rng();
        let str = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        TestStruct {
            name: format!("{}", rng.gen::<i128>()),
            id: rng.gen(),
            content: str,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TestStoreStruct {
    pub payload: Vec<u64>,
    pub d: HashMap<String, String>,
    pub o: f32,
}

impl TestStoreStruct {
    #[allow(dead_code)]
    pub fn new() -> TestStoreStruct {
        let mut rng = rand::thread_rng();
        TestStoreStruct {
            o: rng.gen(),
            payload: Vec::new(),
            d: HashMap::new(),
        }
    }
}

impl StoreableWithSchema for TestStruct {}

#[allow(dead_code)]
pub fn cleanup_test(p: PathBuf) {
    let _ = remove_dir_all(p);
}
