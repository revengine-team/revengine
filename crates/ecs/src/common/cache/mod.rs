pub trait Cache<T> {
    fn pop(&mut self) -> Option<T>;
    fn store(&mut self, item: T);
}

pub struct BaseCache<T> {
    free_items: Vec<T>
}

impl<T> BaseCache<T> {
    pub fn new(capacity: usize) -> Self {
        BaseCache { free_items: Vec::with_capacity(capacity) }
    }
}

impl<T> Cache<T> for BaseCache<T> {
    fn pop(&mut self) -> Option<T> {
        self.free_items.pop()
    }

    fn store(&mut self, item: T) {
        self.free_items.push(item);
    }
}