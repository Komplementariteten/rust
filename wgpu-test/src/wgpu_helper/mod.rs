use wgpu::util::DeviceExt;

/// Example from
/// https://github.com/googlefonts/compute-shader-101/blob/main/compute-shader-hello/src/main.rs
/// https://thebookofshaders.com/
/// https://github.com/andrusha/rust-gpu-wgpu-compute-minimal/tree/main
async fn init(){
    env_logger::init();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        dx12_shader_compiler: Default::default(),
    });
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let features = adapter.features();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        label: None,
        features: features & wgpu::Features::TIMESTAMP_QUERY,
        limits: Default::default()
    }, None, ).await.unwrap();

    let query_set = if features.contains(wgpu::Features::TIMESTAMP_QUERY) {
        Some(device.create_query_set(&wgpu::QuerySetDescriptor {
            count: 2,
            ty: wgpu::QueryType::Timestamp,
            label: None,
        }))
    } else {
        None
    };

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only:false},
                has_dynamic_offset: false,
                min_binding_size: None
            },
            count: None,
        }],
    });
}