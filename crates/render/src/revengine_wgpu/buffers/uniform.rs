use crate::revengine_wgpu::bind_group_builder;
use wgpu::util::DeviceExt;

pub struct UniformBuffer<T> {
    buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    phantom: std::marker::PhantomData<T>,
}

impl<T> UniformBuffer<T>
where
    T: bytemuck::Pod,
{
    pub fn new(device: &wgpu::Device, visibility: wgpu::ShaderStages, label: &str) -> Self {
        Self::init(device, T::zeroed(), visibility, label)
    }

    pub fn default(device: &wgpu::Device, visibility: wgpu::ShaderStages, label: &str) -> Self
    where
        T: Default,
    {
        Self::init(device, T::default(), visibility, label)
    }

    pub fn init(
        device: &wgpu::Device,
        initial_data: T,
        visibility: wgpu::ShaderStages,
        label: &str,
    ) -> Self {
        let buffer = Self::create_buffer(device, &initial_data, label);

        let bind_group_layout = Self::create_layout(device, visibility, label);

        let bind_group = bind_group_builder::Builder::new()
            .buffer::<T>(&buffer, 0..1)
            .build(device, &bind_group_layout, Some(label));

        Self {
            buffer,
            bind_group_layout,
            bind_group,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn copy_to_gpu(&self, queue: &wgpu::Queue, data: &T) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[*data]));
    }

    pub fn create_layout(
        device: &wgpu::Device,
        visibility: wgpu::ShaderStages,
        label: &str,
    ) -> wgpu::BindGroupLayout {
        bind_group_builder::LayoutBuilder::new()
            .uniform_buffer(visibility, false)
            .build(device, Some(label))
    }

    fn create_buffer(device: &wgpu::Device, data: &T, label: &str) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(&[*data]),
            usage: wgpu::BufferUsages::UNIFORM
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            label: Some(label),
        })
    }
}

pub trait UniformObject {
    fn get_bind_group(&self) -> &wgpu::BindGroup;

    fn get_layout(&self) -> &wgpu::BindGroupLayout;
}

impl<T> UniformObject for UniformBuffer<T> {
    fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}
