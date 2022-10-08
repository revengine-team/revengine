use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;

use wgpu::Device;
use wgpu::ShaderModule;
use wgpu::ShaderStages;

pub struct Shader {
    stage: ShaderStages,
    shader: ShaderModule,
}

impl Shader {
    pub fn from_string (
        device: &Device,
        contents: impl Into<Cow<'static, str>>,
        stage: ShaderStages,
        label: Option<&str>,
    ) -> Self {
        Self {
            shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label,
                source: wgpu::ShaderSource::Wgsl(contents.into()),
            }),
            stage,
        }
    }

    pub fn stage(&self) -> ShaderStages {
        self.stage
    }
}

impl Deref for Shader {
    type Target = ShaderModule;

    fn deref(&self) -> &Self::Target {
        &self.shader
    }
}
