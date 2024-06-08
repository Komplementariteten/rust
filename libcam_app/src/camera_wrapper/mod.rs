use std::time::Duration;
use image::*;
use libcamera::{
    camera_manager::CameraManager,
    framebuffer::AsFrameBuffer,
    framebuffer_allocator::{FrameBuffer, FrameBufferAllocator},
    framebuffer_map::MemoryMappedFrameBuffer,
    pixel_format::PixelFormat,
    stream::StreamRole,
};

pub const PIXEL_FORMAT_MJPEG: PixelFormat = PixelFormat::new(u32::from_le_bytes([b'M', b'J', b'P', b'G']), 0);

pub(crate) fn capture() -> ImageResult<DynamicImage> {

    let mgr = CameraManager::new().unwrap();

    let cameras = mgr.cameras();

    let cam = cameras.get(0).expect("No cameras found");

    let mut cam = cam.acquire().expect("Unable to acquire camera_wrapper");

    // This will generate default configuration for each specified role
    let mut cfgs = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();

    // Use MJPEG format so we can write resulting frame directly into jpeg file
    cfgs.get_mut(0).unwrap().set_pixel_format(PIXEL_FORMAT_MJPEG);

    // Ensure that pixel format was unchanged
    assert_eq!(
        cfgs.get(0).unwrap().get_pixel_format(),
        PIXEL_FORMAT_MJPEG,
        "MJPEG is not supported by the camera"
    );

    cam.configure(&mut cfgs).expect("Unable to configure camera_wrapper");

    let mut alloc = FrameBufferAllocator::new(&cam);

    // Allocate frame buffers for the stream
    let cfg = cfgs.get(0).unwrap();
    let stream = cfg.stream().unwrap();
    let buffers = alloc.alloc(&stream).unwrap();

    // Convert FrameBuffer to MemoryMappedFrameBuffer, which allows reading &[u8]
    let buffers = buffers
        .into_iter()
        .map(|buf| MemoryMappedFrameBuffer::new(buf).unwrap())
        .collect::<Vec<_>>();

    // Create capture requests and attach buffers
    let mut reqs = buffers
        .into_iter()
        .map(|buf| {
            let mut req = cam.create_request(None).unwrap();
            req.add_buffer(&stream, buf).unwrap();
            req
        })
        .collect::<Vec<_>>();

    // Completed capture requests are returned as a callback
    let (tx, rx) = std::sync::mpsc::channel();
    cam.on_request_completed(move |req| {
        tx.send(req).unwrap();
    });

    cam.start(None).unwrap();

    // Multiple requests can be queued at a time, but for this example we just want a single frame.
    cam.queue_request(reqs.pop().unwrap()).unwrap();

    let req = rx.recv_timeout(Duration::from_secs(2)).expect("Camera request failed");

    // Get framebuffer for our stream
    let framebuffer: &MemoryMappedFrameBuffer<FrameBuffer> = req.buffer(&stream).unwrap();

    // MJPEG format has only one data plane containing encoded jpeg data with all the headers
    let planes = framebuffer.data();
    let jpeg_data = planes.get(0).unwrap();
    // Actual JPEG-encoded data will be smalled than framebuffer size, its length can be obtained from metadata.
    let jpeg_len = framebuffer.metadata().unwrap().planes().get(0).unwrap().bytes_used as usize;

    return  image::load_from_memory_with_format(&jpeg_data[..jpeg_len], image::ImageFormat::Jpeg);

}