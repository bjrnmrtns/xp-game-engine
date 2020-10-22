use crate::client::command::FrameCommand;
use crate::graphics::clipmap;
use crate::scene;
use crate::transformation;
use nalgebra_glm::vec2;
use xp_physics::{collision_response_non_trianulated, Response, Sphere};

pub fn handle_frame(
    frame_commands: Vec<FrameCommand>,
    player: &mut scene::Entity,
    frame_time: f32,
    clipmap_renderer: &clipmap::Renderer,
) {
    for frame_command in frame_commands {
        if let scene::Entity::Player { pose, max_velocity } = player {
            if let Some(orientation_change) = &frame_command.command.orientation_change {
                pose.orientation = transformation::rotate_around_local_axis(
                    &pose.orientation,
                    0.0,
                    orientation_change.yaw,
                    0.0,
                )
            }
            if let Some(movement) = &frame_command.command.movement {
                let forward = frame_time * *max_velocity * movement.forward;
                let right = frame_time * *max_velocity * movement.right;
                let movement =
                    transformation::move_along_local_axis(&pose.orientation, forward, right, 0.0);
                let sphere_diameter = 2.0;
                let triangles = clipmap_renderer.create_triangle_mesh_around(
                    &[
                        vec2(pose.position.x, pose.position.z),
                        vec2(pose.position.x + movement.x, pose.position.z + movement.z),
                    ],
                    sphere_diameter,
                );

                // detect collision player movement
                let response = collision_response_non_trianulated(
                    Response {
                        sphere: Sphere {
                            c: pose.position,
                            r: 1.0,
                        },
                        movement,
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
                pose.position = response.sphere.c + response.movement;
            }
        }
    }
}
