use std::error::Error;
use std::sync::Arc;

use wknup::vk::pipeline::GraphicsPipelineBuilder;
use wknup::{
    vk::shader::{ShaderModule, ShaderStage, ShaderStageInfo},
    window::WindowManager,
};
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
    // println!(env!("shader.spv"));
    let vulkan = Arc::new(wknup::vk::VulkanBuilder::new(window).build()?);
    let device = vulkan.get_device();
    let swapchain_manager = vulkan.get_swapchain_manager();
    let swapchain = Arc::new(swapchain_manager.create_swapchain()?);

    let shader = Arc::new(ShaderModule::new(Arc::clone(&device), &SHADER));
    let vertex_info = ShaderStageInfo::new(shader.clone(), ShaderStage::Vertex, "main_vs".into());
    let fragment_info =
        ShaderStageInfo::new(shader.clone(), ShaderStage::Fragment, "main_fs".into());
    let pipeline = GraphicsPipelineBuilder::new(vulkan, swapchain)
        .add_stage("vertex".into(), vertex_info)
        .add_stage("fragment".into(), fragment_info)
        .build()?;
    Ok(())
}
