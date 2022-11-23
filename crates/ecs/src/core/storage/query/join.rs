use std::marker::PhantomData;

use crate::core::storage::{UnsafeContainer, Key};

use super::Query;

pub trait Join<TKey: Key, TContainer: UnsafeContainer<TKey>>: Query<TKey, TContainer> {
    fn join(self) -> JoinFetch<TKey, TContainer, Self>
    where Self: Sized;
}

pub struct JoinFetch<TKey: Key, TContainer: UnsafeContainer<TKey>, J: Join<TKey, TContainer>> {
    key_marker: PhantomData<TKey>,
    container_marker: PhantomData<TContainer>,
    join_marker: PhantomData<J>
}

impl<TKey: Key, TContainer: UnsafeContainer<TKey>, J: Join<TKey, TContainer>> 
    JoinFetch<TKey, TContainer, J> {
    pub fn wrap(join: J) -> Self {
        todo!()
    }
}

impl<TKey: Key, TContainer: UnsafeContainer<TKey>, J: Join<TKey, TContainer>>  Iterator 
    for JoinFetch<TKey, TContainer, J> {
    type Item = J::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

macro_rules! impl_join {
    ( $( $name:ident )+ ) => {
        impl<TKey: Key, TContainer: UnsafeContainer<TKey>, $($name: Query<TKey, TContainer>),+> Join<TKey, TContainer> for ($($name,)+) {
            fn join(self) -> JoinFetch<TKey, TContainer, Self>
            where Self: Sized {
                todo!()
            }
        }
    };
}

impl_join! { A }
impl_join! { A B }
impl_join! { A B C }
impl_join! { A B C D }
impl_join! { A B C D E }
impl_join! { A B C D E F }
impl_join! { A B C D E F G }
impl_join! { A B C D E F G H }
impl_join! { A B C D E F G H I }
impl_join! { A B C D E F G H I J }
impl_join! { A B C D E F G H I J K }
impl_join! { A B C D E F G H I J K L }
