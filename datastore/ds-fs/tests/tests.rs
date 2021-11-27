use dsfs::*;

#[cfg(test)]
mod tests {
    #[test]
    fn init_datastore_works() {
        use std::path::Path;
        let this_dir = Path::new(".");
        let d = dsfs::init_datastore(this_dir.to_path_buf());
        assert_eq!(1,1)
    }
}