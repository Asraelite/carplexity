use bevy::prelude::*;

mod notification;

pub struct ColorTheme {
	pub background: Color,
	pub foreground: Color,
	pub primary: Color,
	pub secondary: Color,
	pub tertiary: Color,
}

impl Default for ColorTheme {
	fn default() -> Self {
		Self {
			background: Color::srgb(0.21, 0.22, 0.25),
			foreground: Color::srgb(0.36, 0.47, 0.43),
			primary: Color::srgb(0.49, 0.63, 0.58),
			secondary: Color::srgb(0.67, 0.78, 0.74),
			tertiary: Color::srgb(0.92, 0.96, 0.96),
		}
	}
}
