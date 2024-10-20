use crate::{
	gamemode::{Gamemode, GamemodeData},
	id::Id,
	version,
};

#[derive(Debug, Clone)]
pub struct Driveball {
	blue_score: u32,
	red_score: u32,
}

impl GamemodeData for Driveball {}

impl Driveball {
	pub fn new() -> Self {
		Self {
			blue_score: 0,
			red_score: 0,
		}
	}
}

pub fn new_driveball() -> Gamemode {
	Gamemode {
		name: String::from("Driveball"),
		id: Id::new(0),
		version: version::Version::from_str("0.1.0"),
		api_version: version::Version::from_str("0.1.0"),
		data: Box::new(Driveball::new()),
	}
}
