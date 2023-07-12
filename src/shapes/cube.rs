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

    // 0,0______1,0
    //  |        |
    //  |        |
    //  |        |
    //  |________|
    // 0,1      1,1

    pub fn get_vertices() -> [Vertex; 8] {
        [
            Vertex {
                position: uv::Vec3::new(-1.0, -1.0, 1.0), // Front Bottom Left
                tex_coords: uv::Vec2::new(0.0, 1.0),
            },
            Vertex {
                position: uv::Vec3::new(-1.0, 1.0, 1.0), // Front Top Left
                tex_coords: uv::Vec2::new(0.0, 0.0),
            },
            Vertex {
                position: uv::Vec3::new(1.0, 1.0, 1.0), // Front Top Right
                tex_coords: uv::Vec2::new(1.0, 0.0),
            },
            Vertex {
                position: uv::Vec3::new(1.0, -1.0, 1.0), // Front Bottom Right
                tex_coords: uv::Vec2::new(1.0, 1.0),
            },
            Vertex {
                position: uv::Vec3::new(-1.0, -1.0, -1.0), // Back Bottom Left
                tex_coords: uv::Vec2::new(1.0, 1.0),
            },
            Vertex {
                position: uv::Vec3::new(-1.0, 1.0, -1.0), // Back Top Left
                tex_coords: uv::Vec2::new(1.0, 0.0),
            },
            Vertex {
                position: uv::Vec3::new(1.0, 1.0, -1.0), // Back Top Right
                tex_coords: uv::Vec2::new(0.0, 0.0),
            },
            Vertex {
                position: uv::Vec3::new(1.0, -1.0, -1.0), // Back Bottom Right
                tex_coords: uv::Vec2::new(0.0, 1.0),
            },
        ]
    }

    pub fn get_indices() -> [u16; 36] {
        [
            0, 1, 2, 0, 2, 3, // front face (Z-positive)
            7, 6, 5, 7, 5, 4, // back face (Z-negative)
            3, 2, 6, 3, 6, 7, // right face (X-positive)
            4, 5, 1, 4, 1, 0, // left face (X-negative)
            1, 5, 6, 1, 6, 2, // top face (Y-positive)
            4, 0, 3, 4, 3, 7, // bottom face (Y-negative)
        ]
    }

    pub fn get_vertices_len() -> u32 {
        8
    }

    pub fn get_indices_len() -> u32 {
        36
    }
}
