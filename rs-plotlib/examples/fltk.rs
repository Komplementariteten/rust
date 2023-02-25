use image::io::Reader as ImageReader;
use rs_plotlib::image::DisplayBuffer;
use std::error::Error;
use std::num::NonZeroU32;

fn main() -> Result<(), Box<dyn Error>> {
    let org_img = match ImageReader::open("crab.jpg") {
        Ok(img) => img.decode()?,
        Err(e) => panic!("Failed to read {:?}", e),
    };
    // let img: DisplayImageBuffer = org_img.to_rgba8();
    let mut buffer = DisplayBuffer::new(NonZeroU32::new(10).unwrap(), NonZeroU32::new(10).unwrap());
    // buffer.append_image(&img);
    buffer.show()
}
