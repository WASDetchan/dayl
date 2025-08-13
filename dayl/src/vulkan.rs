use ash::{Entry, vk};

use std::sync::Arc;

use wknup::{
    vk::device::{Device, DeviceBuilder},
    vk::instance::{Instance, InstanceBuilder},
    vk::surface::SurfaceManager,
    vk::swapchain::SwapchainManager,
    window::WindowManager,
};

pub fn init_entry() -> Arc<Entry> {
    Arc::new(Entry::linked())
}

pub fn init_instance(window: &WindowManager, entry: Arc<Entry>) -> Arc<Instance> {
    let wm_required_extensions = window
        .get_vk_extensions()
        .unwrap_or_else(|e| panic!("Could not init vulkan instance: {}", e));

    let instance = InstanceBuilder::new(entry)
        .extensions(wm_required_extensions)
        .validation_layers(vec![String::from("VK_LAYER_KHRONOS_validation")])
        .application_props(String::from("dayl"), 1)
        .api_version(vk::make_api_version(0, 1, 1, 0))
        .build()
        .unwrap_or_else(|e| panic!("Could not init vulkan instance: {}", e));
    Arc::new(instance)
}
pub fn init_surface(window: &WindowManager, instance: Arc<Instance>) -> Arc<SurfaceManager> {
    Arc::new(
        SurfaceManager::init(instance, window)
            .unwrap_or_else(|e| panic!("Could not init vulkan surface: {}", e)),
    )
}
pub fn init_device(instance: Arc<Instance>, surface: Arc<SurfaceManager>) -> Arc<Device> {
    Arc::new(
        DeviceBuilder::new(instance, surface)
            .build()
            .unwrap_or_else(|e| panic!("Could not init vulkan logical device: {}", e)),
    )
}
pub fn init_swapchain_manager(
    surface: Arc<SurfaceManager>,
    device: Arc<Device>,
) -> Arc<SwapchainManager> {
    Arc::new(SwapchainManager::new(device, surface))
}
