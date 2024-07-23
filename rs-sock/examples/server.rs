use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;
use log::{debug, error, info};

fn main() {

    env_logger::init();

    let listener = TcpListener::bind("0.0.0.0:12000").expect("failed to bind listener");

    static STOP: AtomicBool = AtomicBool::new(false);
    static THREAD_SYNC: AtomicUsize = AtomicUsize::new(0);

    for stream in listener.incoming() {
        match  stream {
            Ok(stream) => {
                thread::spawn(move || {
                    THREAD_SYNC.fetch_add(1, Ordering::SeqCst);
                    let mut client_stream = match init_stream(stream) {
                        Ok(s) => s,
                        Err(e) => {
                            error!("Error in handling Client Connection: {}", e);
                            return;
                        }
                    };
                    info!("Client connected");
                    while !STOP.load(Relaxed) {
                        let bytes = match read_stream(&mut client_stream) {
                            Ok(b_vec) => b_vec,
                            Err(e) => {
                                error!("Error reading from Stream: {}", e);
                                break;
                            }
                        };
                        if bytes.len() > 0 {
                            debug!("{} bytes read", bytes.len());
                        }
                    }
                    THREAD_SYNC.fetch_sub(1, Ordering::SeqCst);
                });
            },
            Err(e) => {
                panic!("Failed to handle incoming Stream{}" ,e);
            }
        }
    }
}

fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut rx_bytes = [0u8; 128];
    match stream.read(&mut rx_bytes) {
        Ok(bytes_read) => Ok(rx_bytes[..bytes_read].to_vec()),
        Err(e) => Err(Box::new(e))
    }
}

fn init_stream(stream: TcpStream) -> Result<TcpStream, Box<dyn Error>>{

    // Socket Options
    match stream.set_nodelay(true) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(e)),
    }
    match stream.set_nonblocking(true) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(e)),
    }

    match stream.set_read_timeout(Some(Duration::from_millis(10))) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(e)),
    }
    match stream.set_write_timeout(Some(Duration::from_millis(10))) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(e)),
    }

    debug!("Socket Opts set");

    Ok(stream)

}
