use std::any::TypeId;

use fxhash::FxHashMap;

use super::Resource;

pub type ContainerId = usize;

pub struct MetaTable {
    type_map: FxHashMap<TypeId, ContainerId>,
}

impl MetaTable {
    pub fn new(type_map: FxHashMap<TypeId, ContainerId>) -> Self {
        MetaTable {
            type_map
        }
    }

    pub fn container_id<T: Resource>(&self) -> Option<ContainerId> {
        self.type_map.get(&TypeId::of::<T>())
            .map(|&id| id)
            .or(None)
    }
}