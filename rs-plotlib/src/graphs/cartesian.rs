use crate::graphs::axis::{Axis, AxisDirection, ContinualAxis};
use crate::graphs::FunctionGraph;
use std::ops::RangeInclusive;

/* Start XYCoordinateSystem */
pub struct XYCoordinateSystem {
    x_axis: ContinualAxis<f64>,
    y_axis: ContinualAxis<f64>,
    data: Vec<(f64, f64)>,
    fixed_axis: bool,
}

impl XYCoordinateSystem {
    fn new_xy() -> XYCoordinateSystem {
        XYCoordinateSystem {
            y_axis: ContinualAxis::new(AxisDirection::Vertical),
            x_axis: ContinualAxis::new(AxisDirection::Horizontal),
            data: vec![],
            fixed_axis: false,
        }
    }
}

impl FunctionGraph for XYCoordinateSystem {
    type PointType = (f64, f64);
    type PointValueType = f64;
    type AxisType = ContinualAxis<Self::PointValueType>;

    fn new() -> Self {
        XYCoordinateSystem::new_xy()
    }

    #[inline(always)]
    fn add(&mut self, v: Self::PointType) -> bool {
        if !self.fixed_axis {
            let (start, end) = match v.0 {
                x if x > *self.x_axis.range.end() => (*self.x_axis.range.start(), x),
                x if x < *self.x_axis.range.start() => (x, *self.x_axis.range.end()),
                _ => (*self.x_axis.range.start(), *self.x_axis.range.end()),
            };
            self.x_axis.range = RangeInclusive::new(start, end);
            let (start, end) = match v.1 {
                y if y > *self.y_axis.range.end() => (*self.y_axis.range.start(), y),
                y if y < *self.y_axis.range.start() => (y, *self.y_axis.range.end()),
                _ => (*self.y_axis.range.start(), *self.y_axis.range.end()),
            };
            self.y_axis.range = RangeInclusive::new(start, end);
        }
        if self.x_axis.range.contains(&v.0) && self.y_axis.range.contains(&v.1) {
            self.data.push(v);
            true
        } else {
            false
        }
    }

    fn append(&mut self, v: &[Self::PointType]) -> bool {
        let mut any = false;
        for item in v {
            any = any || self.add(*item)
        }
        any
    }

    fn fix_axis(&mut self) {
        self.fixed_axis = true;
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn axis(&self) -> Vec<Self::AxisType> {
        vec![self.x_axis.clone(), self.y_axis.clone()]
    }

    fn dims2d(
        &self,
    ) -> (
        RangeInclusive<Self::PointValueType>,
        RangeInclusive<Self::PointValueType>,
    ) {
        (self.x_axis.value_range(), self.y_axis.value_range())
    }
}
/* End XYCoordinateSystem */
