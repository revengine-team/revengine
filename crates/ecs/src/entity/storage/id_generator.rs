use crate::entity::EntityId;

pub struct EntityIdGenerator {
    id: EntityId
}

impl EntityIdGenerator {
    pub fn new() -> Self {
        EntityIdGenerator { id: 0 }
    }

    pub fn new_id(&mut self) -> EntityId {
        let new_id = self.id;
        self.id += 1;

        new_id
    }
}