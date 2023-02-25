use crate::graphs::cartesian::XYCoordinateSystem;
use std::ops::{AddAssign, DivAssign, RangeInclusive, RemAssign, SubAssign};

pub mod axis;
mod cartesian;

pub trait FunctionGraph {
    type PointType;
    type PointValueType: SubAssign<Self::PointValueType>
        + PartialEq<Self::PointValueType>
        + AddAssign<Self::PointValueType>
        + DivAssign<Self::PointValueType>
        + PartialOrd<Self::PointValueType>
        + RemAssign<Self::PointValueType>;
    type AxisType;
    fn new() -> Self;
    fn add(&mut self, v: Self::PointType) -> bool;
    fn append(&mut self, v: &[Self::PointType]) -> bool;
    fn fix_axis(&mut self);
    fn size(&self) -> usize;
    fn axis(&self) -> Vec<Self::AxisType>;
    fn dims2d(
        &self,
    ) -> (
        RangeInclusive<Self::PointValueType>,
        RangeInclusive<Self::PointValueType>,
    );
}

pub struct Graph {}

impl Graph {
    pub fn new_xy() -> XYCoordinateSystem {
        XYCoordinateSystem::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::graphs::{FunctionGraph, Graph};
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
