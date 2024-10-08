use crate::srvmiddleware::init_idrive;
use http::server::{HttpServer, ServerCfg};
use std::borrow::Borrow;
use std::env;
use async_std::task;
use crate::client::SyncClient;

pub mod client;
pub mod srvmiddleware;

fn start_server(port: u32) {
    let middleware = match init_idrive() {
        Ok(m) => m,
        Err(e) => panic!("{:?}", e),
    };

    let cfg = ServerCfg {
        main_thread_name: "icloud-sync server".to_string(),
        http_port: port,
    };
    let server_result = HttpServer::run(cfg, middleware);
    println!("Sever ended with {:?}", server_result);
}

fn run_client(con: String, out_folder: String, tmp_folder: String) {
    let mut s = SyncClient::new(con, tmp_folder, out_folder);
    loop {
        match task::block_on(s.sync()) {
            Ok(s) => if s == 0 {
                break;
            },
            Err(e) => println!("client error with {:?}", e)
        }
    }
    println!("Sync done");
}

fn main() {
    let srv_port: u32 = 8088;

    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        println!("arg: {}", arg)
    }
    if args.iter().any(|s| s == "--server") {
        start_server(srv_port);
    } else if args.iter().any(| s | s == "--client") {
        if args.len() != 5 {
            let cmd_name = args.get(0).unwrap().to_string();
            println!("{} --client http://example:port /target/folder /tmp/folder", cmd_name);
        } else {
            let con = args.get(2).unwrap().to_string();
            let out_folder = args.get(3).unwrap().to_string();
            let tmp_folder = args.get(4).unwrap().to_string();
            run_client(con, out_folder, tmp_folder);
        }
    }

}
