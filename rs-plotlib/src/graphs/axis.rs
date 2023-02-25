use std::ops::{AddAssign, DivAssign, MulAssign, RangeInclusive, SubAssign};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AxisDirection {
    Horizontal,
    Vertical,
    #[allow(dead_code)]
    Diagonal,
}

pub trait Axis: Clone {
    type AxisType;
    type ValueType: AddAssign<Self::ValueType>
        + PartialEq<Self::ValueType>
        + SubAssign<Self::ValueType>
        + MulAssign<Self::ValueType>
        + DivAssign<Self::ValueType>
        + PartialOrd<Self::ValueType>;
    fn value_range(&self) -> RangeInclusive<Self::ValueType>;
    fn length(&self) -> Self::ValueType;
    fn direction(&self) -> AxisDirection;
    fn start(&self) -> Self::ValueType;
    fn end(&self) -> Self::ValueType;
    fn new(direction: AxisDirection) -> Self::AxisType;
    fn name(&self) -> &str;
}

#[derive(Clone)]
pub struct ContinualAxis<T> {
    direction: AxisDirection,
    pub range: RangeInclusive<T>,
    pub title: String,
    pub major_ticks: Vec<T>,
    pub minor_ticks: Vec<T>,
}

impl Axis for ContinualAxis<f64> {
    type AxisType = ContinualAxis<f64>;
    type ValueType = f64;
    fn value_range(&self) -> RangeInclusive<Self::ValueType> {
        self.range.clone()
    }

    fn length(&self) -> Self::ValueType {
        self.range.end() - self.range.start()
    }

    fn direction(&self) -> AxisDirection {
        self.direction.clone()
    }

    fn start(&self) -> Self::ValueType {
        self.range.start().clone()
    }

    fn end(&self) -> Self::ValueType {
        self.range.end().clone()
    }

    fn new(direction: AxisDirection) -> Self {
        ContinualAxis {
            direction,
            range: 0.0..=0.0,
            title: String::new(),
            major_ticks: vec![],
            minor_ticks: vec![],
        }
    }

    fn name(&self) -> &str {
        self.title.as_str()
    }
}
