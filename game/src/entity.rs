use crate::client::command::FrameCommand;
use crate::input::player_input_state::{ForwardMovement, StrafeMovement};
use crate::terrain::Generator;
use crate::transformation;
use nalgebra_glm::{identity, quat_identity, quat_to_mat4, translate, vec3, Mat4, Quat, Vec3};

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
    pub collider: Collider,
    pub velocity: f32,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            pose: Pose {
                position: vec3(0.0, 20.0, 0.0),
                orientation: quat_identity(),
            },
            collider: Collider::Sphere { radius: 1.0 },
            velocity: 3.0,
        }
    }

    pub fn handle_frame(
        &mut self,
        frame_command: FrameCommand,
        frame_time: f32,
        generator: &dyn Generator,
    ) {
        if let Some(orientation_change) = frame_command.command.orientation_change {
            self.orient(orientation_change.horizontal);
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
        self.move_(forward, right);
        self.pose.position.y -= 9.81 * frame_time;
        let height_at_pos = generator.generate([self.pose.position.x, self.pose.position.z]);
        if self.pose.position.y < height_at_pos + 1.0 {
            self.pose.position.y = height_at_pos + 1.0;
        };
    }

    pub fn move_(&mut self, forward: f32, right: f32) {
        self.pose.position = transformation::move_along_local_axis(
            &self.pose.position,
            &self.pose.orientation,
            forward,
            right,
            0.0,
        );
    }
    pub fn orient(&mut self, around_y: f32) {
        self.pose.orientation =
            transformation::rotate_around_local_axis(&self.pose.orientation, 0.0, around_y, 0.0);
    }
}
