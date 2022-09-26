use std::ffi::c_void;
use gl;

use super::{elements::ElementsBuffer, Buffer, vertices::{VertexBuffer}};

pub struct BufferArray {
    id: u32,
    vertex_buffer_count: u32,
    indices_size: usize
}

impl Buffer for BufferArray {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for BufferArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

impl BufferArray {
    pub fn create_for(vertex_buffer: &VertexBuffer, elements_buffer: &ElementsBuffer) -> BufferArray {
        let mut buffer_array = BufferArray::new();

        buffer_array.use_elements_buffer(elements_buffer);
        buffer_array.add_vertex_buffer(vertex_buffer);

        buffer_array
    }

    pub fn indices_size(&self) -> usize {
        self.indices_size
    }

    fn use_elements_buffer(&mut self, elements_buffer: &ElementsBuffer) {
        self.bind();
        elements_buffer.bind();
        self.indices_size = elements_buffer.indices_size();
    }

    fn add_vertex_buffer(&mut self, vertex_buffer: &VertexBuffer) {
        self.bind();
        vertex_buffer.bind();

        for (_, component) in vertex_buffer.components().iter().enumerate() {
            unsafe {
                gl::EnableVertexAttribArray(self.vertex_buffer_count);
                gl::VertexAttribPointer(
                    self.vertex_buffer_count,
                    component.size() as gl::types::GLint,
                    component.elements_type() as gl::types::GLuint,
                    gl::FALSE,
                    vertex_buffer.stride() as gl::types::GLint,
                    component.offset() as *const c_void
                );
            }

            self.vertex_buffer_count += 1;
        }   
    }

    fn new() -> BufferArray {
        let mut id: u32 = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        BufferArray { id: id, vertex_buffer_count: 0, indices_size: 0 }
    }
}