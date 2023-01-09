use crate::context::Component;

use self::{buffer::Buffer, keys::SparseArray};

mod keys;
mod buffer;

pub struct SparseSet {
    sparse: SparseArray,
    dense: Buffer
}

impl SparseSet {
    pub fn new<T: Component>() -> Self {
        SparseSet {
            sparse: SparseArray::new(),
            dense: Buffer::new::<T>()
        }
    }
}