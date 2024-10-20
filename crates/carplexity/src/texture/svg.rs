// Idea:
// Sticker texture files are SVGs.
// They are used to dynamically generate textures at the required resolution at runtime.

use bevy::{
	prelude::*,
	render::{
		render_asset::RenderAssetUsages,
		render_resource::{Extent3d, TextureDimension, TextureFormat},
	},
};
use resvg::{tiny_skia::PixmapMut, usvg};

use crate::assets::SvgData;

pub fn render_svg_to_image(svg: SvgData, width: u32, height: u32) -> Result<Image, String> {
	let tree = usvg::Tree::from_data(&svg.data, &usvg::Options { ..default() })
		.map_err(|e| e.to_string())?;
	let mut pixmap_data = vec![0; (width * height * 4) as usize];
	let mut pixmap =
		PixmapMut::from_bytes(&mut pixmap_data, width, height).ok_or("Failed to create pixmap")?;
	let resvg_image = resvg::render(&tree, usvg::Transform::identity(), &mut pixmap);
	let image = Image::new_fill(
		Extent3d {
			width,
			height,
			depth_or_array_layers: 1,
		},
		TextureDimension::D2,
		pixmap.data_mut(),
		TextureFormat::Rgba8UnormSrgb,
		RenderAssetUsages::RENDER_WORLD,
	);
	Ok(image)
}
