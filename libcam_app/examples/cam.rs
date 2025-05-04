use opencv::core::{CV_32F, Size, mean, mean_def};
use opencv::imgproc::{COLOR_RGB2GRAY, INTER_LINEAR};
use opencv::prelude::*;
use opencv::{Result, highgui, imgproc, videoio};

fn main() -> Result<()> {
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
    if !cam.is_opened()? {
        panic!("Unable to open default camera!");
    }
    /* let input = GMat::default().expect("can't create gmat");
    let vga = gapi::resize(&input, Size::default(), 0.5, 0.5, INTER_LINEAR)?;
    let gray = gapi::bgr2_gray(&vga)?;
    let output = gapi::sobel_def(&gray, CV_32F, 1, 0).expect("failed to get grad.");
    let mut ac = gapi::GComputation::new(input, output);
    */
    let mut gray = Mat::default();
    let mut grad = Mat::default();
    let mut mean: f64 = 0.0;
    let mut default_mean: f64 = 0.0;
    loop {
        let mut frame = Mat::default();
        assert!(cam.read(&mut frame)?);
        if frame.size()?.width > 0 {
            highgui::imshow(window, &frame)?;
            imgproc::cvt_color_def(&frame, &mut gray, COLOR_RGB2GRAY).expect("Failed to convert color");
            imgproc::sobel_def(&gray, &mut grad, CV_32F, 1, 0).expect("Failed to sobel");
            let mean_s = mean_def(&grad).expect("Failed to mean");
            if default_mean != 0.0 {
                default_mean = (default_mean + mean_s[0]) / 2.0;
            } else {
                default_mean = mean_s[0];
            }
            if(default_mean > 5.5 || default_mean < 0.53) {
                println!("mean: {}", default_mean);
            }
            
       }

        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }

    Ok(())
}