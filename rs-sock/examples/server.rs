use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use log::error;

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
                    let client_stream = match init_stream(stream) {
                        Ok(s) => s,
                        Err(e) => {
                            error!("Error in handling Client Connection: {}", e);
                            return;
                        }
                    };
                    while !STOP.load(Relaxed) {
                        read_stream(&client_stream);
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

fn read_stream(stream: &TcpStream) {

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


    Ok(())

}
