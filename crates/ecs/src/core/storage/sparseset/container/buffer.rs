use std::{alloc::Layout, ptr::NonNull};

pub struct Buffer {
    item_layout: Layout,
    len: usize,
    capacity: usize,
        
    ptr: NonNull<u8>,
}

impl Buffer {
    pub fn new<T>() -> Self {
        Buffer {
            item_layout: std::alloc::Layout::new::<T>(),
            len: 0,
            capacity: 0,

            ptr: std::ptr::NonNull::dangling(),
        }
    }
}