#![feature(test)]
extern crate flexbuffers;
extern crate serde_json;
extern crate test;

#[cfg(test)]
mod tests {
    use dslib::{DataOption, Datastore, Schema};
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

    impl Schema for TestStruct {
        fn indexes(&self) -> Vec<String> {
            let mut v = Vec::new();
            v.push(format!("name={}", self.name));
            v.push(format!("Ind={}", self.id));
            return v;
        }

        fn desc(&self) -> Option<DataOption> {
            Some(DataOption {})
        }

        fn name(&self) -> &str {
            STORE_TAB
        }
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
            let _add_result = ds.add(test_data);
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

        let add_result = ds.add(test_data);
        assert!(add_result.is_ok());

        let inx = add_result.unwrap();
        let readen_test_data: Option<TestStruct> = ds.get(STORE_TAB, &inx);

        assert!(readen_test_data.is_some());
    }
}
