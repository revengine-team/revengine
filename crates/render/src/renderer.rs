/// Represents context for render pass.
pub struct RenderingContext<'a> {
    /// Handle to the rendering device
    pub device: &'a wgpu::Device,
    /// Queue stores all commands that will be executed
    pub queue: &'a mut wgpu::Queue,
    /// Target to which result of rendering will be written to
    pub output: &'a wgpu::TextureView,
}

impl<'a> RenderingContext<'a> {
    /// Creates empty [`wgpu::CommandEncoder`].
    ///
    /// Encoder can record render passes and transfer operations between something like [`crate::revengine_wgpu::prelude::Buffer`].
    pub fn create_encoder(&mut self, label: &str) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some(label) })
    }

    /// Submit series of commands for execution.
    pub fn submit(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(Some(encoder.finish()));
    }
}
