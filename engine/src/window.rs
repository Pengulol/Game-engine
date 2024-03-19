use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage}, image::ImageUsage, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter}, pipeline::{graphics:: vertex_input::{
        Vertex, VertexDefinition
    }, PipelineShaderStageCreateInfo}, shader::ShaderInterface, swapchain::{Surface, SwapchainCreateInfo}, VulkanLibrary
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
};
use std::sync::Arc;
use crate::{core::application::WindowProps, vulkan_instance::VulkanInstance};
use crate::render;



pub struct Window {
    event_loop: EventLoop<()>,
    window: Arc<winit::window::Window>,
}

impl Window {
    pub fn new(window_prop : WindowProps) -> Self {

        let event_loop = EventLoop::new().unwrap();
        let window_builder = WindowBuilder::new()
            .with_title(window_prop.title)
            .with_inner_size(winit::dpi::LogicalSize::new(window_prop.width, window_prop.height))
            .with_resizable(window_prop.resizable);
        

         

        let window =Arc::new( window_builder.build(&event_loop).unwrap());

        let context = Arc::new(VulkanInstance::new(&window).unwrap());
        
        Window { event_loop, window }
    }

    pub fn run(self) {
    //    self.event_loop.run(move |event, _, control_flow| {
    //         match event {
    //             Event::WindowEvent {
    //                 event: WindowEvent::CloseRequested,
    //                 ..
    //             } => *control_flow = ControlFlow::Exit,
    //             _ => (),
    //         }
    //     });
    }
}

