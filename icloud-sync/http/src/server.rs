use std::borrow::BorrowMut;
use std::error::Error;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread::{Builder, park_timeout, JoinHandle};
use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use crate::protocol::{HttpStatus, read_header, respond, respond_with_error};

#[derive(Debug, Eq, PartialEq)]
pub enum HttpServerReturn {
    Ok,
    ChannelError,
    ShutdownError,
    ListenerError
}

enum HttpServerCtrl {
    Close
}

type GeneralServerError = Box<dyn Error + Sync + Send + 'static>;

#[derive(Debug)]
pub struct ServerCfg {
    pub main_thread_name: String,
    pub http_port: u32
}

#[derive(Debug)]
pub struct HttpServer{
    hndl: Option<ServerHandle>,
}

#[derive(Debug)]
struct ServerHandle {
    main_th: JoinHandle<Result<HttpServerReturn, GeneralServerError>>,
    ctrl_tx: SyncSender<HttpServerCtrl>
}

impl HttpServer {
    pub const fn new() -> HttpServer {
        HttpServer{
            hndl: None
        }
    }

    pub const fn is_started(&self) -> bool {
        self.hndl.is_some()
    }

    fn liste_to_connection(cfg: &ServerCfg) -> Result<TcpListener, GeneralServerError> {
        match TcpListener::bind(format!("0.0.0.0:{}", cfg.http_port)) {
            Ok(l) => Ok(l),
            Err(e) => Err(GeneralServerError::try_from(e).unwrap())
        }
    }

    fn handle_connection(mut stream: TcpStream) -> JoinHandle<()> {
        let ct = Builder::new().spawn(move || {
            let header = read_header(&mut stream);
            if !header.is_ok() {
                respond_with_error(stream, HttpStatus::BadRequest);
            } else {
                //respond(stream, )
                respond_with_error(stream, HttpStatus::Ok);
            }
        });
        ct.unwrap()
    }

    pub fn start(&mut self, cfg:ServerCfg) -> Result<(), GeneralServerError>{
        let (tx, rx) = sync_channel(0);
        // self.ctrl_tx = Some(tx);
        let th = Builder::new().name(cfg.main_thread_name.clone()).spawn(move || {
            let listener = HttpServer::liste_to_connection(&cfg)?;
            let mut child_threads = Vec::new();
            // main server loop
            loop {
                if let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        HttpServerCtrl::Close => return Ok(HttpServerReturn::Ok)
                    }
                }
                park_timeout(Duration::from_millis(10));
                for stream in listener.incoming() {
                    let h = HttpServer::handle_connection(stream?);
                    child_threads.push(h);
                }
                let mut i = 0 ;
                while i < child_threads.len() {
                    let jh = child_threads.get(i).unwrap();
                    if jh.is_finished() {
                        let jh = child_threads.remove(i);
                        let _ = jh.join();
                    } else {
                        i += 1;
                    }
                }
            }
        });
        if th.is_ok() {
            self.hndl = Some(ServerHandle {
                ctrl_tx: tx,
                main_th: th.unwrap()
            });
            Ok(())
        } else {
            Err(GeneralServerError::try_from(th.err().unwrap()).unwrap())
        }
    }
    pub fn stop(self) -> Result<HttpServerReturn, GeneralServerError> {
        if self.hndl.is_some() {
            self.hndl.as_ref().unwrap().ctrl_tx.send(HttpServerCtrl::Close)?;
        } else {
            panic!("Controll channel is not initialized")
        }
        match self.hndl.unwrap().main_th.join() {
            Ok(r) => r,
            Err(_) => Err(GeneralServerError::try_from("join error").unwrap())
        }
    }
}