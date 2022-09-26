pub mod macros;
pub mod vertex;

use self::vertex::{Vertex, VertexComponent};
use super::{Buffer, BufferUsage, MapBufferUsage};
use gl;
use std::ffi::c_void;

pub struct VertexBuffer {
    id: u32,
    stride: usize,
    components: Vec<VertexComponent>,
}

impl Buffer for VertexBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl MapBufferUsage for VertexBuffer {}

impl VertexBuffer {
    pub fn new<T>(vertices: &Vec<T>, usage: BufferUsage) -> VertexBuffer
    where
        T: Vertex,
    {
        let mut id: u32 = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (T::stride() * vertices.len()) as gl::types::GLintptr,
                &vertices[0] as *const T as *const c_void,
                VertexBuffer::map_usage_to_gl_usage(usage),
            );
        }

        VertexBuffer {
            id: id,
            stride: T::stride(),
            components: T::components(),
        }
    }

    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn components(&self) -> Vec<VertexComponent> {
        self.components.clone()
    }
}
