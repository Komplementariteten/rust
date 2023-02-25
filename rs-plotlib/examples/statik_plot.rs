use graph2img::DisplayImageBuffer;
use image::io::Reader as ImageReader;
use rs_plotlib::image::DisplayBuffer;
use std::error::Error;
use std::num::NonZeroU32;
use rs_plotlib::graph2img::DisplayImageBuffer;

fn main() -> Result<(), Box<dyn Error>> {
    let org_img = match ImageReader::open("crab.jpg") {
        Ok(img) => img.decode()?,
        Err(e) => panic!("Failed to read {:?}", e),
    };
    let img: DisplayImageBuffer = org_img.to_rgba8();
    let mut buffer = DisplayBuffer::new(
        NonZeroU32::new(img.width()).unwrap(),
        NonZeroU32::new(img.height()).unwrap(),
    );
    let mut graph = DisplayBuffer::plot_xy()
    let pixel_first: Vec<u8> = vec![255, 255 ,255, 255, 255, 255 ,255, 255, 255, 255 ,255, 255, 255, 255 ,255, 255, 255, 255 ,255, 255, 255, 255 ,255, 255, 255, 255 ,255, 255, 255, 255 ,255, 255];
    let start: DisplayImageBuffer = DisplayImageBuffer::from_raw(2, 2, pixel_first).unwrap();
    let mut plot = DisplayBuffer::new(NonZeroU32::new(2).unwrap(), NonZeroU32::new(2).unwrap());
    buffer.append_image(&start);
    buffer.show()
}
