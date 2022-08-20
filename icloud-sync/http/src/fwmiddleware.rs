use dirs::home_dir;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Write};
use std::path::Path;
use filewatcher::filescanner::PathFileEntry;
use filewatcher::FileWatcher;
use crate::helper::{from_json_to_entry, write_to};

type GeneralFwMiddlewareError = Box<dyn Error + Sync + Send + 'static>;

#[derive(Debug)]
pub enum FileWatcherInterfaceErrors {
    IDrivePathDoesNotExist,
    HomeDirNotFound
}

impl Display for FileWatcherInterfaceErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct FwInterface {
    fw: FileWatcher
}

impl Error for FileWatcherInterfaceErrors{}

pub const MIDDLEWARE_CACHE_DIR: &str = ".idrive-sync";

pub fn init_idrive() -> Result<FwInterface, GeneralFwMiddlewareError> {
    let hd = match home_dir() {
        Some(d) => d,
        None => return Err(GeneralFwMiddlewareError::try_from(FileWatcherInterfaceErrors::HomeDirNotFound).unwrap())
    };
    let idrive_path = hd.join("Library").join("Mobile Documents").join("com~apple~CloudDocs");
    let cache_path = hd.join(MIDDLEWARE_CACHE_DIR);
    if !idrive_path.as_path().exists() {
        return Err(GeneralFwMiddlewareError::try_from(FileWatcherInterfaceErrors::IDrivePathDoesNotExist).unwrap());
    }
    FwInterface::new(idrive_path, cache_path)
}

impl FwInterface {

    pub fn new<P: AsRef<Path>>(folder:P, cache: P) -> Result<FwInterface, GeneralFwMiddlewareError> {
        return match FileWatcher::new(folder, cache, true) {
            Ok(fw) => Ok(FwInterface{
                fw: fw
            }),
            Err(e) => Err(GeneralFwMiddlewareError::try_from(e).unwrap())
        };
    }

    pub async fn update(&self) -> Vec<PathFileEntry> {
        let list = self.fw.sync();
        if list.is_some() {
            list.unwrap()
        } else {
            Vec::new()
        }
    }

    pub async fn ack_from_stream<R: Read>(&mut self, read: R) -> Result<u32, Box<dyn Error>> {
        let entries = from_json_to_entry(read)?;
        let mut ok_count:u32 = 0;
        for entry in entries {
            self.fw.ack(entry);
            ok_count += 1;
        }
        Ok(ok_count)
    }

    pub async fn stream_update_as_json<W: Write>(&self, writer: &mut W) -> serde_json::Result<()> {
        let r = self.update().await;
        write_to(writer, r)
    }
}
