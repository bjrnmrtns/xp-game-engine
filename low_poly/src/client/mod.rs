mod components;
mod resources;

pub use components::CameraController;
pub use components::CameraNodeThirdPerson;
pub use components::CharacterController;

use crate::client::resources::WorldResource;
use bevy::prelude::*;
use rapier3d::dynamics::{RigidBodyBuilder, RigidBodySet};
use rapier3d::geometry::{ColliderBuilder, ColliderSet};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(WorldResource::default())
            .add_startup_system(client_startup_system.system())
            .add_system(handle_player_camera.system())
            .add_system(create_world.system());
    }
}

fn client_startup_system(
    mut world_resource: ResMut<WorldResource>,
    asset_server: Res<AssetServer>,
) {
    world_resource.handle = asset_server.load("world.world");
}

fn create_world(
    mut world_resource: ResMut<WorldResource>,
    world_assets: Res<Assets<crate::world_loader::World>>,
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !world_resource.loaded {
        let cube_size = 1.0;
        let mesh_handle = meshes.add(Mesh::from(shape::Cube { size: cube_size }));
        if let Some(world) = world_assets.get(&world_resource.handle) {
            for (x, y, z) in &world.objects {
                let rigid_body_cube = RigidBodyBuilder::new_static()
                    .translation(
                        *x as f32 + cube_size / 2.0,
                        *y as f32 + cube_size / 2.0,
                        *z as f32 + cube_size / 2.0,
                    )
                    .build();
                let cube_handle = bodies.insert(rigid_body_cube);
                let collider_cube = ColliderBuilder::cuboid(0.5, 0.5, 0.5).build();
                colliders.insert(collider_cube, cube_handle, &mut bodies);
                commands.spawn(PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                    transform: Transform::from_translation(Vec3::new(
                        *x as f32 + cube_size / 2.0,
                        *y as f32 + cube_size / 2.0,
                        *z as f32 + cube_size / 2.0,
                    )),
                    ..Default::default()
                });
            }
            world_resource.loaded = true;

            let grid_texture_handle = asset_server.load("grid.png");
            let rigid_body_ground = RigidBodyBuilder::new_static()
                .translation(0.0, -0.1, 0.0)
                .build();
            let rb_ground_handle = bodies.insert(rigid_body_ground);
            let collider_ground = ColliderBuilder::cuboid(12.0, 0.2, 12.0).build();
            colliders.insert(collider_ground, rb_ground_handle, &mut bodies);
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: 24.0 })),
                material: materials.add(StandardMaterial {
                    albedo_texture: Some(grid_texture_handle),
                    shaded: false,
                    ..Default::default()
                }),
                ..Default::default()
            });

            let rigid_body_cube = RigidBodyBuilder::new_static()
                .translation(-8.0, 2.0, -8.0)
                .build();
            let rb_cube_handle = bodies.insert(rigid_body_cube);
            let collider_cube = ColliderBuilder::cuboid(2.0, 2.0, 2.0).build();
            colliders.insert(collider_cube, rb_cube_handle, &mut bodies);
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_translation(Vec3::new(-8.0, 2.0, -8.0)),
                ..Default::default()
            });

            let one_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

            let rigid_body_stepup_cube = RigidBodyBuilder::new_static()
                .translation(8.0, 0.2, 8.0)
                .build();
            let rb_stepup_cube_handle = bodies.insert(rigid_body_stepup_cube);
            let collider_stepup_cube = ColliderBuilder::cuboid(2.0, 0.2, 2.0).build();
            colliders.insert(collider_stepup_cube, rb_stepup_cube_handle, &mut bodies);
            commands.spawn(PbrBundle {
                mesh: one_cube.clone(),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_translation(Vec3::new(8.0, 0.2, 8.0))
                    .mul_transform(Transform::from_scale(Vec3::new(4.0, 0.4, 4.0))),
                ..Default::default()
            });

            commands.spawn(LightBundle {
                transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
                ..Default::default()
            });

            let rigid_body_player = RigidBodyBuilder::new_dynamic()
                .translation(0.0, 20.0, 0.0)
                .build();
            let rb_player_handle = bodies.insert(rigid_body_player);
            let collider_player = ColliderBuilder::ball(1.0).friction(0.0).build();
            colliders.insert(collider_player, rb_player_handle, &mut bodies);
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 1.0,
                        subdivisions: 3,
                    })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    ..Default::default()
                })
                .with(rb_player_handle)
                .with_children(|parent| {
                    parent
                        .spawn(CameraNodeThirdPerson {
                            transform: Transform::identity(),
                            global_transform: GlobalTransform::identity(),
                        })
                        .with_children(|parent| {
                            let mut transform =
                                Transform::from_translation(Vec3::new(-1.0, 1.0, -8.0));
                            transform.rotation = Transform::from_rotation(Quat::from_rotation_y(
                                std::f32::consts::PI,
                            ))
                            .rotation;
                            parent.spawn(Camera3dBundle {
                                transform,
                                ..Default::default()
                            });
                        })
                        .with(CameraController::new());
                })
                .with(CharacterController::new());
        }
    }
}

fn handle_player_camera(mut query: Query<(&CameraController, &mut Transform)>) {
    for (camera_controller, mut camera_orbit) in query.iter_mut() {
        camera_orbit.rotate(Quat::from_rotation_x(camera_controller.rotate_x / 100.0));
    }
}
