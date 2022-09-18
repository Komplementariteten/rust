mod pswsafe;
pub mod pswerrors;

use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use crate::pswerrors::PswSafeError;
use crate::pswsafe::PswSafe;
use crate::PswSafeError::{FailedToOpenFile, FileNotFound, FileReadError};


#[derive(Debug)]
pub struct PswFile {
    pub path: PathBuf,
    safe: PswSafe,
    pub is_open: bool,
    pub is_valid: bool
}


impl PswFile {
    pub fn open(file_name: &str, phrase: &str) -> Result<PswFile, PswSafeError> {
        let path = Path::new(file_name);
        if !path.exists() {
            return Err(FileNotFound)
        }
        let mut fs = match File::open(file_name) {
            Ok(fs) => fs,
            Err(_) => return Err(FailedToOpenFile)
        };
        let mut buff = Vec::new();
        let file_size = match fs.read_to_end(&mut buff) {
            Ok(s) => s,
            Err(_) => return Err(FileReadError)
        };

        let mut safe = PswSafe::new();

        Err(FailedToOpenFile)
    }
}