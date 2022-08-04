#[cfg(test)]
mod tests {
    use nalgebra::{DMatrix, dmatrix};
    use numrecipes::lineq;
    #[test]
    fn test_gaussjordan() {
        let m1 = dmatrix![1, 2; 2, 2];
        let m2 = dmatrix![2, 1, 1, 3];
        let res = lineq::gaussjordan(m1, m2);
        assert_eq!(res[(0, 0)], 1);
        assert_eq!(res[(1, 0)], 2);
    }
}