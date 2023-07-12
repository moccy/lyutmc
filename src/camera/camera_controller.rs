use crate::input::InputManager;

use super::camera::Camera;

pub struct CameraController {
    speed: f32,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    pub fn update_camera(&self, camera: &mut Camera, input_manager: &InputManager) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalized();
        let forward_mag = forward.mag();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if input_manager.key_pressed(winit::event::VirtualKeyCode::W) && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if input_manager.key_pressed(winit::event::VirtualKeyCode::S) {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the fowrard/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.mag();

        if input_manager.key_pressed(winit::event::VirtualKeyCode::D) {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalized() * forward_mag;
        }
        if input_manager.key_pressed(winit::event::VirtualKeyCode::A) {
            camera.eye = camera.target - (forward - right * self.speed).normalized() * forward_mag;
        }
    }
}
