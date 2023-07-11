struct VertexOutput {
    // `@builtin(position)` tells wgpu that this value is used for clip coordinates
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vertex_position: vec3<f32>
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    let x = f32(i32(in_vertex_index) - 1) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    
    // Clip position is the position in screenspace, e.g. the pixel the vertex is on.
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    // Vertex Position is the position in world space of the vertex, not necessarily in screen space.
    out.vertex_position = out.clip_position.xyz;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}