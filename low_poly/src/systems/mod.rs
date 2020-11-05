mod actor;
mod player;
mod physics;
mod camera;

use bevy::prelude::*;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(player::handle_input_system.system())
            .add_plugin(camera::CameraPlugin)
            .add_plugin(actor::GameActorPlugin)
            .add_plugin(physics::GamePhysicsPlugin);
    }
}
