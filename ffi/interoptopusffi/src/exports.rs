use core::slice::SlicePattern;
use interoptopus::{callback, ffi_function};
use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
use rand::Rng;

callback!(CallbackFloatSlice(slice: FFISlice<f64>) -> ());
#[ffi_function]
#[no_mangle]
pub extern "C" fn gen_random_copy(size: u32) -> FFISlice<'static, f64> {
    let mut rng = rand::thread_rng();
    let mut v = vec![];
    for _ in 0..size {
        v.push(rng.gen::<f64>());
    }
    let r = v.as_slice();
    FFISlice::from(r.iter().copied().)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn add_a_to_b_copy(a: &FFISlice<f64>, b: &mut FFISliceMut<f64>) {
    let bsize = b.len();
    let asize = a.len();
    let minsize = std::cmp::min(asize, bsize);
    for i in 0..minsize {
        b[i] += a[i];
    }
}

#[cfg(test)]
mod tests {
    use interoptopus::patterns::slice::FFISlice;
    use crate::exports::gen_random_copy;

    #[test]
    fn gen_and_add() {
        gen_random_copy(100000, get_rand_data);
    }
    
    fn get_rand_data(s: FFISlice<f64>) -> () {
        return ();
    }
}