use std::fs;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use toml;

#[derive(Resource)]
pub struct PackageStore {
	packages: HashMap<String, Package>,
}

pub struct Package {
	config: PackageConfig,
}

#[derive(Serialize, Deserialize)]
pub struct PackageConfig {
	name: String,
	version: String,
	dependencies: Vec<PackageResourceRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRef {
	pub package_hash: String,
	pub package_name: String,
	pub package_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageResourceRef {
	pub package_ref: PackageRef,
	pub resource_path: PathBuf,
}

impl PackageConfig {
	pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
		let toml_str = fs::read_to_string(path)?;
		let config: PackageConfig = toml::from_str(&toml_str)?;
		Ok(config)
	}
}

impl Package {
	pub fn new(config: PackageConfig) -> Self {
		Package { config }
	}

	pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
		let config = PackageConfig::from_toml_file(path)?;
		Ok(Package::new(config))
	}
}
