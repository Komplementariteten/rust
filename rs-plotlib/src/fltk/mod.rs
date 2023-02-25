use crate::graph2img::{as_2d, PlotStyle};
use crate::graphs::{FunctionGraph, Graph};
use crate::image::DisplayBuffer;
use fltk::app::sleep;
use fltk::{app, draw, frame, prelude::*, window};
use std::error::Error;
use std::num::NonZeroU32;

impl DisplayBuffer {
    pub fn show(&self) -> Result<(), Box<dyn Error>> {
        let app = app::App::default();
        let (width, height) = self.dims();
        let mut win = window::Window::default().with_size(width as i32, height as i32);
        let mut frame = frame::Frame::default().size_of(&win);
        let (pixel, colordepth) = self.pixels();
        win.end();
        win.show();

        frame.draw(move |_| {
            draw::draw_image(&pixel, 0, 0, width as i32, height as i32, colordepth).unwrap();
        });

        app.run()?;
        Ok(())
    }

    pub fn plot_xy(data: &[(f64, f64)], style: PlotStyle) -> DisplayBuffer {
        let mut graph = Graph::new_xy();
        data.iter().for_each(|i| {
            let _ = graph.add(i.clone());
            ()
        });
        let width = NonZeroU32::new(400).unwrap();
        let height = NonZeroU32::new(400).unwrap();
        let img = as_2d(graph, width.clone(), height.clone(), style);
        let mut buff = DisplayBuffer::new(width, height);
        buff.append_image(&img);
        buff
    }
}
