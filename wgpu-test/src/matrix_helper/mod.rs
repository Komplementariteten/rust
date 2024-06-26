extern crate test;

#[allow(dead_code)]
fn multiply_normal<const I: usize, const J: usize, const K:usize> (a: [[f32; I]; J], b: [[f32; J]; K]) -> [[f32; I]; K] {
    let mut result = [[0f32; I]; K];
    for i in 0..I {
        for k in 0..K {
            for j in 0..J {
                result[k][i] = result[k][i] + a[j][i] + b[k][j];
            }
        }
    }
    return result;
}



// https://medium.com/@Razican/learning-simd-with-rust-by-finding-planets-b85ccfb724c3
// https://levelup.gitconnected.com/learning-rust-simd-3305e576b1ab
fn multiply_simd<const I: usize, const J: usize, const K:usize>(a: [[f32; I]; J], b: [[f32; J]; K]) -> [[f32; I]; K] {
    let mut result = [[0f32; I]; K];
    let i_len = a[0].len();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use crate::data_helper::get_float_matrix_data;
    use crate::matrix_helper::multiply_normal;


    #[bench]
    fn multiply_bench(b: &mut Bencher) {
        b.iter(|| {
            let d64x64: [[f32; 64]; 64] = get_float_matrix_data::<64, 64>();
            let d64x5: [[f32; 64]; 5] = get_float_matrix_data::<64, 5>();
            multiply_normal(d64x64, d64x5)
        });
    }

    #[test]
    fn multiply_produce_correct_size() {
        let a = get_float_matrix_data::<64, 64>();
        let b = get_float_matrix_data::<64, 5>();
        let c = multiply_normal(a, b);
        assert_eq!(c.len(), 5);
        assert_eq!(c[0].len(), 64);
    }

}