#[cfg(test)]
mod tests {
    use filewatcher::FileWatcher;
    use std::fs::{create_dir_all, remove_dir_all, File, OpenOptions};
    use std::io::Write;
    use std::path::Path;

    fn clear_test_files<P: AsRef<Path>>(test_folder: P, cache_folder: P) {
        let _ = remove_dir_all(test_folder);
        let _ = remove_dir_all(cache_folder);
    }

    fn prepare_testfiles<P: AsRef<Path>>(test_folder: P) {
        let _ = create_dir_all(&test_folder);
        let _ = File::create(&test_folder.as_ref().join("1"));
        let _ = File::create(&test_folder.as_ref().join("2"));
        let _ = File::create(&test_folder.as_ref().join("3"));
        let _ = File::create(&test_folder.as_ref().join("4"));
    }

    fn update_testfiles<P: AsRef<Path>>(test_folder: P) {
        let mut f = match OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&test_folder.as_ref().join("3"))
        {
            Ok(fd) => fd,
            Err(e) => panic!("failed to write file with {}", e.to_string()),
        };
        let rr = f.write_all(b"123");
        if rr.is_err() {
            panic!(
                "failed to write file with {}",
                rr.err().unwrap().to_string()
            );
        }
    }

    #[test]
    fn sync_finds_files_initialu() {
        if let Ok(mut fw) = FileWatcher::new("..", "/tmp/.s", true) {
            let initial = fw.sync();
            assert!(initial.is_some());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn sync_provides_same_without_ack() {
        let test_files_folder = "/tmp/sync_find_new";
        let cache_folder = "/tmp/.s2";
        clear_test_files(test_files_folder, cache_folder);
        if let Ok(mut fw) = FileWatcher::new(test_files_folder, cache_folder, false) {
            prepare_testfiles(test_files_folder);
            let files = fw.sync().unwrap();
            assert!(files.len() == 4);
            update_testfiles(test_files_folder);
            let changed = fw.sync().unwrap();
            assert_eq!(changed.len(), 4);
            assert!(changed[0].path.ends_with("3"));
        }
        clear_test_files(test_files_folder, cache_folder);
    }

    #[test]
    fn sync_finds_new() {
        let test_files_folder = "/tmp/sync_find_new";
        let cache_folder = "/tmp/.s2";
        clear_test_files(test_files_folder, cache_folder);
        if let Ok(mut fw) = FileWatcher::new(test_files_folder, cache_folder, false) {
            prepare_testfiles(test_files_folder);
            let files = fw.sync().unwrap();
            assert!(files.len() == 4);
            for file in files {
                fw.ack(file);
            }
            update_testfiles(test_files_folder);
            let changed = fw.sync().unwrap();
            assert_eq!(changed.len(), 1);
            assert!(changed[0].path.ends_with("3"));
        }
        clear_test_files(test_files_folder, cache_folder);
    }
}
