use crate::core::storage::Key;

pub trait SparseKey: Key {
    fn as_usize(&self) -> usize;
}

macro_rules! impl_key {
    ($name: ty) => {
        impl Key for $name {}

        impl SparseKey for $name {
            fn as_usize(&self) -> usize {
                *self as usize
            }
        }
    };
}

impl_key!(u8);
impl_key!(u16);
impl_key!(u32);
impl_key!(u64);
impl_key!(usize);