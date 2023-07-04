mod device;
mod event;
mod pipeline;
mod shader;
mod shapes;
mod surface;
mod window;

use log::info;
use wgpu;
use winit::{
    event::Event,
    event_loop::EventLoop,
    window::{Theme, Window, WindowBuilder},
};

fn main() {
    env_logger::init();
    info!("Starting LyutMC");

    const WINDOW_SIZE: [u32; 2] = [1920, 1080];

    let event_loop = EventLoop::new();
    let (logical_window_size, _) = window::get_window_sizes(&event_loop, WINDOW_SIZE);

    let window = WindowBuilder::new()
        .with_title("LyutMC")
        .with_theme(Some(Theme::Dark))
        .with_inner_size(logical_window_size)
        .build(&event_loop)
        .expect("Failed to create window.");

    pollster::block_on(run(window, event_loop));
}

async fn run(window: Window, event_loop: EventLoop<()>) {
    let instance = wgpu::Instance::default();
    let surface = surface::create_surface(&instance, &window);
    let adapter = create_adapter(instance, &surface).await;
    let (device, queue) = device::create_device_and_queue(&adapter).await;
    let shader = shader::create_shader("src/shaders/triangle.wgsl", &device);

    let pipeline_layout = pipeline::create_pipeline_layout(&device);

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline =
        pipeline::create_render_pipeline(&device, pipeline_layout, shader, swapchain_format);
    let mut config =
        surface::create_surface_config(swapchain_format, &window, swapchain_capabilities);

    surface.configure(&device, &config);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                event::handle_window_event(
                    event,
                    &window,
                    &device,
                    control_flow,
                    &surface,
                    &mut config,
                )
            }
            Event::MainEventsCleared => {
                // At this point, all input events have been processed.
                // Time to render the scene.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Rendering goes here.
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to get next swapchain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.draw(0..3, 0..1)
                }

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            _ => (),
        }
    });
}

async fn create_adapter(instance: wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(surface),
        })
        .await
        .expect("Failed to find an adapter.")
}
