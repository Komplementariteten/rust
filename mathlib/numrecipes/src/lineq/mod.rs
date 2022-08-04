use std::borrow::Borrow;
use nalgebra::{DMatrix, DVector, Scalar, SVector};

/*
This is meant for learning use nalgebra instead.
They provide good solutions for the stuff i do.
 */
pub fn gaussjordan<T:Scalar + num_traits::identities::Zero + Copy>(a:DMatrix<T>, b:DMatrix<T>) -> DMatrix<T> {
    let mut res = DMatrix::<T>::zeros(a.ncols(), a.nrows());
    res[(0, 0)] = a[(0, 0)];
    res[(1, 0)] = b[(0, 0)];
    return res;
    /* let (mut i, mut icol, mut irow, mut j, mut k, mut l, mut ll, n, m) : (i32, usize, usize, i32, i32, i32, i32, usize, usize) = (0, 0, 0, 0, 0, 0, 0, a.nrows(), b.ncols());
    let (mut big, mut dum, mut pivnv) : (f32, f32, f32);
    let (mut indexc, mut indexr, mut ipiv) : (DVector<i32>, DVector<i32>, DVector<i32>) = (DVector::<i32>::zeros(n), DVector::<i32>::zeros(n),DVector::<i32>::zeros(n));
    for i in 0..n {
        big = 0.0;
        for j in 0..n {
            if(ipiv[j] != 1) {
                for k in 0..m {
                    if(ipiv[k] == 0) {
                        if(a[(j, k)].abs() >= big) {
                            big = a[(j, k)].abs();
                            irow = j;
                            icol = k;
                        }
                    }
                }
            }
        }
        ipiv[icol] += 1;
    } */
}