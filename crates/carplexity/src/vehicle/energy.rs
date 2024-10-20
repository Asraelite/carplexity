use bevy::prelude::*;

#[derive(Component)]
pub struct Energy {
	pub current: f32,
	pub max: f32,
}
