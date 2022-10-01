pub mod uniform;
pub mod vertex;
pub mod vertices;

use std::ops::Deref;

use bytemuck::Pod;
use wgpu::util::DeviceExt;

pub struct Buffer<T: Copy + Pod> {
    buf: wgpu::Buffer,
    len: usize,
    phantom_data: std::marker::PhantomData<T>,
}

// TODO: bind group builder
impl<T: Copy + Pod> Buffer<T> {
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

    pub fn len(&self) -> usize {
        self.len
    }

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
