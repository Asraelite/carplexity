use bevy::prelude::*;

#[derive(Component)]
pub struct Frame {
	pub tick: u64,
	// TODO: Add list of events, including creation/destruction of colliders
}
