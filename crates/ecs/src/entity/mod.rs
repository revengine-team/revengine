pub(crate) mod storage;

pub type EntityId = u32;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Entity {
    id: EntityId,
    generation: u16
}

impl Entity {
    pub(super) fn new(id: EntityId) -> Self {
        Entity { id, generation: 0 }
    }

    pub(super) fn use_existing(entity: Entity) -> Self {
        Entity { id: entity.id, generation: entity.generation + 1 }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn generation(&self) -> u16 {
        self.generation
    }
}