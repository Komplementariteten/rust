use std::num::NonZeroU32;
use graphs::FunctionGraph;
use image::{ImageBuffer, Rgba};
use imageproc::drawing;
use imageproc::drawing::draw_line_segment;

pub type PlotColorDepth = u8;
pub type DisplayImageBuffer = ImageBuffer<Rgba<PlotColorDepth>, Vec<PlotColorDepth>>;

pub fn as_2d<T: FunctionGraph>(graph: T, width: NonZeroU32, height: NonZeroU32) -> DisplayImageBuffer {
    let (x_range, y_range) = graph.dims2d();
    let draw_e

    let img = DisplayImageBuffer::new(width.clone(), height.clone());
    let delta_x = width / x_range.end();
    let deta_y = height / y_range.end();
    draw_line_segment(&img, );
    img
}

fn draw_x_axis() {

}
