use bytemuck::{Pod, Zeroable};
use glam::Vec3;
use wgpu::BufferUsages;

use crate::{
    bind_group_builder::LayoutBuilder,
    prelude::{AsBindGroup, BindGroupBuilder, Buffer},
    renderable::gpu::IntoGpu,
};

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SpotLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub cut_off: f32,
    pub outer_cut_off: f32,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Default)]
pub struct SpotLightWGPU {
    position: [f32; 4],
    direction: [f32; 4],
    diffuse: [f32; 4],
    specular: [f32; 4],
    cut_off: f32,
    outer_cut_off: f32,
    constant: f32,
    linear: f32,
    quadratic: f32,
    _padding: [f32; 3],
}

impl Into<SpotLightWGPU> for SpotLight {
    fn into(self) -> SpotLightWGPU {
        SpotLightWGPU {
            position: self.position.extend(1.0).to_array(),
            direction: self.direction.extend(1.0).to_array(),
            cut_off: self.cut_off,
            outer_cut_off: self.outer_cut_off,
            diffuse: self.diffuse.extend(1.0).to_array(),
            specular: self.specular.extend(1.0).to_array(),
            constant: self.constant,
            linear: self.linear,
            quadratic: self.quadratic,
            _padding: [2.0, 2.0, 8.0],
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct DirLight {
    pub direction: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub strength: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Default)]
pub struct DirLightWGPU {
    pub direction: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub strength: f32,
    pub _padding: [f32; 3],
}

impl Into<DirLightWGPU> for DirLight {
    fn into(self) -> DirLightWGPU {
        DirLightWGPU {
            direction: self.direction.extend(1.0).to_array(),
            diffuse: self.diffuse.extend(1.0).to_array(),
            specular: self.specular.extend(1.0).to_array(),
            strength: self.strength,
            _padding: [2.0, 2.0, 8.0],
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct PointLight {
    pub position: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub strength: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Default)]
pub struct PointLightWGPU {
    pub position: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub strength: f32,
}

impl Into<PointLightWGPU> for PointLight {
    fn into(self) -> PointLightWGPU {
        PointLightWGPU {
            position: self.position.extend(1.0).to_array(),
            diffuse: self.diffuse.extend(1.0).to_array(),
            specular: self.specular.extend(1.0).to_array(),
            constant: self.constant,
            linear: self.linear,
            quadratic: self.quadratic,
            strength: self.strength,
        }
    }
}

pub struct Lights {
    pub dir_lights: [DirLightWGPU; 2],
    pub spot_lights: [SpotLightWGPU; 8],
    pub point_lights: [PointLightWGPU; 16],
}

pub struct LightsBindGroup {
    pub bind_group: wgpu::BindGroup,
}

impl AsBindGroup for Lights {
    fn bind_group(
        &self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        let dir_buffer = Buffer::new(
            device,
            BufferUsages::UNIFORM,
            &self.dir_lights,
            Some("DirLights Bind Group"),
        );

        let spot_buffer = Buffer::new(
            device,
            BufferUsages::UNIFORM,
            &self.spot_lights,
            Some("SpotLights Bind Group"),
        );

        let point_buffer = Buffer::new(
            device,
            BufferUsages::UNIFORM,
            &self.point_lights,
            Some("PointLights Bind Group"),
        );

        BindGroupBuilder::new()
            .buffer::<DirLight>(&dir_buffer, 0..2)
            .buffer::<SpotLight>(&spot_buffer, 0..8)
            .buffer::<PointLight>(&point_buffer, 0..16)
            .build(device, layout, Some("Lights Bind Group"))
    }

    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        LayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStages::FRAGMENT, false)
            .uniform_buffer(wgpu::ShaderStages::FRAGMENT, false)
            .uniform_buffer(wgpu::ShaderStages::FRAGMENT, false)
            .build(device, Some("Lights bind layout"))
    }
}

impl IntoGpu for Lights {
    type Item = LightsBindGroup;

    fn into_gpu(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Self::Item {
        let layout = Self::bind_group_layout(device);

        Self::Item {
            bind_group: self.bind_group(device, queue, &layout),
        }
    }
}
