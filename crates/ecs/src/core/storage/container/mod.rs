pub mod read;
pub mod write;

pub use read::*;
pub use write::*;

use super::{Key, Resource};

pub trait UnsafeContainer<K: Key> {
    unsafe fn insert<V: Resource>(&mut self, key: K, value: V);

    unsafe fn borrow<V: Resource>(&self, key: K) -> Option<&V>;

    unsafe fn borrow_mut<V: Resource>(&self, key: K) -> Option<&mut V>;

    unsafe fn remove<V: Resource>(&mut self, key: K);
}