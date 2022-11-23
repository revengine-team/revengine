pub struct Cache<T> {
    container: Vec<T>,
}

impl<T> Default for Cache<T> {
    fn default() -> Self {
        Cache {
            container: Default::default()
        }
    }
}

impl<T> Cache<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.container.pop()
    }

    pub fn store(&mut self, item: T) {
        self.container.push(item);
    }
}