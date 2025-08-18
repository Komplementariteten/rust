use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum H5FileError {
    GroupNotFound,
    GroupExist,
    NoDataset,
    MoreThanThreeDimNotSupported
}

impl fmt::Display for H5FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for H5FileError{
    
}