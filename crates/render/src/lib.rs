//!
//! Wgpu backend for Recengine
//!
pub mod bind_group_builder;
pub mod buffers;
pub mod mesh;
pub mod render_pass;
pub mod render_pipleine_builder;
pub mod renderer;
pub mod shader;
pub mod texture;

pub mod prelude {

    //! Convinient re-export of common members

    pub use super::bind_group_builder::{Builder as BindGroupBuilder, LayoutBuilder};
    pub use super::buffers::{
        index::IndexBuffer,
        uniform::{AsBindGroup, UniformBuffer},
        vertex::VertexBuffer,
        vertices::Vertex as VertexDesc,
        Buffer,
    };
    pub use super::mesh::{material::BaseMaterial, Mesh, MeshVertex};
    pub use super::render_pass::{
        Builder as RenderPassBuilder, ColorAttachmentDescriptorBuilder,
        DepthStencilAttachmentDescriptorBuilder,
    };
    pub use super::render_pipleine_builder::RenderPipelineBuilder;
    pub use super::renderer::{Renderable, RenderingContext};
    pub use super::shader::Shader;
    pub use super::texture::Texture;
}
