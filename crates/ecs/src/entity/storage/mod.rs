use sparseset::SparseSet;

use crate::common::{cache::{BaseCache, Cache}, id_generator::{IdGenerator, GenerateNewId}};

use super::{EntityId, Entity};

pub trait EntityStorage {
    fn allocate(&mut self) -> EntityId;
    fn free(&mut self, id: EntityId);
}

pub struct CachedEntityStorage {
    entities: SparseSet<Entity>,
    cache: BaseCache<Entity>,

    id_generator: IdGenerator
}

impl CachedEntityStorage {
    pub fn new(capacity: usize) -> Self {
        CachedEntityStorage { 
            entities: SparseSet::with_capacity(capacity), 
            cache: BaseCache::new(capacity),
            id_generator: IdGenerator::new()
        }
    }
}

impl EntityStorage for CachedEntityStorage {
    fn allocate(&mut self) -> EntityId {
        let entity = match self.cache.pop() {
            Some(free_entity) => {
                Entity::use_existing(free_entity)
            }

            None => {
                Entity::new(self.id_generator.new_id())
            }
        };

        self.entities.insert(entity.id as usize, entity);

        entity.id
    }

    fn free(&mut self, id: EntityId) {
        let entity_to_store = self.entities.remove(id as usize).unwrap();
        self.cache.store(entity_to_store);
    }
}

mod tests {
    use crate::entity::storage::{EntityStorage, CachedEntityStorage};

    #[test]
    fn allocate_new_entity() {
        let mut entity_storage = CachedEntityStorage::new(10);

        let new_entity = entity_storage.allocate();

        let expected_id = 0;

        assert!(entity_storage.entities.contains(new_entity as usize));
        assert_eq!(new_entity, expected_id);
    }

    #[test]
    fn free_entity() {
        let mut entity_storage = CachedEntityStorage::new(10);

        let new_entity = entity_storage.allocate();
        entity_storage.free(new_entity);

        assert!(!entity_storage.entities.contains(new_entity as usize));
    }

    #[test]
    fn allocate_entity_from_cache() {
        let mut entity_storage = CachedEntityStorage::new(10);

        let new_entity = entity_storage.allocate();
        entity_storage.free(new_entity);

        let new_entity_from_cache = entity_storage.allocate();
        let new_entity_from_cache_generation = entity_storage.entities
        .get(new_entity_from_cache as usize)
        .unwrap()
        .generation;

        let expected_id = 0;
        let expected_generation = 1;

        assert!(entity_storage.entities.contains(new_entity_from_cache as usize));
        assert_eq!(new_entity_from_cache, expected_id);
        assert_eq!(new_entity_from_cache_generation, expected_generation);
    }
}