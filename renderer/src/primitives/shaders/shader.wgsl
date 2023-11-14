// Vertex shader

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) texture: vec2<f32>,
    @location(2) normal: vec4<f32>
};

struct MVPInput {
    @location(0) mvp0: vec4<f32>,
    @location(1) mvp1: vec4<f32>,
    @location(2) mvp2: vec4<f32>,
    @location(3) mvp3: vec4<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// layout(location = 0) in mat4x4 mvp;

@vertex
fn vs_main(
    model: VertexInput,
    mvp: MVPInput,
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

 

 