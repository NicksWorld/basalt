use ::std::error::Error;

use crate::config::Config;

mod legacy;
mod microsoft;
mod yggdrasil;

use legacy::Legacy;
use microsoft::Microsoft;
use yggdrasil::Yggdrasil;

pub struct Authentication {
	legacy: Option<Legacy>,
	microsoft: Option<Microsoft>,
	yggdrasil: Option<Yggdrasil>,
}

impl Authentication {
	pub async fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
		let legacy = if config.authentication.legacy.enabled {
			Some(Legacy::new(config).await?)
		} else {
			None
		};
		let microsoft = if config.authentication.microsoft.enabled {
			Some(Microsoft::new(config).await?)
		} else {
			None
		};
		let yggdrasil = if config.authentication.yggdrasil.enabled {
			Some(Yggdrasil::new(config).await?)
		} else {
			None
		};
		Ok(Authentication {
			legacy,
			microsoft,
			yggdrasil,
		})
	}
}
