extern crate dslib;
use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;
use dslib::*;

#[derive(PartialEq, Debug)]
pub enum OpenStoreError {
    DatabaseNotFound
}

struct DatastoreFiles {
    is_valid: bool,
    has_index: bool,
    has_data_file: bool,
    data_file_path: PathBuf
}   

impl DatastoreFiles {
    fn new(path: PathBuf) -> DatastoreFiles {
        DatastoreFiles{
            is_valid: false,
            has_data_file: false,
            has_index: false,
            data_file_path: path
        }
    }
}

pub fn init_datastore(path: PathBuf) -> Result<Datastore, OpenStoreError> {
    let mut db_files = Vec::new();
    let ds =  Datastore::new(path.to_path_buf());
    if path.is_dir() {
        for dir_entry in path.read_dir().unwrap() {
            if let Ok(entry) = dir_entry {
                let entry_path = entry.path();
                let extension = entry_path.extension().and_then(OsStr::to_str);
                if let Ok(entry_metadata) = entry.metadata() {
                    if entry_metadata.is_file() && extension == Some(".db") {
                        db_files.push(DatastoreFiles::new(entry_path));
                    }
                }
            }
        }
        for db_file in db_files {
            println!("{:?}, valid:{:?}", db_file.data_file_path, db_file.is_valid)
        }
        return Ok(ds)
    }
    return Err(OpenStoreError::DatabaseNotFound)
}