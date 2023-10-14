use std::error;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;
use bytemuck::{NoUninit, Pod, Zeroable};
use num::traits::{NumRef};
use num::zero;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[repr(C)]
#[derive(Debug, PartialEq)]
struct MatrixArray<T: NumRef + Copy + Debug + Sized, const N:usize> {
    pub rows: usize,
    pub cols: usize,
    data: [T; N]
}

#[derive(Debug, PartialEq)]
pub struct ComputationMatrix<T: NumRef + Copy + Debug + Sized> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>,
}

#[macro_export]
macro_rules! matrix {
    ($rows:tt, $cols:tt, $default:tt) => {
        {const SIZE: usize = $rows * $cols;
        MatrixArray {
            rows: $rows,
            cols: $cols,
            data: [$default; SIZE]
        }}
    };
}

impl<T: NumRef + Copy + Debug + Sized + Pod> ComputationMatrix<T> {

    pub fn new<const ROWS:usize, const COLS:usize>() -> ComputationMatrix<T> {
        let data = Vec::<T>::with_capacity(ROWS * COLS);
        let t = MatrixArray{
            rows: 1,
            cols: 2,
            data: [0; 2]
        };
        return ComputationMatrix{
            rows: ROWS,
            cols: COLS,
            data
        }
    }

    pub fn to_bytes(&self) -> Result<&[u8]> {
        let v_slice = self.data.as_slice();
        let pl = bytemuck::bytes_of(v_slice);
        return match pl.len() {
            0 => Err(Box::try_from(format!("Can't convert type T:{} to bytes", std::any::type_name::<T>())).unwrap()),
            1.. => Ok(pl)
        };
        return Ok(pl);
    }

    pub fn as_slice(&self) -> &[T] {
        return self.data.as_slice()
    }
}

impl <T:  NumRef + Copy + Debug + Sized + Pod, I> Index<I> for ComputationMatrix<T> where I: SliceIndex<[T]> {
    type Output = <I as SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        return &self.data[index];
    }
}

impl<T:  NumRef + Copy + Debug + Sized + Pod, I> IndexMut<I> for ComputationMatrix<T>  where I: SliceIndex<[T]> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        return &mut self.data[index];
    }
}




/* pub trait ComputationMatrix<T: NumRef + Copy + Debug, const ROWS: usize, const COLS: usize> where Self: Sized {
    type Item;
    const NUM_CELLS: usize;

    const NUM_ROWS: usize;

    const NUM_COLS: usize;

    fn new() -> DataContainer<T>;
}

impl <T: NumRef + Copy + Debug, const ROWS: usize, const COLS: usize> ComputationMatrix<T, ROWS, COLS> for DataContainer<T> {
    type Item = T;

    const NUM_CELLS: usize = ROWS * COLS;

    const NUM_ROWS: usize = ROWS;

    const NUM_COLS: usize = COLS;

    fn new() -> DataContainer<T> {
        let d = [zero(); Self::NUM_CELLS];
        return DataContainer{
            data: Vec::from(d)
        };
    }

} */


#[cfg(test)]
mod tests {
    use crate::matrix::{ComputationMatrix};

    #[test]
    fn vector_as_array() {
        let m = matrix!(1, 2, 0.1);
    }
    #[test]
    fn matrix_can_be_constructed() {
        let m = ComputationMatrix::<f32>::new::<4,5>();
        assert_eq!(m.cols, 5);
        assert_eq!(m.rows, 4);
    }
}