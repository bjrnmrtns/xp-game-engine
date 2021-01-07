mod components;
mod resources;

pub use components::{
    CameraCenterController, CameraZoomController, Command1, Command2, PlayerController,
    SelectionRender,
};

use crate::{
    client::{
        components::{CameraCenter, Unit},
        resources::PhysicsState,
    },
    helpers,
    input::CommandEvent,
};
use bevy::prelude::*;

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(PhysicsState::default())
            .add_startup_system(create_world.system())
            .add_system(handle_camera.system())
            .add_system(handle_player.system())
            .add_system(handle_physics.system());
    }
}

fn create_world(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(StandardMaterial {
            albedo: Color::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
            material: materials.add(StandardMaterial {
                albedo: Color::rgba(0.0, 0.0, 1.0, 0.25),
                ..Default::default()
            }),
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.1, 0.0)),
            ..Default::default()
        })
        .with(SelectionRender);

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 120.0, 0.0)),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(1.0, 0.0, 1.0),
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(Unit::default());

    commands
        .spawn(CameraCenter)
        .with(GlobalTransform::identity())
        .with(Transform::identity())
        .with(CameraCenterController::default())
        .with(PlayerController::default())
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0))
                        .mul_transform(Transform::from_rotation(Quat::from_rotation_x(
                            -std::f32::consts::FRAC_PI_2,
                        ))),
                    ..Default::default()
                })
                .with(CameraZoomController::default());
        });
}

fn handle_camera(
    mut query_center: Query<(&CameraCenterController, &mut Transform)>,
    mut query_zoom: Query<(&mut CameraZoomController, &mut Transform)>,
) {
    for (controller, mut center) in query_center.iter_mut() {
        if let Some(move_position) = controller.move_position {
            center.translation.x += move_position.x * 0.5;
            center.translation.z -= move_position.y * 0.5;
        }
    }
    for (mut controller, mut center) in query_zoom.iter_mut() {
        if let Some(zoom) = controller.zoom {
            center.translation.y -= zoom;
            controller.zoom = None;
        }
    }
}

#[derive(Default)]
pub struct CommandEventState {
    pub event_reader: EventReader<CommandEvent>,
}

fn handle_player(
    mut query: Query<&mut PlayerController>,
    mut query_units: Query<(&GlobalTransform, &mut Handle<StandardMaterial>, &mut Unit)>,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut command_event_state: Local<CommandEventState>,
    command_events: Res<Events<CommandEvent>>,
) {
    for command_event in command_event_state.event_reader.iter(&command_events) {
        match command_event {
            CommandEvent::Move(target) => {
                println!("{}", target);
            }
        }
    }
    for mut controller in query.iter_mut() {
        if let Some((begin, end)) = controller.rectangle_select {
            match &controller.command1 {
                Command1::Create => {
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                            material: materials.add(StandardMaterial {
                                albedo: Color::rgb(1.0, 0.0, 0.0),
                                ..Default::default()
                            }),
                            transform: Transform::from_translation(end),
                            ..Default::default()
                        })
                        .with(Unit::default());
                    controller.rectangle_select = None;
                }
                Command1::Select => {
                    for (transform, mut material, mut unit) in query_units.iter_mut() {
                        let position = Vec2::new(transform.translation.x, transform.translation.z);
                        let (top_left, bottom_right) = helpers::calculate_low_high(begin, end);
                        unit.selected = helpers::is_selected(top_left, bottom_right, position);
                        if unit.selected {
                            *material = materials.add(StandardMaterial {
                                albedo: Color::rgb(0.0, 1.0, 0.5),
                                ..Default::default()
                            });
                        } else {
                            *material = materials.add(StandardMaterial {
                                albedo: Color::rgb(0.0, 0.5, 1.0),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
        match &controller.command2 {
            Command2::Move(Some(target)) => {
                for (_, _, mut unit) in query_units.iter_mut() {
                    if unit.selected {
                        unit.target_position = Some(target.clone());
                    }
                }
            }
            _ => {}
        }
    }
}

fn handle_physics(
    time: Res<Time>,
    mut physics_state: ResMut<PhysicsState>,
    mut query_units: Query<(&mut Transform, &mut Unit)>,
) {
    let steps_per_second = 60.0;
    let step_time = 1.0 / steps_per_second;
    let speed = 3.0; // m/s
    let expected_steps = (time.time_since_startup().as_secs_f64() * steps_per_second) as u64;
    for _ in physics_state.steps_done..expected_steps {
        for (mut transform, mut unit) in query_units.iter_mut() {
            if let Some(target) = unit.target_position {
                println!("{}", target);
                let current = Vec2::new(transform.translation.x, transform.translation.z);
                let direction = target - current;
                if direction.length() > 3.0 {
                    let movement = direction.normalize() * (step_time * speed * 3.0) as f32;
                    transform.translation.x += movement.x;
                    transform.translation.z += movement.y;
                } else {
                    unit.target_position = None;
                }
            }
        }
    }
    physics_state.steps_done = expected_steps;
}
