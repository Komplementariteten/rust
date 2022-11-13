use std::ops::RangeInclusive;

pub struct ContinualAxis<T> {
    pub range: RangeInclusive<T>,
    pub title: String,
    pub major_ticks: Vec<T>,
    pub minor_ticks: Vec<T>,
}

impl ContinualAxis<f64> {
    pub fn new() -> ContinualAxis<f64> {
        ContinualAxis {
            range: RangeInclusive::new(0 as f64, 0 as f64),
            title: String::from(""),
            major_ticks: vec![],
            minor_ticks: vec![],
        }
    }
}
