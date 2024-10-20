use std::default::Default;

use bevy::asset::LoadState;
use bevy::color::Srgba;
use bevy::prelude::*;
use bevy::prelude::{Meshable as _, Transform};
use bevy::render::mesh::PlaneMeshBuilder;

use crate::room::{Room, RoomBundle};
use crate::gamemode::physics::PhysicsConfig;
use crate::input::FollowCamera;
use crate::physics::Physics;
use crate::vehicle::control::VehicleControl;
use crate::vehicle::{self, VehicleBundle};

pub fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
) {
	let arena = commands.spawn(RoomBundle::default()).id();
	// let car = create_vehicle(commands, meshes, materials, asset_server);
	let car = commands.spawn(TransformBundle::default()).id();

	commands.entity(arena).push_children(&[car]);

	commands.spawn(bevy::pbr::DirectionalLightBundle {
		transform: Transform::from_xyz(50.0, 50.0, 50.0)
			.looking_at(bevy::math::Vec3::ZERO, bevy::math::Vec3::Y),
		directional_light: bevy::pbr::DirectionalLight {
			illuminance: 1_500.,
			..default()
		},
		..default()
	});

	commands.spawn((
		Camera3dBundle {
			transform: Transform::from_xyz(6.0, 6.0, 6.0).looking_at(Vec3::default(), Vec3::Y),
			projection: PerspectiveProjection {
				// scale: 0.01,
				..default()
			}
			.into(),
			..default()
		},
		EnvironmentMapLight {
			diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
			specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
			intensity: 900.0,
		},
		FollowCamera {
			target: car,
			..default()
		},
	));
}
