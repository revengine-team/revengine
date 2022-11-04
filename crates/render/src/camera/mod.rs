use glam::{Mat4, Vec3};

use crate::{
    prelude::{AsBindGroup, BindGroupBuilder, Buffer, LayoutBuilder},
    renderable::gpu::IntoGpu,
};

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: Vec3::ZERO,
            target: Vec3::X,
            up: Vec3::Y,
            aspect: 1.0,
            fovy: 90.0_f32.to_radians(),
            znear: 0.1,
            zfar: 100.0,
        }
    }
}

impl Camera {
    fn get_view(&self) -> Mat4 {
        Mat4::look_at_rh(self.eye, self.target, self.up)
    }

    fn get_projection(&self) -> Mat4 {
        Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

impl AsBindGroup for Camera {
    fn bind_group(
        &self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        let view_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.get_view()],
            Some("Camera View Matrix"),
        );
        let projection_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.get_projection()],
            Some("Camera Projection Matrix"),
        );
        let position_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::UNIFORM,
            &[self.eye],
            Some("Camera Position"),
        );
        BindGroupBuilder::new()
            .buffer::<Mat4>(&view_buffer, 0..1)
            .buffer::<Mat4>(&projection_buffer, 0..1)
            .buffer::<Vec3>(&position_buffer, 0..1)
            .build(device, layout, Some("Camera Bind Group"))
    }

    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        LayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStages::VERTEX_FRAGMENT, false)
            .uniform_buffer(wgpu::ShaderStages::VERTEX_FRAGMENT, false)
            .uniform_buffer(wgpu::ShaderStages::VERTEX_FRAGMENT, false)
            .build(device, Some("Camera layout"))
    }
}

pub struct CameraBindGroup {
    pub bind_group: wgpu::BindGroup,
}

impl IntoGpu for Camera {
    type Item = CameraBindGroup;

    fn into_gpu(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Self::Item {
        let layout = Self::bind_group_layout(device);
        CameraBindGroup {
            bind_group: self.bind_group(device, queue, &layout),
        }
    }
}
