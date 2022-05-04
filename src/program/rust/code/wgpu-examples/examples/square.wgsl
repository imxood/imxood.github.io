// Vertex shader

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] color: vec3<f32>;
    [[location(2)]] tex_coor: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] tex_coor: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = vec4<f32>(model.color, 1.0);
    out.tex_coor = model.tex_coor;
    return out;
}

// Fragment shader

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    return textureSample(t_diffuse, s_diffuse, in.tex_coor);
}