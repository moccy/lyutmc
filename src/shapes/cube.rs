use ultraviolet as uv;
use wgpu::{util::DeviceExt, Buffer, Device};

use crate::primitives::vertex::Vertex;

pub struct Cube {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
}

impl Cube {
    pub fn new(device: &Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Vertex Buffer"),
            contents: bytemuck::cast_slice(&Cube::get_vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Index Buffer"),
            contents: bytemuck::cast_slice(&Cube::get_indices()),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn get_vertices() -> [Vertex; 8] {
        [
            Vertex {
                position: uv::Vec3::new(-1.0, -1.0, 1.0),
                color: uv::Vec3::new(1.0, 0.0, 0.0), // Red
            },
            Vertex {
                position: uv::Vec3::new(1.0, -1.0, 1.0),
                color: uv::Vec3::new(0.0, 1.0, 0.0), // Green
            },
            Vertex {
                position: uv::Vec3::new(1.0, 1.0, 1.0),
                color: uv::Vec3::new(0.0, 0.0, 1.0), // Blue
            },
            Vertex {
                position: uv::Vec3::new(-1.0, 1.0, 1.0),
                color: uv::Vec3::new(1.0, 1.0, 0.0), // Yellow
            },
            Vertex {
                position: uv::Vec3::new(-1.0, -1.0, -1.0),
                color: uv::Vec3::new(1.0, 0.0, 1.0), // Magenta
            },
            Vertex {
                position: uv::Vec3::new(1.0, -1.0, -1.0),
                color: uv::Vec3::new(0.0, 1.0, 1.0), // Cyan
            },
            Vertex {
                position: uv::Vec3::new(1.0, 1.0, -1.0),
                color: uv::Vec3::new(1.0, 1.0, 1.0), // White
            },
            Vertex {
                position: uv::Vec3::new(-1.0, 1.0, -1.0),
                color: uv::Vec3::new(0.5, 0.5, 0.5), // Grey
            },
        ]
    }

    pub fn get_indices() -> [u16; 36] {
        [
            // front face (Z-positive)
            2, 1, 0, 0, 3, 2, // right face (X-positive)
            6, 5, 1, 1, 2, 6, // back face (Z-negative)
            7, 4, 5, 5, 6, 7, // left face (X-negative)
            3, 0, 4, 4, 7, 3, // top face (Y-positive)
            6, 2, 3, 3, 7, 6, // bottom face (Y-negative)
            1, 5, 4, 4, 0, 1,
        ]
    }

    pub fn get_vertices_len() -> u32 {
        8
    }

    pub fn get_indices_len() -> u32 {
        36
    }
}
