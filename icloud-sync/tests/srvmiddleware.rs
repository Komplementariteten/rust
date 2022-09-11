#[cfg(test)]
mod tests {
    use async_std::task::block_on;
    use dirs::home_dir;
    use http::server::ServerCfg;
    use std::fs::remove_dir_all;
    use std::io::{BufReader, BufWriter, Write};
    use std::str::from_utf8;

    /* fn clear_test_files() {
        let hd = home_dir().unwrap().join(MIDDLEWARE_CACHE_DIR);
        let _ = remove_dir_all(hd);
    } */

    /*
    #[test]
    fn fwinterface_servs() {
        // clear_test_files();
        let m = init_idrive();
        assert!(m.is_ok());
        let middleware = m.unwrap();
        http::server::HttpServer::run(ServerCfg {
            main_thread_name: "fwää".to_string(),
            http_port: 8088
        }, middleware);
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
        let bread = BufReader::<&[u8]>::new(data);
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
        let mut middleware = m.unwrap();
        let mut bwrite = BufWriter::new(Vec::new());
        let wr = block_on(middleware.stream_update_as_json(&mut bwrite));
        assert!(wr.is_ok());
        let _ = bwrite.flush();
        let enc = from_utf8(bwrite.buffer());
        assert!(enc.is_ok());
        let str = enc.unwrap();
        println!("{}", str);
    } */
}
