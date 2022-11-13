use fast_image_resize as fr;
use fast_image_resize::Resizer;
use fltk::enums::ColorDepth;
use graph2img::{DisplayImageBuffer, PlotColorDepth};
use image::{ImageBuffer, Rgba};
use std::num::NonZeroU32;

pub struct DisplayBuffer {
    plot: DisplayImageBuffer,
    number_of_plots: u32,
    resizer: Resizer,
}

impl DisplayBuffer {
    pub fn new(width: NonZeroU32, height: NonZeroU32) -> DisplayBuffer {
        DisplayBuffer {
            plot: DisplayImageBuffer::new(width.get(), height.get()),
            number_of_plots: 0,
            resizer: fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3)),
        }
    }

    pub fn dims(&self) -> (u32, u32) {
        (self.plot.width(), self.plot.height())
    }

    pub fn pixels(&self) -> (Vec<PlotColorDepth>, ColorDepth) {
        (self.plot.clone().into_raw(), ColorDepth::Rgba8)
    }

    pub fn update(&mut self) {
        self.number_of_plots = 0;
    }

    pub fn append_image(&mut self, buff: &DisplayImageBuffer) -> &DisplayBuffer {
        self.number_of_plots += 1;
        let current_width = NonZeroU32::new(self.plot.width()).unwrap();
        let current_height = NonZeroU32::new(self.plot.height()).unwrap();

        let dest_height = NonZeroU32::new(self.plot.height() / self.number_of_plots).unwrap();
        let dest_width = NonZeroU32::new(self.plot.width()).unwrap();

        if self.number_of_plots == 1 {
            self.plot = buff.clone();
            return self;
        }

        let mut current_src = fr::Image::from_vec_u8(
            current_width,
            current_height,
            self.plot.to_vec(),
            fr::PixelType::U8x4,
        )
        .unwrap();
        let alpha_mul_div = fr::MulDiv::default();
        alpha_mul_div
            .multiply_alpha_inplace(&mut current_src.view_mut())
            .unwrap();

        let mut current_dest = fr::Image::new(dest_width, dest_height, current_src.pixel_type());
        let mut dst_view = current_dest.view_mut();
        self.resizer
            .resize(&current_src.view(), &mut dst_view)
            .unwrap();
        alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

        let new_height = NonZeroU32::new(buff.height()).unwrap();
        let new_width = NonZeroU32::new(buff.width()).unwrap();

        let mut new_src =
            fr::Image::from_vec_u8(new_width, new_height, buff.to_vec(), fr::PixelType::U8x4)
                .unwrap();

        let mut new_dest = fr::Image::new(dest_width, dest_height, current_src.pixel_type());
        let mut new_dest_veiw = new_dest.view_mut();
        alpha_mul_div
            .multiply_alpha_inplace(&mut new_src.view_mut())
            .unwrap();
        self.resizer
            .resize(&new_src.view(), &mut new_dest_veiw)
            .unwrap();

        alpha_mul_div.divide_alpha_inplace(&mut dst_view).unwrap();

        let mut new_pixel_vec = new_dest.buffer().to_vec();
        let mut pixel = current_dest.buffer().to_vec();
        pixel.append(&mut new_pixel_vec);

        self.plot = ImageBuffer::from_raw(self.plot.width(), self.plot.width(), pixel).unwrap();
        self
    }
}
