use std::borrow::Cow;

use bytemuck::{Pod, Zeroable};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use render::revengine_wgpu::prelude::*;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    _pos: [f32; 4],
    _tex_coord: [f32; 2],
}

impl VertexDesc for Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 4 * 4,
                    shader_location: 1,
                },
            ],
        }
    }
}

fn vertex(pos: [i8; 3], tc: [i8; 2]) -> Vertex {
    Vertex {
        _pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
        _tex_coord: [tc[0] as f32, tc[1] as f32],
    }
}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let vertex_data = [
        // top (0, 0, 1)
        vertex([-1, -1, 1], [0, 0]),
        vertex([1, -1, 1], [1, 0]),
        vertex([1, 1, 1], [1, 1]),
        vertex([-1, 1, 1], [0, 1]),
        // bottom (0, 0, -1)
        vertex([-1, 1, -1], [1, 0]),
        vertex([1, 1, -1], [0, 0]),
        vertex([1, -1, -1], [0, 1]),
        vertex([-1, -1, -1], [1, 1]),
        // right (1, 0, 0)
        vertex([1, -1, -1], [0, 0]),
        vertex([1, 1, -1], [1, 0]),
        vertex([1, 1, 1], [1, 1]),
        vertex([1, -1, 1], [0, 1]),
        // left (-1, 0, 0)
        vertex([-1, -1, 1], [1, 0]),
        vertex([-1, 1, 1], [0, 0]),
        vertex([-1, 1, -1], [0, 1]),
        vertex([-1, -1, -1], [1, 1]),
        // front (0, 1, 0)
        vertex([1, 1, -1], [1, 0]),
        vertex([-1, 1, -1], [0, 0]),
        vertex([-1, 1, 1], [0, 1]),
        vertex([1, 1, 1], [1, 1]),
        // back (0, -1, 0)
        vertex([1, -1, 1], [0, 0]),
        vertex([-1, -1, 1], [1, 0]),
        vertex([-1, -1, -1], [1, 1]),
        vertex([1, -1, -1], [0, 1]),
    ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data.to_vec(), index_data.to_vec())
}

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
struct Mat4x4 {
    mat: [f32; 16],
}

const MX_REF: Mat4x4 = Mat4x4 {
    mat: [
        1.7342978,
        -0.34566143,
        -0.27681828,
        -0.24913645,
        0.5202893,
        1.1522048,
        0.92272764,
        0.8304548,
        0.0,
        2.0931718,
        -0.55363655,
        -0.4982729,
        0.0,
        0.0,
        5.5786643,
        6.0207977,
    ],
};

struct CubeObj {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniforms: UniformBuffer<Mat4x4>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: Buffer<u16>,
}

impl CubeObj {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let (vertex_data, index_data) = create_vertices();

        let vertex_buf = VertexBuffer::new(device, &vertex_data, Some("Vertex buffer"));

        let index_buf = Buffer::new(
            device,
            wgpu::BufferUsages::INDEX,
            &index_data,
            Some("Index buffer"),
        );

        let diffuse_bytes = include_bytes!("logo.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();

        let texture = Texture::new(device, queue, &diffuse_image, None, None);

        let bind_group_layout = LayoutBuilder::new()
            .texture(
                wgpu::ShaderStages::FRAGMENT,
                false,
                wgpu::TextureViewDimension::D2,
                wgpu::TextureSampleType::Float { filterable: true },
            )
            .filtering_sampler(wgpu::ShaderStages::FRAGMENT)
            .build(&device, Some("Amogus texture"));

        let uniform_buf =
            UniformBuffer::init(device, MX_REF, wgpu::ShaderStages::VERTEX, "Camera matrix");

        let bind_group = BindGroupBuilder::new()
            .texture_view(&texture.view)
            .sampler(&texture.sampler)
            .build(&device, &bind_group_layout, Some("Cube bind group"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cube layout"),
            bind_group_layouts: &[&bind_group_layout, &uniform_buf.get_layout()],
            push_constant_ranges: &[],
        });

        let pipeline = RenderPipelineBuilder::from_layout(&pipeline_layout, &shader)
            .color_format(wgpu::TextureFormat::Bgra8UnormSrgb)
            .add_vertex_buffer_layout(Vertex::desc())
            .fragment_shader(&shader)
            .cull_mode(Some(wgpu::Face::Back))
            .multisample(wgpu::MultisampleState::default())
            .build(&device, Some("Cube pipeline"));

        Self {
            pipeline,
            bind_group,
            uniforms: uniform_buf,
            vertex_buffer: vertex_buf,
            index_buffer: index_buf,
        }
    }
}

impl Renderable for CubeObj {
    fn update(&mut self, context: &mut RenderingContext) {
        self.uniforms.copy_to_gpu(context.queue, &MX_REF);
    }

    fn render(&mut self, context: &mut RenderingContext) {
        let mut encoder = context.create_encoder("Cube Encoder");

        {
            let mut rend_pass = RenderPassBuilder::new()
                .color_attachment(context.output, |col_builer| {
                    col_builer
                        .load_op(wgpu::LoadOp::Clear(wgpu::Color::BLUE))
                        .store_op(true)
                })
                .begin(&mut encoder);

            rend_pass.set_pipeline(&self.pipeline);
            rend_pass.set_bind_group(0, &self.bind_group, &[]);
            rend_pass.set_bind_group(1, &self.uniforms.get_bind_group(), &[]);
            rend_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rend_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            rend_pass.draw_indexed(0..36, 0, 0..1);
            // rend_pass.draw(0..VERTICES.len() as u32, 0..1);
        }

        context.submit(encoder);
    }
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let swapchain_format = surface.get_supported_formats(&adapter)[0];

    // Create the logical device and command queue
    let (device, mut queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };

    surface.configure(&device, &config);

    let mut cube = CubeObj::new(&device, &queue);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Reconfigure the surface with the new size
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
                // On macos the window needs to be redrawn manually after resizing
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut ctx = RenderingContext {
                    device: &device,
                    queue: &mut queue,
                    output: &view,
                };

                cube.update(&mut ctx);
                cube.render(&mut ctx);

                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    // Temporarily avoid srgb formats for the swapchain on the web
    pollster::block_on(run(event_loop, window));
}
