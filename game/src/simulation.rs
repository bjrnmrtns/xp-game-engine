use crate::client::command::FrameCommand;
use crate::entities::EntityType;
use crate::graphics::clipmap;
use crate::{entities, transformation};
use nalgebra_glm::vec2;
use xp_physics::{collision_response_non_trianulated, Response, Sphere};

pub fn handle_frame(
    frame_commands: Vec<FrameCommand>,
    entities: &mut entities::Entities,
    frame_time: f32,
    clipmap_renderer: &clipmap::Renderer,
) {
    let mut player = entities.get_first_entity_with(EntityType::Player).unwrap();
    for frame_command in frame_commands {
        if let Some(orientation_change) = frame_command.command.orientation_change {
            player.orientation = transformation::rotate_around_local_axis(
                &player.orientation,
                0.0,
                orientation_change.yaw,
                0.0,
            )
        }
        if let Some(movement) = frame_command.command.movement {
            let forward = frame_time * player.max_velocity * movement.forward;
            let right = frame_time * player.max_velocity * movement.right;
            let player_movement =
                transformation::move_along_local_axis(&player.orientation, forward, right, 0.0);
            let sphere_diameter = 2.0;
            let triangles = clipmap_renderer.create_triangle_mesh_around(
                &[
                    vec2(player.position.x, player.position.z),
                    vec2(
                        player.position.x + player_movement.x,
                        player.position.z + player_movement.z,
                    ),
                ],
                sphere_diameter,
            );

            // detect collision player movement
            let response = collision_response_non_trianulated(
                Response {
                    sphere: Sphere {
                        c: player.position,
                        r: 1.0,
                    },
                    movement: player_movement,
                },
                triangles.as_slice(),
            );
            /*
            let gravity_movement = vec3(0.0, -1.0, 0.0) * (3.0 * frame_time);

            // detect collision gravity (constant speed of 20 m/s TODO: fix this to 9.81 m/s2
            let response = collision_response_non_trianulated(
                Response {
                    sphere: Sphere {
                        c: response.sphere.c + response.movement,
                        r: 1.0,
                    },
                    movement: gravity_movement,
                },
                triangles.as_slice(),
            );*/
            player.position = response.sphere.c + response.movement;
        }
        entities.update(player.clone());
    }
}
