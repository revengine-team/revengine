pub mod container;
pub mod query;

pub(super) mod meta;

use std::{marker::PhantomData, hash::Hash};

pub use container::*;

use self::{meta::MetaTable, query::Query};

pub trait Resource: 'static {}

impl<T> Resource for T 
where T: 'static {}

pub trait Key: Clone + Copy + Eq + PartialEq + Hash {}

pub struct Storage<TKey: Key, TContainer: UnsafeContainer<TKey>> {
    meta: MetaTable,
    containers: Box<[TContainer]>,

    marker: PhantomData<TKey>
}

impl<TKey: Key, TContainer: UnsafeContainer<TKey>> Storage<TKey, TContainer> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn read<TResource: Resource>(&self) -> Option<ReadContainer<TContainer, TKey, TResource>> {
        self.meta.container_id::<TResource>()
            .map(|id| {
                ReadContainer::wrap(
                    &self.containers[id]
                )   
            }).or(None)
    }

    pub fn write<TResource: Resource>(&self) -> Option<WriteContainer<TContainer, TKey, TResource>> {
        self.meta.container_id::<TResource>()
            .map(|id| {
                WriteContainer::wrap(
                    &self.containers[id]
                )   
            }).or(None)
    }

    pub fn query<Q: Query<TKey, TContainer>>(&self) -> Option<Q> {
        Q::access(&self)
    }
}