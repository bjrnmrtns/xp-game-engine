mod components;
mod resources;
mod setup_plugin;
mod systems;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::time::Duration;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor {
            title: "low_poly".to_string(),
            width: 1200,
            height: 800,
            vsync: true,
            mode: bevy::window::WindowMode::Windowed,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(bevy::DefaultPlugins)
        .add_plugin(setup_plugin::SetupPlugin)
        .add_plugin(systems::GameLogicPlugin)
        .run();
}
//TODO: fixed run time_step? .add_plugin(bevy::app::ScheduleRunnerPlugin::run_loop(
//Duration::from_secs_f64(1.0 / 60.0),
//))
