use datastorelib::datastore::{DataOption, Schema, StoreableWithSchema};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::remove_dir_all;
use std::path::PathBuf;

const TESTSTRUCT_STORE: &str = "test-struct";
pub const STORE_TAB: &str = "test-store";

/* BEGIN KEY VALUE */

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct KeyValueItem {
    element_ids: Vec<String>,
    pub content: String,
}

impl datastorelib::datastore::KeyValueElement for KeyValueItem {}

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
/* END KEY VALUE */

/* BEGIN Test Schema */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TestSchemaStruct {
    pub name: String,
    pub id: i32,
    pub content: String,
}

impl Schema for TestSchemaStruct {
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

impl TestSchemaStruct {
    #[allow(dead_code)]
    pub fn new_rng() -> TestSchemaStruct {
        let mut rng = rand::thread_rng();
        let str = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        TestSchemaStruct {
            name: format!("{}", rng.gen::<i128>()),
            id: rng.gen(),
            content: str,
        }
    }
}
impl StoreableWithSchema for TestSchemaStruct {}

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

/* END Test Schema */

/* Begin Dyn */

#[derive(Serialize, Deserialize)]
pub struct DynTestStruct1 {
    pub f: f64,
    pub c: String,
}

impl DynTestStruct1 {
    #[allow(dead_code)]
    pub fn new_rng() -> DynTestStruct1 {
        let mut rng = rand::thread_rng();
        let str = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(200)
            .map(char::from)
            .collect();
        DynTestStruct1 {
            f: rng.gen(),
            c: str,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DynTestStruct2 {
    pub n: i64,
    pub f: f32,
}

impl DynTestStruct2 {
    #[allow(dead_code)]
    pub fn new_rng() -> DynTestStruct2 {
        let mut rng = rand::thread_rng();
        DynTestStruct2 {
            n: rng.gen(),
            f: rng.gen(),
        }
    }
}
/* End Dyn */

/* Begin Methods */
#[allow(dead_code)]
pub fn cleanup_test(p: PathBuf) {
    let _ = remove_dir_all(p);
}
/* End Methods */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OtherTestStruct {
    pub c: String,
    pub id: i8,
}

impl Schema for OtherTestStruct {
    fn indexes(&self) -> Vec<String> {
        Vec::new()
    }

    fn desc(&self) -> Option<DataOption> {
        None
    }

    fn name(&self) -> &str {
        TESTSTRUCT_STORE
    }
}

impl StoreableWithSchema for OtherTestStruct {}

impl OtherTestStruct {
    #[allow(dead_code)]
    pub fn new_rng() -> OtherTestStruct {
        let mut rng = thread_rng();
        let content = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .map(char::from)
            .collect();
        OtherTestStruct {
            c: content,
            id: rng.gen(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TestStruct {
    pub name: String,
    pub id: f32,
    pub content: String,
}

impl TestStruct {
    #[allow(dead_code)]
    pub fn new_rnd() -> TestStruct {
        let content = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100)
            .map(char::from)
            .collect();
        let name = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect();
        let mut rng = rand::thread_rng();

        TestStruct {
            name: name,
            id: rng.gen(),
            content: content,
        }
    }
}

impl Schema for TestStruct {
    fn indexes(&self) -> Vec<String> {
        vec![format!("name={}", self.name)]
    }

    fn desc(&self) -> Option<DataOption> {
        None
    }

    fn name(&self) -> &str {
        TESTSTRUCT_STORE
    }
}

impl StoreableWithSchema for TestStruct {}
