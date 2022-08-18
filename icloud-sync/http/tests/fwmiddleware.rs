#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;
    use std::io::BufWriter;
    use std::str::from_utf8;
    use async_std::task::block_on;
    use dirs::home_dir;
    use http::fwmiddleware::{MIDDLEWARE_CACHE_DIR, init_idrive};

    fn clear_test_files() {
        let hd = home_dir().unwrap().join(MIDDLEWARE_CACHE_DIR);
        let _ = remove_dir_all(hd);
    }

    #[test]
    fn fwmiddleware_writes_json() {
        clear_test_files();
        let m = init_idrive();
        debug_assert!(m.is_ok());
        let mut middleware = m.unwrap();
        let mut bwrite = BufWriter::new(Vec::new());
        let wr = block_on(middleware.stream_update_as_json(&mut bwrite));
        assert!(wr.is_ok());
        let enc = from_utf8(bwrite.buffer());
        assert!(enc.is_ok());
        let str = enc.unwrap();
        println!("{}", str);
    }
}