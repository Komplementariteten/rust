use dslib::{DataOption, Schema, StoreableWithSchema};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde_derive::{Deserialize, Serialize};

const TESTSTRUCT_STORE: &str = "test-struct";

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
