use std::error::Error;
use std::sync::Arc;

use wknup::vk::pipeline::PipelineManager;
use wknup::window::WindowManager;

const SHADER_BYTES: &[u8; include_bytes!(env!("shader.spv")).len()] =
    include_bytes!(env!("shader.spv"));
const SHADER: [u32; SHADER_BYTES.len() / 4] = unsafe { std::mem::transmute_copy(SHADER_BYTES) };

fn main() -> Result<(), Box<dyn Error>> {
    let window = WindowManager::init();
    start(&window)?;
    Ok(())
}

#[tokio::main]
async fn start(window: &WindowManager) -> Result<(), Box<dyn Error>> {
    let mut vk_manager = wknup::vk::VulkanManager::init(window)?;
    vk_manager.create_swapchain()?;
    let shader = vk_manager.create_shader_module(&SHADER);
    let _pipeline = PipelineManager::init(&vk_manager, shader);
    Ok(())
}
