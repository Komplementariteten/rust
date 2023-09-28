#![feature(test)]

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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use crate::data_helper::get_float_matrix_data;
    use crate::matrix_helper::multiply_normal;
    #[bench]
    fn multiply_bench(b: &mut Bencher) {
        let a = get_float_matrix_data::<64, 64>();
        let b = get_float_matrix_data::<64, 5>();
        let c = multiply_normal(a, b);
        assert_eq!(c.len(), 5);
        assert_eq!(c[0].len(), 64);
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