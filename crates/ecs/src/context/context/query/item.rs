use crate::context::{Component};

pub trait QueryItem {
    
}

impl<'a, T: Component> QueryItem for (&'a T,) {
    
}

impl<'a, T: Component> QueryItem for (&'a mut T,) {
    
}

macro_rules! impl_query_items {
    ( $( $name:ident )+ ) => {
        impl<$($name: QueryItem),+> QueryItem for ($($name,)+) {
            
        }
    };
}

impl_query_items! { A B }
impl_query_items! { A B C }
impl_query_items! { A B C D }
impl_query_items! { A B C D E }
impl_query_items! { A B C D E F }
impl_query_items! { A B C D E F G }
impl_query_items! { A B C D E F G H }
impl_query_items! { A B C D E F G H I }
impl_query_items! { A B C D E F G H I J }
impl_query_items! { A B C D E F G H I J K }
impl_query_items! { A B C D E F G H I J K L }
