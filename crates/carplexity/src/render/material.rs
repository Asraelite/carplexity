use bevy::{
	prelude::*,
	render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
	#[uniform(0)]
	color: LinearRgba,
	#[texture(1)]
	#[sampler(2)]
	color_texture: Option<Handle<Image>>,
	alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn alpha_mode(&self) -> AlphaMode {
		self.alpha_mode
	}
}


// Usage:
//
// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<CustomMaterial>>,
//     asset_server: Res<AssetServer>,
// ) {
//     // cube
//     commands.spawn(MaterialMeshBundle {
//         mesh: meshes.add(Cuboid::default()),
//         transform: Transform::from_xyz(0.0, 0.5, 0.0),
//         material: materials.add(CustomMaterial {
//             color: LinearRgba::BLUE,
//             color_texture: Some(asset_server.load("branding/icon.png")),
//             alpha_mode: AlphaMode::Blend,
//         }),
//         ..default()
//     });
