use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Result};
use std::fs;
use std::path::PathBuf;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Config {
	pub camera_settings: CameraSettings,
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct CameraSettings {
	pub fov: f32,
}

impl Config {
	pub fn new() -> Self {
		Config {
			camera_settings: CameraSettings { fov: 75.0 }, // Default FOV
		}
	}

	pub async fn load() -> Result<Self> {
		let pool = SqlitePool::connect(&format!(
			"sqlite:{}",
			get_config_file_path().to_str().unwrap()
		))
		.await?;

		sqlx::query(
			"CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
		)
		.execute(&pool)
		.await?;

		let fov: f32 = sqlx::query_scalar("SELECT value FROM config WHERE key = 'camera_fov'")
			.fetch_optional(&pool)
			.await?
			.and_then(|value: String| value.parse().ok())
			.unwrap_or(75.0);

		Ok(Config {
			camera_settings: CameraSettings { fov },
		})
	}

	pub async fn save(&self) -> Result<()> {
		let pool = SqlitePool::connect(&format!(
			"sqlite:{}",
			get_config_file_path().to_str().unwrap()
		))
		.await?;

		sqlx::query("INSERT OR REPLACE INTO config (key, value) VALUES ('camera_fov', ?)")
			.bind(self.camera_settings.fov.to_string())
			.execute(&pool)
			.await?;

		Ok(())
	}
}

pub fn get_config_dir() -> PathBuf {
	let config_dir = if cfg!(target_os = "windows") {
		let mut path =
			PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| "C:\\".to_string()));
		path.push("carplexity");
		path
	} else if cfg!(target_os = "macos") {
		let mut path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/".to_string()));
		path.push("Library");
		path.push("Application Support");
		path.push("carplexity");
		path
	} else {
		let mut path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/".to_string()));
		path.push(".config");
		path.push("carplexity");
		path
	};

	if !config_dir.exists() {
		fs::create_dir_all(&config_dir).expect("Failed to create config directory");
	}

	config_dir
}

fn get_config_file_path() -> PathBuf {
	let mut path = get_config_dir();
	path.push("config.db");
	path
}
