use super::{Buffer, BufferUsage, MapBufferUsage};
use gl;
use std::{ffi::c_void, mem};

pub struct ElementsBuffer {
    id: u32,
    indices_size: usize,
}

impl Buffer for ElementsBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for ElementsBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl MapBufferUsage for ElementsBuffer {}

impl ElementsBuffer {
    pub fn new(indices: &Vec<u32>, usage: BufferUsage) -> ElementsBuffer {
        let mut id: u32 = 0;
        let indices_size = indices.len();

        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices_size * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLintptr,
                &indices[0] as *const u32 as *const c_void,
                ElementsBuffer::map_usage_to_gl_usage(usage),
            );
        }

        ElementsBuffer { id, indices_size }
    }

    pub fn indices_size(&self) -> usize {
        self.indices_size
    }
}
