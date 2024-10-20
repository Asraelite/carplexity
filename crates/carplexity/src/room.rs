use std::collections::HashSet;

use bevy::prelude::*;

use crate::{gamemode::physics::PhysicsConfig, physics::Physics, vehicle::Vehicle};

mod module;
mod spec;
mod state;

#[derive(Component)]
pub struct Room {}

impl Room {
	pub fn new() -> Room {
		Room {}
	}
}

#[derive(Bundle)]
pub struct RoomBundle {
	pub room: Room,
	pub physics: Physics,
}

impl Default for RoomBundle {
	fn default() -> Self {
		RoomBundle {
			room: Room::new(),
			physics: Physics {
				engine: PhysicsConfig::Rapier.to_engine(),
				colliders: HashSet::new(),
			},
		}
	}
}

pub fn create_room(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
) {
	let entity = commands
		.spawn((
			PbrBundle {
				mesh: meshes.add(bevy::prelude::Plane3d::default().mesh().size(20.0, 20.0)),
				material: materials.add(bevy::color::Color::srgb(0.3, 0.5, 0.3)),
				transform: Transform::from_xyz(0.0, -2.5, 0.0),
				..default()
			},
			// RigidBody::Fixed,
			// Collider::cuboid(10.0, 0.1, 10.0),
		))
		.id();

	let room_width = 20.0;
	let room_length = 30.0;
	let room_height = 10.0;
	let wall_thickness = 0.1;

	commands.spawn((
		PbrBundle {
			mesh: meshes.add(Cuboid::new(room_width, room_height, room_length)),
			material: materials.add(Color::srgb(0.8, 0.8, 0.8)),
			transform: Transform::from_xyz(0.0, -room_width / 2.0, 0.0),
			..default()
		},
		// RigidBody::Fixed,
		// Collider::cuboid(room_width / 2.0, room_height / 2.0, room_length / 2.0),
	));

	for (rotation, translation) in [
		(
			Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
			Vec3::new(0.0, 0.0, -room_width / 2.0),
		),
		(
			Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
			Vec3::new(0.0, 0.0, room_width / 2.0),
		),
		(
			Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
			Vec3::new(-room_width / 2.0, 0.0, 0.0),
		),
		(
			Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
			Vec3::new(room_width / 2.0, 0.0, 0.0),
		),
	] {
		commands.spawn((
			PbrBundle {
				mesh: meshes.add(Cuboid::new(room_width, room_height, room_length)),
				material: materials.add(Color::srgb(0.8, 0.8, 0.8)),
				transform: Transform::from_xyz(translation.x, translation.y, translation.z)
					.with_rotation(rotation),
				..default()
			},
			// RigidBody::Fixed,
			// Collider::cuboid(room_width / 2.0, room_height / 2.0, room_length / 2.0),
		));
	}

	let car = Vehicle::new();
}

fn camera_with_parent(
	q_child: Query<(&Parent, &Transform), With<Camera>>,
	q_parent: Query<&GlobalTransform>,
) {
	for (parent, child_transform) in q_child.iter() {
		// `parent` contains the Entity ID we can use
		// to query components from the parent:
		let parent_global_transform = q_parent.get(parent.get());

		// do something with the components
	}
}
