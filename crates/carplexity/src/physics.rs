use std::{collections::HashSet, future::Future};

use bevy::prelude::*;
pub use rapier::RapierPhysics;
use rapier3d::{na, prelude::RigidBodyBuilder};

use crate::{room::Room, SECONDS_PER_TICK};

mod rapier;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(FixedUpdate, update_time_warp)
			.add_systems(FixedUpdate, run_arena_physics.after(update_time_warp))
			.add_systems(Update, apply_movement_state);
	}
}

#[derive(Component)]
pub struct Physics {
	pub engine: Box<dyn PhysicsEngine>,
	pub colliders: HashSet<Entity>,
}

#[derive(Bundle)]
pub struct PhysicsBundle {
	pub physics: Physics,
	pub time_warp: TimeWarp,
	pub authority: Authority,
}

/// Whether the physics state is being updated locally or received from the network, i.e. single player or online multiplayer.
#[derive(Component)]
pub enum Authority {
	Local,
	Remote,
}

#[derive(Component)]
pub struct MovementState {
	pub position: Vec3,
	pub velocity: Vec3,
	pub rotation: Quat,
	pub angular_velocity: Vec3,
}

pub trait PhysicsEngine: Send + Sync {
	// type State: Send + Sync + Clone;

	fn update(&mut self, delta_time: f32) -> Result<(), String>;

	// TODO: Is a more stateless/functional design possible?
	// fn add_collider(&mut self, entity: Entity, collider: Collider);
	// fn remove_collider(&mut self, entity: Entity);
}

#[derive(Component)]
pub enum Collider {
	Car,
	Ball,
}

impl Default for Physics {
	fn default() -> Self {
		Self {
			engine: Box::new(RapierPhysics::new()),
			colliders: HashSet::new(),
		}
	}
}

#[derive(Component)]
pub struct TimeWarp {
	/// The tick that is being rendered.
	pub current_tick: u64,
	/// An estimate of how many ticks the tick, the state of which we should now be receiving from the network, is now lagging behind.
	pub target_lag: f32,
	/// The amount of time that should pass for every second of real time. Usually 1.0, lower when the server can't keep up.
	pub time_dilation: f32,
}

impl TimeWarp {
	fn new() -> Self {
		Self {
			current_tick: 0,
			target_lag: 0.0,
			time_dilation: 1.0,
		}
	}
}

fn test() {
	let rigid_body = RigidBodyBuilder::dynamic()
		.translation(na::Vector3::new(0.0, 0.0, 0.0))
		.build();
}

fn update_time_warp(mut time_warp: Query<(&mut TimeWarp, &Authority)>) {
	for (mut time_warp, authority) in &mut time_warp {
		time_warp.current_tick += 1;

		match authority {
			Authority::Local => {
				time_warp.target_lag = 0.0;
				time_warp.time_dilation = 1.0;
			}
			Authority::Remote => {
				// TODO: Calculate
			}
		}
	}
}

fn apply_movement_state(
	fixed_time: Res<Time<Fixed>>,
	mut state_query: Query<(Option<&Parent>, &mut Transform, &MovementState)>,
	physics_query: Query<(&Room, &Physics, &TimeWarp)>,
) {
	for (parent, mut transform, movement_state) in &mut state_query {
		let ticks_ahead = fixed_time.overstep_fraction();
		let time_dilation = match parent.and_then(|parent| physics_query.get(parent.get()).ok()) {
			Some((_, _, time_warp)) => time_warp.time_dilation,
			None => 1.0,
		};
		let delta_seconds = fixed_time.delta_seconds() * time_dilation;

		let future_position = movement_state.position + movement_state.velocity * delta_seconds;
		transform.translation = movement_state.position.lerp(future_position, ticks_ahead);

		let future_rotation = movement_state.rotation
			* Quat::from_axis_angle(
				movement_state.angular_velocity.normalize(),
				movement_state.angular_velocity.length() * delta_seconds,
			);
		transform.rotation = movement_state.rotation.slerp(future_rotation, ticks_ahead);
	}
}

// fn retroactively_apply_inputs(
// 	mut arena_physics: Query<(&mut Physics, &mut Arena)>,
// 	mut input_queue: ResMut<InputQueue>,
// ) {
// 	for (mut physics, mut arena) in arena_physics.iter_mut() {
// 		while let Some((tick, input_state)) = input_queue.pop() {
// 			// Rewind the physics state to the tick of the input
// 			physics.engine.set_state(tick);

// 			// Apply the input state
// 			arena.apply_input(input_state);

// 			// Simulate forward to the current tick
// 			let current_tick = physics.engine.current_tick();
// 			for _ in tick..current_tick {
// 				physics.engine.step();
// 			}
// 		}
// 	}
// }

fn run_arena_physics(
	mut arena_physics: Query<(&mut Physics, &mut Room, &TimeWarp)>,
	colliders: Query<Entity, With<Collider>>,
) {
	arena_physics
		.par_iter_mut()
		.for_each(|(mut physics, mut arena, time_warp)| {
			let warped_tick_rate = SECONDS_PER_TICK * time_warp.time_dilation;
			physics.engine.update(warped_tick_rate);
		});
}
