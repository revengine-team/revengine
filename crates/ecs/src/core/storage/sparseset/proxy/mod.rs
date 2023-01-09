mod commands;

use commands::*;

use crate::context::{Proxy, Entity, Component};

pub struct SparseSetStorageProxy {
    buffer: CommandBuffer
}

impl SparseSetStorageProxy {
    pub fn new() -> Self {
        SparseSetStorageProxy {
            buffer: CommandBuffer::empty()
        }
    }
}

impl Proxy for SparseSetStorageProxy {
    fn new_entity(&mut self) -> Entity {
        todo!()
    }

    fn remove_entity(&mut self, entity: Entity) {
        todo!()
    }

    fn attach_component<T: Component>(&mut self, entity: Entity) {
        todo!()
    }

    fn detach_component<T: Component>(&mut self, entity: Entity) {
        todo!()
    }
}