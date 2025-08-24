use crate::writer::Formater;
use std::io::Read;

pub const LF_CR: [u8; 2] = [10, 13];

pub trait Out{
    type Output;
    type Callback: FnMut(Self::Output);

    fn format(&mut self, data: &[u8]);
    // fn new(c: Self::Callback) -> impl Out;
}

pub struct OutTo<C, O, F> where C: FnMut(O), F: Fn(&[u8]) -> Vec<O> {
    _c: F,
    _o: C,
}

impl<C, O, F> Out for OutTo<C, O, F> where C: FnMut(O), F: Fn(&[u8]) -> Vec<O> {
    type Output = O;
    type Callback = C;

    fn format(&mut self, data: &[u8]) {
        for r in (self._c)(data) {
            (self._o)(r);
        }
    }
}

impl<C, O, F> OutTo<C, O, F> where C: FnMut(O), F: Fn(&[u8]) -> Vec<O> {
    pub const fn new(f: F, o: C) -> OutTo<C, O, F> {
        OutTo {
            _c: f,
            _o: o,
        }
    }
}

pub struct EolFormater<const N: usize, TOut: Out>
{
    _eol: [u8; N],
    _o: TOut
}

impl<const N: usize, TOut: Out> EolFormater<N, TOut>
{
    pub const fn new(eol: [u8; N], f: TOut) -> EolFormater<N, TOut> {
        EolFormater { _eol: eol, _o: f}
    }

}

impl<const N: usize, TOut: Out> Formater for EolFormater<N, TOut>
{
    fn process(&mut self, data: &mut impl Read) -> crate::writer::Result<usize> {
        let sep = self._eol;
        let mut slice = vec![];
        self.read_buf(data, &sep, &mut slice)?;
        self._o.format(&slice);
        Ok(slice.len())
    }
}
