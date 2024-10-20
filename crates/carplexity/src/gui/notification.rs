use bevy::prelude::*;

#[derive(Component)]
pub enum Notification {
	Progress {
		progress: f32,
		message: String,
	},
	Message {
		message: String,
	},
}
