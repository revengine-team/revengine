use std::num::NonZeroU32;

use image::DynamicImage;
use wgpu::Extent3d;

/// Represents an image that has been uploaded to the GPU.
pub struct Texture {
    pub tex: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    _size: Extent3d,
    _format: wgpu::TextureFormat,
}

impl Texture {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &DynamicImage,
        filter_method: Option<wgpu::FilterMode>,
        address_mode: Option<wgpu::AddressMode>,
        // TODO: make this return Result
    ) -> Self {
        let format = match image {
            DynamicImage::ImageLuma8(_) => wgpu::TextureFormat::R8Unorm,
            DynamicImage::ImageRgba8(_) => wgpu::TextureFormat::Rgba8UnormSrgb,
            _ => panic!("unsupported format"),
        };

        // TODO: clear this
        let buffer = image.as_flat_samples_u8().unwrap();

        let bytes_per_pixel = u32::from(buffer.layout.channels);

        let size = Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };

        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            buffer.as_slice(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(image.width() * bytes_per_pixel),
                rows_per_image: NonZeroU32::new(image.height()),
            },
            Extent3d {
                width: image.width(),
                height: image.height(),
                depth_or_array_layers: 1,
            },
        );

        let sampler_info = wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: address_mode.unwrap_or(wgpu::AddressMode::ClampToEdge),
            address_mode_v: address_mode.unwrap_or(wgpu::AddressMode::ClampToEdge),
            address_mode_w: address_mode.unwrap_or(wgpu::AddressMode::ClampToEdge),
            mag_filter: filter_method.unwrap_or(wgpu::FilterMode::Nearest),
            min_filter: filter_method.unwrap_or(wgpu::FilterMode::Nearest),
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        };

        let view = tex.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        // TODO: wrap in Result
        Self {
            tex,
            view,
            sampler: device.create_sampler(&sampler_info),
            _size: size,
            _format: format,
        }
    }
}
