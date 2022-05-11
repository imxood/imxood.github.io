struct Globals {
    transform: mat4x4<f32>;
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
    // z坐标
    [[location(5)]] z_index: f32;
    // font 粗体
    [[location(6)]] bold: f32;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] tex_position: vec2<f32>;
    [[location(1)]] tex_color: vec4<f32>;
    [[location(2)]] bold: f32;
};

[[group(0), binding(0)]] var<uniform> globals: Globals;

[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    var position: vec2<f32> = vec2<f32>(0.0, 0.0);

    // 根据配置: 三角形 顶点的输出 需要 逆时针方向
    // 顶点 0 1 2 3 => 生成 0 1 2, 2 1 3, 两个逆时针方向的三角形
    switch(i32(in.vertex_index)) {
        // 右上角 顶点
        case 0: {
            position = vec2<f32>(in.pos_end.x, in.pos_start.y);
            out.tex_position = vec2<f32>(in.tex_end.x, in.tex_start.y);
            break;
        }
        // 左上角 顶点
        case 1: {
            position = in.pos_start.xy;
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

    // 屏幕坐标转换成 wgsl的 顶点坐标
    out.position = globals.transform * vec4<f32>(position, in.z_index, 1.0);
    out.tex_color = in.tex_color;
    out.bold = in.bold;
    return out;
}

[[group(1), binding(0)]] var t_diffuse: texture_2d<f32>;
[[group(1), binding(1)]] var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // 在纹理的指定位置采样
    let color = textureSample(t_diffuse, s_diffuse, in.tex_position);

    if (in.bold == 0.0 && color.r < 0.5) {
        discard;
    }

    // 只使用采样颜色的透明度, rgb值 来自 输入的颜色
    return vec4<f32>(in.tex_color.rgb, in.tex_color.a * color.a);
}