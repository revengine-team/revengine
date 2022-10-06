use std::borrow::Cow;

use bytemuck::{Pod, Zeroable};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use render::revengine_wgpu::{prelude::*, mesh::material::{ObjectGpu, AsMaterial}};

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

    struct Kek {
        min_x: f32,
        max_x: f32,
        min_y: f32,
        max_y: f32,
        min_z: f32,
        max_z: f32,
    }

    let sp = Kek {
        min_x: -1.0,
        max_x: 1.0,
        min_y: -1.0,
        max_y: 1.0,
        min_z: -1.0,
        max_z: 1.0,
    };

    let a = &[
        // Top
        ([sp.min_x, sp.min_y, sp.max_z], [0., 0., 1.0], [0., 0.]),
        ([sp.max_x, sp.min_y, sp.max_z], [0., 0., 1.0], [1.0, 0.]),
        ([sp.max_x, sp.max_y, sp.max_z], [0., 0., 1.0], [1.0, 1.0]),
        ([sp.min_x, sp.max_y, sp.max_z], [0., 0., 1.0], [0., 1.0]),
        // Bottom
        ([sp.min_x, sp.max_y, sp.min_z], [0., 0., -1.0], [1.0, 0.]),
        ([sp.max_x, sp.max_y, sp.min_z], [0., 0., -1.0], [0., 0.]),
        ([sp.max_x, sp.min_y, sp.min_z], [0., 0., -1.0], [0., 1.0]),
        ([sp.min_x, sp.min_y, sp.min_z], [0., 0., -1.0], [1.0, 1.0]),
        // Right
        ([sp.max_x, sp.min_y, sp.min_z], [1.0, 0., 0.], [0., 0.]),
        ([sp.max_x, sp.max_y, sp.min_z], [1.0, 0., 0.], [1.0, 0.]),
        ([sp.max_x, sp.max_y, sp.max_z], [1.0, 0., 0.], [1.0, 1.0]),
        ([sp.max_x, sp.min_y, sp.max_z], [1.0, 0., 0.], [0., 1.0]),
        // Left
        ([sp.min_x, sp.min_y, sp.max_z], [-1.0, 0., 0.], [1.0, 0.]),
        ([sp.min_x, sp.max_y, sp.max_z], [-1.0, 0., 0.], [0., 0.]),
        ([sp.min_x, sp.max_y, sp.min_z], [-1.0, 0., 0.], [0., 1.0]),
        ([sp.min_x, sp.min_y, sp.min_z], [-1.0, 0., 0.], [1.0, 1.0]),
        // Front
        ([sp.max_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [1.0, 0.]),
        ([sp.min_x, sp.max_y, sp.min_z], [0., 1.0, 0.], [0., 0.]),
        ([sp.min_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [0., 1.0]),
        ([sp.max_x, sp.max_y, sp.max_z], [0., 1.0, 0.], [1.0, 1.0]),
        // Back
        ([sp.max_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [0., 0.]),
        ([sp.min_x, sp.min_y, sp.max_z], [0., -1.0, 0.], [1.0, 0.]),
        ([sp.min_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [1.0, 1.0]),
        ([sp.max_x, sp.min_y, sp.min_z], [0., -1.0, 0.], [0., 1.0]),
    ];

    let verticies = a.iter().map(|x| MeshVertex {
        position: x.0,
        texcoords: x.2,
        normal: x.1,
    }).collect::<Vec<MeshVertex>>();

    let anime =  verticies.iter().map( |x| {
        let MeshVertex{
            mut position,
            mut texcoords,
            mut normal,
        } = x;

        position[1] += 4.0;

        MeshVertex{ position, texcoords, normal }
        
    }).collect();

    let indices = vec![
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    // user side
    let mat = BaseMaterial::new([0.0, 1.0, 0.0], MX_REF.mat);
    let mesh = Mesh::new(verticies, Some(indices.clone()));
    let mesh2 = Mesh::new(anime, Some(indices));

    // render extract
    let ayay = mesh.into_gpu(&device);
    let ayay2 = mesh2.into_gpu(&device);
    let mut ы = ObjectGpu::new(vec![ayay, ayay2], mat.material(&device));

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

                ы.render(   &mut ctx);

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
