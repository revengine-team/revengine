use glam::Vec4;

use crate::{
    bind_group_builder::LayoutBuilder,
    prelude::{AsBindGroup, Texture},
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
        todo!()
    }
}

impl AsPipeline for PbrMaterial {
    fn pipeline(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
    ) -> wgpu::RenderPipeline {
        todo!()
    }
}

impl AsBindGroup for PbrMaterial {
    fn bind_group(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        todo!()
    }

    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        LayoutBuilder::new();
        todo!()
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
        device: &wgpu::Device,
        encoder: &'a mut wgpu::CommandEncoder,
        rp_desc: &'a wgpu::RenderPassDescriptor,
        transform_data: &'a crate::transform::TransformBindGroup,
        camera_data: &'a crate::camera::CameraBindGroup,
        // lighting_data: &LightingBindGroup,
    ) -> wgpu::RenderPass<'a> {
        todo!()
    }
}
