use bevy::prelude::*;

use clap::Args;

use crate::network::DEFAULT_IDENTITY_SERVER_PORT;

mod database;

pub fn run(args: &IdentServerArgs) {
	println!("Running Carplexity identity server");

	// TODO
	std::process::exit(0);
}

#[derive(Args)]
pub struct IdentServerArgs {
	#[arg(long, default_value = "127.0.0.1")]
	bind_address: String,
	#[arg(long, default_value_t = DEFAULT_IDENTITY_SERVER_PORT)]
	port: u16,
	#[arg(long, default_value = ".")]
	data_dir: String,
}
