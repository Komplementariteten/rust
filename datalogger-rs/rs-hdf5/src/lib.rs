#![feature(test)]
mod h5error;

use std::cell::Ref;
use crate::h5error::H5FileError::{MoreThanThreeDimNotSupported, NoDataset};
use hdf5::{Dataset, File, Group, H5Type};
use hdf5::types::VarLenUnicode;
use ndarray::{ArrayView, Dimension as NdDimension};
use std::error;
use hdf5::dataset::Layout;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;
const DEFLATE_RATE: u8 = 3;

pub struct H5File {
    _f: File,
}

impl H5File {
    fn create_ds_1d<T, D>(group: Group, name: &str, d: D) -> Result<Dataset>
    where
        T: H5Type,
        D: NdDimension,
    {
        let dim: usize = d[0];
        match group
            .new_dataset::<T>()
            .chunk((1, dim))
            .shape((1.., dim))
            .deflate(DEFLATE_RATE)
            .create(name)
        {
            Ok(ds) => Ok(ds),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn create_ds_2d<T, D>(group: Group, name: &str, d: D) -> Result<Dataset>
    where
        T: H5Type,
        D: NdDimension,
    {
        let (ny, nx) = (d[0], d[1]);
        match group
            .new_dataset::<T>()
            .chunk((1, ny, nx))
            .shape((1.., ny, nx))
            .deflate(DEFLATE_RATE)
            .create(name)
        {
            Ok(ds) => Ok(ds),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn add_to_ds_1d<'d, A, T, D>(ds: Dataset, data: A) -> Result<usize>
    where
        A: Into<ArrayView<'d, T, D>>,
        T: H5Type,
        D: NdDimension,
    {
        let av = data.into();
        let dims = av.raw_dim();
        let mut current_chunk: usize = 0;
        if let Some(mut nchunks) = ds.num_chunks() {
            ds.resize((nchunks + 1, dims[0]))?;
            current_chunk = nchunks;
        }
        match ds.write_slice(av, (current_chunk, ..)) {
            Ok(_) => Ok(current_chunk),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn add_to_ds_2d<'d, A, T, D>(ds: Dataset, data: A) -> Result<usize>
    where
        A: Into<ArrayView<'d, T, D>>,
        T: H5Type,
        D: NdDimension,
    {
        let av = data.into();
        let dims = av.raw_dim();
        let mut current_chunk: usize = 0;
        if let Some(nchunks) = ds.num_chunks() {
            ds.resize((nchunks + 1, dims[0], dims[1]))?;
            current_chunk = nchunks;
        }
        match ds.write_slice(av, (current_chunk, .., ..)) {
            Ok(_) => Ok(current_chunk),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn add_from_typedesc() -> Result<()> {
        Ok(())
    }

    pub fn add_from_data<'d, A, T, D>(&mut self, group: &str, ds_name: &str, data: A) -> Result<()>
    where
        A: Into<ArrayView<'d, T, D>>,
        T: H5Type,
        D: NdDimension,
    {
        let g = match self.add_or_get_group(group) {
            Ok(g) => g,
            Err(e) => return Err(e),
        };

        let av = data.into();
        let dims = av.raw_dim();
        let ndims = dims.ndim();
        let ds = match g.dataset(ds_name) {
            Ok(ds) => ds,
            Err(_) => match ndims {
                1 => Self::create_ds_1d::<T, D>(g, ds_name, dims).expect("Failed to create 1d DS"),
                2 => Self::create_ds_2d::<T, D>(g, ds_name, dims).expect("Failed to create 2d DS"),
                _ => panic!("{}", MoreThanThreeDimNotSupported),
            },
        };

        if ndims == 1 {
            return match Self::add_to_ds_1d(ds, av) {
                Err(e) => Err(e),
                Ok(_) => Ok(()),
            };
        }

        if ndims == 2 {
            return match Self::add_to_ds_2d(ds, av) {
                Err(e) => Err(e),
                Ok(_) => Ok(()),
            };
        }

        Err(Box::new(MoreThanThreeDimNotSupported))
    }

    pub fn add_group(&mut self, group_name: &str) -> Result<()> {
        match self.add_or_get_group(group_name) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn add_or_get_group(&mut self, group_name: &str) -> Result<Group> {
        if let Ok(g) = self._f.group(group_name) {
            return Ok(g);
        }

        match self._f.create_group(group_name) {
            Ok(g) => Ok(g),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn new(path: &str) -> Result<H5File> {
        let file = match File::create(path) {
            Ok(f) => f,
            Err(e) => return Err(Box::new(e)),
        };

        let h5f = H5File { _f: file };
        Ok(h5f)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::H5File;
    use ndarray::{arr1, arr2, Array1};
    use std::fs;
    use std::str::FromStr;
    use test::Bencher;
    use hdf5::types::VarLenUnicode;

    #[test]
    fn test_string() {
        let ls = VarLenUnicode::from_str("as");
        let testfile_2d = "bench.hdf";
        if fs::exists(testfile_2d).expect("Error checking file existing") {
            fs::remove_file(testfile_2d).expect("Failed to remove test hdf5 file");
        }

        let mut f = match H5File::new(testfile_2d) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        // f.add_string("g", "ds", &arr1(&["", ""]));
    }

    #[bench]
    fn wirte_1d_bench(b: &mut Bencher){
        let testfile_2d = "bench.hdf";
        if fs::exists(testfile_2d).expect("Error checking file existing") {
            fs::remove_file(testfile_2d).expect("Failed to remove test hdf5 file");
        }

        let mut f = match H5File::new(testfile_2d) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let data = Array1::from_shape_fn((100), |(i)|(10.22 - i as f32));

        b.iter(|| {
            f.add_from_data("bench", "int_data", &data)
                .expect("Failed to add Data");
        })
    }
    #[test]
    fn base_test() {
        let testfile_path = "test.h5";

        if fs::exists(testfile_path).expect("Error checking file existing") {
            fs::remove_file(testfile_path).expect("Failed to remove test hdf5 file");
        }

        let mut f = match H5File::new(testfile_path) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        f.add_from_data("test_group", "numbers", &arr1(&[1, 2, 3, 4]))
            .expect("Failed to add Data")
    }

    #[test]
    fn test_2d() {
        let testfile_2d = "2dtest.hdf";
        if fs::exists(testfile_2d).expect("Error checking file existing") {
            fs::remove_file(testfile_2d).expect("Failed to remove test hdf5 file");
        }

        let mut f = match H5File::new(testfile_2d) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        f.add_from_data("test_group", "numbers", &arr2(&[[1, 2, 3, 4], [5,6,7,8]]))
            .expect("Failed to add Data");
        f.add_from_data("test_group", "numbers", &arr2(&[[9, 10, 11, 12], [13,14,15,16]]))
            .expect("Failed to add Data");

    }

    #[test]
    fn multi_add() {
        let testfile_path = "test2.h5";
        let group_name = "test_grp";
        let ds1_name = "number";
        let ds2_name = "reals";
        let ds3_name = "names";
        if fs::exists(testfile_path).expect("Error checking file existing") {
            fs::remove_file(testfile_path).expect("Failed to remove test hdf5 file");
        }

        let mut f = match H5File::new(testfile_path) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        f.add_from_data(group_name, ds1_name, &arr1(&[5, 6, 7, 8]))
            .expect("Failed to add Data");
        f.add_from_data(group_name, ds1_name, &arr1(&[10, 11, 12, 13]))
            .expect("Second add failed");
        f.add_from_data(group_name, ds2_name, &arr1(&[10.1, 11.2, 12.3]))
            .expect("Second add failed");
        f.add_from_data(group_name, ds3_name, &arr2(&[[ VarLenUnicode::from_str("a11").expect(""), VarLenUnicode::from_str("a12").expect("")], [VarLenUnicode::from_str("a21").expect(""), VarLenUnicode::from_str("a22").expect("")]])).expect("Third add failed");
    }
}
