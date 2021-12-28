mod test_util;

#[cfg(test)]
mod tests {
    use crate::test_util::{OtherTestStruct, TestStruct};
    use datastore::datastore_cfg::DatastoreConfig;
    use datastore::storage_manager::StorageManager;
    use std::path::Path;

    #[test]
    fn test_batch_with_sm() {
        let base_dir = "/tmp/test_batch";
        let m_res = StorageManager::new(base_dir);
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
    }

    #[test]
    fn test_key_value_store() {
        let base_dir = "/tmp/test_kvp";
        let m_res = StorageManager::new(base_dir);
        assert_eq!(m_res.is_ok(), true);
        let mut m = m_res.unwrap();
        let get1: Option<String> = m.get_kv("key");
        assert_eq!(get1.is_none(), true);
        m.set_kv("key", "value".to_string());
        let get2: Option<String> = m.get_kv("key");
        assert_eq!(get2.is_some(), true);
    }

    #[test]
    fn test_sm_path_managing() {
        let base_p = "/tmp/test_store1";
        let m_result = StorageManager::new(base_p);
        assert_eq!(m_result.is_ok(), true);
        let m = m_result.unwrap();
        assert_eq!(m.is_locked(), true);
        std::mem::drop(m);
        let test_p = Path::new("/tmp/test_store1/.config/.lock");
        assert_eq!(test_p.exists(), false);
    }
    #[test]
    fn test_multi_sm_panic() {
        let base_p = "/tmp/test_store2";
        let s1 = StorageManager::new(base_p);
        let s2 = StorageManager::new(base_p);
        assert_eq!(s1.is_ok(), true);
        assert_eq!(s2.is_ok(), false);
        assert_eq!(s2.is_err(), true);
    }
}
