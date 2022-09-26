pub trait Vertex {
    fn stride() -> usize;
    fn components() -> Vec<VertexComponent>;
}

#[derive(Clone, Copy)]
pub struct VertexComponent {
    size: usize,
    elements_type: u32,
    offset: usize,
}

impl VertexComponent {
    pub fn new(size: usize, elements_type: u32, offset: usize) -> VertexComponent {
        VertexComponent {
            size,
            elements_type,
            offset,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn elements_type(&self) -> u32 {
        self.elements_type
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}
