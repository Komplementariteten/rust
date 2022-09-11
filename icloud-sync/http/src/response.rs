use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str::from_utf8;

use regex::Regex;
use time::format_description::well_known::Rfc2822;
use time::OffsetDateTime;

use crate::response::ProtocolError::{InvalidHeader, InvalidHttpPath};

#[derive(PartialEq, Eq, Debug)]
pub enum HttpVerb {
    Get,
    Post,
    Options,
    Put,
    Head,
    Trace,
    Delete,
    Connect,
    NotFound,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub msg: String,
    pub date: OffsetDateTime,
    pub header: HashMap<String, String>,
    pub is_compressed: bool,
    pub content: Vec<u8>,
}

macro_rules! format_header {
    ($n: tt, $v: tt) => {
        format!("{}: {}\n", $n, $v)
    };
}

impl Into<Vec<u8>> for HttpResponse {
    fn into(self) -> Vec<u8> {
        let mut r = String::new();
        r.push_str(format!("HTTP/1.1 {} {}\n", self.status, self.msg).as_str());
        let date_str = match self.date.format(&Rfc2822) {
            Ok(d) => d,
            Err(_) => "0.0.0".to_string(),
        };

        r.push_str(format_header!("Date", date_str).as_str());
        let length_in_byte = self.content.len();
        r.push_str(format_header!("Content-Length", length_in_byte).as_str());
        for (key, value) in self.header {
            r.push_str(format_header!(key, value).as_str());
        }
        r.push_str("\n");
        if !self.content.is_empty() {
            let mut bytes = r.into_bytes();
            let mut body_data = self.content;
            bytes.append(&mut body_data);
            bytes
        } else {
            r.into_bytes()
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum HttpStatus {
    BadRequest,
    Ok,
    NotImplemented,
    NotFound,
    ServerError,
}

impl Into<(u16, String)> for HttpStatus {
    fn into(self) -> (u16, String) {
        match self {
            HttpStatus::BadRequest => (400, String::from("Bad Request")),
            HttpStatus::Ok => (200, String::from("OK")),
            HttpStatus::NotImplemented => (501, String::from("Not Implemented")),
            HttpStatus::NotFound => (404, String::from("Not found")),
            HttpStatus::ServerError => (500, String::from("Internal Server Error")),
        }
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    NotInitialized,
    HttpVersionNotSupported,
    WriteResponseError,
    InvalidHeader,
    ConnectionFailed,
    InvalidHttpPath,
    PathRegexError,
    WriteRequestError,
    Base64DecodeFailed,
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ProtocolError {}

impl From<String> for HttpVerb {
    fn from(name: String) -> Self {
        match name.to_lowercase().as_str() {
            "get" => HttpVerb::Get,
            "post" => HttpVerb::Post,
            "put" => HttpVerb::Put,
            "head" => HttpVerb::Head,
            "options" => HttpVerb::Options,
            "trace" => HttpVerb::Trace,
            "delete" => HttpVerb::Delete,
            _ => HttpVerb::NotFound,
        }
    }
}

#[derive(Debug)]
pub struct HttpHeaders {
    pub verb: HttpVerb,
    pub resource: String,
    pub http_version: String,
    pub header: HashMap<String, String>,
}

impl TryFrom<HttpInitializer> for HttpHeaders {
    type Error = ProtocolError;

    fn try_from(value: HttpInitializer) -> Result<Self, Self::Error> {
        if value.verb.is_none() || value.resource.is_none() {
            return Err(ProtocolError::NotInitialized);
        }
        Ok(HttpHeaders {
            verb: value.verb.unwrap(),
            resource: value.resource.unwrap().to_string().to_lowercase(),
            http_version: value.http_version.unwrap().to_string().to_lowercase(),
            header: value.header,
        })
    }
}

impl HttpResponse {
    pub fn ok_with_json(json: &[u8]) -> HttpResponse {
        let mut addition_header: HashMap<String, String> = Default::default();
        addition_header.insert(
            "Content-Type".to_string(),
            "application/json; charset=utf-8".to_string(),
        );
        let (status, msg) = HttpStatus::Ok.into();

        HttpResponse {
            status: status,
            msg: msg,
            date: OffsetDateTime::now_utc(),
            header: addition_header,
            is_compressed: false,
            content: json.to_vec(),
        }
    }
    pub fn ok_bin(data: Vec<u8>, compressed: bool) -> HttpResponse {
        let mut addition_header: HashMap<String, String> = Default::default();
        addition_header.insert(
            "Content-Type".to_string(),
            "application/octet-stream; charset=utf-8".to_string(),
        );
        addition_header.insert("Content-Transfer-Encoding".to_string(), "base64".to_string());

        let data = base64::encode(data);
        let (status, msg) = HttpStatus::Ok.into();
        HttpResponse {
            status: status,
            msg: msg,
            date: OffsetDateTime::now_utc(),
            header: addition_header,
            is_compressed: compressed,
            content: data.into_bytes(),
        }
    }

    pub fn server_error() -> HttpResponse {
        let (status, msg) = HttpStatus::ServerError.into();
        HttpResponse {
            status: status,
            msg: msg.clone(),
            date: OffsetDateTime::now_utc(),
            header: Default::default(),
            is_compressed: false,
            content: msg.into_bytes(),
        }
    }
    pub fn server_error_with_text(body: &[u8]) -> HttpResponse {
        let (status, msg) = HttpStatus::ServerError.into();
        HttpResponse {
            status: status,
            msg: msg,
            date: OffsetDateTime::now_utc(),
            header: Default::default(),
            is_compressed: false,
            content: body.to_vec(),
        }
    }
    pub fn not_found() -> HttpResponse {
        let (status, msg) = HttpStatus::NotFound.into();
        HttpResponse {
            status: status,
            msg: msg,
            date: OffsetDateTime::now_utc(),
            header: Default::default(),
            is_compressed: false,
            content: vec![],
        }
    }
    pub fn bad_request() -> HttpResponse {
        let (status, msg) = HttpStatus::BadRequest.into();
        HttpResponse {
            status: status,
            msg: msg,
            date: OffsetDateTime::now_utc(),
            header: Default::default(),
            is_compressed: false,
            content: vec![],
        }
    }
    pub fn not_implemented() -> HttpResponse {
        let (status, msg) = HttpStatus::NotImplemented.into();
        HttpResponse {
            status: status,
            msg: msg,
            date: OffsetDateTime::now_utc(),
            header: Default::default(),
            is_compressed: false,
            content: vec![],
        }
    }
    pub fn ok() -> HttpResponse {
        let (status, msg) = HttpStatus::Ok.into();
        HttpResponse {
            status: status,
            msg: msg,
            date: OffsetDateTime::now_utc(),
            header: Default::default(),
            is_compressed: false,
            content: vec![],
        }
    }
}

#[derive(Debug)]
struct HttpInitializer {
    verb: Option<HttpVerb>,
    resource: Option<String>,
    http_version: Option<String>,
    header: HashMap<String, String>,
}

pub trait BaseHttpRouting {
    fn get(&mut self, resource: String, aditional_header: HashMap<String, String>) -> HttpResponse;
    fn post(
        &mut self,
        resource: String,
        aditional_header: HashMap<String, String>,
        stream: Vec<u8>,
    ) -> HttpResponse;
    fn head(&mut self, resource: String, aditional_header: HashMap<String, String>)
        -> HttpResponse;
    fn put(
        &mut self,
        resource: String,
        aditional_header: HashMap<String, String>,
        stream: Vec<u8>,
    ) -> HttpResponse;
    fn delete(
        &mut self,
        resource: String,
        aditional_header: HashMap<String, String>,
    ) -> HttpResponse;
    fn options(
        &mut self,
        resource: String,
        aditional_header: HashMap<String, String>,
    ) -> HttpResponse;
    fn error<W: Write>(&self, stream: &mut W, status: HttpResponse) -> Result<(), ProtocolError> {
        let bytes: Vec<u8> = status.into();
        let _ = match stream.write(&bytes) {
            Ok(_) => {
                let _ = stream.flush();
                return Ok(());
            }
            Err(_) => return Err(ProtocolError::WriteResponseError),
        };
    }
    fn execute(
        &mut self,
        header: HttpHeaders,
        stream: &mut TcpStream,
        body: Vec<u8>,
    ) -> Result<(), ProtocolError> {
        let response = match header.verb {
            HttpVerb::Get => self.get(header.resource, header.header),
            HttpVerb::Post => self.post(header.resource, header.header, body),
            HttpVerb::Head => self.head(header.resource, header.header),
            HttpVerb::Put => self.put(header.resource, header.header, body),
            HttpVerb::Options => self.options(header.resource, header.header),
            _ => HttpResponse::not_implemented(),
        };
        let _ = stream.flush();
        let bytes: Vec<u8> = response.into();
        let _ = match stream.write(&bytes) {
            Ok(_) => {
                let _ = stream.shutdown(Shutdown::Write);
                return Ok(());
            }
            Err(_) => return Err(ProtocolError::WriteResponseError),
        };
    }
}

pub fn respond_from<W: Write>(mut stream: W, resp: HttpResponse) {
    let bytes: Vec<u8> = resp.into();
    stream.write(&bytes).expect("This should not panic");
}

fn update_initializer(i: &mut HttpInitializer, header_line: String) {
    if i.verb.is_none() {
        let mut parts = header_line.split(" ").collect::<Vec<&str>>();
        if parts.len() == 3 {
            i.http_version = match parts.pop() {
                None => None,
                Some(s) => Some(s.to_string()),
            };
        } else {
            i.http_version = Some("not-set".to_string());
        }

        i.resource = match parts.pop() {
            None => None,
            Some(s) => Some(s.to_string()),
        };
        i.verb = match parts.pop() {
            None => Some(HttpVerb::NotFound),
            Some(v) => Some(HttpVerb::from(v.to_string())),
        };
    } else {
        let mut parts = header_line.split(":").collect::<Vec<&str>>();
        if parts.len() == 2 {
            let value = parts.pop().unwrap().to_lowercase();
            let key = parts.pop().unwrap().to_lowercase();
            let _ = i.header.insert(key, value);
        }
    }
}

/* pub fn read_body<R: Read> (stream: &mut R) -> Result<Vec<u8>, ProtocolError> {
    let br = BufReader::new(stream);
    let body_started = false;
    for line in br.lines().map(| l | l.unwrap()) {

        if line.is_empty() {
            body_started = true
        }
    }
} */

pub fn read_http(stream: &mut TcpStream) -> Result<(HttpHeaders, Vec<u8>), ProtocolError> {
    let mut initalizer = HttpInitializer {
        verb: None,
        resource: None,
        http_version: None,
        header: HashMap::new(),
    };
    let mut http_str = "".to_string();
    let mut body = false;
    let mut body_str = "".to_string();
    let mut buff: [u8; 300] = [0; 300];
    loop {
        let readen = match stream.read(&mut buff) {
            Ok(s) => s,
            Err(_) => 0,
        };
        if let Ok(str) = from_utf8(&buff[..readen]) {
            http_str.push_str(str);
        }
        if readen < buff.len() {
            // request readen
            break;
        }
    }

    for line in http_str.lines() {
        if body {
            body_str.push_str(line);
        }
        if line.is_empty() {
            body = true;
        }
        if !body {
            update_initializer(&mut initalizer, line.to_string());
        }
    }
    let header = HttpHeaders::try_from(initalizer)?;
    Ok((header, body_str.into_bytes()))
}

pub fn read_header<R: Read>(stream: &mut R) -> Result<HttpHeaders, ProtocolError> {
    let br = BufReader::new(stream);
    let mut initalizer = HttpInitializer {
        verb: None,
        resource: None,
        http_version: None,
        header: HashMap::new(),
    };

    for line in br.lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            break;
        }
        update_initializer(&mut initalizer, line);
    }
    HttpHeaders::try_from(initalizer)
}
