mod client;
mod input;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor {
            title: "rts".to_string(),
            width: 1200.0,
            height: 800.0,
            vsync: true,
            //mode: bevy::window::WindowMode::Fullscreen { use_size: false },
            mode: bevy::window::WindowMode::Windowed,
            resizable: true,
            cursor_visible: true,
            cursor_locked: false,
            ..Default::default()
        })
        .add_plugins(bevy::DefaultPlugins)
        .add_plugin(input::InputPlugin)
        .add_plugin(client::ClientPlugin)
        .run();
}
