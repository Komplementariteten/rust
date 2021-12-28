#![feature(test)]
extern crate flexbuffers;
extern crate serde_json;
extern crate test;

#[cfg(test)]
mod tests {
    use dslib::{DataOption, Datastore, Schema, StoreableWithSchema};
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use serde_derive::{Deserialize, Serialize};
    use std::fs::remove_dir_all;
    use std::path::Path;
    use test::Bencher;

    const STORE_TAB: &str = "test-data";

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        pub name: String,
        pub id: i32,
        pub content: String,
    }

    #[derive(PartialEq, Serialize, Deserialize, Clone)]
    struct KeyValueElement {
        element_ids: Vec<String>,
        pub content: String,
    }

    impl dslib::KeyValueElement for KeyValueElement {}

    impl KeyValueElement {
        pub fn new() -> KeyValueElement {
            let str = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(100)
                .map(char::from)
                .collect();
            KeyValueElement {
                element_ids: Vec::new(),
                content: str,
            }
        }
    }

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

    impl StoreableWithSchema for TestStruct {}

    #[test]
    fn kvp_can_store_and_read() {
        let key = "some-key";
        let p = Path::new("/tmp/store");
        let mut ds = Datastore::new(p.to_path_buf());
        let value = KeyValueElement::new();
        ds.set_kvp(key, value.clone());
        let el: Option<KeyValueElement> = ds.get_kvp_value(key);
        assert_eq!(el.is_some(), true);
        assert_eq!(el.unwrap().content, value.content);
    }

    #[test]
    fn datastore_is_searchable() {
        let p = Path::new("/tmp/store");
        let mut ds = Datastore::new(p.to_path_buf());
        let number_of_test_items = 10;
        for _ in 0..number_of_test_items {
            let _ = ds.insert_and_save(TestStruct::new_rng());
        }
        let r = ds.lookup::<TestStruct>(STORE_TAB, "test-struct");
        assert_eq!(r.is_some(), true);
        let vec = r.unwrap();
        assert_eq!(vec.len(), number_of_test_items);
    }

    #[bench]
    fn bench_store_many(b: &mut Bencher) {
        let p = Path::new("./store");
        let mut ds = Datastore::new(p.to_path_buf());
        b.iter(|| {
            // for _ in 0..30 {
            let test_data = TestStruct {
                content: "Foo Baar".to_string(),
                id: -55,
                name: "test".to_string(),
            };
            let _add_result = ds.insert_and_save(test_data);
            // }
        });
        ds.close();
        let _ = remove_dir_all(p);
    }

    #[test]
    fn test_data_store() {
        let p = Path::new("/tmp/store");
        let mut ds = Datastore::new(p.to_path_buf());

        let test_data = TestStruct {
            content: "Foo Baar".to_string(),
            id: -55,
            name: "test".to_string(),
        };

        let add_result = ds.insert_and_save(test_data);
        assert!(add_result.is_ok());

        let inx = add_result.unwrap();
        let readen_test_data: Option<TestStruct> = ds.get(STORE_TAB, &inx);

        assert!(readen_test_data.is_some());
    }
}
