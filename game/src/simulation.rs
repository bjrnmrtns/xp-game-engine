use crate::scene::Entity;
use crate::{physics, transformation};
use crate::{scene, window_input};

pub trait FrameInputHandler {
    fn handle(
        &mut self,
        frame: u64,
        command: &window_input::input_state::InputState,
        player: &mut scene::Entity,
        frame_time: f32,
    );
}

pub struct Client {
    physics: physics::Physics,
    last_frame: Option<u64>,
}

impl Client {
    pub fn new(time_step: f64) -> Self {
        Self {
            physics: physics::Physics::new(time_step),
            last_frame: None,
        }
    }
}

impl FrameInputHandler for Client {
    fn handle(
        &mut self,
        frame: u64,
        input_state: &window_input::input_state::InputState,
        player: &mut Entity,
        frame_time: f32,
    ) {
        assert!(self.last_frame < Some(frame));
        self.last_frame = Some(frame);
        if let scene::Entity::Player { pose, max_velocity } = player {
            if let Some(movement) = &input_state.movement {
                let forward = frame_time * *max_velocity * movement.forward;
                let right = frame_time * *max_velocity * movement.right;
                let movement =
                    transformation::move_along_local_axis(&pose.orientation, forward, right, 0.0);
                self.physics.move_ball(movement);
            }
        }
        self.physics.step();
        if let scene::Entity::Player { pose, .. } = player {
            pose.position = self.physics.get_position_ball();
        }
    }
}
