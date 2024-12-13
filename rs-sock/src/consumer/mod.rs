use std::fs::File;
use std::io::{BufReader, BufWriter, IoSlice, Read, Write};
use std::net::TcpStream;

#[derive(Debug, PartialEq, Clone)]
pub enum ConsumerState {
    Response(Vec<u8>),
    Ready,
    Finished,
}

pub enum ConsumeResult {
    
}

pub struct ReaderWriter<T: Write + Read> {
    r: BufReader<T>,
    w: BufWriter<T>
}

impl<'a> Into<ReaderWriter<&TcpStream>> for &TcpStream{
    fn into(self) -> ReaderWriter<&'a TcpStream> {
        ReaderWriter{
            r: BufReader::new(self),
            w: BufWriter::new(self),
        }
    }
}

pub type ConsumerResult<T> = Result<T, ConsumerError>;
pub type ConsumerError = Box<dyn std::error::Error>;
pub trait Consumer {
    fn read_next(&mut self, read_write: &mut BufReader<u8>) -> ConsumerResult<ConsumerState>;
}

fn consume<T: std::io::Write>(readable: &mut ReaderWriter<T>, mut c: Box<dyn Consumer>) -> ConsumerResult<()> {
    let r = match c.read_next(readable) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    if ConsumerState::Ready == r {
        return Ok(())
    }
    if r == ConsumerState::Finished {
        return Ok(())
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::IoSlice;
    use super::*;

    struct TestConsumer {
        data: Vec<u8>
    }

    impl TestConsumer {
        fn new(buff_size: usize) -> TestConsumer {
            TestConsumer {
                data: vec![0; buff_size]
            }
        }
    }
    impl Consumer for TestConsumer {
        fn read_next(&mut self, read_write: &mut dyn ReadWrite) -> ConsumerResult<ConsumerState> {
            let data = self.data.as_mut_slice();
            let _ = read_write.read(data);
            match read_write.write_all([1, 2, 3, 4, 5, 6, 7, 8].as_ref()) {
                Ok(_) => Ok(ConsumerState::Finished),
                Err(e) => Err(Box::new(e)),
            }
        }
    }
    
    #[test]
    fn consume_consumes_next() {
        let co = TestConsumer::new(2);
        let data = [1; 100];
        let mut read_write = IoSlice::new(&data);
        consume(&mut read_write, Box::new(co)).unwrap()
    }
}