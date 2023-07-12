use std::f32::consts::PI;

use ultraviolet as uv;

pub struct Camera {
    pub eye: uv::Vec3,
    pub target: uv::Vec3,
    pub up: uv::Vec3,
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

    pub fn build_view_projection_matrix(&self) -> uv::Mat4 {
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
