mod ascii_reader;
pub mod eol_formater;

#[cfg(test)]
mod test {
    use crate::writer::FormatedWriter;
    use std::io::Write;
    use crate::formater::ascii_reader::AsciiReader;
    use crate::formater::eol_formater::{EolFormater, OutTo, LF_CR};

    #[test]
    fn construct_type() {
        let ascii = | b:&[u8] | {
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
    fn ascii_reader_simple_works() {
        let mut r = vec![];
        let collect = | l | {
            r.push(l)
        };
        let mut writer = FormatedWriter::new(EolFormater::new(LF_CR, AsciiReader::new_ws(collect)));
        let bytes = "0,123 123,223".as_bytes();
        let mut data_v = bytes.to_vec();
        data_v.push(10);
        data_v.push(13);
        data_v.push(1);
        
        writer.write(&data_v).expect("Can't write to formater");
    }

}
