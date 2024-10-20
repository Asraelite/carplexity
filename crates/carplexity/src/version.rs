mod versions {
	use super::Version;

	pub const V0_1_0: Version = Version::from_semver(0, 1, 0);

	pub const LATEST: Version = Version::from_str(env!("CARGO_PKG_VERSION"));
}
pub use versions::*;

#[derive(Debug)]
pub enum Version {
	SemVer(u32, u32, u32),
}

impl Version {
	pub const fn from_semver(major: u32, minor: u32, patch: u32) -> Self {
		Self::SemVer(major, minor, patch)
	}

	pub const fn from_str(version_str: &str) -> Self {
		let mut major = 0;
		let mut minor = 0;
		let mut patch = 0;
		let mut part = 0;
		let mut current = 0;

		let mut i = 0;
		while i < version_str.len() {
			let c = version_str.as_bytes()[i];
			if c == b'.' {
				match part {
					0 => major = current,
					1 => minor = current,
					_ => break,
				}
				part += 1;
				current = 0;
			} else if c >= b'0' && c <= b'9' {
				current = current * 10 + (c - b'0') as u32;
			} else {
				break;
			}
			i += 1;
		}

		if part == 2 {
			patch = current;
		}

		Self::SemVer(major, minor, patch)
	}
}
