use glam::Vec4;

use crate::{
    bind_group_builder::LayoutBuilder,
    camera::CameraBindGroup,
    light::{Lights, LightsBindGroup},
    prelude::{
        AsBindGroup, BindGroupBuilder, Buffer, Camera, MeshVertex, RenderPipelineBuilder, Shader,
        Texture, Transform, VertexDesc,
    },
    texture::TextureDefaults,
    transform::TransformBindGroup,
};

use super::{AsMaterial, AsPipeline, Material};

pub type Color = Vec4;

pub struct PbrMaterial {
    pub base_color: Color,
    pub base_texture: Option<Texture>,
    pub emissive: Color,
    pub roughness: f32,
    pub metalic: f32,
    pub normal_map: Option<Texture>,
}

impl PbrMaterial {
    pub fn new(
        base_color: Color,
        base_texture: Option<Texture>,
        emissive: Color,
        roughness: f32,
        metalic: f32,
        normal_map: Option<Texture>,
    ) -> Self {
        Self {
            base_color,
            base_texture,
            emissive,
            roughness,
            metalic,
            normal_map,
        }
    }
}

pub struct PbrMaterialGpu {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

impl AsMaterial for PbrMaterial {
    fn material(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Box<dyn Material + Send + Sync> {
        let transform_layout = Transform::bind_group_layout(device);
        let camera_layout = Camera::bind_group_layout(device);
        let bind_layout = Self::bind_group_layout(device);
        let lights_layout = Lights::bind_group_layout(device);
        let bind_group = self.bind_group(device, queue, &bind_layout);

        let pipeline_layout = Self::pipeline_layout(
            device,
            &[
                &bind_layout,
                &transform_layout,
                &camera_layout,
                &lights_layout,
            ],
            Some("PbrMaterial pipeline layout"),
        );
        let pipeline = self.pipeline(device, &pipeline_layout);

        Box::new(PbrMaterialGpu {
            pipeline,
            bind_group,
        })
    }
}

impl AsPipeline for PbrMaterial {
    fn pipeline(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
    ) -> wgpu::RenderPipeline {
        let v_shader = Shader::from_string(
            device,
            include_str!("assets/shaders/pbr_material_vertex.wgsl"),
            wgpu::ShaderStages::VERTEX,
            Some("PbrMaterial vertex shader"),
        );

        let f_shader = Shader::from_string(
            device,
            include_str!("assets/shaders/pbr_material_fragment.wgsl"),
            wgpu::ShaderStages::FRAGMENT,
            Some("PbrMaterial fragment shader"),
        );

        RenderPipelineBuilder::from_layout(&layout, &v_shader)
            .color_format(wgpu::TextureFormat::Rgba8UnormSrgb)
            .add_vertex_buffer_layout(MeshVertex::desc())
            .fragment_shader(&f_shader)
            .cull_mode(Some(wgpu::Face::Back))
            .multisample(wgpu::MultisampleState::default())
            .build(&device, Some("Base material pipeline"))
    }
}

impl AsBindGroup for PbrMaterial {
    fn bind_group(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        let color_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.base_color],
            Some("PbrMaterial base_color uniform"),
        );
        let emissive_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.emissive],
            Some("PbrMaterial emissive uniform"),
        );
        let roughness_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.roughness],
            Some("PbrMaterial roughness uniform"),
        );
        let metalic_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.metalic],
            Some("PbrMaterial metalic uniform"),
        );

        // FIXME
        let texture_binding = TextureDefaults::base_color().into_texture(device, queue);
        let texture = match &self.base_texture {
            Some(texture) => texture,
            None => &texture_binding,
        };

        // FIXME
        let normal_binding = TextureDefaults::base_color().into_texture(device, queue);
        let normal_texture = match &self.normal_map {
            Some(texture) => texture,
            None => &normal_binding,
        };

        BindGroupBuilder::new()
            .buffer::<Vec4>(&color_buffer, 0..1)
            .buffer::<Vec4>(&emissive_buffer, 0..1)
            .buffer::<f32>(&roughness_buffer, 0..1)
            .buffer::<f32>(&metalic_buffer, 0..1)
            .texture_view(&texture.view)
            .sampler(&texture.sampler)
            .texture_view(&normal_texture.view)
            .sampler(&normal_texture.sampler)
            .build(device, layout, Some("BaseMaterial bind group"))
    }

    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        LayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStages::FRAGMENT, false)
            .uniform_buffer(wgpu::ShaderStages::FRAGMENT, false)
            .uniform_buffer(wgpu::ShaderStages::FRAGMENT, false)
            .uniform_buffer(wgpu::ShaderStages::FRAGMENT, false)
            .texture(
                wgpu::ShaderStages::FRAGMENT,
                false,
                wgpu::TextureViewDimension::D2,
                wgpu::TextureSampleType::Float { filterable: true },
            )
            .filtering_sampler(wgpu::ShaderStages::FRAGMENT)
            .texture(
                wgpu::ShaderStages::FRAGMENT,
                false,
                wgpu::TextureViewDimension::D2,
                wgpu::TextureSampleType::Float { filterable: true },
            )
            .filtering_sampler(wgpu::ShaderStages::FRAGMENT)
            .build(device, Some("PbrMaterial bind layout"))
    }
}

impl Default for PbrMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::new(1.0, 1.0, 1.0, 1.0),
            base_texture: None,
            emissive: Color::new(0.0, 0.0, 0.0, 0.0),
            roughness: 0.01,
            metalic: 0.01,
            normal_map: None,
        }
    }
}

impl Material for PbrMaterialGpu {
    fn begin_render_pass<'a>(
        &'a self,
        _device: &wgpu::Device,
        encoder: &'a mut wgpu::CommandEncoder,
        rp_desc: &'a wgpu::RenderPassDescriptor,
        transform_data: &'a TransformBindGroup,
        camera_data: &'a CameraBindGroup,
        lights_data: &'a LightsBindGroup,
    ) -> wgpu::RenderPass<'a> {
        let mut rend_pass = encoder.begin_render_pass(rp_desc);

        rend_pass.set_pipeline(&self.pipeline);
        rend_pass.set_bind_group(0, &self.bind_group, &[]);
        rend_pass.set_bind_group(1, &transform_data.bind_group, &[]);
        rend_pass.set_bind_group(2, &camera_data.bind_group, &[]);
        rend_pass.set_bind_group(3, &lights_data.bind_group, &[]);

        rend_pass
    }
}
