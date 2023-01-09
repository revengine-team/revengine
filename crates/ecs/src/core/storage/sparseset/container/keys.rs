use crate::context::Entity;

pub struct SparseArray {
    array: Vec<Option<usize>>
}

impl SparseArray {
    pub fn new() -> Self {
        SparseArray {
            array: Vec::new()
        }
    }

    pub fn insert(&mut self, entity: Entity, index: usize) {
        /*
        let index_inner = key.as_usize();

        if index_inner >= self.array.len() {
            self.array.resize_with(index_inner + 1, || None);
        }

        self.array[index_inner] = Some(index);
         */
    }

    pub fn index_of(&self, entity: Entity) -> Option<usize> {
        /*
        let index = key.as_usize();

        self.array.get(index)
            .map(|value| value.as_ref())
            .unwrap_or(None)
        */
        todo!()
    }

    pub fn remove(&mut self, entity: Entity) -> Option<usize> {
        /*
        let index = key.as_usize();

        self.array.get_mut(index)
            .and_then(|value| value.take())
        */

        todo!()
    }
}