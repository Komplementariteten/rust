use wgpu_test::data_helper;

fn main() {
    let a = data_helper::get_float_matrix_data::<5, 6>();
    let b = data_helper::get_float_matrix_data::<6, 5>();
    for _ in 0..10_000_000 {
        let _r = data_helper::multiply(a, b);
    }
}