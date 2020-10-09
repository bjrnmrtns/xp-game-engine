use crate::client::command::FrameCommand;
use crate::graphics::clipmap;
use crate::input::player_input_state::{ForwardMovement, StrafeMovement};
use crate::transformation;
use nalgebra_glm::{
    identity, quat_identity, quat_to_mat4, translate, vec2, vec3, Mat4, Quat, Vec3,
};
use xp_physics::{collision_response_non_trianulated, Response, Sphere};

pub enum Collider {
    Sphere { radius: f32 },
}

pub struct Pose {
    pub position: Vec3,
    pub orientation: Quat,
}

impl Pose {
    pub fn to_mat4(&self) -> Mat4 {
        let translate = translate(&identity(), &self.position);
        let rotate = quat_to_mat4(&self.orientation);
        translate * rotate
    }
}

pub struct Entity {
    pub pose: Pose,
    pub velocity: f32,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            pose: Pose {
                position: vec3(0.0, 1.0, 0.0),
                orientation: quat_identity(),
            },
            velocity: 3.0,
        }
    }

    pub fn handle_frame(
        &mut self,
        frame_command: FrameCommand,
        frame_time: f32,
        clipmap_renderer: &clipmap::Renderable,
    ) {
        if let Some(orientation_change) = frame_command.command.orientation_change {
            self.pose.orientation = transformation::rotate_around_local_axis(
                &self.pose.orientation,
                0.0,
                orientation_change.horizontal,
                0.0,
            )
        }
        let forward = match frame_command.command.forward {
            Some(ForwardMovement::Positive) => frame_time * self.velocity,
            Some(ForwardMovement::Negative) => frame_time * -self.velocity,
            None => 0.0,
        };
        let right = match frame_command.command.strafe {
            Some(StrafeMovement::Right) => frame_time * self.velocity,
            Some(StrafeMovement::Left) => frame_time * -self.velocity,
            None => 0.0,
        };
        let player_movement =
            transformation::move_along_local_axis(&self.pose.orientation, forward, right, 0.0);
        let sphere_diameter = 2.0;
        let triangles = clipmap_renderer.create_triangle_mesh_around(
            &[
                vec2(self.pose.position.x, self.pose.position.z),
                vec2(
                    self.pose.position.x + player_movement.x,
                    self.pose.position.z + player_movement.z,
                ),
            ],
            sphere_diameter,
        );

        // detect collision player movement
        let response = collision_response_non_trianulated(
            Response {
                sphere: Sphere {
                    c: self.pose.position,
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
        self.pose.position = response.sphere.c + response.movement;
    }
}
