#![feature(test)]
extern crate flexbuffers;
extern crate serde_json;
extern crate serde_traitobject;
extern crate test;

mod test_util;

#[cfg(test)]
mod tests {
    use serde_traitobject as s;
    use std::collections::HashMap;
    use std::path::Path;
    use test::Bencher;

    use datastorelib::datastore::Datastore;

    const KEY_VALUE_STORE: &str = "key-value-store";

    use crate::test_util::{
        cleanup_test, DynTestStruct1, DynTestStruct2, KeyValueItem, TestSchemaStruct, TestStruct,
        STORE_TAB,
    };

    #[test]
    fn test_datastore_with_box_hm() {
        let s_name = "hm";
        let p = Path::new("/tmp/store7");
        let mut ds = Datastore::new(p.to_path_buf());
        let mut h: HashMap<_, s::Box<dyn s::Any>> = HashMap::new();
        h.insert("a".to_string(), s::Box::new(DynTestStruct1::new_rng()));
        h.insert("b".to_string(), s::Box::new(DynTestStruct2::new_rng()));
        let inx = ds.add_hm(&s_name, h);
        let g_hm: Option<HashMap<_, s::Box<dyn s::Any>>> = ds.get_hm(&s_name, &inx);
        assert_eq!(g_hm.is_some(), true);
        let hm = g_hm.unwrap();
        assert_eq!(hm.contains_key("b"), true);
    }

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
            let _ = ds.insert_and_save(TestSchemaStruct::new_rng());
        }
        let r = ds.lookup::<TestSchemaStruct>(STORE_TAB, "test-struct");
        assert_eq!(r.is_some(), true);
        let vec = r.unwrap();
        assert_eq!(vec.len(), number_of_test_items);
        cleanup_test(p.to_path_buf());
    }

    #[test]
    fn test_add_and_get_hm() {
        let vec_store = "hm";
        let p = Path::new("/tmp/store6");
        let mut ds = Datastore::new(p.to_path_buf());
        let mut hm = HashMap::new();
        hm.insert("a".to_string(), vec![1.0, 2.3, 5.11, -0.12]);
        hm.insert("b".to_string(), vec![1.0, 2.3, 5.11, -0.12]);
        let inx = ds.add_hm(&vec_store, hm);
        let v_opt: Option<HashMap<String, Vec<f64>>> = ds.get_hm(&vec_store, &inx);
        assert_eq!(v_opt.is_some(), true);
        let v = v_opt.unwrap();
        assert_eq!(v.contains_key("b"), true);
        cleanup_test(p.to_path_buf());
    }

    #[test]
    fn test_add_and_get_vec() {
        let vec_store = "v";
        let p = Path::new("/tmp/store5");
        let mut ds = Datastore::new(p.to_path_buf());
        let inx = ds.add_vec(&vec_store, vec![1.0, 2.3, 5.11, -0.12]);
        let v_opt: Option<Vec<f32>> = ds.get_vec(&vec_store, &inx);
        assert_eq!(v_opt.is_some(), true);
        let v = v_opt.unwrap();
        assert_eq!(v.len(), 4);
        cleanup_test(p.to_path_buf());
    }

    #[bench]
    fn bench_store_many(b: &mut Bencher) {
        let p = Path::new("/tmp/store3");
        let mut ds = Datastore::new(p.to_path_buf());
        b.iter(|| {
            // for _ in 0..30 {
            let test_data = TestSchemaStruct {
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

        let test_data = TestSchemaStruct {
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
