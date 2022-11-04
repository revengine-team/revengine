//!
//! Wgpu backend for Recengine
//!
pub mod bind_group_builder;
pub mod buffers;
pub mod camera;
pub mod light;
pub mod material;
pub mod mesh;
pub mod render_pass;
pub mod render_pipleine_builder;
pub mod renderable;
pub mod renderer;
pub mod shader;
pub mod texture;
pub mod transform;

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
    pub use super::camera::Camera;
    pub use super::material::{AsMaterial, AsPipeline, BaseMaterial, Material};
    pub use super::mesh::{Mesh, MeshVertex};
    pub use super::render_pass::{
        Builder as RenderPassBuilder, ColorAttachmentDescriptorBuilder,
        DepthStencilAttachmentDescriptorBuilder,
    };
    pub use super::render_pipleine_builder::RenderPipelineBuilder;
    pub use super::renderable::Renderable;
    pub use super::renderer::RenderingContext;
    pub use super::shader::Shader;
    pub use super::texture::Texture;
    pub use super::transform::Transform;
    pub use glam::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
    pub use wgpu::{Device, Extent3d, Queue, TextureFormat};
}
