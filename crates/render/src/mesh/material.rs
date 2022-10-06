use wgpu::{
    Color, LoadOp, PipelineLayout, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
};

use super::GpuMesh;
use crate::prelude::*;

pub struct ObjectGpu {
    meshes: Vec<GpuMesh>,
    material: Box<dyn Material>,
}

impl Renderable for ObjectGpu {
    fn update(&mut self, _context: &mut RenderingContext) {}

    fn render(&mut self, context: &mut RenderingContext) {
        let mut encoder = context.create_encoder("Base obj encoder");

        {
            // let mut rend_pass = RenderPassBuilder::new()
            //     .color_attachment(context.output, |col_builer| {
            //         col_builer
            //             .load_op(wgpu::LoadOp::Clear(wgpu::Color::BLUE))
            //             .store_op(true)
            //     })
            //     .begin(&mut encoder);

            // TODO: WHAT THE FUCK IS DESCRIPTOR

            let anime = RenderPassColorAttachment {
                view: context.output,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: LoadOp::Clear(Color::TRANSPARENT),
                    store: true,
                },
            };

            let sex = RenderPassDescriptor {
                label: Some("Base render Pass"),
                color_attachments: &[Some(anime)],
                depth_stencil_attachment: None,
            };

            let mut rend_pass = self
                .material
                .begin_render_pass(context.device, &mut encoder, &sex);

            for mesh in &self.meshes {
                rend_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                if let Some(indicies) = &mesh.index_buffer {
                    rend_pass.set_index_buffer(indicies.slice(..), wgpu::IndexFormat::Uint32);
                    rend_pass.draw_indexed(0..indicies.len() as u32, 0, 0..1);
                } else {
                    rend_pass.draw(0..mesh.vertex_buffer.len() as u32, 0..1);
                }
            }

            // rend_pass.draw(0..VERTICES.len() as u32, 0..1);
        }

        context.submit(encoder);
    }
}

impl ObjectGpu {
    pub fn new(meshes: Vec<GpuMesh>, material: Box<dyn Material>) -> Self {
        Self { meshes, material }
    }
}

pub trait Material {
    fn begin_render_pass<'a>(
        &'a self,
        device: &wgpu::Device,
        encoder: &'a mut wgpu::CommandEncoder,
        rp_desc: &'a wgpu::RenderPassDescriptor,
        // transform_data: &TransformBindGroup,
        // lighting_data: &LightingBindGroup,
    ) -> wgpu::RenderPass<'a>;
}

pub trait AsPipeline {
    fn pipeline_layout(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        label: Option<&str>,
    ) -> PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        })
    }

    fn pipeline(&self, device: &wgpu::Device, layout: &PipelineLayout) -> RenderPipeline;
}

pub trait AsMaterial: AsBindGroup + AsPipeline {
    fn material(&self, device: &wgpu::Device) -> Box<dyn Material>;
}

// TODO: naming
pub struct BaseMaterial {
    color: [f32; 3],
    // TODO: change NOW
    m: [f32; 16],
}

impl BaseMaterial {
    pub fn new(color: [f32; 3], m: [f32; 16]) -> Self {
        Self { color, m }
    }
}

pub struct BaseMaterialGpu {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

impl AsBindGroup for BaseMaterial {
    fn bind_group(&self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        let uniform_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.color],
            Some("BaseMaterial color bind group"),
        );

        let t_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.m],
            Some("BaseMaterial transform bind group"),
        );

        BindGroupBuilder::new()
            .buffer::<[f32; 3]>(&uniform_buffer, 0..1)
            .buffer::<[f32; 16]>(&t_buffer, 0..1)
            .build(device, layout, Some("BaseMaterial bind group"))
    }

    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        LayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStages::FRAGMENT, false)
            .uniform_buffer(wgpu::ShaderStages::VERTEX, false)
            .build(device, Some("BaseMaterial layout"))
    }
}

impl AsPipeline for BaseMaterial {
    fn pipeline(&self, device: &wgpu::Device, layout: &PipelineLayout) -> RenderPipeline {
        let v_shader = Shader::load(
            device,
            "./crates/render/src/mesh/assets/shaders/base_material_vertex.wgsl",
            wgpu::ShaderStages::VERTEX,
            Some("BaseMaterial vertex shader"),
        )
        .expect("Error loading vertex shader");

        let f_shader = Shader::load(
            device,
            "./crates/render/src/mesh/assets/shaders/base_material_fragment.wgsl",
            wgpu::ShaderStages::FRAGMENT,
            Some("BaseMaterial fragment shader"),
        )
        .expect("Error loading fragment shader");

        RenderPipelineBuilder::from_layout(&layout, &v_shader)
            .color_format(wgpu::TextureFormat::Bgra8UnormSrgb)
            .add_vertex_buffer_layout(MeshVertex::desc())
            .fragment_shader(&f_shader)
            .cull_mode(Some(wgpu::Face::Back))
            .multisample(wgpu::MultisampleState::default())
            .build(&device, Some("Base material pipeline"))
    }
}

impl AsMaterial for BaseMaterial {
    fn material(&self, device: &wgpu::Device) -> Box<dyn Material> {
        let bind_layout = Self::bind_group_layout(device);
        let bind_group = self.bind_group(device, &bind_layout);

        let pipeline_layout =
            Self::pipeline_layout(device, &bind_layout, Some("BaseMaterial pipeline layout"));
        let pipeline = self.pipeline(device, &pipeline_layout);

        Box::new(BaseMaterialGpu {
            pipeline,
            bind_group,
        })
    }
}

impl Material for BaseMaterialGpu {
    fn begin_render_pass<'a>(
        &'a self,
        _device: &wgpu::Device,
        encoder: &'a mut wgpu::CommandEncoder,
        rp_desc: &'a wgpu::RenderPassDescriptor,
        // transform_data: &TransformBindGroup,
        // lighting_data: &LightingBindGroup,
    ) -> wgpu::RenderPass<'a> {
        let mut rend_pass = encoder.begin_render_pass(rp_desc);

        rend_pass.set_pipeline(&self.pipeline);
        rend_pass.set_bind_group(0, &self.bind_group, &[]);

        rend_pass
    }
}
