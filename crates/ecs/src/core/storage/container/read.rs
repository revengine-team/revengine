use std::marker::PhantomData;

use crate::core::storage::{Resource, Key, query::{Query}, Storage};

use super::UnsafeContainer;

pub struct ReadContainer<'storage, TContainer: UnsafeContainer<TKey>, TKey: Key, TResource: Resource> {
    container: &'storage TContainer,
    key_marker: PhantomData<TKey>,
    resource_marker: PhantomData<TResource>
}

impl<'s, TContainer: UnsafeContainer<TKey>, TKey: Key, TResource: Resource> 
    ReadContainer<'s, TContainer,TKey, TResource> {
    pub fn wrap(container: &'s TContainer) -> Self {
        ReadContainer {
            container,
            key_marker: PhantomData,
            resource_marker: PhantomData
        }
    }

    pub fn borrow(&self, key: TKey) -> Option<&TResource> {
        unsafe {
            self.container.borrow::<TResource>(key)
        }
    }
}

impl<'s, TContainer: UnsafeContainer<TKey>, TKey: Key, TResource: Resource>
    Query<TKey, TContainer> for ReadContainer<'s, TContainer,TKey, TResource> {
    type Item = &'s TResource;

    fn access(storage: &Storage<TKey, TContainer>) -> Option<Self>
    where Self: Sized {
        todo!()
    }
}