use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use guillotiere::AtlasAllocator;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma};
use lyon::math::Point;
use std::iter;
use std::{collections::HashSet, time::Instant};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use guillotiere::euclid;

use euclid::UnknownUnit;

// pub type Length<T> = euclid::Length<T, UnknownUnit>;
pub type Point2<T> = euclid::Point2D<T, UnknownUnit>;
pub type Point3<T> = euclid::Point3D<T, UnknownUnit>;
pub type Vector2<T> = euclid::Vector2D<T, UnknownUnit>;
// pub type Vector3D<T> = euclid::Vector3D<T, UnknownUnit>;
// pub type HomogeneousVector<T> = euclid::HomogeneousVector<T, UnknownUnit>;
pub type Size2<T> = euclid::Size2D<T, UnknownUnit>;
// pub type Size3D<T> = euclid::Size3D<T, UnknownUnit>;
// pub type Rect<T> = euclid::Rect<T, UnknownUnit>;
pub type Box2<T> = euclid::Box2D<T, UnknownUnit>;
// pub type Box3D<T> = euclid::Box3D<T, UnknownUnit>;
// pub type SideOffsets2D<T> = euclid::SideOffsets2D<T, UnknownUnit>;
// pub type Transform2D<T> = euclid::Transform2D<T, UnknownUnit, UnknownUnit>;
// pub type Transform3D<T> = euclid::Transform3D<T, UnknownUnit, UnknownUnit>;
// pub type Rotation2D<T> = euclid::Rotation2D<T, UnknownUnit, UnknownUnit>;
// pub type Rotation3D<T> = euclid::Rotation3D<T, UnknownUnit, UnknownUnit>;
// pub type Translation2D<T> = euclid::Translation2D<T, UnknownUnit, UnknownUnit>;
// pub type Translation3D<T> = euclid::Translation3D<T, UnknownUnit, UnknownUnit>;
// pub type Scale<T> = euclid::Scale<T, UnknownUnit, UnknownUnit>;
// pub type RigidTransform3D<T> = euclid::RigidTransform3D<T, UnknownUnit, UnknownUnit>;

#[inline]
pub fn size2<T>(w: T, h: T) -> Size2<T> {
    Size2::new(w, h)
}

#[inline]
pub fn vector2<T>(w: T, h: T) -> Vector2<T> {
    Vector2::new(w, h)
}

#[inline]
pub fn point2<T>(x: T, y: T) -> Point2<T> {
    Point2::new(x, y)
}

#[inline]
pub fn point3<T>(x: T, y: T, z: T) -> Point3<T> {
    Point3::new(x, y, z)
}

#[inline]
pub fn box2<T>(min: Point2<T>, max: Point2<T>) -> Box2<T> {
    Box2::new(min, max)
}

// pub fn rect2<T>(w: T, h: T) -> Box2<T> {
//     Box2::new(w, h)
// }

#[repr(C)]
#[derive(Copy, Clone)]
struct Globals {
    pub window_width: f32,
    pub window_height: f32,
    pub transform: [f32; 16],
}

impl Globals {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            window_width,
            window_height,
            transform: orthographic_projection(window_width, window_height),
        }
    }
    pub fn update_window_size(&mut self, window_width: u32, window_height: u32) {
        self.window_width = window_width as f32;
        self.window_height = window_height as f32;
        self.transform = orthographic_projection(self.window_width, self.window_height);
    }
}

/// 正交缩放矩阵, 用于把当前 window的 x轴 和 y轴 缩放到 2 和 -2, 窗口范围是 x: [-1,1] y: [-1,1]
pub fn orthographic_projection(window_width: f32, window_height: f32) -> [f32; 16] {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    [
        2.0 / window_width, 0.0, 0.0, 0.0,
        0.0, -2.0 / window_height, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -1.0, 1.0, 0.0, 1.0,
    ]
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    globals_bind_group_layout: wgpu::BindGroupLayout,
    font_builder: FontBuilder,
    pub globals: Globals,
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

        let format = surface.get_preferred_format(&adapter).unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        let globals = Globals::new(size.width as f32, size.height as f32);

        let globals_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("globals bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        /*
            加载字体
        */

        let mut font_builder = FontBuilder::new(&device, format, &globals_bind_group_layout);
        font_builder.draw(
            FontText::new("你好，上海加油", point2(10.0, 10.0), 16.0)
                .with_color([1.0, 0.0, 0.0, 1.0]),
        );
        font_builder.draw(
            FontText::new("你好，上海加油", point2(10.0, 26.0), 16.0)
                .with_color([1.0, 1.0, 0.0, 1.0])
                .with_bold(),
        );
        font_builder.draw(
            FontText::new("你好，上海加油", point2(10.0, 45.0), 16.0)
                .with_color([1.0, 0.0, 1.0, 1.0]),
        );

        font_builder.draw(
            FontText::new("明天会更好", point2(10.0, 100.0), 60.0).with_color([0.0, 1.0, 0.0, 1.0]),
        );
        font_builder.draw(
            FontText::new("天气真不错！", point2(10.0, 130.0), 60.0)
                .with_color([0.0, 0.0, 1.0, 1.0]),
        );

        font_builder.draw(
            FontText::new("像一朵盛开的鲜花，永不凋谢！", point2(10.0, 190.0), 50.0)
                .with_color([1.0, 0.0, 0.0, 1.0]),
        );

        font_builder.draw(
            FontText::new(
                "像一朵盛开的鲜花，永不凋 谢！\n  hello",
                point2(10.0, 250.0),
                50.0,
            )
            .with_color([1.0, 1.0, 0.0, 1.0])
            .with_bold(),
        );

        Self {
            surface,
            device,
            queue,
            config,
            size,
            globals,
            globals_bind_group_layout,
            font_builder,
            depth_texture,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.globals
                .update_window_size(new_size.width, new_size.height);
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

            self.font_builder
                .render(&self.device, &self.queue, &mut render_pass, &self.globals);

            // render_pass.set_pipeline(&self.render_pipeline);

            // render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
            // render_pass.set_bind_group(1, &self.font_bind_group, &[]);

            // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // render_pass.draw_indexed(0..self.index_len, 0, 0..1);
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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct FontInstance {
    // 顶点起始坐标
    pos_start: [f32; 2],
    // 顶点结束坐标
    pos_end: [f32; 2],
    // 纹理起始坐标
    tex_start: [f32; 2],
    // 纹理起始坐标
    tex_end: [f32; 2],
    // 纹理颜色
    tex_color: [f32; 4],
    // z坐标
    z_index: f32,
    // font 粗体
    bold: f32,
}

impl FontInstance {
    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32x4,
        5 => Float32,
        6 => Float32,
    ];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::VERTEX_ATTRIBUTES,
        }
    }
}

struct FontText {
    pub text: String,
    pub pos: Point,
    pub color: [f32; 4],
    pub size: f32,
    pub bold: bool,
}

impl FontText {
    pub fn new(text: impl ToString, pos: Point, size: f32) -> Self {
        Self {
            text: text.to_string(),
            pos,
            color: [1.0, 1.0, 1.0, 1.0],
            size,
            bold: false,
        }
    }

    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn with_bold(mut self) -> Self {
        self.bold = true;
        self
    }
}

struct FontBuilder {
    fonts: Vec<fontdue::Font>,
    // draw_fonts: Vec<fontdue::layout::Layout<(Point, [f32; 4])>>,
    draw_fonts: Vec<FontText>,
    dirty: bool,

    /// 字体纹理地图
    altas: AtlasAllocator,
    cache_chars: HashSet<char>,

    vertex_buffer: wgpu::Buffer,
    font_instance_num: u32,
    /// 字体纹理
    texture: Texture,
    font_bind_group: wgpu::BindGroup,

    globals_bind_group: wgpu::BindGroup,
    globals_buffer: wgpu::Buffer,

    pipeline: wgpu::RenderPipeline,
}

impl FontBuilder {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        globals_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let altas = AtlasAllocator::new(size2(1024, 1024));
        let altas_size = altas.size();

        let data: &[u8] = include_bytes!("../../fonts/DroidSansFallbackFull.ttf");
        log::info!("ttf font data len: {:?}", data.len());

        let start = Instant::now();
        let fonts =
            vec![fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).unwrap()];
        log::info!("elapsed: {:?}", &start.elapsed());

        // 创建 字体Shader
        let font_shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Font Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("test_wgpu_fontdue.wgsl").into()),
        });

        // 创建 字体纹理
        let font_texture = Texture::new(
            device,
            altas_size.width as u32,
            altas_size.height as u32,
            format,
            Some("font altas texture"),
        );

        // 创建 字体绑定组布局
        let font_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("font_bind_group_layout"),
            });

        // 创建 字体绑定组
        let font_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &font_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&font_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&font_texture.sampler),
                },
            ],
            label: Some("font_bind_group"),
        });

        // 全局 Buffer
        let globals_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Globals Buffer"),
            size: std::mem::size_of::<[f32; 16]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Globals bind group"),
            layout: &globals_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(globals_buffer.as_entire_buffer_binding()),
            }],
        });

        // 创建 字体渲染的 管道布局
        let font_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Font Pipeline Layout"),
            bind_group_layouts: &[globals_bind_group_layout, &font_bind_group_layout],
            push_constant_ranges: &[],
        });

        // 创建 字体渲染的 管道
        let font_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Font Pipeline"),
            layout: Some(&font_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &font_shader,
                entry_point: "vs_main",
                buffers: &[FontInstance::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &font_shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // 创建字符缓冲区
        let font_instance_num = 0u32;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::VERTEX,
            size: font_instance_num as u64,
        });

        Self {
            fonts,
            draw_fonts: Vec::new(),
            altas,
            cache_chars: HashSet::new(),
            globals_bind_group,
            globals_buffer,
            texture: font_texture,
            font_bind_group,
            pipeline: font_pipeline,
            vertex_buffer,
            font_instance_num,
            dirty: false,
        }
    }

    pub fn render<'a>(
        &'a mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass<'a>,
        globals: &Globals,
    ) {
        // if !self.dirty {
        //     return;
        // }

        // 更新字体buffer
        if !self.update_buffer(device, queue, globals) {
            // return;
        }

        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
        render_pass.set_bind_group(1, &self.font_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        // render_pass.draw(0..VERTICES.len() as u32, 0..1);
        render_pass.draw(0..4, 0..self.font_instance_num);
    }

    pub fn update_buffer(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        globals: &Globals,
    ) -> bool {
        let mut updated = false;

        queue.write_buffer(
            &self.globals_buffer,
            0,
            bytemuck::cast_slice(&[globals.transform]),
        );

        let mut font_instances = Vec::<FontInstance>::new();
        let altas_size = self.altas.size();

        for text in self.draw_fonts.iter() {
            let mut font_layout =
                Layout::<(Point, [f32; 4], bool)>::new(CoordinateSystem::PositiveYDown);
            font_layout.append(
                &self.fonts,
                &TextStyle::with_user_data(
                    text.text.as_str(),
                    text.size,
                    0,
                    (text.pos, text.color, text.bold),
                ),
            );

            // 取出字形
            let glyphs = font_layout.glyphs();

            for glyph in glyphs.iter() {
                if self.cache_chars.contains(&glyph.parent) {
                    continue;
                }
                let (pos, color, bold) = glyph.user_data;

                let font = &self.fonts[glyph.font_index];
                let (metrics, bitmap) = font.rasterize_indexed(glyph.key.glyph_index, glyph.key.px);

                let allocate = self
                    .altas
                    .allocate(size2(metrics.width as i32, metrics.height as i32));

                if let Some(allocate) = allocate {
                    let rect = allocate.rectangle;
                    let tex_start = point2(
                        rect.min.x as f32 / altas_size.width as f32,
                        rect.min.y as f32 / altas_size.height as f32,
                    );
                    let tex_end = point2(
                        rect.max.x as f32 / altas_size.width as f32,
                        rect.max.y as f32 / altas_size.height as f32,
                    );

                    let pos_start = point2(pos.x + glyph.x, pos.y + glyph.y);
                    let pos_end = point2(
                        pos_start.x + glyph.width as f32,
                        pos_start.y + glyph.height as f32,
                    );

                    let font = FontInstance {
                        pos_start: pos_start.to_array(),
                        pos_end: pos_end.to_array(),
                        tex_start: tex_start.to_array(),
                        tex_end: tex_end.to_array(),
                        tex_color: color,
                        z_index: 0.0,
                        bold: if bold { 1.0 } else { 0.0 },
                    };

                    font_instances.push(font);

                    let buf: ImageBuffer<Luma<u8>, _> =
                        ImageBuffer::from_raw(metrics.width as u32, metrics.height as u32, bitmap)
                            .unwrap();
                    let image = DynamicImage::ImageLuma8(buf);

                    let mut rgba = image.to_rgba8();

                    // 去除黑色背景
                    for color in rgba.pixels_mut() {
                        if *color == image::Rgba([0x00, 0x00, 0x00, 0xff]) {
                            color[3] = 0x00;
                        }
                    }

                    self.texture.write_texture(
                        &queue,
                        &rgba,
                        [rect.min.x as u32, rect.min.y as u32],
                        rect.width() as u32,
                        rect.height() as u32,
                    );
                }
            }
        }

        let font_instance_num = font_instances.len() as u32;

        if font_instance_num != 0 {
            let vertices_data = bytemuck::cast_slice(&font_instances);
            if font_instance_num == self.font_instance_num {
                queue.write_buffer(&self.vertex_buffer, 0, vertices_data);
            } else {
                self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: vertices_data,
                    usage: wgpu::BufferUsages::VERTEX,
                });
                self.font_instance_num = font_instance_num;
            }
            updated = true;
        }

        // 清理数据
        self.clear();

        updated
    }

    pub fn draw(&mut self, text: FontText) {
        self.draw_fonts.push(text);
        self.dirty = true;
    }

    fn clear(&mut self) {
        self.draw_fonts.clear();
        self.dirty = false;
    }
}

struct FontCache {}

const WINDOW_SIZE: PhysicalSize<u32> = PhysicalSize::new(1000, 800);

fn main() {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("test_wgpu_fontdue")
        .with_inner_size(WINDOW_SIZE)
        .build(&event_loop)
        .unwrap();

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
