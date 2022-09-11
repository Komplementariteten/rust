use async_std::task::block_on;
use dirs::home_dir;
use filewatcher::filescanner::PathFileEntry;
use filewatcher::FileWatcher;
use http::helper::{from_json_to_entry, write_to};
use http::response::{BaseHttpRouting, HttpResponse};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::read;
use std::io::{BufWriter, Read, Write};
use std::mem;
use std::path::{Path, PathBuf};

type GeneralFwMiddlewareError = Box<dyn Error + Sync + Send + 'static>;

#[derive(Debug)]
pub enum FileWatcherInterfaceErrors {
    IDrivePathDoesNotExist,
    HomeDirNotFound,
}

impl Display for FileWatcherInterfaceErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
pub struct FwInterface {
    fw: FileWatcher,
    base_path: PathBuf,
    mem: Vec<PathFileEntry>,
    current: usize,
    transmition_length: usize
}

impl Error for FileWatcherInterfaceErrors {}

pub const MIDDLEWARE_CACHE_DIR: &str = ".idrive-sync";

pub fn init_idrive() -> Result<FwInterface, GeneralFwMiddlewareError> {
    let hd = match home_dir() {
        Some(d) => d,
        None => {
            return Err(GeneralFwMiddlewareError::try_from(
                FileWatcherInterfaceErrors::HomeDirNotFound,
            )
            .unwrap())
        }
    };
    let idrive_path = hd
        .join("Library")
        .join("Mobile Documents")
        .join("com~apple~CloudDocs");
    let cache_path = hd.join(MIDDLEWARE_CACHE_DIR);
    if !idrive_path.as_path().exists() {
        return Err(GeneralFwMiddlewareError::try_from(
            FileWatcherInterfaceErrors::IDrivePathDoesNotExist,
        )
        .unwrap());
    }
    FwInterface::new(idrive_path, cache_path)
}

impl BaseHttpRouting for FwInterface {
    fn get(&mut self, resource: String, aditional_header: HashMap<String, String>) -> HttpResponse {
        let rel_path = match resource.strip_prefix("/") {
            Some(s) => s,
            None => return HttpResponse::bad_request(),
        };

        // List Files
        if rel_path.starts_with("list") {
            let mut writer = BufWriter::new(Vec::new());
            let _ = block_on(self.send_entry_list(&mut writer));
            return HttpResponse::ok_with_json(writer.buffer());
        }

        // Download File
        if rel_path.starts_with("file") {
            let file_id_str = rel_path.strip_prefix("file/").unwrap();
            let file_id = match file_id_str.parse::<u32>() {
                Ok(id) => id,
                Err(_) => return HttpResponse::not_found(),
            };
            let files = match self.mem.iter().find(|&entry| entry.id == file_id) {
                Some(e) => e.clone(),
                None => return HttpResponse::not_found(),
            };

            let file_path = self.base_path.join(files.path);
            println!("Requested File {}", file_path.to_str().unwrap());
            let data = match read(file_path) {
                Ok(d) => d,
                Err(_) => return HttpResponse::server_error(),
            };
            return HttpResponse::ok_bin(data, false);
        }
        return HttpResponse::not_found();
    }

    fn post(
        &mut self,
        resource: String,
        aditional_header: HashMap<String, String>,
        stream: Vec<u8>,
    ) -> HttpResponse {
        let rel_path = match resource.strip_prefix("/") {
            Some(s) => s,
            None => return HttpResponse::bad_request(),
        };

        if rel_path.starts_with("ack") {
            let result = block_on(self.ack_transferred(stream.as_slice()));

            return match result {
                Ok(_) => HttpResponse::ok(),
                Err(_) => HttpResponse::server_error(),
            }
        }
        return HttpResponse::not_implemented();
    }

    fn head(
        &mut self,
        resource: String,
        aditional_header: HashMap<String, String>,
    ) -> HttpResponse {
        HttpResponse::ok()
    }

    fn put(
        &mut self,
        resource: String,
        aditional_header: HashMap<String, String>,
        stream: Vec<u8>,
    ) -> HttpResponse {
        HttpResponse::not_implemented()
    }

    fn delete(
        &mut self,
        resource: String,
        aditional_header: HashMap<String, String>,
    ) -> HttpResponse {
        HttpResponse::not_implemented()
    }

    fn options(
        &mut self,
        resource: String,
        aditional_header: HashMap<String, String>,
    ) -> HttpResponse {
        HttpResponse::not_implemented()
    }
}

unsafe impl Send for FwInterface {}

impl FwInterface {
    pub fn new<P: AsRef<Path>>(
        folder: P,
        cache: P,
    ) -> Result<FwInterface, GeneralFwMiddlewareError> {
        let pathBuff = folder.as_ref().to_path_buf();
        return match FileWatcher::new(folder, cache, true) {
            Ok(fw) => {
                let mut fwi = FwInterface {
                    base_path: pathBuff,
                    mem: Vec::new(),
                    fw,
                    transmition_length: 20,
                    current: 0
                };
                Ok(fwi)
            }
            Err(e) => Err(GeneralFwMiddlewareError::try_from(e).unwrap()),
        };
    }


    async fn update(&mut self) -> Vec<PathFileEntry> {
        let list = self.fw.sync();
        if list.is_some() {
            let v = list.unwrap();
            self.mem = v.clone();
            v
        } else {
            Vec::new()
        }
    }

    pub async fn ack_transferred<R: Read>(&mut self, read: R) -> Result<u32, Box<dyn Error>> {
        let entries = from_json_to_entry(read)?;
        let mut ok_count: u32 = 0;
        for entry in entries {
            self.fw.ack(entry.clone());
            let item_index = self.mem.iter().position(| i| i.id == entry.id).unwrap();
            self.mem.remove(item_index);
            ok_count += 1;
        }
        Ok(ok_count)
    }

    pub async fn send_entry_list<W: Write>(
        &mut self,
        writer: &mut W,
    ) -> serde_json::Result<()> {
        if self.mem.len() > 0 {
            println!("update from mem");
            let data_slice = self.mem.as_slice();
            let length = if data_slice.len() <= self.transmition_length {
                data_slice.len()
            } else {
                self.transmition_length
            };
            write_to(writer, &data_slice[self.current..length])
        } else {
            println!("build up new cache");
            let r = self.update().await;
            self.mem = r.clone();
            let data_slice = r.as_slice();
            let length = if data_slice.len() <= self.transmition_length {
                data_slice.len()
            } else {
                self.transmition_length
            };
            write_to(writer, &data_slice[self.current..length])
        }
    }
}
