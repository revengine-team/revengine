use self::{cache::Cache, gen::Increment};

mod cache;
mod gen;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Identifer {
    id: u32,
    generation: u16
}

impl Identifer {
    pub fn new(id: u32, generation: u16) -> Self {
        Identifer {
            id,
            generation
        }
    }

    #[inline]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    pub fn generation(&self) -> u16 {
        self.generation
    }
}

pub struct Allocator {
    cache: Cache<Identifer>,
    id_gen: Increment
}

impl Default for Allocator {
    fn default() -> Self {
        Allocator {
            cache: Default::default(),
            id_gen: Default::default()
        }
    }
}

impl Allocator {
    pub fn alloc(&mut self) -> Identifer {
        if let Some(mut free_identifer) = self.cache.pop() {
            free_identifer.generation += 1;

            return free_identifer
        }

        let id = self.id_gen.next();
        let generation = 0;

        Identifer::new(id, generation)
    }

    pub fn dealloc(&mut self, identifer: Identifer) {
        self.cache.store(identifer);
    }
}