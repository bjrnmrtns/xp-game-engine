mod client;
mod input;
mod physics;
mod world_loader;

use bevy::prelude::*;

fn main() {
    App::build()
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
        .add_plugin(world_loader::WorldPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(client::ClientPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .run();
}

//TODO: fixed run time_step? .add_plugin(bevy::app::ScheduleRunnerPlugin::run_loop(
//Duration::from_secs_f64(1.0 / 60.0),
//))
