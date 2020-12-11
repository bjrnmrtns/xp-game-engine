mod client;
mod input;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor {
            title: "low_poly".to_string(),
            width: 1200,
            height: 800,
            vsync: true,
            mode: bevy::window::WindowMode::Fullscreen { use_size: false },
            resizable: true,
            cursor_visible: false,
            cursor_locked: true,
            ..Default::default()
        })
        .add_plugins(bevy::DefaultPlugins)
        .add_plugin(input::InputPlugin)
        .add_plugin(client::ClientPlugin)
        .run();
}
