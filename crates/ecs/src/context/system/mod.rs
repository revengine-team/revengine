use super::{Proxy, Context};

pub trait System: 'static {
    fn update(&self, proxy: &mut impl Proxy, context: &impl Context);
}