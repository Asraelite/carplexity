use crate::version;
use crate::id::Id;

pub mod builtin;
pub mod physics;

#[derive(Debug)]
pub struct Gamemode {
	name: String,
	id: Id<Self>,
	version: version::Version,
	api_version: version::Version,
	data: Box<dyn GamemodeData>,
}

// pub struct GamemodeBuilder<T: GamemodeData> {

// }

// impl GamemodeBuilder {
	
// }

pub trait GamemodeData: Send + std::fmt::Debug {}

pub enum Classification {
	/// One of the primary gamemodes of Carplexity.
	Standard,
	/// Less popular, non-standard gamemodes that are built into the game by default.
	Official,
	/// Unofficial gamemodes that were created by users.
	Custom
}

// Gamemode name ideas:
// Boots
// VWater Polo
// Soccar
// Gasketball
// Corbit (you're flying in space)
// Coldavoider
