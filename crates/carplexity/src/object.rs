use bevy::prelude::*;

mod energy_source;

#[derive(Component)]
pub struct DynamicObject {}

#[derive(Component)]
pub struct StaticObject {}

#[derive(Bundle)]
pub struct StaticObjectBundle {
	pub static_object: StaticObject,
	pub shape: Hitbox,
	pub model: Model,
}

impl StaticObjectBundle {
	pub fn from_mesh(mesh: Handle<Mesh>) -> Self {
		Self {
			static_object: StaticObject {},
			shape: Hitbox::Mesh(mesh.clone()),
			model: Model {
				mesh,
				material: Handle::default(),
			},
		}
	}
}

#[derive(Component)]
pub enum Hitbox {
	Mesh(Handle<Mesh>),
	/// Radius in game units.
	Sphere(f32),
}

#[derive(Component)]
pub struct Model {
	pub mesh: Handle<Mesh>,
	pub material: Handle<StandardMaterial>,
}
