use std::fs;

use log::info;

pub fn create_shader(shader_path: &str, device: &wgpu::Device) -> wgpu::ShaderModule {
    let shader_source = fs::read_to_string(shader_path)
        .expect(format!("Failed to load shader at path: {}", shader_path).as_str());

    info!("Loading shader from {}", shader_path);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });
    info!("Shader loaded from {}", shader_path);
    shader
}
