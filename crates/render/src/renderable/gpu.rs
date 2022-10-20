use wgpu::{Device, Queue};

pub trait IntoGpu {
    type Item;

    fn into_gpu(&self, device: &Device, queue: &Queue) -> Self::Item;
}
