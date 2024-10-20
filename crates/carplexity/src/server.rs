use bevy::prelude::*;
use clap::Args;

use crate::{network::DEFAULT_GAME_SERVER_PORT, Cli};

mod config;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
	fn build(&self, app: &mut App) {}
}

pub fn run(args: &ServerArgs) {
	println!("Running Carplexity server");

	// TODO
	std::process::exit(0);
}

#[derive(Args)]
pub struct ServerArgs {
	#[arg(long, default_value = "127.0.0.1")]
	bind_address: String,
	#[arg(long, default_value_t = DEFAULT_GAME_SERVER_PORT)]
	port: u16,
	#[arg(long, default_value = ".")]
	data_dir: String,
}
