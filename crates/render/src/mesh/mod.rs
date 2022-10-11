//!
//! Mesh loading and processing module
//!
pub mod material;
pub mod pbr;

use bytemuck::{Pod, Zeroable};
use wgpu::Device;

use super::prelude::{IndexBuffer, VertexBuffer, VertexDesc};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct MeshVertex {
    pub position: [f32; 3],
    pub texcoords: [f32; 2],
    pub normal: [f32; 3],
    // pub tangent: [f32; 4],
    // pub color: [f32; 4],
}

impl VertexDesc for MeshVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    // position
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    // texcoord
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    // normal
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                },
                // wgpu::VertexAttribute {
                //     // tangent
                //     format: wgpu::VertexFormat::Float32x4,
                //     offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                //     shader_location: 3,
                // },
                // wgpu::VertexAttribute {
                //     // color
                //     format: wgpu::VertexFormat::Float32x4,
                //     offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                //     shader_location: 4,
                // },
            ],
        }
    }
}

pub struct Mesh {
    verticies: Vec<MeshVertex>,
    // TODO: decide if it's always a u32
    indicies: Option<Vec<u32>>,
}

impl Mesh {
    pub fn new(verticies: Vec<MeshVertex>, indicies: Option<Vec<u32>>) -> Self {
        Self {
            verticies,
            indicies,
        }
    }

    pub fn into_gpu(&self, device: &Device) -> GpuMesh {
        let vertex_buffer = VertexBuffer::new(device, &self.verticies, Some("Vertex buffer"));
        let index_buffer = self
            .indicies
            .as_ref()
            .map(|i| IndexBuffer::new(device, &i, Some("Index buffer")));

        GpuMesh {
            vertex_buffer,
            index_buffer,
        }
    }
}

pub struct GpuMesh {
    pub vertex_buffer: VertexBuffer<MeshVertex>,
    pub index_buffer: Option<IndexBuffer<u32>>,
}

impl GpuMesh {
    pub fn new(
        vertex_buffer: VertexBuffer<MeshVertex>,
        index_buffer: Option<IndexBuffer<u32>>,
    ) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}
