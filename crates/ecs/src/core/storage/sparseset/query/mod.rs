mod fetch;

pub use fetch::*;

use crate::context::{Query, QueryItem};

pub struct SparseSetStorageQuery<I: QueryItem> {
    i: I
}

impl<I: QueryItem> Query<I> for SparseSetStorageQuery<I> {
    
}