use crate::context::Context;

mod sparseset;

pub use {
    sparseset::*
};

pub trait Storage: Context {
    
}