use bevy::{
	diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
	prelude::*,
};

#[derive(Component)]
struct FpsText;

fn create_text(mut commands: Commands) {
	let text_fps = commands
		.spawn((
			FpsText,
			TextBundle {
				text: Text::from_sections([
					TextSection {
						value: "FPS: ".into(),
						style: TextStyle {
							font_size: 16.0,
							color: Color::WHITE,
							..default()
						},
					},
					TextSection {
						value: " N/A".into(),
						style: TextStyle {
							font_size: 16.0,
							color: Color::WHITE,
							..default()
						},
					},
				]),
				..Default::default()
			},
		))
		.id();
}

fn update_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
	for mut text in &mut query {
		if let Some(value) = diagnostics
			.get(&FrameTimeDiagnosticsPlugin::FPS)
			.and_then(|fps| fps.smoothed())
		{
			text.sections[1].value = format!("{value:>4.0}");

			text.sections[1].style.color = if value >= 120.0 {
				Color::srgb(0.0, 1.0, 0.0)
			} else if value >= 60.0 {
				Color::srgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
			} else if value >= 30.0 {
				Color::srgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
			} else {
				Color::srgb(1.0, 0.0, 0.0)
			}
		} else {
			text.sections[1].value = " N/A".into();
			text.sections[1].style.color = Color::WHITE;
		}
	}
}

fn toggle_fps_counter_visibility(
	mut fps_text: Query<&mut Visibility, With<FpsText>>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
) {
	if keyboard_input.just_pressed(KeyCode::F12) {
		let mut visibility = fps_text.single_mut();
		*visibility = match *visibility {
			Visibility::Hidden => Visibility::Visible,
			_ => Visibility::Hidden,
		};
	}
}

pub fn set_up(app: &mut App) {
	app.add_systems(Startup, create_text)
		.add_systems(Update, update_fps)
		.add_systems(Update, toggle_fps_counter_visibility);
}
