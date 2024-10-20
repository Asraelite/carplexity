// Idea: Triangle + L-stick = move focus point of car cam.
// Double-tap triangle to reset focus point to forward vector.

// Idea: T or Ctrl+T etc. to instantly go to training, or back to the previous room if already in training.

// Idea: Quick chat / commands with D-pad + right analog stick in a wheel selector. Players can configure what and how many items are in the wheel.

// Idea:
// Hold X to start flipping, release it to stop. Tapping X results in just a very short flip.
// The acceleration curve grows rapidly at first, then slows down rapidly but stays positive until maybe 200ms or so. Then you only have your momentum, but no more acceleration.
// This means that you get the maximum possible acceleration and movement out of a flip if
// L1 in the air is for stalling. It provides continuous thrust out from the bottom of the car, biased towards the direction of the ground.
// R1 is for boosting.
// Square and circle are for air roll left and right, since L1 can't be used for free air roll anymore.

// Idea:
// A "shift center of mass" button that toggles between your vehicle's center of mass being near the back vs. the front.
// This could be used for faster flicks if timed right.
// Maybe L3 or R3.

// Idea:
// Custom quickchats. Same mechanism as the other wheel selector.

use std::collections::HashMap;

use bevy::prelude::*;
use vehicle_control::VehicleControllerInput;

mod scheme;
mod vehicle_control;

pub fn list_gamepads(gamepads: Res<Gamepads>) {
	println!("Currently connected gamepads:");
	for gamepad in gamepads.iter() {
		println!(
			"ID: {:?}; Name: {}",
			gamepad,
			gamepads.name(gamepad).unwrap_or("unknown")
		);
	}
}

#[derive(Component)]
pub struct FollowCamera {
	pub target: Entity,
	pub distance: f32,
	pub pitch: f32,
	pub yaw: f32,
	pub lerp_speed: f32,
}

impl Default for FollowCamera {
	fn default() -> Self {
		Self {
			target: Entity::PLACEHOLDER,
			distance: 10.0,
			pitch: 0.3,
			yaw: 0.0,
			lerp_speed: 5.0,
		}
	}
}

pub struct ControllerInput {
	pub tick: u64,
	pub vehicle_controller: VehicleControllerInput,
}

pub struct InputQueue {
	pub inputs: Vec<(u64, ControllerInput)>,
}

/// The source of input for a given player.
///
/// There is a many-to-many relationship between players and input sources. For example, a single player using both a gamepad and keyboard/mouse, or two players using a single gamepad each.
#[derive(Component)]
pub enum InputSource {
	Gamepad(Gamepad),
	Keyboard,
	Bot, // TODO
}

pub fn update_follow_camera(
	mut query: Query<(&mut Transform, &FollowCamera)>,
	target_query: Query<&Transform, Without<FollowCamera>>,
	time: Res<Time>,
) {
	for (mut camera_transform, follow_camera) in query.iter_mut() {
		if let Ok(target_transform) = target_query.get(follow_camera.target) {
			let target_position = target_transform.translation;

			let offset = Vec3::new(
				follow_camera.distance * follow_camera.yaw.cos() * follow_camera.pitch.cos(),
				follow_camera.distance * follow_camera.pitch.sin(),
				follow_camera.distance * follow_camera.yaw.sin() * follow_camera.pitch.cos(),
			);

			let desired_position = target_position + offset;

			camera_transform.translation = camera_transform.translation.lerp(
				desired_position,
				follow_camera.lerp_speed * time.delta_seconds(),
			);

			camera_transform.look_at(target_position, Vec3::Y);
		} else {
			warn!("FollowCamera target not found");
		}
	}
}

pub fn handle_input(
	mut camera_query: Query<(&mut Transform, &mut FollowCamera)>,
	gamepads: Res<Gamepads>,
	time: Res<Time>,
) {
	// for (mut camera_transform, mut follow_camera) in camera_query.iter_mut() {
	// 	if let Ok(gamepad) = gamepads.iter() {
	// 		let right_stick = gamepad.right_stick();

	// 		let yaw_input = right_stick.x;
	// 		let pitch_input = -right_stick.y;

	// 		follow_camera.yaw += yaw_input * time.delta_seconds();
	// 		follow_camera.pitch += pitch_input * time.delta_seconds();

	// 		follow_camera.pitch = follow_camera
	// 			.pitch
	// 			.clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
	// 	}
	// }
}

pub fn control_cars(
	gamepads: Res<Gamepads>,
	button_inputs: Res<ButtonInput<GamepadButton>>,
	button_axes: Res<Axis<GamepadButton>>,
	axes: Res<Axis<GamepadAxis>>,
) {
	for gamepad in gamepads.iter() {
		if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
			info!("{:?} just pressed South", gamepad);
		} else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South))
		{
			info!("{:?} just released South", gamepad);
		}

		let right_trigger = button_axes
			.get(GamepadButton::new(
				gamepad,
				GamepadButtonType::RightTrigger2,
			))
			.unwrap();
		if right_trigger.abs() > 0.01 {
			info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
		}

		let left_stick_x = axes
			.get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
			.unwrap();
		if left_stick_x.abs() > 0.01 {
			info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
		}
	}
}

pub fn set_up_input(app: &mut App) {
	app.add_systems(Update, handle_input);
	app.add_systems(Update, control_cars);
	app.add_systems(Update, update_follow_camera);
}
