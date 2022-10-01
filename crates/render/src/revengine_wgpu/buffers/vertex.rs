use super::vertices::Vertex;
use super::Buffer;
use bytemuck::Pod;
use std::ops::Deref;

pub struct VertexBuffer<T: Vertex + Pod> {
    buffer: Buffer<T>,
}

impl<T: Vertex + Pod> VertexBuffer<T> {
    pub fn new(device: &wgpu::Device, vertices: &[T], label: Option<&str>) -> Self {
        let vertex_buffer = Buffer::new(device, wgpu::BufferUsages::VERTEX, vertices, label);

        Self {
            buffer: vertex_buffer,
        }
    }
}

impl<T: Vertex + Pod> Deref for VertexBuffer<T> {
    type Target = wgpu::Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
