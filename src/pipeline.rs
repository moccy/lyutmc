use wgpu::{PipelineLayoutDescriptor, RenderPipelineDescriptor};

use crate::{camera::camera_uniform::CameraUniform, primitives::vertex::Vertex};

pub fn create_pipeline_layout(
    device: &wgpu::Device,
    camera: &CameraUniform,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&camera.layout],
        push_constant_ranges: &[],
    })
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    swapchain_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[Vertex::get_buffer_layout()], // What type of vertices to pass to the vertex shader.
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        // Describes how to interpret vertices when converting them to triangles
        primitive: wgpu::PrimitiveState {
            // Every 3 vertices will be 1 triangle.
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            // If the vertices are arranged in CW direction, the triangle is facing forward
            front_face: wgpu::FrontFace::Cw,
            // Cull any triangles facing backwards (remove them from the render)
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0, // Used to determine which samples should be active, this means all of them
            alpha_to_coverage_enabled: false, // Related to anti-aliasing
        },
        multiview: None,
    })
}
