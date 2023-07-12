use std::f32::consts::PI;

use ultraviolet as uv;
use wgpu::util::DeviceExt;

pub struct Camera {
    eye: uv::Vec3,
    target: uv::Vec3,
    up: uv::Vec3,
    aspect: f32,
    fov_y_rad: f32,
    z_near: f32,
    z_far: f32,
}

impl Camera {
    pub fn new(eye: uv::Vec3, target: uv::Vec3, aspect: f32) -> Self {
        Camera {
            eye,
            target,
            up: uv::Vec3::unit_y(),
            aspect,
            fov_y_rad: PI / 4.0,
            z_near: 0.1,
            z_far: 100.0,
        }
    }

    fn build_view_projection_matrix(&self) -> uv::Mat4 {
        let view = uv::Mat4::look_at(self.eye, self.target, self.up);
        let projection = uv::projection::perspective_wgpu_dx(
            self.fov_y_rad,
            self.aspect,
            self.z_near,
            self.z_far,
        );
        projection * view
    }
}

pub struct CameraUniform {
    pub bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
}

impl CameraUniform {
    pub fn new(camera: &Camera, device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera.build_view_projection_matrix()]),
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
        }
    }
}
