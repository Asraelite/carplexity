use bevy::prelude::*;

pub const RED: Color = Color::srgb(1.0, 0.0, 0.0);
pub const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
pub const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
pub const YELLOW: Color = Color::srgb(1.0, 1.0, 0.0);
pub const ORANGE: Color = Color::srgb(1.0, 0.5, 0.0);
pub const PURPLE: Color = Color::srgb(0.5, 0.0, 0.5);
pub const CYAN: Color = Color::srgb(0.0, 1.0, 1.0);
pub const MAGENTA: Color = Color::srgb(1.0, 0.0, 1.0);
pub const GRAY: Color = Color::srgb(0.5, 0.5, 0.5);

pub fn colors_conflict(a: Color, b: Color, tolerance: f32) -> bool {
	oklab_color_distance(a, b).abs() <= tolerance
}

// The Oklab color space provides a more perceptually uniform color distance metric than sRGB.
fn oklab_color_distance(a: Color, b: Color) -> f32 {
	let a_oklab: Oklaba = a.into();
	let b_oklab: Oklaba = b.into();
	((a_oklab.lightness - b_oklab.lightness).powi(2)
		+ (a_oklab.a - b_oklab.a).powi(2)
		+ (a_oklab.b - b_oklab.b).powi(2))
	.sqrt()
}
