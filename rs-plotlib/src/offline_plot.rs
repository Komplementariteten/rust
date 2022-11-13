use std::error::Error;
use fltk::{app, frame, window, draw, prelude::*};
use crate::image::DisplayBuffer;

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
}