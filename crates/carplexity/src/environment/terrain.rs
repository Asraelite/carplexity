use noise::{Fbm, MultiFractal as _, NoiseFn, Simplex};
use rapier3d::na::{DMatrix, Vector3};

use super::Seed;

pub fn sample_height(seed: Seed, x: f64, y: f64) -> f64 {
	let simplex = Simplex::new(seed.0 as u32);
	let fbm = Fbm::<Simplex>::new(seed.0 as u32)
		.set_octaves(4)
		.set_frequency(0.005)
		.set_lacunarity(2.5)
		.set_persistence(0.5);

	let large_scale = fbm.get([x * 0.01, y * 0.01]);
	let medium_scale = fbm.get([x * 0.05, y * 0.05]) * 0.2;
	let small_scale = fbm.get([x * 0.1, y * 0.1]) * 0.1;

	large_scale + medium_scale + small_scale
}

pub fn create_heightfield(seed: Seed, width: usize, height: usize, scale: f32) -> Vec<f32> {
	let mut heights = vec![0.0; width * height];

	for y in 0..height {
		for x in 0..width {
			let sample_x = x as f64 * scale as f64;
			let sample_y = y as f64 * scale as f64;
			heights[y * width + x] = sample_height(seed, sample_x, sample_y) as f32;
		}
	}

	heights
}
