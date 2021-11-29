extern crate serde_json;
extern crate flexbuffers;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Index;
    use serde::{Serialize, Deserialize};
    use super::*;
    use serde_json::{Result, Value};
    use dslib::Store;

    #[test]
    fn test_store_to_bin_and_back() {
        let mut sp = Store {
            data: vec![1, 0, 1, 0],
            index: HashMap::new()
        };
        sp.index.insert("foo".to_string(), 1);

        let mut s = flexbuffers::FlexbufferSerializer::new();
        if let Ok(_) = sp.serialize(&mut s){
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

        let v:Value = serde_json::from_slice(bytes).unwrap();
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

        let v:Value = serde_json::from_str(json_str).unwrap();
        let j_obj = v.get("value").unwrap();
        let j_dou = j_obj.as_f64().unwrap();
        assert!(j_dou < 1e-6);
    }
}