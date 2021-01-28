mod components;
mod navigation;
mod resources;

pub use components::SelectionRender;

pub use resources::GameInfo;

use crate::{
    client::{
        components::{Building, CameraCenter, EmptyBundle, Unit},
        navigation::{Cell, FlowField, IVec2},
        resources::{BuildingIdGenerator, FlowFields, PhysicsState, UnitIdGenerator},
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
            .add_resource(BuildingIdGenerator::default())
            .add_resource(PhysicsState::default())
            .add_resource(FlowFields::new(64, 64))
            .add_startup_system(create_world.system())
            .add_system(handle_camera.system())
            .add_system(handle_player.system())
            .add_system(handle_physics.system());
    }
}

fn create_world(
    commands: &mut Commands,
    mut flow_fields: ResMut<FlowFields>,
    mut building_id_generator: ResMut<BuildingIdGenerator>,
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

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 8.0 })),
        material: materials.add(StandardMaterial {
            albedo: Color::rgb(1.0, 0.0, 1.0),
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
    flow_fields
        .flow_field
        .block_position_with_size(&IVec2::new(0, 0), 8);

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
    mut flow_fields: ResMut<FlowFields>,
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
                    .with(Unit::new(unit_id_generator.generate(), *target));
            }
            CommandEvent::Move(target) => {
                flow_fields.flow_field.reset();
                flow_fields.flow_field.set_destination(target.clone());
                flow_fields.flow_field.calculate_flow();
                flow_fields.flow_field.print_flow();
                for (_, _, mut unit) in query_units.iter_mut() {
                    if unit.selected {
                        unit.destination = Some(target.clone());
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

fn steering_flow_field(current: &Unit, flowfield: &FlowField) -> Vec2 {
    let desired_vel = flowfield.get_flow_bilininterpol(&current.position) * current.max_speed;
    let velocity_change = desired_vel - current.velocity;
    velocity_change * (current.max_force / current.max_speed)
}

fn steering_seek(destination: &Vec2, current: &Unit) -> Vec2 {
    let desired_velocity = (*destination - current.position).normalize() * current.max_speed;
    let desired_steering = desired_velocity - current.velocity;
    desired_steering * (current.max_force / current.max_speed)
}

fn steering_seperation(current: &Unit, all_units: &[Unit]) -> Vec2 {
    let mut total = Vec2::zero();
    let mut count = 0;
    for unit in all_units {
        if current.id != unit.id {
            let distance = current.position.distance(unit.position);
            if distance < current.min_seperation + unit.radius + current.radius {
                let push = current.position - unit.position;
                total = total + push / current.radius;
                count += 1;
            }
        }
    }
    if count == 0 {
        Vec2::zero()
    } else {
        (total / count as f32) * current.max_force
    }
}

fn steering_cohesion(current: &Unit, all_units: &[Unit]) -> Vec2 {
    let mut center_of_mass = current.position;
    let mut count = 1;
    for unit in all_units {
        if current.id != unit.id {
            let distance = current.position.distance(unit.position);
            if distance < current.max_cohesion {
                center_of_mass = center_of_mass + unit.position;
                count += 1;
            }
        }
    }
    if count == 1 {
        Vec2::zero()
    } else {
        steering_seek(&(center_of_mass / count as f32), &current)
    }
}

fn steering_alignment(current: &Unit, all_units: &[Unit]) -> Vec2 {
    let mut average_heading = Vec2::zero();
    let mut count = 0;
    for unit in all_units {
        if current.id != unit.id {
            let distance = current.position.distance(unit.position);
            if distance < current.max_cohesion && unit.velocity.length() > 0.0 {
                average_heading = average_heading + unit.velocity.normalize();
                count += 1;
            }
        }
    }
    if count == 0 {
        Vec2::zero()
    } else {
        let desired = average_heading / count as f32 * current.max_speed;
        (desired - current.velocity) * (current.max_force / current.max_speed)
    }
}

fn handle_physics(
    time: Res<Time>,
    mut physics_state: ResMut<PhysicsState>,
    flow_fields: Res<FlowFields>,
    mut query_units: Query<(&mut Transform, &mut Unit)>,
    query_buildings: Query<(&Transform, &Building)>,
) {
    let steps_per_second = 60.0f32;
    let step_time = 1.0 / steps_per_second;
    let expected_steps = (time.time_since_startup().as_secs_f32() * steps_per_second) as u64;

    let all_units = query_units
        .iter_mut()
        .map(|(_, unit)| unit.clone())
        .collect::<Vec<_>>();

    for _ in physics_state.steps_done..expected_steps {
        for (_, mut current) in query_units.iter_mut() {
            if let Some(destination) = current.destination {
                let flow_direction = steering_flow_field(&current, &flow_fields.flow_field);
                let seperation = steering_seperation(&current, all_units.as_slice());
                let cohesion = steering_cohesion(&current, all_units.as_slice());
                let alignment = steering_alignment(&current, all_units.as_slice());
                current.forces = flow_direction + seperation + (cohesion * 0.1) + alignment;
                current.forces = flow_direction;
            }
        }
        for (_, mut unit) in query_units.iter_mut() {
            if let Some(_) = unit.destination {
                unit.velocity = unit.velocity + unit.forces * step_time;
                unit.velocity = if unit.velocity.length() > unit.max_speed {
                    unit.velocity.normalize() * unit.max_speed
                } else {
                    unit.velocity
                };
                unit.position = unit.position + unit.velocity * step_time;
            }
        }
    }

    for (mut transform, unit) in query_units.iter_mut() {
        transform.translation.x = unit.position.x;
        transform.translation.z = unit.position.y;
    }

    physics_state.steps_done = expected_steps;
}
