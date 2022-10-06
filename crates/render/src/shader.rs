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
    pub fn load<T: AsRef<Path>>(
        device: &Device,
        path: T,
        stage: ShaderStages,
        label: Option<&str>,
    ) -> Result<Self, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        Ok(Self {
            shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&contents)),
            }),
            stage,
        })
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
