#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use http::server::{HttpServer, HttpServerReturn, ServerCfg};

    #[test]
    pub fn server_start_and_stops() {
        let mut s = http::server::HttpServer::new();
        let start_ok = s.start(ServerCfg {
            main_thread_name: "test-thread".to_string(),
            http_port: 8080
        });
        assert!(start_ok.is_ok());
        sleep(Duration::from_secs(1));
        let stop = s.stop();
        assert!(stop.is_ok());
        let r = stop.unwrap();
        assert_eq!(r, HttpServerReturn::Ok);
    }

    #[test]
    pub fn start_and_serve_for_10min() {
        let mut s = HttpServer::new();
        let start_result = s.start(ServerCfg {
            main_thread_name: "manual-test".to_string(),
            http_port: 8080
        });
        assert!(start_result.is_ok());
        sleep(Duration::from_secs(60 * 10));
        let _ = s.stop();
    }
}