use crate::components::{actor, physics};
use crate::resources;
use bevy::prelude::*;

pub struct GameActorPlugin;

impl Plugin for GameActorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(process_commands_system.system());
    }
}

pub fn process_commands_system(mut query: Query<(&mut physics::Velocity, &mut actor::Controller)>) {
    for (mut velocity, mut controller) in query.iter_mut() {
        *velocity.x_mut() = controller.right;
        *velocity.z_mut() = controller.forward;
        controller.forward = 0.0;
        controller.right = 0.0;
    }
}
