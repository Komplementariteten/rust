#![feature(test)]
extern crate flexbuffers;
extern crate serde_json;
extern crate test;

mod test_util;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use test::Bencher;

    use dslib::Datastore;
    const KEY_VALUE_STORE: &str = "key-value-store";

    use crate::test_util::{cleanup_test, KeyValueItem, TestStruct, STORE_TAB};

    #[test]
    fn kvp_can_store_and_read() {
        let key = "some-key";
        let p = Path::new("/tmp/store1");
        let mut ds = Datastore::new(p.to_path_buf());
        let value = KeyValueItem::new();
        ds.set_kvp(KEY_VALUE_STORE, key, value.clone());
        let el: Option<KeyValueItem> = ds.get_kvp_value(KEY_VALUE_STORE, key);
        assert_eq!(el.is_some(), true);
        assert_eq!(el.unwrap().content, value.content);
        cleanup_test(p.to_path_buf());
    }

    #[test]
    fn datastore_is_searchable() {
        let p = Path::new("/tmp/store2");
        let mut ds = Datastore::new(p.to_path_buf());
        let number_of_test_items = 10;
        for _ in 0..number_of_test_items {
            let _ = ds.insert_and_save(TestStruct::new_rng());
        }
        let r = ds.lookup::<TestStruct>(STORE_TAB, "test-struct");
        assert_eq!(r.is_some(), true);
        let vec = r.unwrap();
        assert_eq!(vec.len(), number_of_test_items);
        cleanup_test(p.to_path_buf());
    }

    #[bench]
    fn bench_store_many(b: &mut Bencher) {
        let p = Path::new("/tmp/store3");
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
        cleanup_test(p.to_path_buf());
    }

    #[test]
    fn test_data_store() {
        let p = Path::new("/tmp/store4");
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
        cleanup_test(p.to_path_buf());
    }
}
