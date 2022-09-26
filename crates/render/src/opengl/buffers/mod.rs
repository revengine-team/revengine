pub mod buffer_array;
pub mod elements;
pub mod vertices;

use gl;

pub trait Buffer {
    fn bind(&self);
    fn unbind(&self);
}

pub enum BufferUsage {
    Static,
    Dynamic,
    Stream,
}

trait MapBufferUsage {
    fn map_usage_to_gl_usage(usage: BufferUsage) -> gl::types::GLenum {
        match usage {
            BufferUsage::Static => gl::STATIC_DRAW,

            BufferUsage::Dynamic => gl::DYNAMIC_DRAW,

            BufferUsage::Stream => gl::STREAM_DRAW,
        }
    }
}
