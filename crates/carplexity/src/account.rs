use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component)]
pub struct Account {
	pub id: Uuid,
	pub username: String,
	pub display_name: String,
}
