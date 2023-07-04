mod device;
mod event;
mod pipeline;
mod shader;
mod surface;
mod window;

use std::fs;

use log::info;
use wgpu::{
    self, Device, DeviceDescriptor, Features, PipelineLayoutDescriptor, RenderPipelineDescriptor,
    Surface, SurfaceConfiguration, TextureFormat,
};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Theme, Window, WindowBuilder},
};

fn main() {
    env_logger::init();
    info!("Starting LyutMC");

    const WINDOW_SIZE: [u32; 2] = [1920, 1080];

    let event_loop = EventLoop::new();
    let (logical_window_size, physical_window_size) =
        window::get_window_sizes(&event_loop, WINDOW_SIZE);

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
    let shader = create_shader("src/shaders/triangle.wgsl", &device);

    let pipeline_layout = create_pipeline_layout(&device);

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline =
        create_render_pipeline(&device, pipeline_layout, shader, swapchain_format);
    let mut config =
        surface::create_surface_config(swapchain_format, &window, swapchain_capabilities);

    surface.configure(&device, &config);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                handle_window_event(event, &window, &device, control_flow, &surface, &mut config)
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

fn handle_window_event(
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
                        window::toggle_fullscreen(&window);
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }
}

fn create_render_pipeline(
    device: &wgpu::Device,
    pipeline_layout: wgpu::PipelineLayout,
    shader: wgpu::ShaderModule,
    swapchain_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    })
}

fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    })
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

fn create_shader(shader_path: &str, device: &wgpu::Device) -> wgpu::ShaderModule {
    let shader_source = fs::read_to_string(shader_path)
        .expect(format!("Failed to load shader at path: {}", shader_path).as_str());

    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    })
}
