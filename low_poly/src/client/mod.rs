mod components;
mod resources;

pub use components::EntityController;

use bevy::prelude::*;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(client_system.system());
    }
}

fn client_system() {}
