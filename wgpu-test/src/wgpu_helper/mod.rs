pub mod computation_device;

/// Example from
/// https://github.com/googlefonts/compute-shader-101/blob/main/compute-shader-hello/src/main.rs
/// https://thebookofshaders.com/
/// https://github.com/andrusha/rust-gpu-wgpu-compute-minimal/tree/main
#[cfg(test)]
mod test {
    use crate::wgpu_helper::computation_device::ComputationDevice;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        }
    }

    #[test]
    fn computation_device_does_not_panic() {
        let device = aw!(ComputationDevice::init()).unwrap();
        aw!(device.test_run_add_42());
    }
}