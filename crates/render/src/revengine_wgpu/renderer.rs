pub trait Renderable {
    fn should_render(&self, _context: &RenderingContext) -> bool {
        true
    }
    fn update(&mut self, context: &mut RenderingContext);
    fn render(&mut self, context: &mut RenderingContext);
}

pub struct RenderingContext<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a mut wgpu::Queue,
    pub output: &'a wgpu::TextureView,
}

impl<'a> RenderingContext<'a> {
    pub fn create_encoder(&mut self, label: &str) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some(label) })
    }

    pub fn submit(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(Some(encoder.finish()));
    }
}
