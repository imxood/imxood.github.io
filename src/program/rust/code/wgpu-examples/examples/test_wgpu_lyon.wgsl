// Vertex shader
struct Globals {
    resolution: vec2<f32>;
};

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[group(0), binding(0)]] var<uniform> globals: Globals;

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let invert = vec2<f32>(2.0, -2.0);
    var transformed_pos = model.position / globals.resolution * invert + vec2<f32>(-1.0, 1.0);

    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(transformed_pos, 0.0, 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}