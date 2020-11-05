use crate::components::physics;
use crate::resources;

use crate::components::physics::Position;
use bevy::prelude::*;

pub struct GamePhysicsPlugin;

impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(process_velocity_system.system());
    }
}

pub fn process_velocity_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut physics::Position, &physics::Velocity)>,
) {
    for (mut transform, mut position, velocity) in query.iter_mut() {
        *position = Position(position.0 + velocity.0);
        transform.translation.set_z(position.x());
        transform.translation.set_y(position.y());
        transform.translation.set_x(position.z());
    }
}
