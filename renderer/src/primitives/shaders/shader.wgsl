// Vertex shader

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) texture: vec2<f32>,
    @location(2) normal: vec4<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = vec3<f32>(1.0, 0.0, 0.0);
    out.clip_position = model.position;
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

 

 