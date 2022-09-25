mod pswfile;
pub mod pswerrors;
mod pswdb;
mod util;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use crate::pswerrors::PswSafeError;
use crate::pswfile::PswSafe;
use crate::PswSafeError::{FailedToOpenFile, FileNotFound, FileReadError};
const BLOCK_SIZE: usize = 16;


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
        safe.load(&buff)?;
        if safe.check_key(phrase.as_bytes().to_vec())? {
            return Ok(PswFile {
                is_open: false,
                is_valid: true,
                safe,
                path: path.to_path_buf()
            })
        }
        Err(FailedToOpenFile)
    }
}