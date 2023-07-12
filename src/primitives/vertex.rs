use bytemuck_derive::{Pod, Zeroable};
use ultraviolet as uv;
use wgpu::VertexBufferLayout;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: uv::Vec3,
    pub tex_coords: uv::Vec2,
}

impl Vertex {
    pub fn get_buffer_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // How big is a vertex?
            step_mode: wgpu::VertexStepMode::Vertex, // How often to step forward the vertex buffer
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
            // Attributes can be specified more concisely:
            // attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
        }
    }
}
