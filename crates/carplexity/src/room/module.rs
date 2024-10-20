use bevy::prelude::*;

pub struct Orientation {
	y_rotation: u8,
	x_rotation: u8,
}

pub enum Module {
	Ramp,
	Block,
}

// impl Module {
// 	pub fn to_mesh(&self) -> Mesh {
// 		Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
//     .with_inserted_attribute(
// 			Mesh::ATTRIBUTE_POSITION,
// 			vec![
// 				            // top (facing towards +y)
// 							[-0.5, 0.5, -0.5], // vertex with index 0
// 							[0.5, 0.5, -0.5], // vertex with index 1
// 							[0.5, 0.5, 0.5], // etc. until 23
// 							[-0.5, 0.5, 0.5],
// 							// bottom   (-y)
// 							[-0.5, -0.5, -0.5],
// 							[0.5, -0.5, -0.5],
// 							[0.5, -0.5, 0.5],
// 							[-0.5, -0.5, 0.5],
// 							// right    (+x)
// 							[0.5, -0.5, -0.5],
// 							[0.5, -0.5, 0.5],
// 							[0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
// 							[0.5, 0.5, -0.5],
// 							// left     (-x)
// 							[-0.5, -0.5, -0.5],
// 							[-0.5, -0.5, 0.5],
// 							[-0.5, 0.5, 0.5],
// 							[-0.5, 0.5, -0.5],
// 							// back     (+z)
// 							[-0.5, -0.5, 0.5],
// 							[-0.5, 0.5, 0.5],
// 							[0.5, 0.5, 0.5],
// 							[0.5, -0.5, 0.5],
// 							// forward  (-z)
// 							[-0.5, -0.5, -0.5],
// 							[-0.5, 0.5, -0.5],
// 							[0.5, 0.5, -0.5],
// 							[0.5, -0.5, -0.5],
// 			],
// 		)
// 	}
// }
