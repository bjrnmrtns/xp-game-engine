use crate::client::resources::{MeshMap, WorldAssetHandle, WorldGrid};
use crate::client::{CameraController, CameraPlayerOrbit, CharacterController};
use bevy::prelude::*;
use rapier3d::dynamics::{RigidBodyBuilder, RigidBodySet};
use rapier3d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase};

pub fn create_world_ground_plane(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut bodies: &mut ResMut<RigidBodySet>,
    colliders: &mut ResMut<ColliderSet>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
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
}

pub fn create_cube(
    commands: &mut Commands,
    mut bodies: &mut ResMut<RigidBodySet>,
    colliders: &mut ResMut<ColliderSet>,
    mesh_map: &Res<MeshMap>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    new_grid_cell: (i32, i32, i32),
) -> Entity {
    let rigid_body_cube = RigidBodyBuilder::new_static()
        .translation(
            new_grid_cell.0 as f32 + 0.5,
            new_grid_cell.1 as f32 + 0.5,
            new_grid_cell.2 as f32 + 0.5,
        )
        .build();
    let cube_handle = bodies.insert(rigid_body_cube);
    let collider_cube = ColliderBuilder::cuboid(0.5, 0.5, 0.5).build();
    colliders.insert(collider_cube, cube_handle, &mut bodies);
    commands
        .spawn(PbrBundle {
            mesh: mesh_map.hanldes.get("one_cube").unwrap().clone(),
            material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
            transform: Transform::from_translation(Vec3::new(
                new_grid_cell.0 as f32 + 0.5,
                new_grid_cell.1 as f32 + 0.5,
                new_grid_cell.2 as f32 + 0.5,
            )),
            ..Default::default()
        })
        .current_entity()
        .unwrap()
}

pub fn create_player(
    commands: &mut Commands,
    mut bodies: &mut ResMut<RigidBodySet>,
    colliders: &mut ResMut<ColliderSet>,
    mesh_map: &MeshMap,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let rigid_body_player = RigidBodyBuilder::new_dynamic()
        .translation(0.0, 20.0, 0.0)
        .build();
    let rb_player_handle = bodies.insert(rigid_body_player);
    let collider_player = ColliderBuilder::ball(0.5).friction(0.0).build();
    colliders.insert(collider_player, rb_player_handle, &mut bodies);
    commands
        .spawn(PbrBundle {
            mesh: mesh_map.hanldes.get("player").unwrap().clone(),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .with(rb_player_handle)
        .with_children(|parent| {
            parent
                .spawn(CameraPlayerOrbit {
                    transform: Transform::identity(),
                    global_transform: GlobalTransform::identity(),
                })
                .with_children(|parent| {
                    let mut third_person_camera_transform =
                        Transform::from_translation(Vec3::new(-1.0, 2.0, -8.0));
                    third_person_camera_transform.rotation =
                        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI))
                            .rotation;
                    parent.spawn(Camera3dBundle {
                        transform: third_person_camera_transform,
                        ..Default::default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: mesh_map.hanldes.get("one_cube").unwrap().clone(),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
                        ..Default::default()
                    });
                })
                .with(CameraController::new());
        })
        .with(CharacterController::new())
        .current_entity()
        .unwrap()
}
