use wgpu::{DeviceDescriptor, Features};

/// Creates an adapter.
///
/// An adapter is a handle to the physical GPU.
///
/// Adapters can be used to create connections to the GPU by calling Adapter::request_device
pub async fn create_adapter(instance: wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await
        .expect("Failed to find an adapter.")
}

/// Creates a Device and a Queue.
///
/// A Device represents a connection to the physical device and is created from an adapter.
///
/// A Queue effectively represents a queue of commands that will be submitted to the GPU.
/// The commands take the form of CommandBuffers (single-use) or RenderBundles (reusable),
/// which will be submitted to the queue.
pub async fn create_device_and_queue(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create virtual device and queue.")
}
