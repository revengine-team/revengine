use bytemuck::{Pod, Zeroable};
use std::borrow::Cow;
use wgpu::{util::DeviceExt, vertex_attr_array, Device};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use render::revengine_wgpu::prelude::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
}

impl VertexDesc for Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &vertex_attr_array![0 => Float32x2],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
];

const INDICES: &[u32] = &[0, 1, 2];

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Color {
    color: [f32; 3],
}

struct TriangleObj {
    pipeline: wgpu::RenderPipeline,
    uniforms: UniformBuffer<Color>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    color: Color,
}

impl Renderable for TriangleObj {
    fn update(&mut self, context: &mut RenderingContext) {
        self.uniforms.copy_to_gpu(context.queue, &self.color)
    }

    fn render(&mut self, context: &mut RenderingContext) {
        let mut encoder = context.create_encoder("Triangle Encoder");

        {
            let mut rend_pass = RenderPassBuilder::new()
                .color_attachment(context.output, |col_builer| {
                    col_builer
                        .load_op(wgpu::LoadOp::Clear(wgpu::Color::BLUE))
                        .store_op(true)
                })
                .begin(&mut encoder);

            rend_pass.set_pipeline(&self.pipeline);
            rend_pass.set_bind_group(0, self.uniforms.get_bind_group(), &[]);
            rend_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            rend_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            rend_pass.draw_indexed(0..3, 0, 0..1);
        }

        context.submit(encoder);
    }
}

impl TriangleObj {
    fn new(device: &Device, color: Color) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let uniforms =
            UniformBuffer::<Color>::new(device, wgpu::ShaderStages::FRAGMENT, "Triangle Uniform");

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
            label: Some("Triangle verices"),
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
            label: Some("Triangle indices"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Trianle pipline layout"),
            bind_group_layouts: &[uniforms.get_layout()],
            push_constant_ranges: &[],
        });

        let pipeline = RenderPipelineBuilder::from_layout(&pipeline_layout, &shader)
            .add_vertex_buffer_layout(Vertex::desc())
            .fragment_shader(&shader)
            .color_format(wgpu::TextureFormat::Bgra8UnormSrgb)
            .build(device, Some("Triangle pipeline"));

        Self {
            pipeline,
            uniforms,
            vertex_buffer,
            index_buffer,
            color,
        }
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

    // Create the logical device and command queue
    let (device, mut queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::downlevel_defaults(), // limits: wgpu::Limits::downlevel_webgl2_defaults()
                                                            // .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_format = surface.get_supported_formats(&adapter)[0];

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };

    surface.configure(&device, &config);

    let color = Color {
        color: [1.0, 0.0, 1.0],
    };
    let mut triangle = TriangleObj::new(&device, color);

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

                triangle.update(&mut ctx);
                triangle.render(&mut ctx);

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
