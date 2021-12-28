extern crate flexbuffers;
extern crate serde_json;

#[cfg(test)]
mod tests {
    use super::*;
    use dslib::store::Store;
    use dslib::*;
    use rand::Rng;
    use serde::Serialize;
    use serde_derive::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
    struct TestStruct {
        pub payload: Vec<u64>,
        pub d: HashMap<String, String>,
        pub o: f32,
    }

    impl TestStruct {
        pub fn new() -> TestStruct {
            let mut rng = rand::thread_rng();
            TestStruct {
                o: rng.gen(),
                payload: Vec::new(),
                d: HashMap::new(),
            }
        }
    }

    #[test]
    fn replace_replaces_in_store_correctly() {
        let mut s = Store::new();
        let mut inxes = Vec::new();
        let t = TestStruct::new();
        let t2 = TestStruct::new();
        let inx1 = vec!["123".to_string(), "345".to_string(), "new_678".to_string()];
        for _ in 0..20 {
            let inx = s.add_with_index(&t, inx1.clone(), None);
            inxes.push(inx);
        }
        s.replace(inxes.first().unwrap(), &t2, inx1.clone(), None);
        assert_ne!(t.o, t2.o);
    }

    #[test]
    fn test_store_add() {
        let mut s = Store::new();
        assert_eq!(s.len(), 0);
        let data: Vec<u8> = vec![1, 2];
        s.add_raw(data);
        assert_eq!(s.len(), 1);
    }
    #[test]
    fn test_add_can_be_found() {
        let mut s = Store::new();
        let inx1 = s.add_with_index("foo".to_string(), vec!["bar".to_string()], None);
        let inx2 = s.add_with_index("foo2".to_string(), vec!["bar".to_string()], None);
        let inx3 = s.add_with_index(2, vec!["bar".to_string()], None);

        // Assert String
        let string_r = s.lookup_as_hashmap::<String>("bar");
        assert!(string_r.is_some());
        let string_m = string_r.unwrap();
        assert_eq!(string_m.len(), 2);
        assert!(string_m.contains_key(&inx1));
        assert!(string_m.contains_key(&inx2));

        // Assert i32
        let string_r = s.lookup_as_hashmap::<i32>("bar");
        assert!(string_r.is_some());
        let string_m = string_r.unwrap();
        assert_eq!(string_m.len(), 1);
        assert!(string_m.contains_key(&inx3));
    }

    #[test]
    fn test_store_pop_removes_correctly() {
        let mut s = Store::new();
        let numer_of_items: u64 = 3000;
        let mut indexies = Vec::new();
        for _i in 0..numer_of_items {
            let index = s.add_raw(vec![1, 2, 3]);
            indexies.push(index);
        }
        assert_eq!((s.len() as u64), numer_of_items);
        for item in indexies.iter() {
            if item % 2 == 0 {
                let _ = s.remove(&format_inx!(item));
            }
        }
        assert_eq!((s.len() as u64), (numer_of_items / 2));
    }

    #[test]
    fn test_add_with_index_works_correctly() {
        let mut s = Store::new();
        s.add_with_index(vec![1, 2], vec!["foo".to_string()], Some(DataOption {}));
        assert_eq!(s.len(), 1);
        assert_eq!(s.inx_len(), 2);
    }

    #[test]
    fn test_store_save_and_load() {
        let mut s = Store::new();
        s.add_raw(vec![1, 2]);
        s.add_raw(vec![1, 2]);
        let s_res = s.as_slice();
        assert!(s_res.is_ok());
        let s_vec = s_res.unwrap_or(Vec::new());
        let l_res = Store::load(s_vec);
        assert!(l_res.is_ok());
        let l_s = l_res.unwrap_or(Store::new());
        assert_eq!(l_s.len(), 2);
    }

    #[test]
    fn test_store_adds_struct_and_gets_back() {
        let mut sp = Store::new();
        let t = TestStruct {
            payload: vec![1, 2],
            o: 9.245e-12,
            d: HashMap::from([
                ("1".to_string(), "ello".to_string()),
                ("889".to_string(), "foo".to_string()),
            ]),
        };
        let test = t.clone();
        let inx = sp.add_single(t);
        let ret: Option<TestStruct> = sp.get(inx.as_str());
        assert!(ret.is_some());
        let res = ret.unwrap();
        assert_eq!(res.o, test.o);
        assert_eq!(res.d, test.d);
        assert_eq!(res.payload, test.payload);
    }

    #[test]
    fn test_store_to_bin_and_back() {
        let sp = Store::new();

        let mut s = flexbuffers::FlexbufferSerializer::new();
        if let Ok(_) = sp.serialize(&mut s) {
            let b = s.take_buffer();
            assert!(!b.is_empty());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_bin_handling() {
        let hg_hu = r#"{
            "name": 12,
            "value": -1234E-10
        }"#;

        let bytes = hg_hu.as_bytes();

        let v: serde_json::Value = serde_json::from_slice(bytes).unwrap();
        let v_obj = v.get("value").unwrap();
        let v_f64 = v_obj.as_f64().unwrap();
        assert!(v_f64 < 0.0);
        assert!(v_f64 > -1.3e-7);
    }

    #[test]
    fn test_str_handling() {
        let json_str = r#"{
            "name": 12,
            "value": 1234E-10
            }"#;

        let v: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let j_obj = v.get("value").unwrap();
        let j_dou = j_obj.as_f64().unwrap();
        assert!(j_dou < 1e-6);
    }
}
