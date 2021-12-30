mod test_util;

#[cfg(test)]
mod tests {
    extern crate rand;
    use crate::test_util::cleanup_test;
    use datastorelib::batch::Batch;
    use datastorelib::datastore::{DataOption, Datastore, Schema, StoreableWithSchema};
    use rand::Rng;
    use serde_derive::{Deserialize, Serialize};
    use std::path::Path;

    const TEST_SERIAL_STORE: &str = "test_store";
    const TEST_SERIAL2_STORE: &str = "teststore2";

    #[derive(Serialize, Deserialize)]
    struct TestSerial {
        pub n: u64,
    }

    impl Schema for TestSerial {
        fn indexes(&self) -> Vec<String> {
            Vec::new()
        }

        fn desc(&self) -> Option<DataOption> {
            None
        }

        fn name(&self) -> &str {
            TEST_SERIAL_STORE
        }
    }

    impl StoreableWithSchema for TestSerial {}

    impl TestSerial {
        pub fn new() -> TestSerial {
            let mut rng = rand::thread_rng();
            TestSerial { n: rng.gen() }
        }
    }

    #[derive(Serialize, Deserialize)]
    struct TestSerial2 {
        pub n: f64,
    }

    impl Schema for TestSerial2 {
        fn indexes(&self) -> Vec<String> {
            let mut v = Vec::new();
            v.push(format!("#{}", self.n));
            v
        }

        fn desc(&self) -> Option<DataOption> {
            None
        }

        fn name(&self) -> &str {
            TEST_SERIAL2_STORE
        }
    }

    impl StoreableWithSchema for TestSerial2 {}

    impl TestSerial2 {
        pub fn new() -> TestSerial2 {
            let mut rng = rand::thread_rng();
            TestSerial2 { n: rng.gen() }
        }
    }

    #[test]
    fn test_multi_batch_can_execute() {
        let mut b1 = Batch::new();
        let mut b2 = Batch::new();
        let p = Path::new("/tmp/test_batch1");
        let mut ds = Datastore::new(p.to_path_buf());
        let test_items = 100;
        let removed_items_num = 20;
        for _ in 0..test_items {
            b1.add(TestSerial2::new());
            b2.add(TestSerial2::new());
        }
        for index in 0..removed_items_num as usize {
            b1.remove(index + 50);
        }
        let indexies1 = ds.execute(TEST_SERIAL2_STORE, b1);

        assert_eq!(indexies1.len(), test_items - removed_items_num);

        for index in 0..removed_items_num as usize {
            let index = &indexies1[index];
            let _ = ds.remove(TEST_SERIAL2_STORE, index);
        }

        let indexies2 = ds.execute(TEST_SERIAL_STORE, b2);
        assert_eq!(indexies2.len(), test_items);
        ds.close();
        cleanup_test(p.to_path_buf());
    }

    #[test]
    fn test_batch_can_be_executed() {
        let mut b = Batch::new();
        let p = Path::new("/tmp/test_batch2");
        let mut ds = Datastore::new(p.to_path_buf());
        let test_items = 100;
        for _ in 0..test_items {
            b.add(TestSerial2::new());
        }
        let add_items = ds.execute(TEST_SERIAL2_STORE, b);
        assert_eq!(add_items.len(), test_items);
        ds.close();
        let get_opt: Option<TestSerial2> =
            ds.get(TEST_SERIAL2_STORE, add_items.last().unwrap().as_str());
        assert_eq!(get_opt.is_some(), true);
        cleanup_test(p.to_path_buf());
    }

    #[test]
    fn test_batch_items_can_be_removed() {
        let test_items: u32 = 100;
        let mut b = Batch::new();
        for _ in 0..test_items {
            b.add(TestSerial2::new());
        }
        assert_eq!(b.items, test_items);
        b.remove(0);
        assert_eq!(b.items, test_items - 1);
        b.remove(0);
        assert_eq!(b.items, test_items - 1);
        b.remove((test_items / 2) as usize);
        assert_eq!(b.items, test_items - 2);
        let items = b.clear();
        assert_eq!(items.len(), (test_items - 2) as usize);
    }

    #[test]
    fn test_batch_clear() {
        let mut b = Batch::new();
        for _ in 0..100 {
            b.add(TestSerial::new());
        }
        assert_eq!(b.items, 100);
        let d = b.clear();
        assert_eq!(b.items, 0);
        assert_eq!(d.len(), 100);
    }

    /*  Will not Work, Serialize has a genric Parameter
    #[test]
    fn test_dyn_vec() {
        let mut dyn_vec: Vec<Box<dyn Serialize>> = Vec::new();
        dyn_vec.push(Box::new(1));
        dyn_vec.push(Box::new(1.24e-7));
        dyn_vec.push(Box::new(TestSerial::new()))
    } */
}
