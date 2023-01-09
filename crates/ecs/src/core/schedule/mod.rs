use crate::context::{System, Proxy, Context};

mod sequential;
mod parallel;

pub use {
    sequential::*,
    parallel::*
};

pub trait Schedule<P: Proxy, C: Context> {
    fn for_each<F>(&self, f: F)
    where F: Fn(&Box<dyn Execute<P, C>>);
}

pub trait Execute<P: Proxy, C: Context> {
    fn execute(&self, proxy: &mut P, context: &C);
}

impl<P: Proxy, C: Context, T: System> Execute<P, C> for T {
    #[inline]
    fn execute(&self, proxy: &mut P, context: &C) {
        self.update(proxy, context)
    }
}