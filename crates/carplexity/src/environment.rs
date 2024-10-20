use entity::Entity;

use crate::gamemode::Gamemode;

mod entity;
pub mod terrain;

#[derive(Debug, Clone, Copy)]
pub struct Seed(pub u128);

pub struct Environment {
	pub gamemode: Option<Gamemode>,
	pub entities: Vec<Entity>,
}

impl Environment {
	pub fn new() -> Self {
		Self { gamemode: None, entities: Vec::new() }
	}

	pub fn with_gamemode(mut self, gamemode: Gamemode) -> Self {
		self.gamemode = Some(gamemode);
		self
	}
}
