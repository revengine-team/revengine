use crate::context::{Entity, Component};

pub trait Proxy {
    fn new_entity(&mut self) -> Entity;
    fn remove_entity(&mut self, entity: Entity);

    fn attach_component<T: Component>(&mut self, entity: Entity);
    fn detach_component<T: Component>(&mut self, entity: Entity);
}