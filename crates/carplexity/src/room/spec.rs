use super::module::Module;

pub struct ArenaSpec {
	base_width: f32,
	base_height: f32,
	modules: Vec<Module>,
}

impl ArenaSpec {
	pub fn new(base_width: f32, base_height: f32) -> Self {
		Self {
			base_width,
			base_height,
			modules: Vec::new(),
		}
	}
}
