use winit::window::Window;

pub fn create_surface(instance: &wgpu::Instance, window: &Window) -> wgpu::Surface {
    unsafe {
        instance
            .create_surface(window)
            .expect("Failed to create surface.")
    }
}

pub fn create_surface_config(
    swapchain_format: wgpu::TextureFormat,
    window: &Window,
    swapchain_capabilities: wgpu::SurfaceCapabilities,
) -> wgpu::SurfaceConfiguration {
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    }
}
