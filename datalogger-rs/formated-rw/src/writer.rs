use std::error;
use std::io::{ErrorKind, Read, Write as IoWrite, Write};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;

pub trait Formater {
    fn process(&mut self, data: &mut impl Read, remaining: &mut impl Write) -> Result<usize>;
}

const BUFFER_SIZE: usize = 0xFFFF;

#[derive(Debug, PartialEq, Clone, Copy)]
struct RemainingWriter {
    _remain: [u8; BUFFER_SIZE],
    _pos: usize,
    _cur: usize,
}

impl RemainingWriter {
    const fn new() -> RemainingWriter {
        RemainingWriter {
            _pos: 0,
            _cur: 0,
            _remain: [0; BUFFER_SIZE],
        }
    }

    pub(crate) fn remaining(&self) -> Option<Vec<u8>> {
        if self._pos == 0 {
            return None
        }
        let mut copy = vec![0u8; self._pos];
        copy.copy_from_slice(&self._remain[..self._pos]);
        return Some(copy)
    }
}

impl Read for RemainingWriter {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        if self._pos == 0 {
            return Ok(0usize)
        }
        // Handle small reads
        if self._cur >= self._pos {
            self._cur = 0;
            return Ok(0usize)
        }
        let written = buf.write(&self._remain[self._cur..self._pos])?;
        self._cur += written;
        Ok(written)
    }
}

impl Write for RemainingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut len = buf.len();
        if len == 0 {
            return Ok(0)
        }
        len += self._pos;
        self._remain[self._pos..len].copy_from_slice(buf);
        self._pos = len;
        Ok(self._pos)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self._pos = 0;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct FormatedWriter<TF: Formater> {
    _f: TF,
    _r: RemainingWriter,
}

impl<TF: Formater> FormatedWriter<TF> {
    const fn new(formater: TF) -> FormatedWriter<TF> {
        FormatedWriter {
            _r: RemainingWriter::new(),
            _f: formater,
        }
    }

}

impl<TF: Formater> IoWrite for FormatedWriter<TF> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut chained_reader = self._r.chain(buf);
        let mut processed_bytes = 0;
        loop {
            let n = match self._f.process(&mut chained_reader, &mut self._r) {
                Ok(n) => n,
                Err(e) => return Err(std::io::Error::new(ErrorKind::InvalidData, e)),
            };
            processed_bytes += n;
            if n == 0 {
                break;
            }
        }
        Ok(processed_bytes)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self._r.flush()?;
        Ok(())
    }
}

/* impl<const N:usize, const M:usize> FmtWrite for FormatedWriter<N> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        todo!()
    }
} */

#[cfg(test)]
mod tests {
    use crate::writer::{FormatedWriter, Formater};
    use std::io::{Read, Write};
    const SEP: [u8; 4] = [0, 10, 13, 0];
    #[derive(Debug)]
    struct TestFormater {}

    impl Formater for TestFormater {
        fn process(
            &mut self,
            data: &mut impl Read,
            remain: &mut impl Write,
        ) -> crate::writer::Result<usize> {
            // data.
            let mut data_v = vec![];
            data.read_to_end(&mut data_v).expect("TODO: panic message");
            if let Some(n) = data_v.array_windows::<4>().position(|s| s.eq(&SEP)) {
                remain.flush().expect("TODO: panic message");
                remain.write(&data_v[(n + 4)..])?;
                return Ok(n)
            }
            remain.write(data_v.as_slice())?;
            Ok(0)
        }
    }

    #[test]
    fn read_multiple_correct() {
        const SIZE: usize = 56;
        let mut data: [u8; SIZE] = [0; SIZE];
        let last = 40;
        data[10] = 10;
        data[11] = 13;
        data[last -1] = 10;
        data[last] = 13;
        data[SIZE - 1] = 22;
        let mut reader = FormatedWriter::new(TestFormater {});
        let read = reader.write(&data).expect("can't write");
        let r1 = reader._r.remaining().expect("nothing remaing");
        let len1 = r1.len();
        assert_eq!(len1, SIZE - last);
        assert_eq!(r1[len1 - 1], 22);
        assert_eq!(read, 0usize);
        let data2 = [13, 0, 1, 2, 3];
        let read2 = reader.write(&data2).expect("can't write");
        let r1 = reader._r.remaining().expect("nothing remaing");
        assert_eq!(r1.len(), 3usize);
        assert_eq!(r1[r1.len() - 1], 3);
        assert_eq!(read2, 54usize);
    }

    #[test]
    fn read_all() {
        let mut data: [u8; 4096] = [0; 4096];
        data[1201..1205].copy_from_slice(&SEP);
        let mut reader = FormatedWriter::new(TestFormater {});
        assert_eq!(reader.write(&data).expect("can't write"), 1201)
    }

    #[test]
    fn read_remain_correct() {
        const SIZE: usize = 4096;
        let mut data: [u8; SIZE] = [0; SIZE];
        data[1201..1205].copy_from_slice(&SEP);
        data[SIZE - 1] = 255;
        let mut reader = FormatedWriter::new(TestFormater {});
        reader.write(&data).expect("can't write");
        let remaining = reader._r.remaining().expect("no remaing");
        assert!(remaining.len() > 0);
        let last = remaining.last().expect("no data").clone();
        assert_eq!(last, 255u8);
    }

    #[test]
    fn read_can_continue_and_finish() {
        const SIZE: usize = 56;
        let mut data: [u8; SIZE] = [0; SIZE];
        data[SIZE - 1] = 10;
        let mut reader = FormatedWriter::new(TestFormater {});
        let read = reader.write(&data).expect("can't write");
        let r1 = reader._r.remaining().expect("nothing remaing");
        let len1 = r1.len();
        assert_eq!(len1, SIZE);
        assert_eq!(r1[len1-1], 10);
        assert_eq!(read, 0usize);
        let data2 = [13, 0];
        let read2 = reader.write(&data2).expect("can't write");
        let r2 = reader._r.remaining();
        assert_eq!(r2, None);
        assert_eq!(read2, 54usize);
    }

    #[test]
    fn read_can_continue_with_remain() {
        const SIZE: usize = 56;
        let mut data: [u8; SIZE] = [0; SIZE];
        data[SIZE - 1] = 10;
        let mut reader = FormatedWriter::new(TestFormater {});
        let read = reader.write(&data).expect("can't write");
        let r1 = reader._r.remaining().expect("nothing remaing");
        let len1 = r1.len();
        assert_eq!(len1, SIZE);
        assert_eq!(r1[len1 - 1], 10);
        assert_eq!(read, 0usize);
        let data2 = [13, 0, 1, 2, 3];
        let read2 = reader.write(&data2).expect("can't write");
        let r1 = reader._r.remaining().expect("nothing remaing");
        assert_eq!(r1.len(), 3usize);
        assert_eq!(r1[r1.len() - 1], 3);
        assert_eq!(read2, 54usize);
    }
}
