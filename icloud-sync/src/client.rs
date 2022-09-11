use std::str::from_utf8;
use async_std::fs::{create_dir_all, OpenOptions};
use async_std::io::WriteExt;
use async_std::path::Path;
use filewatcher::filescanner::PathFileEntry;
use http::request::Request;
use crate::client::SyncError::{CreateTempFileError, FailedToGetFileList, FilePathError, ReadFileListError, WriteTempFileError};

#[derive(Debug)]
pub enum SyncError {
    FailedToGetFileList,
    ReadFileListError,
    FilePathError,
    CreateTempFileError,
    WriteTempFileError,
}

#[derive(Debug)]
pub struct SyncClient {
    base_url: &'static str,
    tmp_folder: &'static str,
    file_list: Vec<PathFileEntry>
}

impl SyncClient {
    pub fn new(con: &'static str, tmp_folder: &'static str, output_folder: &'static str) -> SyncClient {
        SyncClient {
            base_url: con,
            tmp_folder,
            file_list: Vec::new()
        }
    }

    fn update_file_list<'a>(&mut self, json:&'a str) -> Result<(), SyncError> {
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

    pub async fn sync(&mut self) -> Result<(), SyncError> {

        let list_str = format!("{}list", self.base_url);
        let list_bytes = match list_str.gets().expect("connection url not valid").json().bytes_async().await {
            Ok(b) => b,
            Err(_) => return Err(FailedToGetFileList)
        };
        let str = from_utf8(list_bytes.as_slice()).expect("can't read bytes as utf8");
        let mut content_str = "".to_string();
        let mut body_started = false;
        for line in str.lines() {
            if body_started {
                content_str.push_str(line);
            }
            if line.len() == 0 {
                body_started = true;
            }
        }

        let _ = self.update_file_list(content_str.as_str())?;

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
                let out_path = Path::new(self.tmp_folder).join(path_str);
                println!("Writing {}:{}", out_path.parent().unwrap().to_str().unwrap(), out_path.file_name().unwrap().to_str().unwrap());

                match create_dir_all(out_path.parent().unwrap()).await {
                    Ok(_) => (),
                    Err(_) => panic!("can't create parent directories")
                };

                let mut fp = match OpenOptions::new().write(true).truncate(true).create(true).open(out_path).await {
                    Ok(f) => f,
                    Err(_) => return Err(CreateTempFileError)
                };

                match fp.write_all(file_bytes.unwrap().as_slice()).await {
                    Ok(_) => (),
                    Err(_) => return Err(WriteTempFileError)
                };

            }
        }

        Ok(())
    }
}
