use crate::graphs::axis::{Axis, ContinualAxis};
use crate::graphs::FunctionGraph;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_line_segment_mut;
use std::num::NonZeroU32;

pub type PlotColorDepth = u8;
pub type DisplayImageBuffer = ImageBuffer<Rgba<PlotColorDepth>, Vec<PlotColorDepth>>;

pub struct PlotStyle {
    pub graph_area: (f32, f32),
    pub major_ticks: f32,
    pub minot_ticks: f32,
    pub axis_color: Rgba<PlotColorDepth>,
}

impl Default for PlotStyle {
    fn default() -> Self {
        PlotStyle {
            graph_area: (0.9, 0.9),
            major_ticks: 10.0,
            minot_ticks: 5.0,
            axis_color: Rgba::from([0, 0, 0, 255]),
        }
    }
}

pub fn as_2d<T: FunctionGraph<AxisType = ContinualAxis<f64>>>(
    graph: T,
    width: NonZeroU32,
    height: NonZeroU32,
    style: PlotStyle,
) -> DisplayImageBuffer {
    let mut img: DisplayImageBuffer = DisplayImageBuffer::new(width.get(), height.get());
    img.fill(255);
    let mut axes = graph.axis();
    draw_x_axis(&mut img, style, axes.pop().unwrap());
    img
}

fn draw_x_axis(img: &mut DisplayImageBuffer, style: PlotStyle, axis: ContinualAxis<f64>) {
    let axis_length = axis.length();
    let image_width = img.width() as f32;
    let image_height = img.height();
    let (start, end) = axis.range.into_inner();
    let even_height = image_height % 2;
    draw_line_segment_mut(img, (3.0, 0.0), (3.0, image_width), style.axis_color);
}
