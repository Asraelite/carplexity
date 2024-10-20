use std::hash::Hasher;

#[derive(Debug, Clone)]
pub struct SimpleHasher {
	state: u64,
}

impl SimpleHasher {
	pub const fn new() -> Self {
		Self { state: 0 }
	}

	pub const fn updated(self, bytes: &[u8]) -> Self {
		let mut i = 0;
		let mut state = self.state;
		while i < bytes.len() {
			state = state.wrapping_mul(31).wrapping_add(bytes[i] as u64);
			i += 1;
		}
		Self { state }
	}

	pub fn update_str(&mut self, s: &str) -> &mut Self {
		*self = self.clone().updated(s.as_bytes());
		self
	}

	pub const fn finish(&self) -> u64 {
		self.state
	}
}

pub const fn hash_str(s: &str) -> u64 {
	let mut hasher = SimpleHasher::new();
	let mut i = 0;
	hasher = hasher.updated(s.as_bytes());
	hasher.finish()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hash_str() {
		const HASH1: u64 = hash_str("hello");
		const HASH2: u64 = hash_str("world");

		assert_ne!(HASH1, HASH2);
		assert_eq!(HASH1, hash_str("hello"));
		assert_eq!(HASH2, hash_str("world"));
	}
}
