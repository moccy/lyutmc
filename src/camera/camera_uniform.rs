use ultraviolet as uv;
use wgpu::util::DeviceExt;

use super::camera::Camera;

pub struct CameraUniform {
    pub view_projection: uv::Mat4,
    pub bind_group: wgpu::BindGroup,
    pub buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
}

impl CameraUniform {
    pub fn new(camera: &Camera, device: &wgpu::Device) -> Self {
        let view_projection = camera.build_view_projection_matrix();

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[view_projection]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        CameraUniform {
            buffer,
            layout,
            bind_group,
            view_projection,
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_projection = camera.build_view_projection_matrix();
    }
}
