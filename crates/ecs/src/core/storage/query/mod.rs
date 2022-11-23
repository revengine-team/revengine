pub mod join;

pub use join::*;

use super::{UnsafeContainer, Key, Storage};

pub trait Query<TKey: Key, TContainer: UnsafeContainer<TKey>> {
    type Item;

    fn access(storage: &Storage<TKey, TContainer>) -> Option<Self>
    where Self: Sized;
}

macro_rules! impl_query {
    ( $( $name:ident )+ ) => {
        impl<TKey: Key, TContainer: UnsafeContainer<TKey>, $($name: Query<TKey, TContainer>),+> Query<TKey, TContainer> for ($($name,)+) {
            type Item = ($(<$name as Query<TKey, TContainer>>::Item),*,);

            fn access(storage: &Storage<TKey, TContainer>) -> Option<Self>
            where Self: Sized {
                match ($(<$name as Query<TKey, TContainer>>::access(storage)),*,) {
                    //todo: tuple(Option) -> Option(tuple)

                    _ => None
                } 
            }
        }
    };
}

impl_query! { A }
impl_query! { A B }
impl_query! { A B C }
impl_query! { A B C D }
impl_query! { A B C D E }
impl_query! { A B C D E F }
impl_query! { A B C D E F G }
impl_query! { A B C D E F G H }
impl_query! { A B C D E F G H I }
impl_query! { A B C D E F G H I J }
impl_query! { A B C D E F G H I J K }
impl_query! { A B C D E F G H I J K L }