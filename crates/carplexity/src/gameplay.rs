// Idea:
// Feedback at the end of matches against random players. Click icons beside a player's name. They could be "this player was toxic", "our play styles work together", "our play styles are incompatible".

// Idea:
// Energy (boost) is collected by driving over boost pads / regions. The faster you are travelling while moving over them, the more energy you gain.
// This encourages players to maintain their momentum while collecting energy.
// It could, for example, shift the defense metagame toward more fast-paced rotations of the defending team.

use bevy::prelude::*;
use bevy::state::prelude::*;

use crate::{input::FollowCamera, room::RoomBundle};

pub struct GameplayPlugin;

mod color;

impl Plugin for GameplayPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			handle_escape_pressed.run_if(in_state(GameState::InGame)),
		)
		.add_systems(Startup, run_startup)
		.add_systems(OnEnter(GameState::TitleScreen), enter_title_screen)
		.add_systems(OnExit(GameState::TitleScreen), exit_title_screen)
		.add_systems(OnEnter(GameState::InGame), test_setup)
		.init_state::<GameState>();
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
	#[default]
	Startup,
	TitleScreen,
	InGame,
	Editor,
}

#[derive(Component)]
pub enum Team {
	Neutral,
	Assigned(u32),
}

#[derive(Component)]
pub struct TeamMetadata {
	pub name: String,
	pub color: Color,
}

impl TeamMetadata {
	pub fn default_from_index(index: u32) -> Self {
		let (name, color) = match index {
			0 => ("Left".to_string(), color::RED),
			1 => ("Right".to_string(), color::BLUE),
			2 => ("Up".to_string(), color::GREEN),
			3 => ("Down".to_string(), color::YELLOW),
			4 => ("Forward".to_string(), color::ORANGE),
			5 => ("Backward".to_string(), color::PURPLE),
			6 => ("Ana".to_string(), color::CYAN),
			7 => ("Kata".to_string(), color::MAGENTA),
			_ => (format!("Team {}", index), color::GRAY),
		};
		Self { name, color }
	}
}

fn run_startup(mut next_state: ResMut<NextState<GameState>>) {
	next_state.set(GameState::TitleScreen);
}

fn handle_escape_pressed(
	mut next_state: ResMut<NextState<GameState>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		next_state.set(GameState::TitleScreen);
	}
}

fn enter_title_screen(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	camera_query: Query<Entity, With<Camera>>,
) {

	for camera_entity in camera_query.iter() {
		commands.entity(camera_entity).despawn_recursive();
	}

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
	));
}

fn exit_title_screen() {}

pub fn test_setup(
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
