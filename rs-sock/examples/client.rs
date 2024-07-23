use std::io::Write;
use std::net::{TcpStream};
use std::time::Duration;
use log::error;

fn main() {
    env_logger::init();
    let address = "127.0.0.1:12000".parse().expect("Is no Address");
    let mut client_con = TcpStream::connect_timeout(&address, Duration::from_millis(200)).expect("Failed to connect to Server");

    // client_con.set_nonblocking(true).expect("Failed to set Socket to nonblocking");

    for i in 0..100 {
        let text = format!("Number: {}", i);
        let bytes = text.as_bytes();
        match client_con.write(bytes) {
            Ok(_r) => print!("."),
            Err(e) => error!("Failed to write to sock: {}", e)
        }
    }
}