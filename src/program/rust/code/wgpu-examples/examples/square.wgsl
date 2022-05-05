// Vertex shader

struct VertexInput {
    [[builtin(instance_index)]] vertex_index: u32;
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
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let vertex_index = i32(in.vertex_index);
    switch(vertex_index) {
        case 0: {
            out.position = vec4<f32>(in.position, 1.0, 1.0);
            out.color = vec4<f32>(in.color, 1.0);
            break;
        }
        case 1: {
            out.position = vec4<f32>(in.position * 0.5, 0.0, 1.0);
            out.color = vec4<f32>(in.color * 0.1, 1.0);
            break;
        }
        // case 2: {
        //     out.position = vec4<f32>(in.position, 1.0, 1.0);
        //     out.color = vec4<f32>(in.color, 1.0);
        //     break;
        // }
        // case 3: {
        //     out.position = vec4<f32>(in.position, 1.0, 1.0);
        //     out.color = vec4<f32>(in.color, 1.0);
        //     break;
        // }
        default: {
        }
    }
    out.tex_coor = in.tex_coor;
    return out;
}

// Fragment shader

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
    // return textureSample(t_diffuse, s_diffuse, in.tex_coor);
}