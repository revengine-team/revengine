use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use render::{
    light::{DirLight, DirLightWGPU, Lights, PointLight, PointLightWGPU, SpotLight, SpotLightWGPU},
    material::{pbr::PbrMaterial, AsMaterial},
    prelude::*,
    renderable::{gpu::IntoGpu, Renderable},
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

    let a = surface.get_supported_formats(&adapter);
    dbg! {&a};
    let swapchain_format = a
        .iter()
        .find(|&&x| x == wgpu::TextureFormat::Rgba8UnormSrgb)
        .unwrap();

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
        format: *swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
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

    let verticies = a
        .iter()
        .map(|x| MeshVertex {
            position: x.0,
            texcoords: x.2,
            normal: x.1,
        })
        .collect::<Vec<MeshVertex>>();

    let indices = vec![
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    // user side
    // let mat = BaseMaterial::from_color(Vec3::new(1.0, 0.0, 0.0));
    let image = image::open("examples/cube/logo.png").unwrap();
    let texture = Texture::new(&device, &queue, &image, None, None);
    // let mat = BaseMaterial::new(Vec3::new(1.0, 0.0, 1.0), texture);
    let mat = PbrMaterial::new(
        Vec4::new(1.0, 1.0, 1.0, 1.0),
        Some(texture),
        Vec4::new(0.0, 0.0, 0.0, 0.0),
        0.1,
        0.2,
        None,
    );
    let mesh = Mesh::new(verticies, Some(indices.clone()));
    let mut transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    let mut camera = Camera {
        eye: Vec3::new(4.0, 4.0, 4.0),
        target: Vec3::ZERO,
        ..Default::default()
    };

    let mut lights = Lights {
        dir_lights: [Default::default(); 2],
        spot_lights: [Default::default(); 8],
        point_lights: [Default::default(); 16],
    };

    lights.point_lights[1] = PointLight {
        position: Vec3::new(1.0, 2.0, 3.0),
        diffuse: Vec3::new(4.0, 5.0, 6.0),
        ..Default::default()
    }
    .into();

    lights.dir_lights[1] = DirLight {
        direction: Vec3::new(1.0, 2.0, 3.0),
        diffuse: Vec3::new(4.0, 5.0, 6.0),
        ..Default::default()
    }
    .into();

    lights.spot_lights[1] = SpotLight {
        position: Vec3::new(1.0, 2.0, 3.0),
        diffuse: Vec3::new(4.0, 5.0, 6.0),
        ..Default::default()
    }
    .into();
    // render extract

    let quat = Quat::from_rotation_y(0.03);
    let mut t: f32 = 1.0;

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

                transform.rotation *= quat;
                camera.fovy = 180.0 * t.sin();

                let transfered_mesh = mesh.into_gpu(&device, &ctx.queue);
                let transfered_mat = mat.material(&device, &ctx.queue);
                let transfered_trans = transform.into_gpu(&device, &ctx.queue);
                let transfered_camera = camera.into_gpu(&device, &ctx.queue);
                let transfered_lights = lights.into_gpu(&device, &ctx.queue);

                let obj = Renderable::new(vec![transfered_mesh], transfered_mat, transfered_trans);

                let color_attachment =
                    ColorAttachmentDescriptorBuilder::new(ctx.output).get_descriptor();

                let rp_desc = wgpu::RenderPassDescriptor {
                    label: Some("RP Descriptor"),
                    color_attachments: &[Some(color_attachment)],
                    depth_stencil_attachment: None,
                };

                obj.render(&mut ctx, &rp_desc, &transfered_camera, &transfered_lights);

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
    // Temporarily avoid srgb formats for the swapchain on the web
    pollster::block_on(run(event_loop, window));
}
