use std::marker::PhantomData;

use super::SparseKey;

pub struct SparseArray<K> {
    array: Vec<Option<usize>>,
    marker: PhantomData<K>
}

impl<K: SparseKey> SparseArray<K> {
    pub fn new() -> Self {
        SparseArray {
            array: Vec::new(),
            marker: PhantomData
        }
    }

    pub fn insert(&mut self, key: K, index: usize) {
        let index_inner = key.as_usize();

        if index_inner >= self.array.len() {
            self.array.resize_with(index_inner + 1, || None);
        }

        self.array[index_inner] = Some(index);
    }

    pub fn contains(&self, key: K) -> bool {
        let index = key.as_usize();

        self.array.get(index)
            .map(|value| value.is_some())
            .unwrap_or(false)
    }

    pub fn index_of(&self, key: K) -> Option<&usize> {
        let index = key.as_usize();

        self.array.get(index)
            .map(|value| value.as_ref())
            .unwrap_or(None)
    }

    pub fn remove(&mut self, key: K) -> Option<usize> {
        let index = key.as_usize();

        self.array.get_mut(index)
            .and_then(|value| value.take())
    }
}