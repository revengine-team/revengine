//! Wrappers around Buffer for storing Indicies
use super::Buffer;
use bytemuck::Pod;
use std::ops::Deref;

/// Wraper around Buffer to store Indicies
pub struct IndexBuffer<T: Pod> {
    len: usize,
    buffer: Buffer<T>,
}

impl<T: Pod> IndexBuffer<T> {
    /// Creates a new [`IndexBuffer<T>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use render::revengine_wgpu::buffers::vertex::IndexBuffer;
    ///
    /// let result = IndexBuffer::new(&device, &INDEX_DATA, Some("Index buffer"));
    /// ```
    pub fn new(device: &wgpu::Device, indicies: &[T], label: Option<&str>) -> Self {
        let index_buffer = Buffer::new(device, wgpu::BufferUsages::INDEX, indicies, label);

        Self {
            len: indicies.len(),
            buffer: index_buffer,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Pod> Deref for IndexBuffer<T> {
    type Target = wgpu::Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
