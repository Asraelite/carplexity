use bevy::{ecs::system::SystemId, prelude::*};
use control::VehicleControl;

use crate::physics::Collider;

pub mod control;
mod energy;

#[derive(Resource)]
pub struct VehicleSpawner {
	create_vehicle: SystemId,
}

#[derive(Component)]
pub struct Vehicle {}

#[derive(Bundle)]
pub struct VehicleBundle {
	pub vehicle: Vehicle,
	pub mesh: PbrBundle,
	pub control: VehicleControl,
	pub collider: Collider,
}

impl Vehicle {
	pub fn new() -> Vehicle {
		Vehicle {}
	}
}

pub fn create_vehicle(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((VehicleBundle {
		vehicle: Vehicle::new(),
		mesh: PbrBundle {
			mesh: meshes.add(Cuboid::new(2.0, 1.0, 4.0)),
			material: materials.add(StandardMaterial {
				base_color: Srgba::hex("#ffd891").unwrap().into(),
				metallic: 0.5,
				perceptual_roughness: 0.2,
				..default()
			}),
			transform: Transform::from_xyz(0.0, 0.5, 0.0),
			..default()
		},
		control: VehicleControl::new(),
		collider: Collider::Car,
	},));
}
