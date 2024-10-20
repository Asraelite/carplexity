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
			.add_systems(FixedUpdate, run_room_physics.after(update_time_warp))
			.add_systems(Update, update_transforms.after(run_room_physics));
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

/// The source of truth for the physics state.
#[derive(Component)]
pub enum Authority {
	/// Normal, offline play.
	Local,
	/// Connected to a game server.
	Remote,
	/// Watching a replay.
	Replay,
}

#[derive(Component)]
pub struct PhysicalState {
	pub position: Vec3,
	pub velocity: Vec3,
	pub rotation: Quat,
	pub angular_velocity: Vec3,
}

#[derive(Component)]
pub struct PhysicalStateHistory {
	pub states: Vec<(u64, PhysicalState)>,
}

#[derive(Component)]
pub struct PositionHistory {
	pub positions: Vec<Vec3>,
}

pub trait PhysicsEngine: Send + Sync {
	// type State: Send + Sync + Clone;

	fn update(&mut self, delta_time: f32) -> Result<(), String>;
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

impl Default for TimeWarp {
	fn default() -> Self {
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
			Authority::Local | Authority::Replay => {
				time_warp.target_lag = 0.0;
				time_warp.time_dilation = 1.0;
			}
			Authority::Remote => {
				// TODO: Calculate
			}
		}
	}
}

fn update_transforms(
	fixed_time: Res<Time<Fixed>>,
	mut physical_state_query: Query<(
		&PhysicalState,
		&PhysicalStateHistory,
		Option<&Parent>,
		&mut Transform,
	)>,
	room_query: Query<(&Room, &Physics, &TimeWarp, &Authority)>,
) {
	for (physical_state, physical_state_history, parent, mut transform) in &mut physical_state_query
	{
		// TODO: Use interpolation instead of extrapolation if the authority is replay.
		let ticks_ahead = fixed_time.overstep_fraction();
		let time_dilation = parent
			.and_then(|parent| room_query.get(parent.get()).ok())
			.map_or(1.0, |(_, _, time_warp, _)| time_warp.time_dilation);
		let delta_seconds = fixed_time.delta_seconds() * time_dilation;

		let future_position = physical_state.position + physical_state.velocity * delta_seconds;
		transform.translation = physical_state.position.lerp(future_position, ticks_ahead);

		let future_rotation = physical_state.rotation
			* Quat::from_axis_angle(
				physical_state.angular_velocity.normalize(),
				physical_state.angular_velocity.length() * delta_seconds,
			);
		transform.rotation = physical_state.rotation.slerp(future_rotation, ticks_ahead);
	}
}

fn run_room_physics(
	fixed_time: Res<Time<Fixed>>,
	mut room_query: Query<(&mut Physics, &Room, &TimeWarp, &Authority)>,
) {
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
