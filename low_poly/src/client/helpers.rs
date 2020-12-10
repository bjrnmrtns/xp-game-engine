use crate::client::components::ToolCenter;
use crate::client::resources::MeshMap;
use crate::client::{CameraController, CameraPivot, CharacterController};
use bevy::prelude::*;
use rapier3d::dynamics::{RigidBodyBuilder, RigidBodySet};
use rapier3d::geometry::{ColliderBuilder, ColliderSet};

pub fn create_world_ground_plane(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut bodies: &mut ResMut<RigidBodySet>,
    colliders: &mut ResMut<ColliderSet>,
    mesh_map: &Res<MeshMap>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let grid_texture_handle = asset_server.load("grid.png");
    let rigid_body_ground = RigidBodyBuilder::new_static()
        .translation(0.0, -0.6, 0.0)
        .build();
    let rb_ground_handle = bodies.insert(rigid_body_ground);
    let collider_ground = ColliderBuilder::cuboid(12.0, 0.2, 12.0).build();
    colliders.insert(collider_ground, rb_ground_handle, &mut bodies);
    commands.spawn(PbrBundle {
        mesh: mesh_map.hanldes.get("ground_plane").unwrap().clone(),
        material: materials.add(StandardMaterial {
            albedo_texture: Some(grid_texture_handle),
            shaded: false,
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)),
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
            new_grid_cell.0 as f32,
            new_grid_cell.1 as f32,
            new_grid_cell.2 as f32,
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
                new_grid_cell.0 as f32,
                new_grid_cell.1 as f32,
                new_grid_cell.2 as f32,
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
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .translation(0.0, 20.0, 0.0)
        .build();
    let rigid_body_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::ball(0.5).friction(0.0).build();
    let collider_handle = colliders.insert(collider, rigid_body_handle, &mut bodies);
    commands
        .spawn(PbrBundle {
            mesh: mesh_map.hanldes.get("player").unwrap().clone(),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .with(rigid_body_handle)
        .with(collider_handle)
        .with(CharacterController::new())
        .with_children(|parent| {
            parent
                .spawn(PbrBundle {
                    mesh: mesh_map.hanldes.get("tool").unwrap().clone(),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
                    ..Default::default()
                })
                .with(ToolCenter);
            parent
                .spawn(CameraPivot)
                .with(GlobalTransform::identity())
                .with(Transform::identity())
                .with_children(|parent| {
                    parent.spawn(Camera3dBundle {
                        transform: Transform::from_translation(Vec3::new(-1.0, 2.0, -8.0))
                            .mul_transform(Transform::from_rotation(Quat::from_rotation_y(
                                std::f32::consts::PI,
                            ))),
                        ..Default::default()
                    });
                })
                .with(CameraController::new());
        })
        .current_entity()
        .unwrap()
}
