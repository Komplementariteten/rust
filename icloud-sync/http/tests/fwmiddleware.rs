#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;
    use std::io::{BufReader, BufWriter, Write};
    use std::str::from_utf8;
    use async_std::task::block_on;
    use dirs::home_dir;
    use filewatcher::filescanner::PathFileEntry;
    use http::fwmiddleware::{MIDDLEWARE_CACHE_DIR, init_idrive};

    fn clear_test_files() {
        let hd = home_dir().unwrap().join(MIDDLEWARE_CACHE_DIR);
        let _ = remove_dir_all(hd);
    }

    #[test]
    fn json_can_be_read() {
        clear_test_files();
        let m = init_idrive();
        assert!(m.is_ok());
        let mut middleware = m.unwrap();
        let mut bwrite = BufWriter::new(Vec::new());
        let wr = block_on(middleware.stream_update_as_json(&mut bwrite));
        assert!(wr.is_ok());
        let fr = bwrite.flush();
        assert!(fr.is_ok());
        let data = bwrite.get_ref();
        let mut bread = BufReader::<&[u8]>::new(data);
        let c = block_on(middleware.ack_from_stream(bread));
        if c.is_err() {
            let enc = from_utf8(data);
            let str = enc.unwrap();
            println!("Err: {}", c.err().unwrap());
            println!("Json: {}", str);
        } else {
            assert!(c.is_ok());
            assert!(c.unwrap() > 1000);
        }
    }

    #[test]
    fn fwmiddleware_writes_json() {
        clear_test_files();
        let m = init_idrive();
        assert!(m.is_ok());
        let middleware = m.unwrap();
        let mut bwrite = BufWriter::new(Vec::new());
        let wr = block_on(middleware.stream_update_as_json(&mut bwrite));
        assert!(wr.is_ok());
        let flush_err = bwrite.flush();
        let enc = from_utf8(bwrite.buffer());
        assert!(enc.is_ok());
        let str = enc.unwrap();
        println!("{}", str);
    }
}