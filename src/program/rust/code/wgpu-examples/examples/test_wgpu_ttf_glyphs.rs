use lyon::lyon_tessellation::VertexBuffers;
use std::iter;
use ttf_glyphs::GlyphVertex;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Globals {
    pub transform: [f32; 16],
}

impl Globals {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            transform: Self::orthographic_projection(window_width, window_height),
        }
    }

    /// 正交缩放矩阵
    ///     屏幕坐标先进行一个缩放, 缩放因子: (2.0 / window_width, -2.0 / window_height), 再进行一个位移 (-1.0, 1.0)
    pub fn orthographic_projection(window_width: f32, window_height: f32) -> [f32; 16] {
        #[cfg_attr(rustfmt, rustfmt_skip)]
    [
        2.0 / window_width, 0.0, 0.0, 0.0,
        0.0, -2.0 / window_height, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -1.0, 1.0, 0.0, 1.0,
    ]
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

impl From<GlyphVertex> for Vertex {
    fn from(v: GlyphVertex) -> Self {
        let GlyphVertex { position, color } = v;
        Self { position, color }
    }
}

impl Vertex {
    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x4, 1 => Float32x4
    ];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_len: u32,
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
            log::info!("adapter info: {:?}", adapter.get_info());
        }
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let globals_buffer_byte_size = std::mem::size_of::<Globals>() as u64;

        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Globals Buffer"),
            contents: bytemuck::cast_slice(&[Globals::new(size.width as f32, size.height as f32)]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // 创建 Shade
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./test_wgpu_ttf_glyphs.wgsl").into()),
        });

        let globals_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Globals bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(globals_buffer_byte_size),
                    },
                    count: None,
                }],
            });

        let globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Globals bind group"),
            layout: &globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(globals_buffer.as_entire_buffer_binding()),
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&globals_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                // clamp_depth: false,
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None,
        });

        let VertexBuffers { vertices, indices } = build_ttf_glyphs();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_len = indices.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            globals_buffer,
            globals_bind_group,
            vertex_buffer,
            index_buffer,
            index_len,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.queue.write_buffer(
                &self.globals_buffer,
                0,
                bytemuck::cast_slice(&[Globals::new(
                    new_size.width as f32,
                    new_size.height as f32,
                )]),
            );
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.index_len, 0, 0..1);
            // render_pass.draw(0..self.index_len as u32, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn build_ttf_glyphs() -> VertexBuffers<Vertex, u16> {
    use ttf_glyphs::{Font, Section};
    let mut font = Font::new(include_bytes!("../../fonts/DroidSansFallbackFull.ttf"), 0).unwrap();

    let (rect, buffer) = Section::new()
        .with_position([0.0, 0.0, 1.0, 1.0])
        .with_size(16.0)
        .with_color([1.0, 1.0, 0.0, 1.0])
        .with_max_width(1024.0)
        .append("饕餮，")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .build(&mut font);

    let (rect, buffer) = Section::with_buffer(buffer)
        .with_position([rect.min.x, rect.max.y + 50.0, 1.0, 1.0])
        .with_size(20.0)
        .with_border((1.0, [1.0, 0.0, 0.0, 0.5]))
        .with_color([0.0, 0.0, 1.0, 1.0])
        .with_max_width(1024.0)
        .append("饕餮，")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .build(&mut font);

    let (rect, buffer) = Section::with_buffer(buffer)
        .with_position([rect.min.x, rect.max.y + 50.0, 1.0, 1.0])
        .with_size(50.0)
        .with_color([1.0, 0.0, 0.0, 1.0])
        .with_border((10.0, [0.5, 0.2, 0.7, 1.0]))
        .with_max_width(1024.0)
        .append("饕餮，")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .append("你好。")
        .build(&mut font);

    let VertexBuffers {
        mut vertices,
        indices,
    } = buffer;

    let vertices = vertices.drain(..).map(|v| v.into()).collect();
    VertexBuffers { vertices, indices }
}

fn main() {
    std::env::set_var("RUST_LOG", "DEBUG,wgpu_core=INFO,wgpu_hal=INFO");
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // State::new uses async code, so we're going to wait for it to finish
    let mut state: State = pollster::block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so w have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
