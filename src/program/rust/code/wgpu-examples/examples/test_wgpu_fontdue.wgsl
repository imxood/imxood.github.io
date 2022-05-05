struct Globals {
    resolution: vec2<f32>;
};

struct VertexInput {
    [[builtin(vertex_index)]] vertex_index: u32;
    // 顶点起始坐标
    [[location(0)]] pos_start: vec2<f32>;
    // 顶点结束坐标
    [[location(1)]] pos_end: vec2<f32>;
    // 纹理起始坐标
    [[location(2)]] tex_start: vec2<f32>;
    // 纹理起始坐标
    [[location(3)]] tex_end: vec2<f32>;
    // 纹理颜色
    [[location(4)]] tex_color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_position: vec2<f32>;
    [[location(1)]] tex_color: vec4<f32>;
};

[[group(0), binding(0)]] var<uniform> globals: Globals;

[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    var position: vec2<f32>;

    switch(i32(in.vertex_index)) {
        // 右上角 顶点
        case 0: {
            position = vec2<f32>(in.pos_end.x, in.pos_start.y);
            out.tex_position = vec2<f32>(in.tex_end.x, in.tex_start.y);
            break;
        }
        // 左上角 顶点
        case 1: {
            position = in.pos_start;
            // out.position = vec4<f32>(, 0.0, 1.0);
            out.tex_position = in.tex_start;
            break;
        }
        // 右下角 顶点
        case 2: {
            position = in.pos_end;
            out.tex_position = in.tex_end;
            break;
        }
        // 左下角 顶点
        case 3: {
            position = vec2<f32>(in.pos_start.x, in.pos_end.y);
            out.tex_position = vec2<f32>(in.tex_start.x, in.tex_end.y);
            break;
        }
        default: {
        }
    }

    let invert = vec2<f32>(2.0, -2.0);
    let transformed_pos = position / globals.resolution * invert + vec2<f32>(-1.0, 1.0);

    out.position = vec4<f32>(transformed_pos, 0.0, 1.0);
    out.tex_color = in.tex_color;
    return out;
}

// Fragment shader
[[group(1), binding(0)]] var t_diffuse: texture_2d<f32>;
[[group(1), binding(1)]] var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // 采样的颜色
    let v = textureSample(t_diffuse, s_diffuse, in.tex_position).a;
    // 只取采样颜色的透明度 a, rgb值 来自 输入的颜色
    return vec4<f32>(in.tex_color.rgb, in.tex_color.a * v);
    // var color = textureSample(t_diffuse, s_diffuse, in.tex_position);
    // color.a = color.a * 0.1;
    // return color;
}