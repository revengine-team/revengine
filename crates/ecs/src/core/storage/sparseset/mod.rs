pub mod query;
pub mod proxy;

mod container;
mod meta;

use crate::context::{Context, Component, QueryItem};

use self::{meta::MetaTable, container::SparseSet, query::SparseSetStorageQuery};

use super::Storage;

pub struct SparseSetStorage {
    meta: MetaTable,
    sets: Box<[SparseSet]>
}

impl SparseSetStorage {
    pub fn new(meta: fxhash::FxHashMap<std::any::TypeId, usize>) -> Self {
        SparseSetStorage {
            meta: MetaTable::new(meta),
            sets: Box::new([])
        }
    }
}

impl Context for SparseSetStorage {
    type Query<I: QueryItem> = SparseSetStorageQuery<I>;

    fn query<I: QueryItem>(&self) -> Self::Query<I> {
        

        todo!()
    }
}

impl Storage for SparseSetStorage {}