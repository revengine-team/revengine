// pub mod pbr;
use crate::{
    camera::CameraBindGroup, prelude::*, texture::TextureDefaults, transform::TransformBindGroup,
};
use glam::Vec3;
use wgpu::{PipelineLayout, RenderPipeline};

pub trait Material {
    fn begin_render_pass<'a>(
        &'a self,
        device: &wgpu::Device,
        encoder: &'a mut wgpu::CommandEncoder,
        rp_desc: &'a wgpu::RenderPassDescriptor,
        transform_data: &'a TransformBindGroup,
        camera_data: &'a CameraBindGroup, // lighting_data: &LightingBindGroup,
    ) -> wgpu::RenderPass<'a>;
}

pub trait AsPipeline {
    fn pipeline_layout(
        device: &wgpu::Device,
        layout: &[&wgpu::BindGroupLayout],
        label: Option<&str>,
    ) -> PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label,
            bind_group_layouts: layout,
            push_constant_ranges: &[],
        })
    }

    fn pipeline(&self, device: &wgpu::Device, layout: &PipelineLayout) -> RenderPipeline;
}

pub trait AsMaterial: AsBindGroup + AsPipeline {
    fn material(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Box<dyn Material + Send + Sync>;
}

// TODO: naming
pub struct BaseMaterial {
    color: Vec3,
    texture: Option<Texture>,
}

impl BaseMaterial {
    pub fn from_color(color: Vec3) -> Self {
        Self {
            color,
            texture: None,
        }
    }

    pub fn new(color: Vec3, texture: Texture) -> Self {
        Self {
            color,
            texture: Some(texture),
        }
    }
}

pub struct BaseMaterialGpu {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

impl AsBindGroup for BaseMaterial {
    fn bind_group(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        let uniform_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.color],
            Some("BaseMaterial color bind group"),
        );

        // FIXME: dont't create texture if it's provided
        let binding = TextureDefaults::base_color().into_texture(device, queue);
        let texture = match &self.texture {
            Some(texture) => &texture,
            None => &binding,
        };

        BindGroupBuilder::new()
            .buffer::<[f32; 3]>(&uniform_buffer, 0..1)
            .texture_view(&texture.view)
            .sampler(&texture.sampler)
            .build(device, layout, Some("BaseMaterial bind group"))
    }

    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        LayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStages::FRAGMENT, false)
            .texture(
                wgpu::ShaderStages::FRAGMENT,
                false,
                wgpu::TextureViewDimension::D2,
                wgpu::TextureSampleType::Float { filterable: true },
            )
            .filtering_sampler(wgpu::ShaderStages::FRAGMENT)
            .build(device, Some("BaseMaterial layout"))
    }
}

impl AsPipeline for BaseMaterial {
    fn pipeline(&self, device: &wgpu::Device, layout: &PipelineLayout) -> RenderPipeline {
        let v_shader = Shader::from_string(
            device,
            include_str!("assets/shaders/base_material_vertex.wgsl"),
            wgpu::ShaderStages::VERTEX,
            Some("BaseMaterial vertex shader"),
        );

        let f_shader = Shader::from_string(
            device,
            include_str!("assets/shaders/base_material_fragment.wgsl"),
            wgpu::ShaderStages::FRAGMENT,
            Some("BaseMaterial fragment shader"),
        );

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
    fn material(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Box<dyn Material + Send + Sync> {
        let transform_layout = Transform::bind_group_layout(device);
        let camera_layout = Camera::bind_group_layout(device);
        let bind_layout = Self::bind_group_layout(device);
        let bind_group = self.bind_group(device, queue, &bind_layout);

        let pipeline_layout = Self::pipeline_layout(
            device,
            &[&bind_layout, &transform_layout, &camera_layout],
            Some("BaseMaterial pipeline layout"),
        );
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
        transform_data: &'a TransformBindGroup,
        camera_data: &'a CameraBindGroup,
        // lighting_data: &LightingBindGroup,
    ) -> wgpu::RenderPass<'a> {
        let mut rend_pass = encoder.begin_render_pass(rp_desc);

        rend_pass.set_pipeline(&self.pipeline);
        rend_pass.set_bind_group(0, &self.bind_group, &[]);
        rend_pass.set_bind_group(1, &transform_data.bind_group, &[]);
        rend_pass.set_bind_group(2, &camera_data.bind_group, &[]);

        rend_pass
    }
}
