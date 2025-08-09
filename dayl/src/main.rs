use std::error::Error;
use std::sync::Arc;

use wknup::vk::pipeline::PipelineManager;
use wknup::vk::shader::{ShaderStage, ShaderStageInfo};
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
    println!(env!("shader.spv"));
    let mut vk_manager = wknup::vk::VulkanManager::init(window)?;
    vk_manager.create_swapchain()?;
    let shader = Arc::new(vk_manager.create_shader_module(&SHADER));
    let vertex_info = ShaderStageInfo::new(shader.clone(), ShaderStage::Vertex, "main_vs".into());
    let fragment_info =
        ShaderStageInfo::new(shader.clone(), ShaderStage::Fragment, "main_fs".into());
    let mut pipeline = PipelineManager::init(&vk_manager);
    pipeline.add_stage("vertex".into(), vertex_info);
    pipeline.add_stage("fragment".into(), fragment_info);
    Ok(())
}
