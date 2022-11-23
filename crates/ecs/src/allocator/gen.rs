pub struct Increment {
    current: u32
}

impl Default for Increment {
    fn default() -> Self {
        Increment { current: 0 }
    }
}

impl Increment {
    pub fn next(&mut self) -> u32 {
        let new = self.current;
        self.current += 1;

        new
    }
}