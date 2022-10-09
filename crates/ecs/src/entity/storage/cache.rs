use crate::entity::Entity;

pub struct EntityCache {
    free_entities: Vec<Entity>
}

impl EntityCache {
    pub fn new(capacity: usize) -> Self {
        EntityCache { free_entities: Vec::with_capacity(capacity) }
    }

    pub fn pop(&mut self) -> Option<Entity> {
        self.free_entities.pop()
    }

    pub fn store(&mut self, entity: Entity) {
        self.free_entities.push(entity);
    }
}