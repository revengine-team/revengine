//!
//! Wrappers around wgpu's Buffer type
//! 
pub mod uniform;
pub mod vertex;
pub mod vertices;

use std::ops::Deref;

use bytemuck::Pod;
use wgpu::util::DeviceExt;

/// Wrapper around `wgpu`s Buffer type
pub struct Buffer<T: Copy + Pod> {
    buf: wgpu::Buffer,
    len: usize,
    phantom_data: std::marker::PhantomData<T>,
}

// TODO: bind group builder
impl<T: Copy + Pod> Buffer<T> {
    /// Creates a new [`Buffer<T>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use render::revengine_wgpu::buffers::Buffer;
    ///
    /// let result = Buffer::new(&device, wgpu::BufferUsages::VERTEX, &VERTEX_DATA, Some("Vertex buffer"));
    /// ```
    pub fn new(
        device: &wgpu::Device,
        usage: wgpu::BufferUsages,
        data: &[T],
        label: Option<&str>,
    ) -> Self {
        let contents = bytemuck::cast_slice(data);

        Self {
            buf: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label,
                contents,
                usage,
            }),
            len: data.len(),
            phantom_data: std::marker::PhantomData,
        }
    }

    /// Returns length of Buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if Buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: Pod> Deref for Buffer<T> {
    type Target = wgpu::Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}
