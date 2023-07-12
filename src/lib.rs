use camera::{Camera, CameraUniform};
use input::InputManager;
use shapes::{cube::Cube, triangle::Triangle};
use winit::{event::Event, event_loop::EventLoop, window::WindowBuilder};

mod camera;
mod device;
mod event;
mod input;
mod pipeline;
mod primitives;
mod shader;
mod shapes;
mod surface;
mod window;

pub async fn run(window_title: &str, window_size: [u32; 2]) {
    let instance = wgpu::Instance::default();

    let event_loop = EventLoop::new();
    let (logical_window_size, _) = window::get_window_sizes(&event_loop, window_size);
    let window = WindowBuilder::new()
        .with_title(window_title)
        .with_inner_size(logical_window_size)
        .build(&event_loop)
        .expect("Failed to create window.");

    let mut input_manager = InputManager::new();

    // Creates a surface, which is a handle to something we can render images to.
    let surface = surface::create_surface(&instance, &window);

    // Creates an adapter, which is a handle to the physical GPU.
    let adapter = device::create_adapter(instance, &surface).await;
    // With the above adapter, we can create a connection to the GPU (device)
    // and a queue.
    let (device, queue) = device::create_device_and_queue(&adapter).await;

    let camera = Camera::new(
        (0.0, 1.0, 2.0).into(),
        (0.0, 0.0, 0.0).into(),
        logical_window_size.width as f32 / logical_window_size.height as f32,
    );
    let camera_uniform = CameraUniform::new(&camera, &device);

    // A handle to a compiled shader module.
    let shader = shader::create_shader("src/shaders/shader.wgsl", &device);
    let pipeline_layout = pipeline::create_pipeline_layout(&device, &camera_uniform);

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let active_render_pipeline =
        pipeline::create_render_pipeline(&device, &pipeline_layout, &shader, swapchain_format);
    let mut config =
        surface::create_surface_config(swapchain_format, &window, swapchain_capabilities);

    surface.configure(&device, &config);

    // let triangle = Triangle::new(&device);
    let cube = Cube::new(&device);

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
                    &mut input_manager,
                )
            }
            Event::MainEventsCleared => {
                // At this point, all input events have been processed.
                input_manager.set_key_state(winit::event::VirtualKeyCode::Space, false);
                // Time to render the scene.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Get the next frame in the swap chain to draw to
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to get next swapchain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                // Create a render pass, which is a type of command buffer
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"), // for debugging
                    // This is what @location(0) in the fragment shader targets
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        // The texture view we created is basically saying render to a texture on our surface
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            // Load is how to handle the colours stored from the previous frame.
                            // In this case, we are clearing it to a black colour.
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            // Store is whether we want to store the texture on the underlying texture or not
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&active_render_pipeline);

                render_pass.set_bind_group(0, &camera_uniform.bind_group, &[]);
                render_pass.set_vertex_buffer(0, cube.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(cube.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                // Draw something with 3 vertices and 1 instance.
                render_pass.draw_indexed(0..Cube::get_indices_len(), 0, 0..1);

                // We drop render_pass so we can call encoder.finish(),
                // since render_pass borrows encoder mutably.
                std::mem::drop(render_pass);

                queue.submit(Some(encoder.finish()));
                frame.present();
            }
            _ => (),
        }
    });
}
