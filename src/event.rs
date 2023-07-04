use wgpu::{Device, Surface, SurfaceConfiguration};
use winit::{event::WindowEvent, event_loop::ControlFlow, window::Window};

pub fn handle_window_event(
    event: WindowEvent<'_>,
    window: &Window,
    device: &Device,
    control_flow: &mut ControlFlow,
    surface: &Surface,
    config: &mut SurfaceConfiguration,
) {
    match event {
        WindowEvent::CloseRequested => control_flow.set_exit(),
        WindowEvent::Resized(dimensions) => {
            config.width = dimensions.width;
            config.height = dimensions.height;
            surface.configure(&device, &config);
        }
        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            config.width = new_inner_size.width;
            config.height = new_inner_size.height;
            surface.configure(&device, &config);
        }
        WindowEvent::KeyboardInput { input, .. } => {
            if input.state == winit::event::ElementState::Released {
                match input.virtual_keycode {
                    Some(winit::event::VirtualKeyCode::F11) => {
                        super::window::toggle_fullscreen(&window);
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }
}
