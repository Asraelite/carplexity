use crate::physics::{PhysicsEngine, RapierPhysics};

pub enum PhysicsConfig {
	Rapier,
}

impl PhysicsConfig {
	pub fn to_engine(&self) -> Box<dyn PhysicsEngine> {
		match self {
			PhysicsConfig::Rapier => Box::new(RapierPhysics::new()),
		}
	}
}
