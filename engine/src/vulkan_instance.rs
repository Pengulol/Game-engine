use std::sync::Arc;

use vulkano::{device::{
    physical::{PhysicalDevice, PhysicalDeviceType},
    DeviceExtensions, QueueFlags,
    }, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, swapchain::Surface, Version, VulkanLibrary};
use winit::window::Window;
use anyhow::Result;

use crate::core::logger_wrapper::SimpleLogger;

#[derive(Clone)]
pub struct VulkanInstance {
    physical_device: Arc<PhysicalDevice>,
    queue_family_index: u32,
    device_extensions: DeviceExtensions,
}

impl VulkanInstance {
    pub fn new(comp_window: &Window) -> Result<VulkanInstance> {
        let library = VulkanLibrary::new().unwrap();

        let mut required_extensions = Surface::required_extensions(&comp_window);

        if cfg!(debug_assertions) {
            required_extensions.ext_debug_utils = library.supported_extensions().ext_debug_utils;
        }

        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..InstanceCreateInfo::application_from_cargo_toml()
            },
        ).unwrap();

        let surface =unsafe{ Surface::from_window_ref(Arc::clone(&instance), &comp_window)? };

        let mut device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device,queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| {
                p.api_version() >= Version::V1_3 || p.supported_extensions().khr_dynamic_rendering
            })
            .filter(|p| {p.supported_extensions().contains(&device_extensions)})
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
                    
            }).min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => unreachable!(),
            }).expect("No suitable device found");
               

        // SimpleLogger.log(
        //     "Using device {} (type: {:?})",
        //     physical_device.properties().device_name,
        //     physical_device.properties().device_type,
        // );

        if physical_device.api_version() < Version::V1_3 {
            device_extensions.khr_dynamic_rendering = true;
        }

        Ok(VulkanInstance
         {
            physical_device,
            queue_family_index,
            device_extensions,
        })
    }

pub fn physical_device(&self) -> &Arc<PhysicalDevice> {
    &self.physical_device
}
pub fn queue_family_index(&self) -> u32 {
    self.queue_family_index
}
pub fn device_extensions(&self) -> &DeviceExtensions {
    &self.device_extensions
}
}