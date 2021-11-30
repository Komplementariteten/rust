extern crate flexbuffers;
extern crate serde_json;

#[cfg(test)]
mod tests {
    use super::*;
    use dslib::Store;
    use serde::Serialize;
    use serde_json::Value;

    #[test]
    fn test_store_add() {
        let mut s = Store::new();
        assert!(s.len() == 0);
        let data: Vec<u8> = vec![1, 2];
        s.add(data);
        assert!(s.len() == 1);
    }

    #[test]
    fn test_store_save_and_load() {
        let mut s = Store::new();
        s.add(vec![1, 2]);
        s.add(vec![1, 2]);
        let s_res = s.as_slice();
        assert!(s_res.is_ok());
        let s_vec = s_res.unwrap_or(Vec::new());
        let l_res = Store::load(s_vec);
        assert!(l_res.is_ok());
        let l_s = l_res.unwrap_or(Store::new());
        assert!(l_s.len() == 2);
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

        let v: Value = serde_json::from_slice(bytes).unwrap();
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

        let v: Value = serde_json::from_str(json_str).unwrap();
        let j_obj = v.get("value").unwrap();
        let j_dou = j_obj.as_f64().unwrap();
        assert!(j_dou < 1e-6);
    }
}
