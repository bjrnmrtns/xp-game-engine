mod resources;

pub use resources::PhysicsSteps;

use crate::client::CharacterController;
use bevy::prelude::*;
use rapier3d::dynamics::{IntegrationParameters, JointSet, RigidBodyHandle, RigidBodySet};
use rapier3d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier3d::ncollide::na::{Quaternion, UnitQuaternion, Vector3};
use rapier3d::pipeline::PhysicsPipeline;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(IntegrationParameters::default())
            .add_resource(PhysicsPipeline::new())
            .add_resource(BroadPhase::new())
            .add_resource(NarrowPhase::new())
            .add_resource(RigidBodySet::new())
            .add_resource(ColliderSet::new())
            .add_resource(JointSet::new())
            .add_resource(PhysicsSteps::new())
            .add_system(physics_system.system());
    }
}

fn physics_system(
    time: Res<Time>,
    integration_parameters: Res<IntegrationParameters>,
    mut physics_steps: ResMut<PhysicsSteps>,
    mut pipeline: ResMut<PhysicsPipeline>,
    mut broad_phase: ResMut<BroadPhase>,
    mut narrow_phase: ResMut<NarrowPhase>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
    mut query: Query<(&CharacterController, &mut Transform, &RigidBodyHandle)>,
) {
    for (character_controller, mut transform, rigid_body_handle) in query.iter_mut() {
        let mut rb = bodies.get_mut(*rigid_body_handle).unwrap();
        let translation = rb.position.translation;
        // translation of physics engine is leading
        transform.translation = Vec3::new(translation.x, translation.y, translation.z);
        transform.rotate(Quat::from_rotation_y(character_controller.rotate_y / 100.0));
        // rotation of controller is leading
        rb.position.rotation = UnitQuaternion::from_quaternion(Quaternion::from([
            transform.rotation.x,
            transform.rotation.y,
            transform.rotation.z,
            transform.rotation.w,
        ]));
        rb.mass_properties.inv_principal_inertia_sqrt = Vector3::new(0.0, 0.0, 0.0);

        let jump_speed = if character_controller.jump { 10.0 } else { 0.0 };

        if let Some(move_forward) = character_controller.move_forward {
            let movement = transform.forward().normalize() * move_forward * 10.0;
            rb.wake_up(true);
            rb.linvel = Vector3::new(movement.x, rb.linvel.y + jump_speed, movement.z);
        } else {
            rb.linvel = Vector3::new(0.0, rb.linvel.y + jump_speed, 0.0);
        }
    }
    let expected_steps = (time.seconds_since_startup / integration_parameters.dt() as f64) as u64;
    for _ in physics_steps.done..expected_steps {
        pipeline.step(
            &(Vector3::y() * -40.0),
            &integration_parameters,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
            None,
            None,
            &(),
        );
    }
    physics_steps.done = expected_steps;
}
