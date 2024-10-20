use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
	pub listens: Vec<Listen>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Listen {
	UdpIpv4 { bind_address: String, port: u16 },
}

impl ServerConfig {
	pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
		toml::from_str(toml_str)
	}
}
