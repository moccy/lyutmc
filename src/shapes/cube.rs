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
    pub fn get_vertices() -> [Vertex; 16] {
        [
            Vertex::new((-1.0, -1.0, 1.0), (0.0, 1.0)), // 0 - Front Bottom Left
            Vertex::new((-1.0, 1.0, 1.0), (0.0, 0.0)), // 1 - Front Top Left
            Vertex::new((1.0, 1.0, 1.0), (1.0, 0.0)), // 2 - Front Top Right
            Vertex::new((1.0, -1.0, 1.0), (1.0, 1.0)), // 3 - Front Bottom Right

            Vertex::new((-1.0, -1.0, -1.0), (1.0, 1.0)), // 4 - Back Bottom Left
            Vertex::new((-1.0, 1.0, -1.0), (1.0, 0.0)), // 5 - Back Top Left
            Vertex::new((1.0, 1.0, -1.0), (0.0, 0.0)), // 6 - Back Top Right
            Vertex::new((1.0, -1.0, -1.0), (0.0, 1.0)), // 7 - Back Bottom Right

            Vertex::new((-1.0, 1.0, 1.0), (0.0, 0.0)),   // 8 - Top Front Left (new)
            Vertex::new((1.0, 1.0, 1.0), (1.0, 0.0)),     // 9 - Top Front Right (new)
            Vertex::new((-1.0, 1.0, -1.0), (0.0, 1.0)),  // 10 - Top Back Left (new)
            Vertex::new((1.0, 1.0, -1.0), (1.0, 1.0)),   // 11 - Top Back Right (new)

            Vertex::new((-1.0, -1.0, 1.0), (0.0, 0.0)),   // 12 - Bottom Front Left (new)
            Vertex::new((1.0, -1.0, 1.0), (1.0, 0.0)),     // 13 - Bottom Front Right (new)
            Vertex::new((-1.0, -1.0, -1.0), (0.0, 1.0)),  // 14 - Bottom Back Left (new)
            Vertex::new((1.0, -1.0, -1.0), (1.0, 1.0)),   // 15 - Bottom Back Right (new)
        ]
    }

    pub fn get_indices() -> [u16; 36] {
        [
            0, 1, 2, 0, 2, 3, // front face (Z-positive)
            7, 6, 5, 7, 5, 4, // back face (Z-negative)
            3, 2, 6, 3, 6, 7, // right face (X-positive)
            4, 5, 1, 4, 1, 0, // left face (X-negative)
            8, 10, 11, 8, 11, 9, // top face (Y-positive)
            15, 14, 12, 13, 15, 12, // bottom face (Y-negative)
        ]
    }

    pub fn get_vertices_len() -> u32 {
        16
    }

    pub fn get_indices_len() -> u32 {
        36
    }
}
