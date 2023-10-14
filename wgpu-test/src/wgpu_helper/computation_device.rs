use std::collections::HashMap;
use wgpu::{BindGroupLayout, Device, Features, PipelineLayout, QuerySet, Queue, ShaderModule};
use wgpu::util::DeviceExt;

use std::time::Instant;
use std::error;
use std::fmt;

const MATRIX_SHADER_NAME: &str = "Matrix_multiply";

type ComputationResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct ComputationDevice {
    device: Device,
    queue: Queue,
    pipeline_layout: PipelineLayout,
    query_set: QuerySet,
    features: Features,
    bind_group_layout: BindGroupLayout,
    shader_modules: HashMap<String, ShaderModule>
}

impl ComputationDevice {

    fn load_shader(&mut self) {
        let cs_module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some(MATRIX_SHADER_NAME),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader/matrix_multiplication.wgsl").into()),
        });
        self.shader_modules.insert(MATRIX_SHADER_NAME.to_string(), cs_module);
    }

    pub async fn test_run_add_42(self) {
        let start_instant = Instant::now();
        println!("shader compilation {:?}", start_instant.elapsed());
        let cs_module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("Matrix_multiply"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader/add_42.wgsl").into()),
        });
        let input_f = &[1.0f32, 2.0f32, 3.0f32, 10.0f32];
        let input: &[u8] = bytemuck::bytes_of(input_f);
        let input_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: input,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });
        let output_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: input.len() as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // This works if the buffer is initialized, otherwise reads all 0, for some reason.
        let query_buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &[0; 16],
            usage: /* wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST | */  wgpu::BufferUsages::QUERY_RESOLVE,
        });


        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{
            label: None,
            layout: Some(&self.pipeline_layout),
            module: &cs_module,
            entry_point: "main"
        });
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: input_buf.as_entire_binding(),
            }],
        });
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.write_timestamp(&self.query_set, 0);
        {
            let mut cpass = encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch_workgroups(input_f.len() as u32, 1, 1);
        }
        encoder.write_timestamp(&self.query_set, 1);
        encoder.copy_buffer_to_buffer(&input_buf, 0, &output_buf, 0, input.len() as u64);
        encoder.resolve_query_set(&self.query_set, 0..2, &query_buf, 0);
        self.queue.submit(Some(encoder.finish()));

        let buf_slice = output_buf.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buf_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
        let query_slice = query_buf.slice(..);

        // Assume that both buffers become available at the same time. A more careful
        // approach would be to wait for both notifications to be sent.
        // let _query_future = query_slice.map_async(wgpu::MapMode::Read, |_| ());
        println!("pre-poll {:?}", std::time::Instant::now());
        self.device.poll(wgpu::Maintain::Wait);
        println!("post-poll {:?}", std::time::Instant::now());
        if let Some(Ok(())) = receiver.receive().await {
            let data_raw = &*buf_slice.get_mapped_range();
            let data: &[f32] = bytemuck::cast_slice(data_raw);
            println!("data: {:?}", &*data);
        }
        /* if self.features.contains(wgpu::Features::TIMESTAMP_QUERY) {
            let ts_period = self.queue.get_timestamp_period();
            let ts_data_raw = &*query_slice.get_mapped_range();
            let ts_data: &[u64] = bytemuck::cast_slice(ts_data_raw);
            println!(
                "compute shader elapsed: {:?}ms",
                (ts_data[1] - ts_data[0]) as f64 * ts_period as f64 * 1e-6
            );
        } */
    }

    pub async fn init() -> Option<Self> {
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
            device.create_query_set(&wgpu::QuerySetDescriptor {
                count: 2,
                ty: wgpu::QueryType::Timestamp,
                label: None,
            })
        } else {
            return None
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

        let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        return Some(ComputationDevice {
            device,
            queue,
            pipeline_layout: compute_pipeline_layout,
            query_set,
            features,
            bind_group_layout,
            shader_modules: HashMap::new()
        })
    }
}