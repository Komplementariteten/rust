use dirs::home_dir;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use filewatcher::filescanner::PathFileEntry;
use filewatcher::FileWatcher;
use crate::helper::write_to;

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
    return match FileWatcher::new(idrive_path, cache_path, true) {
        Ok(fw) => Ok(FwInterface{
            fw: fw
        }),
        Err(e) => Err(GeneralFwMiddlewareError::try_from(e).unwrap())
    };
}

impl FwInterface {
    pub async fn update(&mut self) -> Vec<PathFileEntry> {
        let list = self.fw.sync();
        if list.is_some() {
            list.unwrap()
        } else {
            Vec::new()
        }
    }
    pub fn mark_as_send(&mut self, entry: PathFileEntry) {
        self.fw.ack(entry);
    }
    pub async fn stream_update_as_json<W: Write>(&mut self, writer: &mut W) -> serde_json::Result<()> {
        let r = self.update().await;
        write_to(writer, r)
    }
}
