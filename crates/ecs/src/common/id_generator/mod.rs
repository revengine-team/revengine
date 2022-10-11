pub type Id = u32;

pub trait GenerateNewId {
    fn new_id(&mut self) -> Id;
}

pub struct IdGenerator {
    id: Id
}

impl IdGenerator {
    pub fn new() -> Self {
        IdGenerator { id: 0 }
    }
}

impl GenerateNewId for IdGenerator {
    fn new_id(&mut self) -> Id {
        let new_id = self.id;
        self.id += 1;
    
        new_id
    }
}