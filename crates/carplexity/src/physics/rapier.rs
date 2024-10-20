use std::future::Future;

use bevy::prelude::*;
use rapier3d::prelude::*;

use crate::SECONDS_PER_TICK;

pub struct RapierPhysics {
	rigid_body_set: RigidBodySet,
	collider_set: ColliderSet,
	pipeline: PhysicsPipeline,
	parameters: Parameters,
}

struct Parameters {
	gravity: nalgebra::Vector3<f32>,
	integration_parameters: IntegrationParameters,
	island_manager: IslandManager,
	broad_phase: DefaultBroadPhase,
	narrow_phase: NarrowPhase,
	impulse_joint_set: ImpulseJointSet,
	multibody_joint_set: MultibodyJointSet,
	ccd_solver: CCDSolver,
	query_pipeline: QueryPipeline,
	physics_hooks: (),
	event_handler: (),
}

impl RapierPhysics {
	pub fn new() -> Self {
		let mut rigid_body_set = RigidBodySet::new();
		let mut collider_set = ColliderSet::new();

		let parameters = Parameters {
			gravity: vector![0.0, -9.81, 0.0],
			integration_parameters: IntegrationParameters {
				dt: SECONDS_PER_TICK,
				..IntegrationParameters::default()
			},
			island_manager: IslandManager::new(),
			broad_phase: DefaultBroadPhase::new(),
			narrow_phase: NarrowPhase::new(),
			impulse_joint_set: ImpulseJointSet::new(),
			multibody_joint_set: MultibodyJointSet::new(),
			ccd_solver: CCDSolver::new(),
			query_pipeline: QueryPipeline::new(),
			physics_hooks: (),
			event_handler: (),
		};

		RapierPhysics {
			rigid_body_set,
			collider_set,
			pipeline: PhysicsPipeline::new(),
			parameters,
		}
	}

	pub fn create_rigid_body(&mut self, collider: crate::physics::Collider) -> RigidBodyHandle {
		match collider {
			crate::physics::Collider::Car => {
				let rigid_body = RigidBodyBuilder::dynamic()
					.translation(vector![0.0, 10.0, 0.0])
					.build();
				let collider = ColliderBuilder::cuboid(1.0, 1.0, 1.0).build();
				let rigid_body_handle = self.rigid_body_set.insert(rigid_body);
				self.collider_set.insert_with_parent(
					collider,
					rigid_body_handle,
					&mut self.rigid_body_set,
				);
				rigid_body_handle
			}
			crate::physics::Collider::Ball => {
				let rigid_body = RigidBodyBuilder::dynamic()
					.translation(vector![0.0, 10.0, 0.0])
					.build();
				let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
				let rigid_body_handle = self.rigid_body_set.insert(rigid_body);
				self.collider_set.insert_with_parent(
					collider,
					rigid_body_handle,
					&mut self.rigid_body_set,
				);
				rigid_body_handle
			}
		}
	}
}

impl super::PhysicsEngine for RapierPhysics {
	fn update(&mut self, delta_time: f32) -> Result<(), String> {
		self.pipeline.step(
			&self.parameters.gravity,
			&self.parameters.integration_parameters,
			&mut self.parameters.island_manager,
			&mut self.parameters.broad_phase,
			&mut self.parameters.narrow_phase,
			&mut self.rigid_body_set,
			&mut self.collider_set,
			&mut self.parameters.impulse_joint_set,
			&mut self.parameters.multibody_joint_set,
			&mut self.parameters.ccd_solver,
			Some(&mut self.parameters.query_pipeline),
			&self.parameters.physics_hooks,
			&self.parameters.event_handler,
		);
		Ok(())
	}

	// fn add_collider(&mut self, entity: Entity, collider: crate::physics::Collider) {
	// 	let rigid_body_handle = self.create_rigid_body(collider);
	// 	// self.collider_set.insert();
	// }

	// fn remove_collider(&mut self, entity: Entity) {
	// 	// TODO
	// 	// self.collider_set.remove(entity);
	// }
}

// fn remove_missing_entities(entities: Query<Entity, With<crate::physics::Collider>>, rapier_physics: &mut RapierPhysics) {
// 	for entity in &entities {
// 		if !self.collider_set.contains_key(entity) {
// 			self.collider_set.remove(entity);
// 		}
// 	}
// }
