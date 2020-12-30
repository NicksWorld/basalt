use ::serde::Deserialize;
use ::std::{
	fs::File,
	io::{Read, Result},
	path::Path,
};

#[derive(Clone, Deserialize)]
pub struct Authentication {
	pub legacy: AuthenticationMethod,
	pub microsoft: AuthenticationMethod,
	pub required: bool,
	pub yggdrasil: AuthenticationMethod,
}

#[derive(Clone, Deserialize)]
pub struct AuthenticationMethod {
	pub enabled: bool,
	pub url: String,
}

#[derive(Clone, Deserialize)]
pub struct Config {
	pub authentication: Authentication,
	pub minecraft: Minecraft,
	pub network: Network,
	pub whitelist: Whitelist,
}

#[derive(Clone, Deserialize)]
pub struct Minecraft {
	pub max_players: i32,
	pub motd: String,
}

#[derive(Clone, Deserialize)]
pub struct Network {
	pub bind: String,
	pub port: u16,
}

#[derive(Clone, Deserialize)]
pub struct Whitelist {
	pub enabled: bool,
	pub list: Vec<String>,
}

impl Config {
	pub async fn read<P: AsRef<Path>>(path: P) -> Result<Config> {
		let mut config = File::open(path)?;
		let mut buffer = String::new();
		config.read_to_string(&mut buffer)?;
		Ok(::toml::from_str(&buffer)?)
	}
}
