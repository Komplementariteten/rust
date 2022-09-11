#[cfg(test)]
mod test_util;

mod tests {
    use crate::test_util::{cleanup_test, OtherTestStruct, TestStruct};
    use datastore::storage_manager::StorageManager;
    use serde_traitobject as st;
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn test_sm_can_reopen_files() {
        let test_key = "one-key";
        let base_dir = "/tmp/test_reopen";
        cleanup_test(Path::new(base_dir).to_path_buf());
        let m_res = StorageManager::default(base_dir);
        assert_eq!(m_res.is_ok(), true);
        let mut m = m_res.unwrap();
        let mut b = m.batch::<TestStruct>();
        let inx = b
            .add(TestStruct::new_rnd())
            .add(TestStruct::new_rnd())
            .add(TestStruct::new_rnd())
            .commit("batch_tab");
        assert_eq!(inx.len(), 3);
        m.set_kv(test_key, "foo bar".to_string());
        drop(m);
        let second_m = StorageManager::default(base_dir);
        assert_eq!(second_m.is_ok(), true);
        m = second_m.unwrap();
        let kv_entry = m.get_kv::<String>(test_key);
        assert_eq!(kv_entry.is_some(), true);
        drop(m);
        cleanup_test(Path::new(base_dir).to_path_buf());
    }

    #[test]
    fn set_kv_vector_can_update() {
        let test_key = "one-key";
        let value1 = vec![1, 2, 3];
        let value2 = vec![5, 6, 7];
        let base_dir = "/tmp/test_vec";
        cleanup_test(Path::new(base_dir).to_path_buf());
        let m_res = StorageManager::default(base_dir);
        assert_eq!(m_res.is_ok(), true);
        let mut m = m_res.unwrap();
        m.set_kv_vector(test_key, value1.clone());
        let v1: Option<Vec<i32>> = m.get_kv_vector(test_key);
        assert!(v1.is_some());
        assert_eq!(v1.unwrap(), value1);
        m.set_kv_vector(test_key, value2.clone());
        let v2: Option<Vec<i32>> = m.get_kv_vector(test_key);
        assert!(v2.is_some());
        assert_eq!(v2.unwrap(), value2);
        drop(m);
        cleanup_test(Path::new(base_dir).to_path_buf());
    }

    #[test]
    fn set_kv_can_update() {
        let test_key = "one-key";
        let value1 = "value-one";
        let value2 = 1;
        let base_dir = "/tmp/test_kv_update";
        cleanup_test(Path::new(base_dir).to_path_buf());
        let m_res = StorageManager::default(base_dir);
        assert_eq!(m_res.is_ok(), true);
        let mut m = m_res.unwrap();
        m.set_kv(test_key, value1.to_string());
        let v1: Option<String> = m.get_kv(test_key);
        assert!(v1.is_some());
        assert_eq!(v1.unwrap(), value1);
        m.set_kv(test_key, value2);
        let v2: Option<i32> = m.get_kv(test_key);
        assert!(v2.is_some());
        assert_eq!(v2.unwrap(), value2);
        cleanup_test(Path::new(base_dir).to_path_buf());
    }

    #[test]
    fn test_add_any_hm() {
        let base_dir = "/tmp/test_anyhm";
        let m_res = StorageManager::default(base_dir);
        assert_eq!(m_res.is_ok(), true);
        let mut m = m_res.unwrap();

        let mut h: HashMap<String, st::Box<dyn st::Any>> = HashMap::new();
        h.insert("foo".to_string(), st::Box::new(11));
        h.insert("bar".to_string(), st::Box::new("hallo welt".to_string()));
        h.insert("foo bar".to_string(), st::Box::new(TestStruct::new_rnd()));

        let inx = m.add_any(h);
        assert_eq!(inx.len() > 0, true);
        std::mem::drop(m);
        cleanup_test(Path::new(base_dir).to_path_buf());
    }

    #[test]
    fn test_batch_with_sm() {
        let base_dir = "/tmp/test_batch";
        let m_res = StorageManager::default(base_dir);
        assert_eq!(m_res.is_ok(), true);
        let mut m = m_res.unwrap();
        let mut b = m.batch::<TestStruct>();
        let inx = b
            .add(TestStruct::new_rnd())
            .add(TestStruct::new_rnd())
            .add(TestStruct::new_rnd())
            .commit("batch_tab");
        assert_eq!(inx.len(), 3);

        let mut b2 = m.batch::<OtherTestStruct>();
        let inx2 = b2
            .add(OtherTestStruct::new_rng())
            .add(OtherTestStruct::new_rng())
            .commit("batch_tab");
        assert_eq!(inx2.len(), 2);
        std::mem::drop(m);
        cleanup_test(Path::new(base_dir).to_path_buf());
    }

    #[test]
    fn test_key_value_store() {
        let base_dir = "/tmp/test_kvp";
        cleanup_test(Path::new(base_dir).to_path_buf());
        let m_res = StorageManager::default(base_dir);
        assert_eq!(m_res.is_ok(), true);
        let mut m = m_res.unwrap();
        let get1: Option<String> = m.get_kv("key");
        assert_eq!(get1.is_none(), true);
        m.set_kv("key", "value".to_string());
        let get2: Option<String> = m.get_kv("key");
        assert_eq!(get2.is_some(), true);
        std::mem::drop(m);
        cleanup_test(Path::new(base_dir).to_path_buf());
    }

    #[test]
    fn test_sm_path_managing() {
        let base_p = "/tmp/test_sm1";
        let m_result = StorageManager::default(base_p);
        assert_eq!(m_result.is_ok(), true);
        let m = m_result.unwrap();
        assert_eq!(m.is_locked(), true);
        std::mem::drop(m);
        let test_p = Path::new("/tmp/test_sm/.config/.lock");
        assert_eq!(test_p.exists(), false);
        cleanup_test(Path::new(base_p).to_path_buf());
    }
    #[test]
    fn test_multi_sm_panic() {
        let base_p = "/tmp/test_sm2";
        let s1 = StorageManager::default(base_p);
        let s2 = StorageManager::default(base_p);
        assert_eq!(s1.is_ok(), true);
        assert_eq!(s2.is_ok(), false);
        assert_eq!(s2.is_err(), true);
        std::mem::drop(s1);
        cleanup_test(Path::new(base_p).to_path_buf());
    }
}
