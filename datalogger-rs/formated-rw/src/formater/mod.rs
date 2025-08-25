use std::io::Write;
use std::str::FromStr;
use crate::formater::char_reader::CharReader;
use crate::formater::eol_formater::{EolFormater};
use crate::writer::{FormatedWriter};

mod char_reader;
mod eol_formater;

pub fn formated_line_writer<const M: usize, TOut>(line_break: [u8; M], sep: char, out: impl FnMut(TOut)) -> impl Write where TOut: FromStr {
    FormatedWriter::new(EolFormater::new(line_break, CharReader::new(sep, out)))
}

#[cfg(test)]
mod test {
    use crate::writer::FormatedWriter;
    use std::io::Write;
    use crate::formater::char_reader::CharReader;
    use crate::formater::eol_formater::{EolFormater, OutTo, LF_CR};
    use crate::formater::formated_line_writer;

    #[test]
    fn test_formated_line_writer() {
        let mut out: Vec<i32> = vec![];
        {
            let outf = |v| {
                out.push(v);
            };

            let mut writer = formated_line_writer([0, 1, 2, 0], ' ', outf);
            let mut bytes = vec![];
            for b in "1 2".as_bytes() {
                bytes.push(b.clone())
            }
            bytes.push(0);
            bytes.push(1);
            bytes.push(2);
            bytes.push(0);
            for b in "222 45612".as_bytes() {
                bytes.push(b.clone())
            }
            bytes.push(0);
            bytes.push(1);
            bytes.push(2);
            bytes.push(0);
            writer.write(bytes.as_slice()).expect("Failed to write");
        }
        assert_eq!(out.len(), 4)
     }

    #[test]
    fn construct_type() {
        let ascii = | _b:&[u8] | {
            let mut b = vec![];
            b.push(0.234f32);
            return b;
        };
        let mut r = vec![];
        let formater = OutTo::new(ascii, | l | {
            r.push(l)
        });
        let mut writer = FormatedWriter::new(EolFormater::new(LF_CR, formater));
        let bytes: [u8; 10] = [1, 51, 52, 53, 56, 67, 10, 13, 44, 42];
        let _ = writer.write(&bytes).expect("Can't write to formater");
        assert_eq!(r.len(), 2)
    }

    #[test]
    fn test_simple_char_reader_with_whitespace() {
        let mut r = vec![];
        {
            let collect = |l| {
                r.push(l);
            };

            let mut writer = FormatedWriter::new(EolFormater::new(LF_CR, CharReader::new_ws(collect)));
            let bytes = "0.123 123.223".as_bytes();
            let mut data_v = bytes.to_vec();
            data_v.push(10);
            data_v.push(13);
            data_v.push(1);

            writer.write(&data_v).expect("Can't write to formater");
        }
        assert_eq!(r.len(), 2)
    }
    #[test]
    fn test_simple_char_reader_with_semicolon() {
        let mut r = vec![];
        {
            let collect = |l| {
                r.push(l);
            };

            let mut writer = FormatedWriter::new(EolFormater::new(LF_CR, CharReader::new_sm(collect)));
            let bytes = "0.123;123.223".as_bytes();
            let mut data_v = bytes.to_vec();
            data_v.push(10);
            data_v.push(13);
            data_v.push(1);

            writer.write(&data_v).expect("Can't write to formater");
        }
        assert_eq!(r.len(), 2)
    }

    #[test]
    fn test_simple_char_reader_with_any() {
        let mut r: Vec<f32> = vec![];
        {
            let collect = |l| {
                r.push(l);
            };

            let mut writer = FormatedWriter::new(EolFormater::new(LF_CR, CharReader::new('-', collect)));
            let bytes = "0.123-123.223".as_bytes();
            let mut data_v = bytes.to_vec();
            data_v.push(10);
            data_v.push(13);
            data_v.push(1);

            writer.write(&data_v).expect("Can't write to formater");
        }
        assert_eq!(r.len(), 2)
    }
}
