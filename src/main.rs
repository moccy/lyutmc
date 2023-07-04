use log::info;
use wgpu::{self, DeviceDescriptor, Features, PipelineLayoutDescriptor, RenderPipelineDescriptor};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Theme, Window, WindowBuilder},
};

fn main() {
    env_logger::init();
    info!("Starting LyutMC");

    const WINDOW_SIZE: [u32; 2] = [1920, 1080];

    let event_loop = EventLoop::new();
    let (logical_window_size, physical_window_size) = get_window_sizes(&event_loop, WINDOW_SIZE);
    let window = WindowBuilder::new()
        .with_title("LyutMC")
        .with_theme(Some(Theme::Dark))
        .with_inner_size(LogicalSize {
            width: 1920,
            height: 1080,
        })
        .build(&event_loop)
        .expect("Failed to create window.");

    pollster::block_on(run(window, event_loop));
}

async fn run(window: Window, event_loop: EventLoop<()>) {
    let instance = wgpu::Instance::default();
    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Failed to create surface.")
    };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an adapter.");

    let (device, queue) = adapter
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
        .expect("Failed to create virtual device and queue.");

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
            "shaders/triangle.wgsl"
        ))),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
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
    });

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
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
                _ => (),
            },
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

fn get_window_sizes(
    event_loop: &EventLoop<()>,
    window_size: [u32; 2],
) -> (LogicalSize<u32>, PhysicalSize<u32>) {
    let monitor = event_loop
        .primary_monitor()
        .or_else(|| event_loop.available_monitors().next())
        .expect("Failed to find a monitor.");
    let dpi = monitor.scale_factor();
    let logical: LogicalSize<u32> = window_size.into();
    let physical: PhysicalSize<u32> = logical.to_physical(dpi);

    (logical, physical)
}
