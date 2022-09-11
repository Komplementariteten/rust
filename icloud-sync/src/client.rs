use std::str::from_utf8;
use async_std::fs::{copy, create_dir_all, OpenOptions, remove_file};
use async_std::io::WriteExt;
use async_std::path::Path;
use serde_json::json;
use filewatcher::filescanner::PathFileEntry;
use http::request::Request;
use crate::client::SyncError::{CreateTempFileError, FailedToAcknowledge, FailedToGetFileList, FilePathError, ReadFileListError, WriteTempFileError};

#[derive(Debug)]
pub enum SyncError {
    FailedToGetFileList,
    ReadFileListError,
    FilePathError,
    CreateTempFileError,
    WriteTempFileError,
    FailedToAcknowledge
}

#[derive(Debug)]
pub struct SyncClient {
    base_url: &'static str,
    tmp_folder: &'static str,
    output_folder: &'static str,
    file_list: Vec<PathFileEntry>
}

impl SyncClient {
    pub fn new(con: &'static str, tmp_folder: &'static str, output_folder: &'static str) -> SyncClient {
        SyncClient {
            base_url: con,
            tmp_folder,
            output_folder,
            file_list: Vec::new()
        }
    }

    fn reade_file_list<'a>(&mut self, json:&'a str) -> Result<(), SyncError> {
        let pe: Vec<PathFileEntry> = match serde_json::from_str(json) {
            Ok(l) => l,
            Err(e) => {
                println!("{}", json);
                panic!("{:?}", e)
            }
        };
        self.file_list = pe;
        Ok(())
    }

    pub async fn sync(&mut self) -> Result<usize, SyncError> {

        let list_str = format!("{}list", self.base_url);
        let list_json = match list_str.gets().expect("connection url not valid").json().text_async().await {
            Ok(b) => b,
            Err(_) => return Err(FailedToGetFileList)
        };

        let _ = self.reade_file_list(list_json.as_str())?;

        let mut ack_entries = Vec::new();

        for file in &self.file_list {
            let file_url = format!("{}file/{}", self.base_url, file.id);
            println!("requesting-file:{}", file_url);
            let file_bytes = file_url.gets().expect("can't get file").bytes_async().await;
                //.expect("failed to read file");
            if file_bytes.is_ok() {
                let path_str = match file.path.to_str() {
                    Some(p) => p,
                    None => return Err(FilePathError)
                };
                let tmp_path = Path::new(self.tmp_folder).join(path_str);
                let out_path = Path::new(self.output_folder).join(path_str);
                println!("Writing {}:{}", tmp_path.parent().unwrap().to_str().unwrap(), tmp_path.file_name().unwrap().to_str().unwrap());

                match create_dir_all(tmp_path.parent().unwrap()).await {
                    Ok(_) => (),
                    Err(_) => panic!("can't create parent directories")
                };

                let mut fp = match OpenOptions::new().write(true).truncate(true).create(true).open(tmp_path.clone()).await {
                    Ok(f) => f,
                    Err(_) => return Err(CreateTempFileError)
                };

                let bytes = file_bytes.unwrap();

                match fp.write_all(&bytes).await {
                    Ok(_) => {
                        fp.flush();
                        match create_dir_all(out_path.parent().unwrap()).await {
                            Ok(_) => (),
                            Err(_) => panic!("can't create parent directories")
                        };
                        match copy(tmp_path.clone(), out_path).await {
                            Ok(_) => (),
                            Err(_) => panic!("failed to copy to target")
                        }
                        match remove_file(tmp_path).await {
                            Ok(_) => (),
                            Err(_) => panic!("failed to remove tmp file")
                        };
                    },
                    Err(_) => return Err(WriteTempFileError)
                };



                let hash = crc32fast::hash(&bytes);
                if hash == file.crc32 {
                    println!("hash check ok for {}", file.path.to_str().unwrap());

                    ack_entries.push(file.clone());
                } else {
                    println!("hash does not match for {} {} != {}", file.path.to_str().unwrap(), hash, file.crc32);
                }
            } else {
                println!("Failed to download file {} with {:?}", file.path.to_str().unwrap(), file_bytes.err().unwrap());
            }

        }

        let json_str = serde_json::to_string(&ack_entries);
        if json_str.is_ok() {
            let ack_url = format!("{}ack", self.base_url);
            let json = json_str.unwrap();
            println!("Posting to {} => {}", ack_url.as_str(), json.as_str());
            let req_res = match ack_url.post().expect("failed to send ack request").json().body(json.as_str()).text_async().await {
                Ok(s) => s,
                Err(e) => panic!("Failed to ack {:?}", e)
            };
            println!("post returned {}", req_res);
            return Ok(ack_entries.len());
        } else {
            panic!("{}", json_str.unwrap());
        }
        Err(FailedToAcknowledge)
    }
}
