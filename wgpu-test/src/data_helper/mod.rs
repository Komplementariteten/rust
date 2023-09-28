use rand::Rng;

pub fn get_float_matrix_data<const N: usize, const M: usize>() -> [[f32; N]; M]{
    let mut rng = rand::thread_rng();
    let mut matrix:[[f32; N]; M] = [[0f32; N]; M];
    for m in matrix.iter_mut() {
        for n in m.iter_mut() {
            *n = rng.gen();
        }
    }

    return matrix;
}

#[cfg(test)]
mod tests {
    use crate::data_helper::{get_float_matrix_data};
    #[test]
    fn get_float_matrix_data_generates_correct_size() {
        let matrix: [[f32; 3]; 3] = get_float_matrix_data();
        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 3);
    }

    #[test]
    fn get_float_matrix_data_generates_values() {
        let matrix = get_float_matrix_data::<105, 207>();
        for vec in matrix {
            for s in vec {
                assert_ne!(s, 0f32);
            }
        }
    }

}