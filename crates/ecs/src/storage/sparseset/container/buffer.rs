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

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn extend(&mut self) {
        (self.len == self.capacity).then(
            || self.grow()
        );

        self.len += 1;
    }

    pub unsafe fn push<T>(&mut self, value: T) {
        (self.len == self.capacity).then(
            || self.grow()
        );

        unsafe {
            *self.borrow_mut_unchecked(self.len) = value;
        }

        self.len += 1;
    }

    pub unsafe fn borrow<T>(&self, index: usize) -> Option<&T> {
        (index < self.len).then_some(
            unsafe {
                self.borrow_unchecked(index)
            }
        )
    }

    pub unsafe fn borrow_mut<T>(&self, index: usize) -> Option<&mut T> {
        (index < self.len).then_some(
            unsafe {
                self.borrow_mut_unchecked(index)
            }
        )
    }

    pub fn remove(&mut self, index: usize) {
        (index < self.len).then(|| unsafe {
            self.remove_unchecked(index)
        }).or_else(|| {
            panic!("the index: {} was out of bounds", index)
        });
    }

    pub fn swap_remove(&mut self, index: usize) {
        (index < self.len).then(|| unsafe {
            self.swap_remove_unchecked(index)
        }).or_else(|| {
            panic!("the index: {} was out of bounds", index)
        });
    }

    pub unsafe fn borrow_unchecked<T>(&self, index: usize) -> &T {
        &*(self.ptr.as_ptr()
                        .add(index * self.item_layout.size())
                        as *const T)
    }
    
    pub unsafe fn borrow_mut_unchecked<T>(&self, index: usize) -> &mut T {
        &mut *(self.ptr.as_ptr()
                        .add(index * self.item_layout.size())
                        as *mut T)
    }

    pub unsafe fn remove_unchecked(&mut self, index: usize) {
        std::ptr::copy(
            self.ptr.as_ptr().add((index + 1) * self.item_layout.size()),
            self.ptr.as_ptr().add(index * self.item_layout.size()),
            (self.len - index - 1) * self.item_layout.size()
        );

        self.len -= 1;
    }

    pub unsafe fn swap_remove_unchecked(&mut self, index: usize) {
        std::ptr::copy(
            self.ptr.as_ptr().add((self.len - 1) * self.item_layout.size()),
            self.ptr.as_ptr().add(index * self.item_layout.size()),
            self.item_layout.size()
        );

        self.len -= 1;
    }

    fn grow(&mut self) {
        (self.capacity == 0).then(|| {
            self.capacity = 1;
    
                self.ptr = unsafe {
                    let new_ptr = std::alloc::alloc(
                        self.item_layout
                    );
    
                    NonNull::new(new_ptr).unwrap()
                }
        }).or_else(|| {
            self.capacity <<= 1;

            self.ptr = unsafe {
                let new_ptr = std::alloc::realloc(
                    self.ptr.as_ptr(),
                    Layout::from_size_align_unchecked(self.item_layout.size() * self.capacity, self.item_layout.align()),
                    self.capacity * self.item_layout.size()
                );
        
                NonNull::new(new_ptr).unwrap()
            };

            Some(())
        });
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        (self.capacity > 0).then(|| {
            unsafe {
                std::alloc::dealloc(
                    self.ptr.as_ptr(),
                    Layout::from_size_align_unchecked(
                        self.capacity * self.item_layout.size(), self.item_layout.align()
                    )
                )
            };
        });
    }
}