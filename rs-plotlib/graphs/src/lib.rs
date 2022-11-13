use crate::cartesian::XYCoordinateSystem;
use std::ops::{Add, AddAssign, Div, DivAssign, RangeInclusive, RemAssign, Sub, SubAssign};

mod axis;
mod cartesian;

pub trait FunctionGraph {
    type CoordinateSystem;
    type PointType;
    type PointValueType: SubAssign<Self::PointValueType>
        + PartialEq<Self::PointValueType>
        + AddAssign<Self::PointValueType>
        + DivAssign<Self::PointValueType>
        + Div<u32>
        + Div<i32>
        + PartialOrd<Self::PointValueType>
        + RemAssign<Self::PointValueType>;
    fn new() -> Self::CoordinateSystem;
    fn add(&mut self, v: Self::PointType) -> bool;
    fn append(&mut self, v: &[Self::PointType]) -> bool;
    fn fix_axis(&mut self);
    fn size(&self) -> usize;
    fn dims2d(
        &self,
    ) -> (
        RangeInclusive<Self::PointValueType>,
        RangeInclusive<Self::PointValueType>,
    );
}

pub struct Graph {}

impl Graph {
    fn new_xy() -> XYCoordinateSystem {
        XYCoordinateSystem::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FunctionGraph, Graph};
    use rand::prelude::*;

    #[test]
    fn xy_cartesian_can_fix_axis() {
        let mut cart = Graph::new_xy();
        let mut rng = rand::thread_rng();
        let mut ok = true;
        for _ in 0..2000 {
            let x: f64 = rng.gen_range(0.1..100.2);
            let y: f64 = rng.gen_range(0.1..100.2);
            ok = ok && cart.add((x, y));
        }
        assert!(ok);
        cart.fix_axis();
        for _ in 0..2000 {
            let x: f64 = rng.gen_range(100.3..200.2);
            let y: f64 = rng.gen_range(100.3..200.2);
            ok = ok && cart.add((x, y));
        }
        assert!(!ok);
        assert_eq!(cart.size(), 2000);
    }
}
