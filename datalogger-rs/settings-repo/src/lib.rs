extern crate core;

use std::error;
use std::fs::File;
use std::path::{Path, PathBuf};
use serde::de::DeserializeOwned;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn load<TJson>(path: &PathBuf) -> Result<TJson> where TJson: DeserializeOwned{
    let jf = File::open(path).expect("Failed to open JSon file");
    match serde_json::from_reader::<File, TJson>(jf) {
        Ok(t) => Ok(t),
        Err(e) => Err(Box::new(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
