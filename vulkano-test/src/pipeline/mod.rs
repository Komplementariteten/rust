use crate::pipeline;
use crate::wrapper::{get_device, get_render_pass, init_swapchain};
use std::sync::Arc;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::Device;
use vulkano::image::SwapchainImage;
use vulkano::instance::Instance;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::input_assembly::{InputAssemblyState, PrimitiveTopology};
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::graphics::GraphicsPipelineCreationError;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Subpass;
use vulkano::shader::{EntryPoint, ShaderModule};
use vulkano::swapchain::{Surface, Swapchain};

pub struct PipelineInfo {
    pub(crate) device: Arc<Device>,
    pub(crate) swapchain: Arc<Swapchain>,
    pub(crate) pipeline: Arc<GraphicsPipeline>,
    pub(crate) images: Vec<Arc<SwapchainImage>>,
    pub(crate) memory_allocator: StandardMemoryAllocator,
    pub(crate) descriptor_allocator: StandardDescriptorSetAllocator,
    pub(crate) command_buffer_allocator: StandardCommandBufferAllocator,
}

pub struct PipelineBuilder {
    buffer_definition: BuffersDefinition,
    builder_info: PiplineBuilderInfo,
}

pub struct PiplineBuilderInfo {
    pub(crate) vulcan: Arc<Instance>,
    pub(crate) topology: PrimitiveTopology,
    pub(crate) vertex_shader: Arc<ShaderModule>,
    pub(crate) fragment_shader: Arc<ShaderModule>,
}

impl PipelineBuilder {
    pub fn new<T>(info: PiplineBuilderInfo) -> Self {
        let buffer_definition = BuffersDefinition::new().vertex::<T>();
        PipelineBuilder {
            buffer_definition: bufferDefinition,
            builder_info: info,
        }
    }
    pub fn build(
        self,
        surface: &Arc<Surface>,
    ) -> Result<PipelineInfo, GraphicsPipelineCreationError> {
        // let vulkan_instance = init_vulkan().expect("Failed to initialize Vulkan");

        let (device, queue) = get_device(&self.builder_info.vulcan, &surface)
            .expect("No device found for given specifications");

        let (swapchain, images) =
            init_swapchain(&device, surface).expect("failed to initialize spawpchain");
        let render_pass =
            get_render_pass(&device, &swapchain).expect("failed to initialize renderpass");

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();
        // Before we draw we have to create what is called a pipeline. This is similar to an OpenGL
        // program, but much more specific.
        let pipeline = match GraphicsPipeline::start()
            .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
            // We have to indicate which subpass of which render pass this pipeline is going to be used
            // in. The pipeline will only be usable from this particular subpass.
            .render_pass(subpass)
            // We need to indicate the layout of the vertices.
            .vertex_input_state(self.buffer_definition)
            // The content of the vertex buffer describes a list of triangles.
            .input_assembly_state(InputAssemblyState::new().topology(self.topology))
            // A Vulkan shader can in theory contain multiple entry points, so we have to specify
            // which one.
            .vertex_shader(
                self.builder_info
                    .vertex_shader
                    .entry_point("main")
                    .unwrap()
                    .clone(),
                (),
            )
            // Use a resizable viewport set to draw over the entire window
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(
                self.builder_info
                    .fragment_shader
                    .entry_point("main")
                    .unwrap()
                    .clone(),
                (),
            )
            // Now that our builder is filled, we call `build()` to obtain an actual pipeline.
            .build(device.clone())
        {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());
        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(device.clone(), Default::default());

        Ok(PipelineInfo {
            device,
            swapchain,
            pipeline,
            images,
            memory_allocator,
            descriptor_allocator: descriptor_set_allocator,
            command_buffer_allocator: command_buffer_allocator,
        })
    }
}
