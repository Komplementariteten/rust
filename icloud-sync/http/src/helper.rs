use std::error::Error;
use std::io::{Read, Write};
use serde::Serialize;
use filewatcher::filescanner::PathFileEntry;

pub fn from_json_to_entry<R: Read>(reader: R) -> Result<Vec<PathFileEntry>, Box<dyn Error>> {
    let read_res = serde_json::from_reader(reader)?;
    Ok(read_res)
}
pub fn write_to<T: Serialize, W: Write>(writer:&mut W, vector: Vec<T>) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, &vector)
}