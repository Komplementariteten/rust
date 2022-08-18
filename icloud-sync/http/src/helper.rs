use std::io::Write;
use serde::Serialize;


pub fn from_vec_to_utf8_json<T: Serialize>(vector: Vec<T>) -> serde_json::Result<Vec<u8>> {
    serde_json::to_vec_pretty(&vector)
}
pub fn write_to<T: Serialize, W: Write>(writer:&mut W, vector: Vec<T>) -> serde_json::Result<()> {
    serde_json::to_writer_pretty(writer, &vector)
}