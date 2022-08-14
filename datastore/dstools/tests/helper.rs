#[cfg(test)]
mod tests {

    use dstools::helper;
    use dstools::helper::list_files;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn list_files_list_current_folder() {
        let d = env::current_dir();
        assert_eq!(d.is_ok(), true);
        let v = list_files(d.unwrap().to_path_buf());
        assert!(v.len() > 0);
    }
}
