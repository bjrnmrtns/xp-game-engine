pub struct Events<T> {
    events: Vec<T>,
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Self { events: vec![] }
    }
}

impl<T> Events<T> {
    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn send(&mut self, event: T) {
        self.events.push(event);
    }

    pub fn values(&self) -> std::slice::Iter<'_, T> {
        self.events.iter()
    }
}
