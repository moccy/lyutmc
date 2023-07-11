struct VertexInput {
    @location(0) vertex_position: vec3<f32>,
    @location(1) vertex_color: vec3<f32>,
}

struct VertexOutput {
    // `@builtin(position)` tells wgpu that this value is used for clip coordinates
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vertex_color: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Passthrough color
    out.vertex_color = model.vertex_color;
    // Clip position is the position in screenspace, e.g. the pixel the vertex is on.
    out.clip_position = vec4<f32>(model.vertex_position / 2.0, 1.0);

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.vertex_color, 1.0);
}