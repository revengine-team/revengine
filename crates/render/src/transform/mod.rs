use glam::{Mat4, Quat, Vec3};

use crate::{
    prelude::{AsBindGroup, BindGroupBuilder, Buffer, LayoutBuilder},
    renderable::gpu::IntoGpu,
};

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

pub struct TransformBindGroup {
    pub bind_group: wgpu::BindGroup,
}

impl Transform {
    pub const IDENTITY: Self = Transform {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            ..Self::IDENTITY
        }
    }

    pub fn calculate_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}

impl IntoGpu for Transform {
    type Item = TransformBindGroup;

    fn into_gpu(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Self::Item {
        let layout = Transform::bind_group_layout(device);

        let bind_group = self.bind_group(device, queue, &layout);

        TransformBindGroup { bind_group }
    }
}

impl AsBindGroup for Transform {
    fn bind_group(
        &self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        let uniform = Buffer::<Mat4>::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.calculate_matrix()],
            Some("Transform Buffer"),
        );

        BindGroupBuilder::new()
            .buffer::<Mat4>(&uniform, 0..1)
            .build(device, layout, Some("Tranform Bind Group"))
    }

    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        LayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStages::VERTEX_FRAGMENT, false)
            .build(device, Some("Transform BindGroup"))
    }
}
