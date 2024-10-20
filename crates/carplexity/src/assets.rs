// Game data directory structure:
// carplexity/
//	packages/
//		<package hash>-<package name>-<package version>/
//			carplexity_module.toml
// 			ace_1.aac
//			pisa_diffuse_rgb9e5_zstd.ktx2
//			pisa_specular_rgb9e5_zstd.ktx2
//		<package-hash>-<package-name>-<package-version>.zip
//	local_packages/
//		<package-name>/
//			carplexity_module.toml
//			wip_asset.png

// The install directory would look the same, so there are packages in the install directory and in the data directory.
// The packages in `local_packages/` do not undergo hash checks and their contents can be modified at runtime.

// Idea:
// Unnecessary but useful assets e.g. common community gamemodes, music etc. are not included in the binary. They can be downloaded at runtime and stored in the data directory, or they can be bundled with the binary in the build script using some flag or option.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SvgData {
	pub data: Vec<u8>,
}
