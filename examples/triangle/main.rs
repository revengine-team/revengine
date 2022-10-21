use bytemuck::{Pod, Zeroable};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use render::{
    material::AsMaterial,
    prelude::*,
    renderable::{gpu::IntoGpu, Renderable},
};

#[repr(C)]
#[derive(Copy, Clone, Zeroable, Pod)]
struct Mat4x4 {
    mat: [f32; 16],
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
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };

    surface.configure(&device, &config);

    #[rustfmt::skip]
    let vertices = &[
        ([-0.5, -0.5, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
        ([0.5, -0.5, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
        ([0.0, 0.7, 0.0], [0.0, 0.0, 1.0], [0.5, 1.0]), 
    ];

    let vertices = vertices
        .iter()
        .map(|x| MeshVertex {
            position: x.0,
            texcoords: x.2,
            normal: x.1,
        })
        .collect::<Vec<MeshVertex>>();

    let indices = vec![0, 1, 2];

    let mat = BaseMaterial::from_color(Vec3::new(1.0, 1.0, 1.0));
    let mesh = Mesh::new(vertices, Some(indices.clone()));
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, -1.0));

    let camera = Camera {
        eye: Vec3::new(0.0, 0.0, 1.0),
        target: Vec3::ZERO,
        ..Default::default()
    };

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

                let transfered_mesh = mesh.into_gpu(&device, &ctx.queue);
                let transfered_mat = mat.material(&device, &ctx.queue);
                let transfered_trans = transform.into_gpu(&device, &ctx.queue);
                let transfered_camera = camera.into_gpu(&device, &ctx.queue);

                let mut obj =
                    Renderable::new(vec![transfered_mesh], transfered_mat, transfered_trans);

                let color_attachment =
                    ColorAttachmentDescriptorBuilder::new(ctx.output).get_descriptor();

                let rp_desc = wgpu::RenderPassDescriptor {
                    label: Some("RP Descriptor"),
                    color_attachments: &[Some(color_attachment)],
                    depth_stencil_attachment: None,
                };

                obj.render(&mut ctx, &rp_desc, &transfered_camera);

                frame.present();
                window.request_redraw();
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
    pollster::block_on(run(event_loop, window));
}
