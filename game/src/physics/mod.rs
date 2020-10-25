use nalgebra_glm::{vec3, Vec3};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::nalgebra::{Isometry3, Vector3};
use nphysics3d::ncollide3d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics3d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet, Ground,
    RigidBodyDesc,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

pub struct Physics {
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometrical_world: DefaultGeometricalWorld<f64>,
    body_set: DefaultBodySet<f64>,
    colliders: DefaultColliderSet<f64>,
    constraints: DefaultJointConstraintSet<f64>,
    forces: DefaultForceGeneratorSet<f64>,
    rb_ball_handle: DefaultBodyHandle,
}

impl Physics {
    pub fn new(time_step: f64) -> Self {
        let mut mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
        mechanical_world.set_timestep(time_step);
        let geometrical_world = DefaultGeometricalWorld::new();
        let mut body_set = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();
        let ground_thickness = 0.2;
        let ground_width = 10.0;
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector3::new(
            ground_width,
            ground_thickness,
            ground_width,
        )));
        let ground_handle = body_set.insert(Ground::new());
        let ground_collider = ColliderDesc::new(ground_shape)
            .translation(Vector3::y() * -ground_thickness)
            .build(BodyPartHandle(ground_handle, 0));
        colliders.insert(ground_collider);
        let ball_shape = ShapeHandle::new(Ball::new(0.5));
        let rb_ball = RigidBodyDesc::new()
            .translation(Vector3::new(0.0, 20.0, 0.0))
            .build();
        let rb_ball_handle = body_set.insert(rb_ball);
        let rb_collider = ColliderDesc::new(ball_shape.clone())
            .density(1.0)
            .build(BodyPartHandle(rb_ball_handle, 0));
        colliders.insert(rb_collider);
        Self {
            mechanical_world,
            geometrical_world,
            body_set,
            colliders,
            constraints: DefaultJointConstraintSet::new(),
            forces: DefaultForceGeneratorSet::new(),
            rb_ball_handle,
        }
    }
    pub fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.body_set,
            &mut self.colliders,
            &mut self.constraints,
            &mut self.forces,
        );
    }
    pub fn get_position_ball(&self) -> Vec3 {
        let pos = self
            .body_set
            .get(self.rb_ball_handle)
            .unwrap()
            .part(0)
            .unwrap()
            .position()
            .translation
            .vector;
        vec3(pos.x as f32, pos.y as f32, pos.z as f32)
    }
    pub fn move_ball(&mut self, v: Vec3) {
        let pos = self.get_position_ball() + v;
        let rb = self.body_set.rigid_body_mut(self.rb_ball_handle).unwrap();
        rb.set_position(Isometry3::translation(
            pos.x as f64,
            pos.y as f64,
            pos.z as f64,
        ));
    }
}
