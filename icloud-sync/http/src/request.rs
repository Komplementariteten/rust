use std::io::{BufRead, BufReader, Read, Write};
use crate::response::ProtocolError::{Base64DecodeFailed, InvalidHeader, InvalidHttpPath, ReadContentError};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;
use async_std::task;
use async_std::task::JoinHandle;
use regex::Regex;
use crate::response::ProtocolError;


pub struct HttpPath {
    pub ssl: bool,
    pub con: String,
    pub resource: String,
}

impl HttpPath {
    fn connect(&self) -> Result<TcpStream, ProtocolError> {
        match TcpStream::connect(self.con.as_str()){
            Ok(s) => Ok(s),
            Err(_) => Err(ProtocolError::ConnectionFailed),
        }
    }
}

const HTTP_PATH_REGEX: &str = r"^(?P<http>http{1}s?)://(?P<domain>[a-zA-Z.\-0-9]+):?(?P<port>[0-9]*)(?P<path>[a-zA-Z=?.%&$§_/\-0-9]*)$";

impl TryFrom<&str> for HttpPath {
    type Error = ProtocolError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re: Regex = Regex::new(HTTP_PATH_REGEX).unwrap();
        let cap = match re.captures(value) {
            Some(c) => c,
            None => return Err(InvalidHttpPath),
        };
        let is_ssl = if let Some(http_name) = cap.name("http") {
            http_name.as_str().contains("s")
        } else {
            return Err(InvalidHttpPath);
        };
        let domain = match cap.name("domain") {
            Some(d) => d.as_str().to_string(),
            None => return Err(InvalidHttpPath),
        };

        let port = match cap.name("port") {
            Some(p) => match p.as_str().parse::<u32>() {
                Ok(p) => p,
                Err(_) => 80,
            },
            None => 80,
        };
        let path = match cap.name("path") {
            Some(p) => p.as_str().to_string(),
            None => "".to_string(),
        };
        Ok(HttpPath {
            con: format!("{}:{}", domain, port).to_string(),
            resource: path,
            ssl: is_ssl,
        })
    }
}

pub struct HttpRequest {
    req: String,
    path: HttpPath
}

impl HttpRequest {
    pub fn json(mut self) -> HttpRequest {
        self.req.push_str("Content-Type: application/json");
        self.req.push_str("Accept: application/json");
        self
    }
    pub fn base64(mut self) -> HttpRequest {
        self.req.push_str("Content-Transfer-Encoding: base64");
        self
    }
    pub fn body(mut self, content: &str) -> HttpRequest {
        let body = format!("\r\n\r\n{}", content);
        self.req.push_str(body.as_str());
        self
    }
    pub fn bytes_async(self) -> JoinHandle<Result<Vec<u8>, ProtocolError>> {
            task::spawn(async {
                self.bytes()
            })
    }
    pub fn text_async(self) -> JoinHandle<Result<String, ProtocolError>> {
        task::spawn(async {
            self.text()
        })
    }

    pub fn text(self) -> Result<String, ProtocolError> {
        let mut con = match TcpStream::connect(self.path.con) {
            Ok(c) => c,
            Err(_) => return Err(ProtocolError::ConnectionFailed)
        };
        let _ = match con.write(self.req.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(ProtocolError::WriteRequestError)
        };
        let _ = con.shutdown(Shutdown::Write);
        let mut data = Vec::new();
        // let mut s = "".to_string();
        let mut buff: [u8; 300] = [0; 300];
        loop {
            let readen = match con.read(&mut buff) {
                Ok(s) => s,
                Err(_) => 0,
            };
            data.extend(&buff[..readen]);
            if readen < buff.len() {
                // request readen
                break;
            }
        }

        let breader = BufReader::new(data.as_slice());
        /*  let utf8_str = match from_utf8(data.as_slice()) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e)
        }; */
        let mut body_started = false;
        let mut body_str = "".to_string();
        for line_r in breader.lines() {
            if line_r.is_err() {
                println!("data could not be read as lines");
                break;
            }
            let line = line_r.unwrap();
            if body_started {
                body_str.push_str(line.as_str());
            }
            if line.len() == 0 {
                body_started = true;
            }
        }
        Ok(body_str)
    }

    pub fn bytes(self) -> Result<Vec<u8>, ProtocolError> {
        let mut con = match TcpStream::connect(self.path.con) {
            Ok(c) => c,
            Err(_) => return Err(ProtocolError::ConnectionFailed)
        };
        let _ = match con.write(self.req.as_bytes()) {
            Ok(s) => s,
            Err(_) => return Err(ProtocolError::WriteRequestError)
        };
        let _ = con.shutdown(Shutdown::Write);
        let mut data = Vec::new();
        // let mut s = "".to_string();
        let mut buff: [u8; 300] = [0; 300];
        loop {
            let readen = match con.read(&mut buff) {
                Ok(s) => s,
                Err(_) => 0,
            };
            data.extend(&buff[..readen]);
            if readen < buff.len() {
                // request readen
                break;
            }
        }

        let utf8_str = match from_utf8(data.as_slice()) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e)
        };
        let mut body_started = false;
        let mut body_str = "".to_string();
        for line in utf8_str.lines() {
            if body_started {
                body_str.push_str(line);
            }
            if line.len() == 0 {
                body_started = true;
            }
        }
        if body_str.len() > 20 {
            println!("Body: {}!", &body_str[body_str.len()-10..]);
            let buff = match base64::decode(&body_str) {
                Ok(s) => s,
                Err(e) => {
                    println!("Decoding error: {:?} for {}", e, body_str);
                    return Err(Base64DecodeFailed)
                }
            };

            return Ok(buff);
        }
        Err(ReadContentError)
    }
}

pub trait Request {
    fn gets(self) -> Result<HttpRequest, ProtocolError>;
    fn post(self) -> Result<HttpRequest, ProtocolError>;
    fn delete(self) -> Result<HttpRequest, ProtocolError>;
}

impl Request for &str {
    fn gets(self) -> Result<HttpRequest, ProtocolError> {
        let path = match HttpPath::try_from(self) {
            Ok(p) => p,
            Err(_) => return Err(ProtocolError::InvalidHttpPath),
        };

        Ok(HttpRequest {
            req: format!("get {}\r\nUser-Agent: icloud-sync\r\n", path.resource),
            path: path
        })
    }

    fn post(self) -> Result<HttpRequest, ProtocolError> {
        let path = match HttpPath::try_from(self) {
            Ok(p) => p,
            Err(_) => return Err(ProtocolError::InvalidHttpPath),
        };

        Ok(HttpRequest {
            req: format!("post {}\r\nUser-Agent: icloud-sync\r\n", path.resource),
            path: path
        })
    }

    fn delete(self) -> Result<HttpRequest, ProtocolError> {
        let path = match HttpPath::try_from(self) {
            Ok(p) => p,
            Err(_) => return Err(ProtocolError::InvalidHttpPath),
        };

        Ok(HttpRequest {
            req: format!("delete {}\r\nUser-Agent: icloud-sync\r\n", path.resource),
            path: path
        })
    }
}

impl Request for String {
    fn gets(self) -> Result<HttpRequest, ProtocolError> {
        self.as_str().gets()
    }

    fn post(self) -> Result<HttpRequest, ProtocolError> {
        self.as_str().post()
    }

    fn delete(self) -> Result<HttpRequest, ProtocolError> {
        self.as_str().delete()
    }
}


#[derive(Debug)]
struct HttpHeader {
    name: String,
    value: String,
}

const HEAD_SEP: &str = ":";

impl TryFrom<String> for HttpHeader {
    type Error = ProtocolError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains(HEAD_SEP) {
            let parts = value.split(HEAD_SEP).collect::<Vec<&str>>();
            if parts.len() == 2 {
                return Ok(HttpHeader {
                    name: parts[0].to_string(),
                    value: parts[1].to_string(),
                });
            }
            return Err(InvalidHeader);
        }
        Err(InvalidHeader)
    }
}