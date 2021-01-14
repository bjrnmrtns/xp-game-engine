mod components;
mod resources;

pub use components::SelectionRender;

pub use resources::GameInfo;

use crate::{
    client::{
        components::{CameraCenter, EmptyBundle, Unit},
        resources::{PhysicsState, UnitIdGenerator},
    },
    helpers,
    input::{CameraViewEvent, CommandEvent},
};
use bevy::{prelude::*, render::camera::Camera};

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GameInfo::default())
            .add_resource(UnitIdGenerator::default())
            .add_resource(PhysicsState::default())
            .add_startup_system(create_world.system())
            .add_system(handle_camera.system())
            .add_system(handle_player.system())
            .add_system(handle_physics.system());
    }
}

fn create_world(
    commands: &mut Commands,
    mut unit_id_generator: ResMut<UnitIdGenerator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_info: ResMut<GameInfo>,
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
        .with(Unit::new(unit_id_generator.generate()));

    game_info.camera_center = commands
        .spawn(EmptyBundle)
        .with(GlobalTransform::identity())
        .with(Transform::identity())
        .with(CameraCenter)
        .with_children(|parent| {
            game_info.camera = parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0))
                        .mul_transform(Transform::from_rotation(Quat::from_rotation_x(
                            -std::f32::consts::FRAC_PI_2,
                        ))),
                    ..Default::default()
                })
                .current_entity();
        })
        .current_entity();
}

#[derive(Default)]
pub struct EventStates {
    pub command_event_reader: EventReader<CommandEvent>,
    pub camera_view_event_reader: EventReader<CameraViewEvent>,
}

fn handle_camera(
    mut query_center: Query<(&mut Transform, &CameraCenter)>,
    mut query_zoom: Query<(&Camera, &mut Transform)>,
    mut event_states: Local<EventStates>,
    camera_view_events: Res<Events<CameraViewEvent>>,
    game_info: Res<GameInfo>,
) {
    if let (Some(camera), Some(camera_center)) = (game_info.camera, game_info.camera_center) {
        for camera_view_event in event_states
            .camera_view_event_reader
            .iter(&camera_view_events)
        {
            match camera_view_event {
                CameraViewEvent::Zoom(zoom) => {
                    let (_, mut transform) = query_zoom.get_mut(camera).unwrap();
                    transform.translation.y -= zoom;
                }
                CameraViewEvent::CameraMove(translation) => {
                    let (mut transform, _) = query_center.get_mut(camera_center).unwrap();
                    transform.translation +=
                        Vec3::new(translation.x * 0.5, 0.0, translation.y * 0.5);
                }
            }
        }
    }
}

fn handle_player(
    mut query_units: Query<(&GlobalTransform, &mut Handle<StandardMaterial>, &mut Unit)>,
    commands: &mut Commands,
    mut unit_id_generator: ResMut<UnitIdGenerator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut event_states: Local<EventStates>,
    command_events: Res<Events<CommandEvent>>,
) {
    for command_event in event_states.command_event_reader.iter(&command_events) {
        match command_event {
            CommandEvent::Create(target) => {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(StandardMaterial {
                            albedo: Color::rgb(1.0, 0.0, 0.0),
                            ..Default::default()
                        }),
                        transform: Transform::from_translation(Vec3::new(target.x, 0.5, target.y)),
                        ..Default::default()
                    })
                    .with(Unit::new(unit_id_generator.generate()));
            }
            CommandEvent::Move(target) => {
                for (_, _, mut unit) in query_units.iter_mut() {
                    if unit.selected {
                        unit.desired_position = Some(target.clone());
                    }
                }
            }
            CommandEvent::Select(low, high) => {
                for (transform, mut material, mut unit) in query_units.iter_mut() {
                    let position = Vec2::new(transform.translation.x, transform.translation.z);
                    unit.selected = helpers::is_selected(*low, *high, position);
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
}

fn handle_physics(
    time: Res<Time>,
    mut physics_state: ResMut<PhysicsState>,
    mut query_units: Query<(&mut Transform, &mut Unit)>,
) {
    let steps_per_second = 60.0f32;
    let step_time = 1.0 / steps_per_second;
    let max_velocity = 2.0;
    let expected_steps = (time.time_since_startup().as_secs_f32() * steps_per_second) as u64;
    let all_units = query_units
        .iter_mut()
        .map(|(transform, unit)| (*transform, unit.clone()))
        .collect::<Vec<_>>();
    for _ in physics_state.steps_done..expected_steps {
        for (mut transform, mut unit) in query_units.iter_mut() {
            if let Some(desired_position) = unit.desired_position {
                let current_position = Vec2::new(transform.translation.x, transform.translation.z);
                let forward_3d = transform.forward();
                let current_direction = Vec2::new(forward_3d.x, forward_3d.z).normalize();
                let desired_direction = desired_position - current_position;

                let mut seperation_direction = Vec3::zero();
                for (transform_other, unit_other) in &all_units {
                    if unit_other.id != unit.id {
                        let distance = transform.translation.distance(transform_other.translation);
                        if distance < 2.0 {
                            seperation_direction += (1.0 - distance)
                                * (transform_other.translation - transform.translation);
                        }
                    }
                }

                let total_direction = desired_direction.normalize()
                    + Vec2::new(seperation_direction.x, seperation_direction.z);

                let angle = total_direction.angle_between(current_direction);
                let rotation = if angle > std::f32::consts::FRAC_PI_2 * step_time {
                    std::f32::consts::FRAC_PI_2 * step_time
                } else if angle < -std::f32::consts::FRAC_PI_2 * step_time {
                    -std::f32::consts::FRAC_PI_2 * step_time
                } else {
                    angle
                };

                if desired_direction.length() > 2.0 {
                    let movement = transform.forward().normalize() * max_velocity * step_time;
                    transform.translation += movement;
                } else {
                    unit.desired_position = None;
                }
                transform.rotation *= Quat::from_rotation_y(rotation);
            }
        }
    }
    physics_state.steps_done = expected_steps;
}
