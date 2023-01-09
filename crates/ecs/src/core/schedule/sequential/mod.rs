use crate::context::{Proxy, Context};

use super::{Schedule, Execute};

pub struct SequentialSchedule<P: Proxy, C: Context> {
    systems: Box<[Box<dyn Execute<P, C>>]>
}

impl<P: Proxy, C: Context> SequentialSchedule<P, C> {
    pub fn new(systems: Box<[Box<dyn Execute<P, C>>]>) -> Self {
        SequentialSchedule {
            systems
        }
    }
}

impl<P: Proxy, C: Context> Schedule<P, C> for SequentialSchedule<P, C> {
    fn for_each<F>(&self, f: F)
    where F: Fn(&Box<dyn Execute<P, C>>) {
        for system in self.systems.iter() {
            f(system)
        }
    }
}