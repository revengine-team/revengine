//! Vertex objects is cool

/// Trait to describe Vertex-like object.
pub trait Vertex {
    /// Vertex-like object need to specify its alyout.
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
