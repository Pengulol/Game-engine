use std::{any::Any, error::Error, sync::Arc};
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, CommandBufferLevel,
        CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo,
        SubpassContents,
    },
    device::{
        self, physical::{PhysicalDevice, PhysicalDeviceType}, Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags
    },
    image::{view::ImageView, Image, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        cache::PipelineCache, compute::ComputePipelineCreateInfo, graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        }, layout::PipelineDescriptorSetLayoutCreateInfo, ComputePipeline, DynamicState, GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, RenderPassCreateInfo, Subpass},
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};
// fn create_instance(event_loop :&impl HasRawDisplayHandle) -> Arc<Instance> {
//     let lib = VulkanLibrary::new().expect("no local Vulkan library/DLL");
   
//     let required_extensions = Surface::required_extensions(event_loop);

//     Instance::new(
//         lib,
//         InstanceCreateInfo {

//             flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
//             enabled_extensions: required_extensions,
//             ..Default::default()
//         },
//     )
//     .unwrap()
// }
// fn create_surface(instance: Arc<Instance>, window: Arc<impl raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle + Any + Send + Sync>) -> Arc<Surface> {
//     Surface::from_window(instance, window).unwrap()
// }



pub fn create_device(instance: &Arc<Instance>,surface:&Arc<Surface>) -> (Arc<Device>, impl ExactSizeIterator<Item = Arc<Queue>>) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .unwrap();
    Device::new(
        physical_device,
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device")

   
}


pub fn create_swapchain(
    device: Arc<Device>,
    surface: Arc<Surface>,
    create_info: SwapchainCreateInfo,
) ->  (Arc<Swapchain>, Vec<Arc<Image>>){
    let (swapchain, images) = Swapchain::new(device, surface, create_info).unwrap();
    (swapchain, images)
}
fn recreate_swapchain() {
    //TODO
}


pub fn create_randerpass(device : Arc<Device>
    , create_info: RenderPassCreateInfo
) ->Arc<RenderPass>
{
    RenderPass::new(device, create_info).unwrap()

}





pub fn create_framebuffers(
    render_pass: Arc<RenderPass>,
    create_info: FramebufferCreateInfo
) -> Arc<Framebuffer> {
    Framebuffer::new(render_pass, create_info).unwrap()
}

pub fn create_compute_pipeline( device: Arc<Device>,
    cache: Option<Arc<PipelineCache>>,
    create_info: ComputePipelineCreateInfo
) ->Arc<ComputePipeline>
{
    ComputePipeline::new(device, cache, create_info).unwrap()

}

pub fn create_graphics_pipeline(device: Arc<Device>,
    cache: Option<Arc<PipelineCache>>,
    create_info: GraphicsPipelineCreateInfo
) -> Arc<GraphicsPipeline> {
    GraphicsPipeline::new(device, cache,create_info).unwrap()
}

pub fn create_standardMemoryAllocator_default(device: Arc<Device>) -> Arc<StandardMemoryAllocator> {
    Arc::new(StandardMemoryAllocator::new_default(device))
}





// pub fn create_vertexBuffer(  allocator: Arc<dyn MemoryAllocator>,
//     create_info: BufferCreateInfo,
//     allocation_info: AllocationCreateInfo,
//     iter: I) -> Arc<Buffer> where I: IntoIterator<BufferContents>,
//     I::IntoIter: ExactSizeIterator {
//     Buffer::new( allocator, create_info).unwrap()
// }

// Record a command buffer, conaining commands that the device must execute. Then build the command buffer and submit it to a Queue.


// fn create_graphics_pipeline(device: Arc<Device>, render_pass: Arc<dyn RenderPassAbstract + Send + Sync>) -> Arc<GraphicsPipeline> {
//     mod vs {
//         vulkano_shaders::shader! {
//             ty: "vertex",
//             src: "
//                 #version 450

//                 layout(location = 0) in vec2 position;

//                 void main() {
//                     gl_Position = vec4(position, 0.0, 1.0);
//                 }"
//         }
//     }

//     mod fs {
//         vulkano_shaders::shader! {
//             ty: "fragment",
//             src: "
//                 #version 450

//                 layout(location = 0) out vec4 f_color;

//                 void main() {
//                     f_color = vec4(1.0, 0.0, 0.0, 1.0);
//                 }"
//         }
//     }

//     let vs = vs::Shader::load(device.clone()).unwrap();
//     let fs = fs::Shader::load(device.clone()).unwrap();

//     Arc::new(GraphicsPipeline::start()
//         .vertex_input_single_buffer::<Vertex>()
//         .vertex_shader(vs.main_entry_point(), ())
//         .triangle_list()
//         .viewports_dynamic_scissors_irrelevant(1)
//         .fragment_shader(fs.main_entry_point(), ())
//         .render_pass(vulkano::framebuffer::Subpass::from(render_pass.clone(), 0).unwrap())
//         .build(device.clone())
//         .unwrap())
// }


 