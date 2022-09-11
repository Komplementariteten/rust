use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread::Builder;

use crate::response::{read_http, BaseHttpRouting, HttpResponse};

#[derive(Debug, Eq, PartialEq)]
pub enum HttpServerReturn {
    Ok,
    ResponseError,
    StreamError,
    ShutdownError,
    ListenerError,
    ThreaddingError,
    NotStarted,
}

#[derive(Debug)]
pub struct ServerCfg {
    pub main_thread_name: String,
    pub http_port: u32,
}

#[derive(Debug)]
pub struct HttpServer {}

#[derive(Debug)]
struct ServerHandle {}

impl HttpServer {
    fn listen_to_connection(cfg: &ServerCfg) -> Option<TcpListener> {
        match TcpListener::bind(format!("0.0.0.0:{}", cfg.http_port)) {
            Ok(l) => Some(l),
            Err(_) => None,
        }
    }

    pub fn run<R: BaseHttpRouting + Send + 'static>(
        cfg: ServerCfg,
        routing: R,
    ) -> HttpServerReturn {
        let routing_arc = Arc::new(Mutex::new(routing));
        // self.ctrl_tx = Some(tx);
        let th = Builder::new()
            .name(cfg.main_thread_name.clone())
            .spawn(move || {
                let listener_opt = HttpServer::listen_to_connection(&cfg);
                if listener_opt.is_none() {
                    return HttpServerReturn::ListenerError;
                }
                let listener = listener_opt.unwrap();
                // main server loop
                loop {
                    // park_timeout(Duration::from_millis(10));
                    for stream in listener.incoming() {
                        let mut stream = stream.unwrap();
                        let interface = Arc::clone(&routing_arc);
                        let _ = Builder::new().spawn(move || {
                            let http_r = read_http(&mut stream);
                            let mut route_handle = interface.lock().unwrap();
                            if !http_r.is_ok() {
                                match route_handle.error(&mut stream, HttpResponse::bad_request()) {
                                    Ok(_) => HttpServerReturn::Ok,
                                    Err(_) => HttpServerReturn::StreamError,
                                }
                            } else {
                                let (header, body) = http_r.unwrap();
                                match route_handle.execute(header, &mut stream, body) {
                                    Ok(_) => HttpServerReturn::Ok,
                                    Err(_) => HttpServerReturn::ResponseError,
                                }
                            }
                        });
                    }
                }
            });
        if th.is_ok() {
            match th.unwrap().join() {
                Ok(r) => r,
                Err(_) => HttpServerReturn::ShutdownError,
            }
        } else {
            HttpServerReturn::ThreaddingError
        }
    }
}
