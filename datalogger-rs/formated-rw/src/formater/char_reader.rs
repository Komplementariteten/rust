use crate::formater::eol_formater::{Out, OutTo};
use std::str::FromStr;

const WS: char = ' ';
const SM: char = ';';
pub struct CharReader {}

impl CharReader {
    fn text_to_f32_by_ws(data: &[u8]) -> Vec<f32> {
        Self::read_by_sep(&WS, data)
    }

    fn read_by_sep(sep: &char, data: &[u8]) -> Vec<f32> {
        let mut r = vec![];
        let mut t_s = String::new();
        let str = str::from_utf8(data).expect("Can't read Bytes as String");
        for character in str.chars() {
            if character.eq(sep) {
                if let Ok(f) = t_s.parse::<f32>() {
                    r.push(f);
                    t_s = String::new();
                };
            } else {
                t_s.push(character)
            }
        }
        if let Ok(ft) = t_s.parse::<f32>() {
            r.push(ft);
        }
        r
    }

    fn read_by_sep_any<TOut>(sep: &char, data: &[u8]) -> Vec<TOut>
    where
        TOut: FromStr,
    {
        let mut r = vec![];
        let mut t_s = String::new();
        let str = str::from_utf8(data).expect("Can't read Bytes as String");
        for character in str.chars() {
            if character.eq(sep) {
                if let Ok(f) = t_s.parse::<TOut>() {
                    r.push(f);
                    t_s = String::new();
                };
            } else {
                t_s.push(character)
            }
        }
        if let Ok(ft) = t_s.parse::<TOut>() {
            r.push(ft);
        }
        r
    }

    fn text_to_f32_by_sm(data: &[u8]) -> Vec<f32> {
        Self::read_by_sep(&SM, data)
    }
    
    fn text_to_any<TOut>(sep: char) -> impl Fn(&[u8]) -> Vec<TOut>
    where
        TOut: FromStr,
    {
        let s = sep.to_owned();
        move |d| Self::read_by_sep_any(&s, d)
    }

    pub const fn new_ws<C>(c: C) -> impl Out
    where
        C: FnMut(f32),
    {
        OutTo::new(Self::text_to_f32_by_ws, c)
    }

    pub const fn new_sm<C>(c: C) -> impl Out
    where
        C: FnMut(f32),
    {
        OutTo::new(Self::text_to_f32_by_sm, c)
    }

    pub fn new<C, TOut>(s: char, c: C) -> impl Out
    where
        C: FnMut(TOut), TOut: FromStr
    {
        OutTo::new(Self::text_to_any(s), c)
    }
}
