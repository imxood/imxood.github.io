use lyon::{
    geom::euclid::rect,
    lyon_tessellation::{
        geometry_builder::simple_builder, BuffersBuilder, FillOptions, FillTessellator, FillVertex,
        FillVertexConstructor, StrokeVertex, StrokeVertexConstructor, VertexBuffers,
    },
    math::Point,
    path::{builder::BorderRadii, traits::PathBuilder, Winding},
};
use std::iter;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

// const VERTICES: &[Vertex] = &[
//     Vertex { position: [-0.0868241, 0.49240386], color: [0.5, 0.0, 0.5] }, // A
//     Vertex { position: [-0.49513406, 0.06958647], color: [0.5, 0.0, 0.5] }, // B
//     Vertex { position: [-0.21918549, -0.44939706], color: [0.5, 0.0, 0.5] }, // C
//     Vertex { position: [0.35966998, -0.3473291], color: [0.5, 0.0, 0.5] }, // D
//     Vertex { position: [0.44147372, 0.2347359], color: [0.5, 0.0, 0.5] }, // E
// ];

// const INDICES: &[u16] = &[
//     0, 1, 4,
//     1, 2, 4,
//     2, 3, 4,
// ];

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.75, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.0, 0.5],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.75, 0.5],
        color: [0.5, 0.0, 0.5],
    },
];

const INDICES: &[u16] = &[1, 0, 2, 1, 2, 3];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2, 1 => Float32x3
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

        // 创建 Shade
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("square.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
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

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_len = INDICES.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        // let VertexBuffers { vertices, indices } = build_lyon(size.width, size.height);

        // let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Vertex Buffer"),
        //     contents: bytemuck::cast_slice(&vertices),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });

        // let index_len = indices.len() as u32;

        // let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Index Buffer"),
        //     contents: bytemuck::cast_slice(&indices),
        //     usage: wgpu::BufferUsages::INDEX,
        // });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
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

pub struct WithId(pub u32);

impl FillVertexConstructor<Vertex> for WithId {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            position: vertex.position().to_array(),
            color: [1.0, 1.0, 0.0],
        }
    }
}

impl StrokeVertexConstructor<Vertex> for WithId {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: vertex.position_on_path().to_array(),
            color: [1.0, 1.0, 0.0],
        }
    }
}

fn build_lyon(width: u32, height: u32) -> VertexBuffers<Vertex, u16> {
    let mut geometry: VertexBuffers<Vertex, u16> = VertexBuffers::new();
    // let mut geometry_builder = simple_builder(&mut geometry);
    let mut geometry_builder = BuffersBuilder::new(&mut geometry, WithId(1 as u32));

    let options = FillOptions::tolerance(0.1);
    let mut tessellator = FillTessellator::new();

    let mut builder = tessellator.builder(&options, &mut geometry_builder);

    builder.add_rectangle(&rect(0.0, 0.0, 600.0, 300.0), Winding::Positive);

    // builder.add_rounded_rectangle(
    //     &rect(0.0, 0.0, 100.0, 50.0),
    //     &BorderRadii {
    //         top_left: 10.0,
    //         top_right: 5.0,
    //         bottom_left: 20.0,
    //         bottom_right: 25.0,
    //     },
    //     Winding::Positive,
    // );

    builder.build().unwrap();

    // The tessellated geometry is ready to be uploaded to the GPU.
    for geo in geometry.vertices.iter_mut() {
        geo.position[0] /= width as f32;
        geo.position[1] /= height as f32;
    }
    println!("width: {:?} height: {:?}", width, height);
    println!("geometry.vertices: {:?}", &geometry.vertices);
    println!("geometry.indices: {:?}", &geometry.indices);
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
