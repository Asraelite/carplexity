use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct VehicleControlInputState {
	pub throttle: f32,
	pub reverse: f32,
	pub handbrake: bool,
	pub flip: bool,
	pub stall: bool,
	pub boost: bool,
	pub left_axis_x: f32,
	pub left_axis_y: f32,
	pub right_axis_x: f32,
	pub right_axis_y: f32,
}

impl VehicleControlInputState {
	pub fn from_controller_input() -> Self {
		Self::default()
	}
}

impl Default for VehicleControlInputState {
	fn default() -> Self {
		Self {
			throttle: 0.0,
			reverse: 0.0,
			handbrake: false,
			flip: false,
			stall: false,
			boost: false,
			left_axis_x: 0.0,
			left_axis_y: 0.0,
			right_axis_x: 0.0,
			right_axis_y: 0.0,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VehicleControlAction {
	Jump,
}

#[derive(Component)]
pub struct VehicleControl {
	pub input_state: VehicleControlInputState,
}

impl VehicleControl {
	pub fn new() -> VehicleControl {
		VehicleControl {
			input_state: VehicleControlInputState::default(),
		}
	}
}
