use crate::{
    controllers::CharacterController,
    entity::Entity,
    physics::{
        collisionshape::{Body, BodyStatus},
        CollisionShape,
    },
    registry::{Handle, Registry},
};
use glam::Quat;
use rapier3d::{
    dynamics::{CCDSolver, IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet},
    geometry::{BroadPhase, ColliderBuilder, ColliderHandle, ColliderSet, NarrowPhase},
    na::Vector3,
    pipeline::PhysicsPipeline,
};
use std::collections::HashMap;

struct PhysicsObjectHandle {
    r: RigidBodyHandle,
    c: ColliderHandle,
}

pub struct Physics {
    int_params: IntegrationParameters,
    pipeline: PhysicsPipeline,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
    ccd_solver: CCDSolver,
    physics_objects_dynamic: HashMap<u64, PhysicsObjectHandle>,
    character: Option<Handle<Entity>>,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            int_params: Default::default(),
            pipeline: Default::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            joints: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_objects_dynamic: HashMap::new(),
            character: None,
        }
    }
}

impl Physics {
    pub fn step(&mut self, entities: &mut Registry<Entity>, character_controller: &CharacterController) {
        let step_time = 1.0 / 60.0;
        if let Some(entity_handle) = self.character.clone() {
            let entity = entities.get_mut(entity_handle).unwrap();
            entity.transform.rotation *= Quat::from_rotation_y(-character_controller.rotate * 0.02);
            entity.transform.translation += entity.transform.forward() * character_controller.forward * step_time * 5.0;
        }
        self.pipeline.step(
            &Vector3::new(0.0, 0.0, 0.0),
            &self.int_params,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            &(),
            &(),
        );
    }

    pub fn register_character(&mut self, entity_handle: Handle<Entity>) {
        self.character = Some(entity_handle);
    }

    pub fn register(&mut self, entity_handle: Handle<Entity>, entities: &Registry<Entity>) {
        if let Some(entity) = entities.get(&entity_handle) {
            if let Some(collision_shape) = &entity.collision_shape {
                let collider = match &collision_shape.body {
                    Body::Cuboid(cuboid) => {
                        ColliderBuilder::cuboid(cuboid.half_extent_x, cuboid.half_extent_y, cuboid.half_extent_z)
                            .friction(0.0)
                            .build()
                    }
                    Body::Sphere(sphere) => ColliderBuilder::ball(sphere.radius).friction(0.0).build(),
                };
                let translation = entity.transform.translation;
                let rigid_body = match &collision_shape.body_status {
                    BodyStatus::Static => RigidBodyBuilder::new_static()
                        .translation(translation.x, translation.y, translation.z)
                        .build(),
                    BodyStatus::Dynamic => RigidBodyBuilder::new_dynamic()
                        .translation(translation.x, translation.y, translation.z)
                        .build(),
                };
                let r = self.bodies.insert(rigid_body);
                let c = self.colliders.insert(collider, r, &mut self.bodies);
                match &collision_shape.body_status {
                    BodyStatus::Static => (),
                    BodyStatus::Dynamic => {
                        self.physics_objects_dynamic
                            .insert(entity_handle.id, PhysicsObjectHandle { r, c });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rapier2d::{
        dynamics::{IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodySet},
        geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase},
        na::{Isometry2, Vector2},
        pipeline::PhysicsPipeline,
    };

    #[test]
    fn try_rapier() {
        let int_params = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();
        let mut joints = JointSet::new();
        let physics_hooks = ();
        let physics_events = ();

        colliders.insert(
            ColliderBuilder::cuboid(1.0, 1.0).build(),
            bodies.insert(RigidBodyBuilder::new_static().translation(0.0, -5.0).build()),
            &mut bodies,
        );

        let rigid_body_handle = bodies.insert(RigidBodyBuilder::new_dynamic().translation(0.0, 0.0).build());
        let collider = ColliderBuilder::ball(0.5).friction(0.0).build();
        let collider_handle = colliders.insert(collider, rigid_body_handle, &mut bodies);
        for _ in 0..1000 {
            bodies
                .get_mut(rigid_body_handle)
                .unwrap()
                .set_linvel(Vector2::new(0.0, -1.0), true);
            /*bodies
               .get_mut(rigid_body_handle)
               .unwrap()
               .set_position(Isometry2::new(Vector2::new(0.0, 0.0), 0.0), true);
            */
            physics_pipeline.step(
                &(Vector2::y() * 0.0),
                &int_params,
                &mut broad_phase,
                &mut narrow_phase,
                &mut bodies,
                &mut colliders,
                &mut joints,
                &physics_hooks,
                &physics_events,
            );
            let rb = bodies.get(rigid_body_handle).unwrap();
            let translation = rb.position().translation;
            println!("{} {}", translation.x, translation.y);
        }
    }
}
