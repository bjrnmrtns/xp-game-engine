mod resources;

pub use resources::PhysicsSteps;

use bevy::prelude::*;
use rapier3d::dynamics::{IntegrationParameters, JointSet, RigidBodySet};
use rapier3d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier3d::ncollide::na::Vector3;
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
) {
    let expected_steps = (time.seconds_since_startup / integration_parameters.dt() as f64) as u64;
    for _ in physics_steps.done..expected_steps {
        pipeline.step(
            &(Vector3::y() * -9.81),
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
