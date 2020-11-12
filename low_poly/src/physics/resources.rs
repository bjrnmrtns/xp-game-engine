pub struct PhysicsSteps {
    pub done: u64,
}

impl PhysicsSteps {
    pub fn new() -> Self {
        Self { done: 0 }
    }
}
