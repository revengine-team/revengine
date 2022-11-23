mod buffer;
mod array;
pub mod key;

use buffer::*;
use array::*;
pub use key::*;

use crate::core::storage::{UnsafeContainer, Resource};

pub struct SparseSet<K: SparseKey> {
    sparse: SparseArray<K>,
    dense: Buffer
}

impl<K: SparseKey> SparseSet<K> {
    pub fn new<V: Resource>() -> Self {
        SparseSet {
            sparse: SparseArray::new(),
            dense: Buffer::new::<V>()
        }
    }
}

impl<K: SparseKey> UnsafeContainer<K> for SparseSet<K> {
    unsafe fn insert<V: Resource>(&mut self, key: K, value: V) {
        if let Some(&index) = self.sparse.index_of(key) {
            unsafe {
                *self.dense.borrow_mut_unchecked(index) = value;
            }
        } else {
            self.sparse.insert(key, self.dense.len());
            self.dense.push(value);
        }
    }

    unsafe fn borrow<V: Resource>(&self, key: K) -> Option<&V> {
        self.sparse.index_of(key)
        .map(|&index| {
            unsafe {
                self.dense.borrow_unchecked(index)
            }
        })
    }

    unsafe fn borrow_mut<V: Resource>(&self, key: K) -> Option<&mut V> {
        self.sparse.index_of(key)
        .map(|&index| {
            unsafe {
                self.dense.borrow_mut_unchecked(index)
            }
        })
    }

    unsafe fn remove<V: Resource>(&mut self, key: K) {
        self.sparse.remove(key)
        .and_then(|index| {
            unsafe {
                self.dense.swap_remove_unchecked(index);
            }

            Some(())
        });
    }
}