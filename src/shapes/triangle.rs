use crate::primitives::vertex::Vertex;
use ultraviolet as uv;
use wgpu::{util::DeviceExt, Buffer, Device};

pub struct Triangle {
    pub vertex_buffer: Buffer,
}

impl Triangle {
    pub fn new(device: &Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Vertex Buffer"),
            contents: bytemuck::cast_slice(&Triangle::get_vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self { vertex_buffer }
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

    pub fn get_vertices_len() -> u32 {
        Triangle::get_vertices().len() as u32
    }
}
