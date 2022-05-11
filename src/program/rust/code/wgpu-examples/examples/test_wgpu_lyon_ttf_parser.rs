use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};
use lyon::{
    geom::{point, rect, vector, Transform},
    lyon_tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor,
        Orientation, StrokeOptions, StrokeTessellator, StrokeVertex, StrokeVertexConstructor,
        VertexBuffers,
    },
    math::Rect,
    path::{
        builder::{BorderRadii, WithSvg},
        path::Builder,
        traits::PathBuilder,
        FillRule, Path, Winding,
    },
};
use std::iter;
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
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3, 1 => Float32x3
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
    depth_texture: Texture,
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

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        let globals_buffer_byte_size = std::mem::size_of::<Globals>() as u64;

        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Globals Buffer"),
            contents: bytemuck::cast_slice(&[Globals::new(size.width as f32, size.height as f32)]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // 创建 Shade
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("./test_wgpu_lyon_ttf_parser.wgsl").into(),
            ),
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
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                front_face: wgpu::FrontFace::Ccw,
                strip_index_format: None,
                cull_mode: Some(wgpu::Face::Back),
                // Requires Features::DEPTH_CLAMPING
                // clamp_depth: false,
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None,
        });

        let VertexBuffers { vertices, indices } = build_lyon();

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
            depth_texture,
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
            self.depth_texture =
                Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            // render_pass.draw_indexed(self.index_range0.clone(), 0, 0..1);
            render_pass.draw_indexed(0..self.index_len, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.

    pub fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        label: Option<&str>,
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }

    pub fn write_texture(
        &self,
        queue: &wgpu::Queue,
        data: &[u8],
        origin: [u32; 2],
        width: u32,
        height: u32,
    ) {
        let origin = wgpu::Origin3d {
            x: origin[0],
            y: origin[1],
            z: 0,
        };
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(width * 4),
                rows_per_image: std::num::NonZeroU32::new(height),
            },
            size,
        );
    }

    pub fn from_raw(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: Vec<u8>,
        width: u32,
        height: u32,
        label: Option<&str>,
    ) -> Self {
        let buf: ImageBuffer<Luma<u8>, _> = ImageBuffer::from_raw(width, height, bytes).unwrap();
        let image = DynamicImage::ImageLuma8(buf);

        let rgba = image.to_rgba8();

        let this = Self::new(
            device,
            width,
            height,
            wgpu::TextureFormat::Bgra8Unorm,
            label,
        );
        this.write_texture(queue, &rgba, [0, 0], width, height);
        this
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Self {
        let img = image::load_from_memory(bytes).unwrap();
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let this = Self::new(
            device,
            dimensions.0,
            dimensions.1,
            wgpu::TextureFormat::Bgra8Unorm,
            label,
        );
        this.write_texture(queue, &rgba, [0, 0], dimensions.0, dimensions.1);
        this
    }

    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        label: &str,
    ) -> Self {
        let size = wgpu::Extent3d {
            // 2.
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT // 3.
                | wgpu::TextureUsages::TEXTURE_BINDING,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            // 4.
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual), // 5.
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}

pub struct WithZIndex(pub f32, pub [f32; 3]);

impl FillVertexConstructor<Vertex> for WithZIndex {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: [vertex.position().x, vertex.position().y, self.0],
            color: self.1,
        }
    }
}

impl StrokeVertexConstructor<Vertex> for WithZIndex {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        let pos = vertex.position();
        Vertex {
            position: [pos.x, pos.y, self.0],
            color: self.1,
        }
    }
}

struct OutlineBuilder {
    builder: WithSvg<Builder>,
}

impl OutlineBuilder {
    pub fn new() -> Self {
        Self {
            builder: Path::builder().with_svg(),
        }
    }
    pub fn build(self) -> Path {
        self.builder.build()
    }
}

impl ttf_parser::OutlineBuilder for OutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.move_to(point(x, y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(point(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder.quadratic_bezier_to(point(x1, y1), point(x, y));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder
            .cubic_bezier_to(point(x1, y1), point(x2, y2), point(x, y));
    }

    fn close(&mut self) {
        self.builder.close();
    }
}

fn build_lyon() -> VertexBuffers<Vertex, u16> {
    let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
    let mut fill_tess = FillTessellator::new();
    let mut stroke_tess = StrokeTessellator::new();

    let face =
        ttf_parser::Face::from_slice(include_bytes!("../../fonts/DroidSansFallbackFull.ttf"), 0)
            .map_err(|e| format!("{:?}", &e))
            .unwrap();
    let height = face.height() as f32;
    let descender = face.descender() as f32;

    let glyph_id = face.glyph_index('饕').unwrap();
    let advance_x = face.glyph_hor_advance(glyph_id).unwrap_or(0) as f32;

    let mut outline_builder = OutlineBuilder::new();

    let glyph_rect = face.outline_glyph(glyph_id, &mut outline_builder).unwrap();

    let mut path = outline_builder.build();
    let mut inner_rect = rect(
        glyph_rect.x_min as f32,
        glyph_rect.y_min as f32,
        glyph_rect.width() as f32,
        glyph_rect.height() as f32,
    );
    let mut outline_rect = rect(0.0, face.descender() as f32, advance_x, height);

    /*
        为了防止 倒置后超出屏幕, 向上移动一定的位置
    */
    if descender < 0.0 {
        let transform = Transform::translation(0.0, -descender);
        inner_rect = transform.outer_transformed_rect(&inner_rect);
        outline_rect = transform.outer_transformed_rect(&outline_rect);
        path = path.transformed(&transform);
    }

    /*
        字体坐标系 转换到 屏幕坐标系: 先上下翻转, 再平移
    */
    let transform = Transform::identity()
        .then_scale(1.0, -1.0)
        .then_translate(vector(0.0, inner_rect.max_y() + inner_rect.min_y()));

    let inner_rect = transform.outer_transformed_rect(&inner_rect);
    let outline_rect = transform.outer_transformed_rect(&outline_rect);
    path = path.transformed(&transform);

    // 字形内轮廓添加边框
    stroke_tess
        .tessellate_rectangle(
            &inner_rect,
            &StrokeOptions::DEFAULT,
            &mut BuffersBuilder::new(&mut geometry, WithZIndex(0.3, [1.0, 1.0, 0.0])),
        )
        .unwrap();

    // 字形外轮廓添加边框
    stroke_tess
        .tessellate_rectangle(
            &outline_rect,
            &StrokeOptions::DEFAULT.with_line_width(50.0),
            &mut BuffersBuilder::new(&mut geometry, WithZIndex(0.2, [1.0, 0.0, 0.0])),
        )
        .unwrap();

    // 构建字形的顶点和索引
    fill_tess
        .tessellate_path(
            &path,
            &FillOptions::tolerance(0.001),
            &mut BuffersBuilder::new(&mut geometry, WithZIndex(0.2, [1.0, 1.0, 0.0])),
        )
        .unwrap();

    // println!("geometry.vertices: {:?}", &geometry.vertices);
    // println!("geometry.indices: {:?}", &geometry.indices);
    println!(
        " -- {} vertices {} indices",
        geometry.vertices.len(),
        geometry.indices.len()
    );
    geometry
}

fn main() {
    std::env::set_var("RUST_LOG", "INFO");
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
