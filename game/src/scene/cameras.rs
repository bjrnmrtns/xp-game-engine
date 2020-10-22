use crate::scene;
use crate::scene::view_on;
use nalgebra_glm::*;

pub enum Camera {
    Freelook { position: Vec3, direction: Vec3 },
    Follow,
}

pub struct Cameras {
    cameras: Vec<Camera>,
    selected: usize,
}

impl Cameras {
    pub fn new() -> Self {
        Self {
            cameras: Vec::new(),
            selected: 0,
        }
    }
    pub fn add(&mut self, camera: Camera) {
        self.cameras.push(camera);
    }
    pub fn toggle(&mut self, toggle: usize) {
        self.selected = (self.selected + toggle) % self.cameras.len();
    }
    pub fn get_selected(&mut self) -> Option<&mut Camera> {
        assert!(self.cameras.len() > 0);
        Some(&mut self.cameras[self.selected])
    }
    pub fn get_view(&self, player: &scene::Entity) -> Mat4 {
        assert!(self.cameras.len() > 0);
        match &self.cameras[self.selected] {
            Camera::Follow => {
                if let scene::Entity::Player { pose, .. } = player {
                    return view_on(&pose).0;
                }
            }
            Camera::Freelook {
                position,
                direction,
            } => return look_at(&position, &(position + direction), &vec3(0.0, 1.0, 0.0)),
        }
        identity() as Mat4
    }
}

/*
impl Freelook {
    fn right_vector(&self) -> Vec3 {
        cross(&self.direction, &vec3(0.0, 1.0, 0.0))
    }

    pub fn new(position: Vec3, direction: Vec3) -> Freelook {
        Freelook {
            position,
            direction,
        }
    }

    pub fn move_(&mut self, forward: f32, right: f32) {
        self.position = &self.position + &self.direction * forward + self.right_vector() * right;
    }

    pub fn camera_rotate(&mut self, updown: f32, around: f32) {
        let temp_direction =
            &rotate_vec3(&self.direction, around, &vec3(0.0, 1.0, 0.0)).normalize();
        self.direction = rotate_vec3(&temp_direction, updown, &self.right_vector()).normalize()
    }

    pub fn view(&self) -> Mat4 {
        look_at(
            &self.position,
            &(&self.position + &self.direction),
            &vec3(0.0, 1.0, 0.0),
        )
    }
}

 */
