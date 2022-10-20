pub mod gpu;

use crate::{
    camera::CameraBindGroup,
    mesh::GpuMesh,
    prelude::{Material, RenderingContext},
    transform::TransformBindGroup,
};

pub struct Renderable {
    meshes: Vec<GpuMesh>,
    pub transform: TransformBindGroup,
    pub material: Box<dyn Material + Send + Sync>,
}

impl Renderable {
    pub fn new(
        meshes: Vec<GpuMesh>,
        material: Box<dyn Material + Send + Sync>,
        transform: TransformBindGroup,
    ) -> Self {
        Self {
            meshes,
            material,
            transform,
        }
    }

    pub fn render(
        &self,
        ctx: &mut RenderingContext,
        rp_desc: &wgpu::RenderPassDescriptor,
        camera: &CameraBindGroup,
        // TODO: light
    ) {
        let mut encoder = ctx.create_encoder("Base Obj Encoder");

        {
            let mut rend_pass = self.material.begin_render_pass(
                ctx.device,
                &mut encoder,
                rp_desc,
                &self.transform,
                camera,
            );

            for mesh in &self.meshes {
                mesh.draw(&mut rend_pass);
            }
        }

        ctx.submit(encoder);
    }
}
