#[cfg(test)]
mod tests {
    use graph2img::DisplayImageBuffer;
    use rs_plotlib::graph2img::DisplayImageBuffer;
    use rs_plotlib::image::DisplayBuffer;
    use std::num::NonZeroU32;

    #[test]
    fn images_append_below() {
        let pixel_first: Vec<u8> = vec![
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ];
        let start: DisplayImageBuffer = DisplayImageBuffer::from_raw(2, 2, pixel_first).unwrap();
        let mut plot = DisplayBuffer::new(NonZeroU32::new(2).unwrap(), NonZeroU32::new(2).unwrap());
        plot.append_image(&start);

        let (pixel_start, _) = plot.pixels();
        assert_eq!(*pixel_start.first().unwrap(), 255);
        assert_eq!(*pixel_start.last().unwrap(), 255);

        let pixel_second: Vec<u8> = vec![
            128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
            128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
        ];
        let update: DisplayImageBuffer = DisplayImageBuffer::from_raw(2, 2, pixel_second).unwrap();
        plot.append_image(&update);

        let (pixel_update, _) = plot.pixels();
        assert_eq!(*pixel_update.first().unwrap(), 255);
        assert_eq!(*pixel_update.last().unwrap(), 128);
    }
}
