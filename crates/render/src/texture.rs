//!
//! Module to ease work with textures
//!
use std::num::NonZeroU32;

use image::DynamicImage;
use wgpu::{
    Device, Extent3d, ImageCopyTexture, ImageDataLayout, Queue, SamplerDescriptor,
    TextureDescriptor, TextureFormat, TextureViewDescriptor,
};

/// Represents an image that has been uploaded to the GPU.
#[derive(Debug)]
pub struct Texture {
    /// Texture on the GPU
    pub tex: wgpu::Texture,
    /// Metadata for texture
    pub view: wgpu::TextureView,
    /// Sampler is, in a very simplified way, a description to shader how to work with texture
    pub sampler: wgpu::Sampler,
    _size: Extent3d,
    _format: wgpu::TextureFormat,
}

impl Texture {
    /// Creates a new [`Texture`].
    ///
    /// # Panics
    ///
    /// Panics if used with unsupported formats.
    /// (Curently supports Rgba8 and Luma8)
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image: &DynamicImage,
        filter_method: Option<wgpu::FilterMode>,
        address_mode: Option<wgpu::AddressMode>,
        // TODO: make this return Result
    ) -> Self {
        // FIXME
        let _format = match image {
            DynamicImage::ImageLuma8(_) => wgpu::TextureFormat::R8Unorm,
            DynamicImage::ImageRgba8(_) => wgpu::TextureFormat::Rgba8UnormSrgb,
            _ => panic!("unsupported format"),
        };

        // TODO: clear this
        let buffer = image.as_flat_samples_u8().unwrap();

        // FIXME
        let _bytes_per_pixel = u32::from(buffer.layout.channels);

        let size = Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };

        Self::from_bytes(
            device,
            queue,
            buffer.as_slice(),
            &size,
            filter_method,
            address_mode,
        )
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[u8],
        size: &Extent3d,
        filter_method: Option<wgpu::FilterMode>,
        address_mode: Option<wgpu::AddressMode>,
    ) -> Self {
        let tex = device.create_texture(&Self::default_descriptor(
            *size,
            // FIXME: format
            TextureFormat::Rgba8UnormSrgb,
        ));

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(size.width * 4),
                rows_per_image: NonZeroU32::new(size.height),
            },
            Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
        );

        let sampler_info = Self::default_sampler_descriptor(address_mode, filter_method);

        let view = tex.create_view(&Self::default_view_descriptor(
            // FIXME: format
            TextureFormat::Rgba8UnormSrgb,
        ));

        // TODO: wrap in Result
        Self {
            tex,
            view,
            sampler: device.create_sampler(&sampler_info),
            _size: *size,
            // FIXME: format
            _format: TextureFormat::Rgba8UnormSrgb,
        }
    }

    pub fn default_descriptor(size: Extent3d, format: TextureFormat) -> TextureDescriptor<'static> {
        wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        }
    }

    pub fn default_view_descriptor(format: TextureFormat) -> TextureViewDescriptor<'static> {
        wgpu::TextureViewDescriptor {
            label: None,
            format: Some(format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        }
    }

    pub fn default_sampler_descriptor(
        address_mode: Option<wgpu::AddressMode>,
        filter_method: Option<wgpu::FilterMode>,
    ) -> SamplerDescriptor<'static> {
        wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: address_mode.unwrap_or(wgpu::AddressMode::ClampToEdge),
            address_mode_v: address_mode.unwrap_or(wgpu::AddressMode::ClampToEdge),
            address_mode_w: address_mode.unwrap_or(wgpu::AddressMode::ClampToEdge),
            mag_filter: filter_method.unwrap_or(wgpu::FilterMode::Nearest),
            min_filter: filter_method.unwrap_or(wgpu::FilterMode::Nearest),
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        }
    }
}

pub struct TextureDefaults {
    pub data: [u8; 4],
}

impl TextureDefaults {
    // TODO: naming
    pub fn into_texture(&self, device: &Device, queue: &Queue) -> Texture {
        let alignment = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let mut pixels = vec![0; alignment];
        for i in 0..alignment {
            pixels[i] = self.data[i % 4];
        }

        let size = Extent3d {
            width: alignment as u32 / 4,
            height: 1,
            depth_or_array_layers: 1,
        };

        let desc = Texture::default_descriptor(size, TextureFormat::Rgba8UnormSrgb);

        let texture = device.create_texture(&desc);

        let dst = ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };

        queue.write_texture(
            dst,
            &pixels,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(alignment as u32),
                rows_per_image: NonZeroU32::new(1),
            },
            size,
        );

        let view = texture.create_view(&Texture::default_view_descriptor(
            TextureFormat::Rgba8UnormSrgb,
        ));

        let sampler = device.create_sampler(&Texture::default_sampler_descriptor(None, None));

        Texture {
            tex: texture,
            view,
            sampler,
            _size: size,
            _format: TextureFormat::Rgba8UnormSrgb,
        }
    }

    pub fn base_color() -> Self {
        Self {
            data: [0xff, 0xff, 0xff, 0xff],
        }
    }

    pub fn normal_map() -> Self {
        Self {
            data: [0x00, 0x00, 0xff, 0x00],
        }
    }

    pub fn emissive_texture() -> Self {
        Self {
            data: [0x00, 0x00, 0x00, 0x00],
        }
    }

    pub fn pbr_texture() -> Self {
        Self {
            data: [0xff, 0x00, 0x00, 0x00],
        }
    }
}
