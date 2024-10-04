use rand::Rng;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Vector a(len{a}) is bigger than b(len({b}), can't add a to b")]
    SizeError {a: u64, b: u64}
}

#[no_mangle]
fn add_a_to_b_copy<'a>(a: &'a [f64], b: &'a [f64]) -> Result<Vec<f64>> {
    let a_size = a.len();
    if a.len() > b.len() {
        return Err(ApiError::SizeError { a: a.len() as u64, b: b.len() as u64 });
    }
    let mut r = b.to_vec();
    for i in 0..a_size {
        r[i] = b[i]+a[i];
    }
    return Ok(r);
}

#[no_mangle]
fn gen_random_copy(size: u64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut v = vec![];
    for _ in 0..size {
        v.push(rng.gen::<f64>());
    }
    v
}

type Result<T, E=ApiError> = std::result::Result<T, E>;

uniffi::setup_scaffolding!();

#[cfg(test)]
mod test{
    use crate::{add_a_to_b_copy, gen_random_copy};

    #[test]
    fn add_random() {
        let a = gen_random_copy(10000);
        let b = gen_random_copy(10000);
        let r = add_a_to_b_copy(&a, &b);
        assert!(r.is_ok());
        assert_eq!(r.unwrap().len(), b.len());
    }
}
