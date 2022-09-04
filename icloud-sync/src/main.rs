extern crate core;

use std::borrow::Borrow;
use std::env;
use http::server::{HttpServer, ServerCfg};
use crate::srvmiddleware::init_idrive;

pub mod srvmiddleware;
mod client;

fn start_server(port: u32) {
    let middleware = match init_idrive() {
        Ok(m) => m,
        Err(e) => panic!("{:?}", e)
    };

    let cfg = ServerCfg {
        main_thread_name: "icloud-sync server".to_string(),
        http_port: port
    };
    let server_result = HttpServer::run(cfg, middleware);
    println!("Sever ended with {:?}", server_result);
}

fn main() {
    let srv_port: u32 = 8088;

    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        println!("arg: {}", arg)
    }
    if args.iter().any(| s | s == "--server") {
        start_server(srv_port);
    }
}
