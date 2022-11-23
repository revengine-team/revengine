use crate::core::storage::{Storage, ReadContainer, Resource, WriteContainer};

use self::container::{SparseKey, SparseSet};

pub mod container;

pub type SparseSetStorage<TKey: SparseKey> = Storage<TKey, SparseSet<TKey>>;

pub type ReadSparseSet<'set, TKey: SparseKey, T: Resource> = 
    ReadContainer<'set, SparseSet<TKey>, TKey, T>;
    
pub type WriteSparseSet<'set, TKey: SparseKey, T: Resource> = 
    WriteContainer<'set, SparseSet<TKey>, TKey, T>;