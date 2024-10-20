// Idea:
// To fix the problem of missing boost like in Rocket League, don't drain pills instantly. Cars must maintain contact with them for a short duration to fully deplete them.

use bevy::prelude::*;

use super::StaticObjectBundle;

#[derive(Component)]
pub enum EnergySource {
	Pill {
		energy: f32,
		recharge_rate: f32,
		radius: f32,
	},
}

#[derive(Bundle)]
pub struct EnergySourceBundle {
	pub energy_source: EnergySource,
	pub static_object: StaticObjectBundle,
}
