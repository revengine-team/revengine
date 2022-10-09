pub(self) mod cache;
pub(self) mod id_generator;

use sparseset::SparseSet;

use crate::entity::Entity;

use self::{cache::EntityCache, id_generator::EntityIdGenerator};

pub struct EntityStorage {
    entities: SparseSet<Entity>,
    cache: EntityCache,

    id_generator: EntityIdGenerator
}

impl EntityStorage {
    pub fn new(capacity: usize) -> Self {
        EntityStorage { entities: SparseSet::with_capacity(capacity), cache: EntityCache::new(capacity), id_generator: EntityIdGenerator::new() }
    }
    
    pub fn allocate(&mut self) -> Entity {
        let entity = match self.cache.pop() {
            Some(free_entity) => {
                Entity::use_existing(free_entity)
            }

            None => {
                Entity::new(self.id_generator.new_id())
            }
        };

        self.entities.insert(entity.id as usize, entity);

        entity
    }

    pub fn free(&mut self, item: Entity) {
        let entity_to_store = self.entities.remove(item.id as usize).unwrap();
        self.cache.store(entity_to_store);
    }
}

mod tests {
    use super::EntityStorage;

    #[test]
    fn allocate_new_entity() {
        let mut entity_storage = EntityStorage::new(10);

        let new_entity = entity_storage.allocate();

        let expected_id = 0;
        let expected_generation = 0;

        assert!(entity_storage.entities.contains(new_entity.id as usize));
        assert_eq!(new_entity.id, expected_id);
        assert_eq!(new_entity.generation, expected_generation);
    }

    #[test]
    fn free_entity() {
        let mut entity_storage = EntityStorage::new(10);

        let new_entity = entity_storage.allocate();
        entity_storage.free(new_entity);

        assert!(!entity_storage.entities.contains(new_entity.id as usize));
    }

    #[test]
    fn allocate_entity_from_cache() {
        let mut entity_storage = EntityStorage::new(10);

        let new_entity = entity_storage.allocate();
        entity_storage.free(new_entity);

        let new_entity_from_cache = entity_storage.allocate();
        let expected_id = 0;
        let expected_generation = 1;

        assert!(entity_storage.entities.contains(new_entity_from_cache.id as usize));
        assert_eq!(new_entity_from_cache.id, expected_id);
        assert_eq!(new_entity_from_cache.generation, expected_generation);
    }
}