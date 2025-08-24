use std::ascii::Char;
use crate::formater::eol_formater::{Out, OutTo};

const WS: Char = unsafe { ' '.as_ascii_unchecked() };
const SM: Char = unsafe {';'.as_ascii_unchecked() };
pub struct AsciiReader {
}

impl AsciiReader {
    fn text_to_f32_by_ws(data: &[u8]) -> Vec<f32> {
        Self::read_by_sep(&WS, data)
    }

    fn read_by_sep(sep: &Char, data: &[u8]) -> Vec<f32> {
        let mut r = vec![];
        let mut t_s = String::new();
        if let Some(ascii_chars) = data.as_ascii() {
            for ascii_char in ascii_chars {
                if ascii_char.eq(sep) {
                    if let Ok(f) = t_s.parse::<f32>() {
                        r.push(f);
                        t_s = String::new();
                    };
                }else{
                    t_s.push(ascii_char.to_char())
                }
            }
        }
        r
        
    }
    
    fn text_to_f32_by_sm(data: &[u8]) -> Vec<f32> {
        Self::read_by_sep(&SM, data)
    }


    pub const fn new_ws<C>(c: C) -> impl Out where C: FnMut(f32) {
         OutTo::new(Self::text_to_f32_by_ws, c)
    }

    pub const fn new_sm<C>(c: C) -> impl Out where C: FnMut(f32) {
        OutTo::new(Self::text_to_f32_by_ws, c)
    }

}