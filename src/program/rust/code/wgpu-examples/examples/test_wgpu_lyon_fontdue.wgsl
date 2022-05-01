// Vertex shader
struct Globals {
    resolution: vec2<f32>;
};

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[group(0), binding(0)]] var<uniform> globals: Globals;

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let invert = vec2<f32>(2.0, -2.0);
    var transformed_pos = model.position / globals.resolution * invert + vec2<f32>(-1.0, 1.0);

    var out: VertexOutput;
    out.clip_position = vec4<f32>(transformed_pos, 0.0, 1.0);
    out.tex_coords = vec2<f32>(out.clip_position.x, - out.clip_position.y);
    return out;
}

// Fragment shader
[[group(1), binding(0)]] var t_diffuse: texture_2d<f32>;
[[group(1), binding(1)]] var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}