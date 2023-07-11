use crate::primitives::vertex::Vertex;
use ultraviolet as uv;
use wgpu::{util::DeviceExt, Buffer, Device};

pub struct Triangle {}

impl Triangle {
    pub fn get_vertex_buffer(device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Vertex Buffer"),
            contents: bytemuck::cast_slice(&Triangle::get_vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    pub fn get_vertices() -> [Vertex; 3] {
        [
            Vertex {
                position: uv::Vec3::new(-0.5, -0.5, 0.0), // BL
                color: uv::Vec3::new(1.0, 0.0, 0.0),      // R
            },
            Vertex {
                position: uv::Vec3::new(0.0, 0.5, 0.0), // TOP
                color: uv::Vec3::new(0.0, 1.0, 0.0),    // G
            },
            Vertex {
                position: uv::Vec3::new(0.5, -0.5, 0.0), // BR
                color: uv::Vec3::new(0.0, 0.0, 1.0),     // B
            },
        ]
    }
}
