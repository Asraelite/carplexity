#![allow(unused)]

use std::time::Duration;

use bevy::{asset::LoadState, prelude::*, window::WindowMode, winit::WinitWindows};
use winit::window::{Icon, WindowId};

use clap::{Args, Parser, Subcommand};
use gameplay::GameplayPlugin;
use physics::{Collider, Physics, PhysicsPlugin};
use room::Room;

mod account;
mod assets;
mod bot;
mod config;
mod crypto;
mod debug;
mod environment;
mod gamemode;
mod gameplay;
mod gui;
mod id;
mod ident_server;
mod input;
mod network;
mod package;
mod physics;
mod player;
mod prelude;
mod render;
mod room;
mod server;
mod setup;
mod vehicle;
mod version;
mod wasm;

const TICKS_PER_SECOND: u32 = 240;
const SECONDS_PER_TICK: f32 = 1.0 / TICKS_PER_SECOND as f32;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
	#[command(subcommand)]
	subcommand: Option<CliSubcommand>,
}

#[derive(Subcommand)]
enum CliSubcommand {
	Server(server::ServerArgs),
	IdentServer(ident_server::IdentServerArgs),
}

fn main() {
	let cli = Cli::parse();

	match &cli.subcommand {
		Some(CliSubcommand::Server(args)) => {
			server::run(args);
		}
		Some(CliSubcommand::IdentServer(args)) => {
			ident_server::run(args);
		},
		_ => {
			run_client();
		}
	}
}

fn run_client() {
	println!("Running Carplexity client");

	let mut app = App::new();
	app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
		SECONDS_PER_TICK,
	)))
	.add_plugins(DefaultPlugins.set(WindowPlugin {
		primary_window: Some(Window {
			present_mode: bevy::window::PresentMode::AutoNoVsync,
			title: "Carplexity".to_string(),
			name: Some("Carplexity".to_string()),
			..default()
		}),
		..default()
	}))
	.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
	.add_plugins(GameplayPlugin)
	.add_plugins(PhysicsPlugin)
	.add_systems(Startup, room::create_room)
	.add_systems(Startup, vehicle::create_vehicle)
	.add_systems(Startup, set_window_icon)
	.add_systems(Update, environment_map_load_finish)
	.add_systems(Update, toggle_fullscreen);

	input::set_up_input(&mut app);
	debug::set_up(&mut app);

	app.run();
}

fn environment_map_load_finish(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	environment_maps: Query<&EnvironmentMapLight>,
	label_query: Query<Entity, With<EnvironmentMapLabel>>,
) {
	if let Ok(environment_map) = environment_maps.get_single() {
		if asset_server.load_state(&environment_map.diffuse_map) == LoadState::Loaded
			&& asset_server.load_state(&environment_map.specular_map) == LoadState::Loaded
		{
			if let Ok(label_entity) = label_query.get_single() {
				commands.entity(label_entity).despawn();
			}
		}
	}
}

#[derive(Component)]
struct EnvironmentMapLabel;

fn toggle_fullscreen(keyboard_input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
	if keyboard_input.just_pressed(KeyCode::F11) {
		if let Ok(mut window) = windows.get_single_mut() {
			window.mode = match window.mode {
				WindowMode::Windowed => WindowMode::BorderlessFullscreen,
				_ => WindowMode::Windowed,
			};
		}
	}
}

// TODO: Doesn't work
fn set_window_icon(windows: NonSend<WinitWindows>) {
	let (icon_rgba, icon_width, icon_height) = {
		let image = image::open("assets/icon_2.png")
			.expect("Failed to open icon path")
			.into_rgba8();
		let (width, height) = image.dimensions();
		let rgba = image.into_raw();
		(rgba, width, height)
	};
	let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

	for window in windows.windows.values() {
		window.set_window_icon(Some(icon.clone()));
	}
}
