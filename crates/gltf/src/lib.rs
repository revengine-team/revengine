use gltf;
use render::{material::pbr::PbrMaterial, prelude::*};
use std::{iter::zip, path::Path};

#[derive(Debug)]
pub enum GltfError {
    OpenError,
}

#[derive(Debug)]
pub struct GltfModel {
    pub meshes: Vec<Mesh>,
    pub material: PbrMaterial,
}

impl GltfModel {
    pub fn open<P: AsRef<Path>>(
        path: P,
        device: &Device,
        queue: &Queue,
    ) -> Result<Self, GltfError> {
        let Ok((document, buffers, images))= gltf::import(path) else {return Err(GltfError::OpenError)};

        Ok(Self::load(device, queue, &document, &buffers, &images))
    }

    pub fn load(
        device: &Device,
        queue: &Queue,
        gltf: &gltf::Document,
        buffers: &Vec<gltf::buffer::Data>,
        images: &Vec<gltf::image::Data>,
    ) -> Self {
        let mut meshes = Vec::new();

        for mesh in gltf.meshes() {
            meshes.push(Self::create_mesh(mesh, buffers))
        }

        let gltf_mat = gltf.materials().next().unwrap();

        let (base_color, base_texture) =
            match gltf_mat.pbr_metallic_roughness().base_color_texture() {
                Some(texture) => (
                    Vec4::ONE,
                    Some(Self::create_texture(
                        device,
                        queue,
                        &images[texture.texture().index()],
                    )),
                ),
                None => (
                    gltf_mat.pbr_metallic_roughness().base_color_factor().into(),
                    None,
                ),
            };

        let emissive = Vec3::from_slice(&gltf_mat.emissive_factor()).extend(1.0);
        let roughness = gltf_mat.pbr_metallic_roughness().roughness_factor();
        let metalic = gltf_mat.pbr_metallic_roughness().metallic_factor();

        let normal_map = gltf_mat
            .normal_texture()
            .map(|texture| Self::create_texture(device, queue, &images[texture.texture().index()]));

        let pbr_mat = PbrMaterial::new(
            base_color,
            base_texture,
            emissive,
            roughness,
            metalic,
            normal_map,
        );

        Self {
            meshes,
            material: pbr_mat,
        }
    }

    fn create_mesh(mesh: gltf::Mesh, buffers: &Vec<gltf::buffer::Data>) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let pos_iter = reader.read_positions().unwrap();
            let norm_iter = reader.read_normals().unwrap();

            let tex_coord_iter: Box<dyn Iterator<Item = [f32; 2]>> = match reader.read_tex_coords(0)
            {
                Some(tex_coords_iter) => Box::new(tex_coords_iter.into_f32()),
                None => Box::new(std::iter::repeat([0.0; 2])),
            };

            // TODO: calculate tangents

            for ((pos, normal), tex_coord) in zip(zip(pos_iter, norm_iter), tex_coord_iter) {
                vertices.push(MeshVertex {
                    position: pos,
                    texcoords: tex_coord,
                    normal,
                })
            }

            if let Some(iter) = reader.read_indices() {
                for vertex_index in iter.into_u32() {
                    indices.push(vertex_index);
                }
            }
        }
        Mesh::new(vertices, Some(indices))
    }

    fn create_texture(
        device: &Device,
        queue: &Queue,
        image: &gltf::image::Data,
        // TODO: format
        // image_format: wgpu::TextureFormat,
    ) -> Texture {
        let data = match image.format {
            gltf::image::Format::R8G8B8 => {
                let mut data = vec![0; (image.width * image.height * 4) as usize];

                for i in 0..(image.width * image.height) as usize {
                    data[i * 4 + 0] = image.pixels[i * 3 + 0];
                    data[i * 4 + 1] = image.pixels[i * 3 + 1];
                    data[i * 4 + 2] = image.pixels[i * 3 + 2];
                    data[i * 4 + 3] = 255;
                }

                data
            }
            gltf::image::Format::R8G8B8A8 => image.pixels.clone(),
            // TODO
            //gltf::image::Format::B8G8R8 => todo!(),
            //gltf::image::Format::B8G8R8A8 => todo!(),
            _ => {
                panic!("Unsuppoerted gltf image format!");
            }
        };

        let size = Extent3d {
            width: image.width,
            height: image.height,
            depth_or_array_layers: 1,
        };
        dbg!(&size);
        Texture::from_bytes(device, queue, &data, &size, None, None)
    }
}
