use std::any::TypeId;

use fxhash::FxHashMap;

use crate::context::Component;

type SetId = usize;

pub struct MetaTable {
    set_ids: FxHashMap<TypeId, SetId>
}

impl MetaTable {
    pub fn new(set_ids: FxHashMap<TypeId, SetId>) -> Self {
        MetaTable {
            set_ids
        }
    }

    pub fn set_id<T: Component>(&self) -> Option<SetId> {
        self.set_ids.get(&TypeId::of::<T>())
            .map(|&id| id)
            .or(None)
    }
}